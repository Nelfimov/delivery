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
use std::panic;
use std::panic::AssertUnwindSafe;
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
    match Job::new_repeated_async(Duration::from_secs(1), move |_uuid, _l| {
        let handler = Arc::clone(&move_couriers_handler_job);
        let handle = move_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let run_result = panic::catch_unwind(AssertUnwindSafe(|| {
                    let mut handler = match handler.lock() {
                        Ok(handler) => handler,
                        Err(err) => {
                            tracing::error!(
                                error = %err,
                                "move couriers handler mutex poisoned"
                            );
                            err.into_inner()
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
                }));

                if let Err(err) = run_result {
                    tracing::error!(?err, "move couriers job panicked");
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "move couriers job task panicked");
            }
        })
    }) {
        Ok(job) => {
            if let Err(error) = scheduler.add(job).await {
                tracing::error!(?error, "failed to register move_couriers job");
            }
        }
        Err(error) => tracing::error!(?error, "failed to register move_couriers job"),
    }

    let assign_order_handler = Arc::new(Mutex::new(AssignOrderHandler::new(UnitOfWork::new(
        pool.clone(),
    ))));
    let assign_order_handler_job = Arc::clone(&assign_order_handler);
    let assign_job_handle = runtime_handle.clone();
    match Job::new_repeated_async(Duration::from_secs(1), move |_uuid, _l| {
        let handler = Arc::clone(&assign_order_handler_job);
        let handle = assign_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let run_result = panic::catch_unwind(AssertUnwindSafe(|| {
                    let mut handler = match handler.lock() {
                        Ok(handler) => handler,
                        Err(err) => {
                            tracing::error!(
                                error = %err,
                                "assign order handler mutex poisoned"
                            );
                            err.into_inner()
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
                }));

                if let Err(err) = run_result {
                    tracing::error!(?err, "assign orders job panicked");
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "assign orders job task panicked");
            }
        })
    }) {
        Ok(job) => {
            if let Err(error) = scheduler.add(job).await {
                tracing::error!(?error, "failed to register assign_orders job");
            }
        }
        Err(error) => tracing::error!(?error, "failed to register assign_orders job"),
    }

    let outbox_job = Arc::new(Mutex::new(OutboxJob::new(
        OutboxRepository::new(pool),
        OrdersEventsProducer::new(brokers, group_id),
    )));
    let outbox_handler_job = Arc::clone(&outbox_job);
    let outbox_job_handle = runtime_handle.clone();
    match Job::new_repeated_async(Duration::from_secs(10), move |_uuid, _l| {
        let handler = Arc::clone(&outbox_handler_job);
        let handle = outbox_job_handle.clone();
        Box::pin(async move {
            let join_result = task::spawn_blocking(move || {
                let run_result = panic::catch_unwind(AssertUnwindSafe(|| {
                    let mut handler = match handler.lock() {
                        Ok(handler) => handler,
                        Err(err) => {
                            tracing::error!(
                                error = %err,
                                "outbox job mutex poisoned"
                            );
                            err.into_inner()
                        }
                    };

                    if let Err(err) = handle.block_on(handler.execute()) {
                        tracing::warn!(?err, "outbox job failed");
                    }
                }));

                if let Err(err) = run_result {
                    tracing::error!(?err, "outbox job panicked");
                }
            })
            .await;

            if let Err(join_err) = join_result {
                tracing::error!(?join_err, "outbox job task panicked");
            }
        })
    }) {
        Ok(job) => {
            // if let Err(error) = scheduler.add(job).await {
            //     tracing::error!(?error, "failed to register outbox job");
            // }
        }
        Err(error) => tracing::error!(?error, "failed to register outbox job"),
    }

    scheduler.start().await.unwrap_or_else(|error| {
        tracing::error!(?error, "failed to launch cron scheduler");
    });

    scheduler
}
