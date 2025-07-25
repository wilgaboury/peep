use std::{env, net::{SocketAddr}};

use bootstrap_server::create_server_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).and_then(|port_str| port_str.parse::<u16>().ok()).unwrap_or(80);

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);

    let app = create_server_router();

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

