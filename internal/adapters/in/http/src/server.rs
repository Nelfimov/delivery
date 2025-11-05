use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::signal;
use tracing_subscriber::fmt::init;

use crate::handler::ServerImpl;

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

pub async fn start_server(addr: &str) {
    init();

    let app = openapi::server::new::<Arc<ServerImpl>, ServerImpl, ()>(Arc::new(ServerImpl));

    // let app = app.layer(...);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
