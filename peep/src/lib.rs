use std::{net::{IpAddr, Ipv6Addr}};

use anyhow::anyhow;
use bootstrap_client::BootstrapClient;
use bootstrap_common::SessionMemberLocation;
use local_ip_address::list_afinet_netifas;
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[derive(Clone)]
pub struct PeepClientConfig {
    pub bootstrap: BootstrapClient,
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
    pub session: String,
    pub member: SessionMemberLocation,
    pub bootstrap: BootstrapClient
}

impl PeepClient {
    pub async fn new(config: PeepClientConfig) -> anyhow::Result<Self> {
        let session = config.session(&config.bootstrap).await?;
        let (listener, member) = find_inbound_addr(&session, &config.bootstrap).await?;

        Ok(Self { session, member, bootstrap: config.bootstrap })
    }
}

async fn find_inbound_addr(session: &str, bootstrap_client: &BootstrapClient) -> anyhow::Result<(TcpListener, SessionMemberLocation)> {
    for (_, ip) in list_afinet_netifas()?.iter() {
        if let IpAddr::V6(ipv6) = ip {
            if let Ok(member) = try_create_session_member_location(session, bootstrap_client, ipv6).await {
                return Ok(member);
            }
        }
    }

    Err(anyhow!("could not establish path for inbound traffic"))
}

async fn try_create_session_member_location(session: &str, bootstrap_client: &BootstrapClient, addr: &Ipv6Addr) -> anyhow::Result<(TcpListener, SessionMemberLocation)> {
    let listener = TcpListener::bind("[::]:0").await?;
    let port = listener.local_addr()?.port();
    let member = SessionMemberLocation { addr: addr.clone(), port };

    let task = tokio::spawn(async move {
        let result = listener.accept().await;
        (listener, result)
    });

    println!("trying {}", member.to_string());
    let _ = bootstrap_client.update_session(session, &member).await?;

    let (listener, result) = task.await?;
    let (mut stream, _) = result?;
    stream.shutdown().await?;
                
    Ok((listener, member)) 
}