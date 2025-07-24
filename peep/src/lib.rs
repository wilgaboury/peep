use std::{net::IpAddr, sync::Arc};

use anyhow::anyhow;
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
    pub session: Option<String>
}

impl PeepClientConfig {
    async fn session(&self, bootstrap_client: &BootstrapClient) -> anyhow::Result<String> {
        Ok(if let Some(ref session) = self.session {
            session.clone()
        } else {
            bootstrap_client.create_session().await?
        })
    }
}

#[derive(Clone)]
pub struct PeepClient {
    session: String,
    member: SessionMemberLocation
}

impl PeepClient {
    pub async fn new(config: &PeepClientConfig) -> anyhow::Result<Self> {
        let bootstrap_client  = BootstrapClient::new(config.bootstrap_server_location.clone(), Security::Secure).await?;
        let session = config.session(&bootstrap_client).await?;
        let member = find_inbound_addr(&session, &bootstrap_client).await?;

        Ok(PeepClient { session, member })
    }
}

async fn find_inbound_addr(session: &str, bootstrap_client: &BootstrapClient) -> anyhow::Result<SessionMemberLocation> {
    for (_, ip) in list_afinet_netifas()?.iter() {
        if let IpAddr::V6(ipv6) = ip {
            let listener = TcpListener::bind(format!("[{}]:0", ipv6.to_string())).await?;
            if let Ok(local_addr) = listener.local_addr() {
                if let Ok(member) = SessionMemberLocation::try_from(&local_addr) {
                    if let Ok(_) = bootstrap_client.update_session(session, &member).await {
                        return Ok(member)
                    }
                }
            }
        }
    }

    Err(anyhow!("could not establish path for inbound traffic"))
}

async fn ok() -> impl IntoResponse {
    StatusCode::OK
}