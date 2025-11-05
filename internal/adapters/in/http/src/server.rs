use ports::courier_repository_port::CourierRepositoryPort;
use ports::order_repository_port::OrderRepositoryPort;
use ports::unit_of_work_port::UnitOfWorkPort;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::fmt::init;

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

pub async fn start_server<CR, OR, UOW>(addr: &str, state: AppState<CR, OR, UOW>)
where
    CR: CourierRepositoryPort + Send + Sync + 'static,
    OR: OrderRepositoryPort + Send + Sync + 'static,
    UOW: UnitOfWorkPort + Send + Sync + 'static,
{
    init();

    let shared_state = Arc::new(state);
    let handler = Arc::new(ServerImpl::new(shared_state));
    let app =
        openapi::server::new::<Arc<ServerImpl<CR, OR, UOW>>, ServerImpl<CR, OR, UOW>, ()>(handler);

    // let app = app.layer(...);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
