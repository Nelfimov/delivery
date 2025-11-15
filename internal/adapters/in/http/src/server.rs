use ports::courier_repository_port::CourierRepositoryPort;
use ports::events_producer_port::EventsProducerPort;
use ports::geo_service_port::GeoServicePort;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;

use crate::handler::ServerImpl;
use crate::state::AppState;

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn start_server<CR, OR, UOW, GS, OEB>(addr: &str, state: AppState<CR, OR, UOW, GS, OEB>)
where
    CR: CourierRepositoryPort + Send + 'static,
    OR: OrderRepositoryPort + Send + 'static,
    UOW: UnitOfWorkPort + Send + 'static,
    GS: GeoServicePort + Clone + Send + Sync + 'static,
    OEB: EventsProducerPort + Send + 'static,
{
    let shared_state = Arc::new(state);
    let handler = Arc::new(ServerImpl::new(shared_state));
    let app = openapi::server::new::<
        Arc<ServerImpl<CR, OR, UOW, GS, OEB>>,
        ServerImpl<CR, OR, UOW, GS, OEB>,
        (),
    >(handler);

    // let app = app.layer(...);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
