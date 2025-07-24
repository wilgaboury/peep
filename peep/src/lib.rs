use std::{net::IpAddr, sync::Arc};

use axum::{http::StatusCode, response::IntoResponse};
use bootstrap_client::{BootstrapClient, Security};
use bootstrap_common::SessionMemberLocation;
use local_ip_address::list_afinet_netifas;
use tokio::net::TcpListener;

struct PeepClientState {
    
}

impl PeepClientState {
    fn new() -> Self {
        Self {}
    }
}

pub struct PeepClientConfig {
    pub bootstrap_server_location: String,
    pub session_id: Option<String>
}

#[derive(Clone)]
pub struct PeepClient {
    inner: Arc<PeepClientState>
}

impl PeepClient {
    pub async fn new(config: &PeepClientConfig) -> anyhow::Result<Self> {
        let bootstrap_client  = BootstrapClient::new(config.bootstrap_server_location.clone(), Security::Secure).await?;
        let session_id = if let Some(ref id) = config.session_id {
            id.clone()
        } else {
            bootstrap_client.create_session().await?.session_id
        };

        for (_, ip) in list_afinet_netifas()?.iter() {
            if let IpAddr::V6(ipv6) = ip {
                let listener = TcpListener::bind(format!("[{}]:0", ipv6.to_string())).await?;
                bootstrap_client.update_session(&session_id, &(SessionMemberLocation::try_from(&listener.local_addr()?)?)).await?;


            }
        }

        todo!("bruh")
    }


}

async fn ok() -> impl IntoResponse {
    StatusCode::OK
}