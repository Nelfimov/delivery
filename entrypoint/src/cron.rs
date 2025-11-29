use application::usecases::CommandHandler;
use application::usecases::JobHandler;
use application::usecases::commands::assign_order_command::AssignOrderCommand;
use application::usecases::commands::assign_order_handler::AssignOrderHandler;
use application::usecases::commands::move_couriers_command::MoveCouriersCommand;
use application::usecases::commands::move_couriers_handler::MoveCouriersHandler;
use application::usecases::events::event_bus::EventBus;
use application::usecases::jobs::outbox_job::OutboxJob;
use out_kafka::orders_events_producer::OrdersEventsProducer;
use out_postgres::ConnectionManager;
use out_postgres::PgConnection;
use out_postgres::Pool;
use out_postgres::outbox::outbox_repository::OutboxRepository;
use out_postgres::unit_of_work::UnitOfWork;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use tokio::runtime::Handle;
use tokio::task;
use tokio_cron_scheduler::Job;
use tokio_cron_scheduler::JobScheduler;

pub async fn start_crons(
    pool: Pool<ConnectionManager<PgConnection>>,
    event_bus: impl EventBus + 'static,
    brokers: &str,
    group_id: &str,
) -> JobScheduler {
    let scheduler = JobScheduler::new()
        .await
        .expect("failed to initialize cron scheduler");

    let move_couriers_handler = Arc::new(Mutex::new(MoveCouriersHandler::new(
        UnitOfWork::new(pool.clone()),
        event_bus,
    )));
    let move_couriers_handler_job = Arc::clone(&move_couriers_handler);
    let runtime_handle = Handle::current();
    let move_job_handle = runtime_handle.clone();
    let move_couriers = Job::new_repeated_async(Duration::from_secs(1), move |_uuid, _l| {
        let handler = Arc::clone(&move_couriers_handler_job);
        let handle = move_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let mut handler = match handler.lock() {
                    Ok(handler) => handler,
                    Err(err) => {
                        tracing::error!(
                            error = %err,
                            "move couriers handler mutex poisoned"
                        );
                        return;
                    }
                };

                match MoveCouriersCommand::new() {
                    Ok(command) => {
                        if let Err(err) = handle.block_on(handler.execute(command)) {
                            tracing::warn!(?err, "move couriers job failed");
                        }
                    }
                    Err(err) => tracing::error!(?err, "failed to create move couriers command"),
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "move couriers job task panicked");
            }
        })
    })
    .expect("failed to start move_couriers job");

    scheduler
        .add(move_couriers)
        .await
        .expect("failed to register cron job");

    let assign_order_handler = Arc::new(Mutex::new(AssignOrderHandler::new(UnitOfWork::new(
        pool.clone(),
    ))));
    let assign_order_handler_job = Arc::clone(&assign_order_handler);
    let assign_job_handle = runtime_handle.clone();
    let assign_orders = Job::new_repeated_async(Duration::from_secs(1), move |_uuid, _l| {
        let handler = Arc::clone(&assign_order_handler_job);
        let handle = assign_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let mut handler = match handler.lock() {
                    Ok(handler) => handler,
                    Err(err) => {
                        tracing::error!(
                            error = %err,
                            "assign order handler mutex poisoned"
                        );
                        return;
                    }
                };

                match AssignOrderCommand::new() {
                    Ok(command) => {
                        if let Err(err) = handle.block_on(handler.execute(command)) {
                            tracing::warn!(?err, "assign orders job failed");
                        }
                    }
                    Err(err) => tracing::error!(?err, "failed to create assign order command"),
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "assign orders job task panicked");
            }
        })
    })
    .expect("failed to start assign_order job");

    scheduler
        .add(assign_orders)
        .await
        .expect("failed to register cron job");

    let outbox_job = Arc::new(Mutex::new(OutboxJob::new(
        OutboxRepository::new(pool),
        OrdersEventsProducer::new(brokers, group_id),
    )));
    let outbox_handler_job = Arc::clone(&outbox_job);
    let outbox_job_handle = runtime_handle.clone();
    let outbox = Job::new_repeated_async(Duration::from_secs(10), move |_uuid, _l| {
        let handler = Arc::clone(&outbox_handler_job);
        let handle = outbox_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let mut handler = match handler.lock() {
                    Ok(handler) => handler,
                    Err(err) => {
                        tracing::error!(
                            error = %err,
                            "assign order handler mutex poisoned"
                        );
                        return;
                    }
                };

                if let Err(err) = handle.block_on(handler.execute()) {
                    tracing::warn!(?err, "assign orders job failed");
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "assign orders job task panicked");
            }
        })
    })
    .expect("failed to start assign_order job");

    scheduler
        .add(outbox)
        .await
        .expect("failed to register cron job");

    scheduler
        .start()
        .await
        .expect("failed to launch cron scheduler");

    scheduler
}
