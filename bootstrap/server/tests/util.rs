use std::{net::SocketAddr};

use bootstrap_client::{BootstrapClient, Security};
use bootstrap_server::create_server_router;
use tokio::{net::TcpListener, spawn, sync::oneshot};

pub struct TestBootstrapServer {
    shutdown: Option<oneshot::Sender<()>>,
    client: BootstrapClient
}

impl TestBootstrapServer {
    pub async fn new() -> anyhow::Result<Self> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 0));
        let listener = TcpListener::bind(addr).await?;
        let location = listener.local_addr()?.to_string();
        let app = create_server_router();

        let (tx, rx) = oneshot::channel::<()>();

        let server = axum::serve(listener, app.into_make_service())
            .with_graceful_shutdown(async {
                rx.await.unwrap()
            });

        spawn(async move {
            server.await.unwrap();
        });

        Ok(Self { shutdown: Some(tx), client: BootstrapClient::new(location, Security::Insecure).await? })
    }

    pub fn client(&self) -> &BootstrapClient {
        &self.client
    }
}

impl Drop for TestBootstrapServer {
    fn drop(&mut self) {
        if let Some(shutdown) = std::mem::replace(&mut self.shutdown, None) {
            shutdown.send(()).unwrap();
        }
    }
}

