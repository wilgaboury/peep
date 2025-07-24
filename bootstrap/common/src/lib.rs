use std::net::{IpAddr, Ipv6Addr, SocketAddr};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

pub type SessionMemberLocationsSerde = Vec<SessionMemberLocationSerde>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMemberLocationSerde {
    pub addr: String,
    pub port: u16
}

impl From<&SessionMemberLocation> for SessionMemberLocationSerde {
    fn from(obj: &SessionMemberLocation) -> Self {
        Self { addr: obj.addr.to_string(), port: obj.port }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionMemberLocation {
    pub addr: Ipv6Addr,
    pub port: u16
}

impl TryFrom<&SessionMemberLocationSerde> for SessionMemberLocation {
    type Error = anyhow::Error;

    fn try_from(serde: &SessionMemberLocationSerde) -> anyhow::Result<Self> {
        let addr = serde.addr.parse::<Ipv6Addr>()?;
        Ok(Self {
            addr,
            port: serde.port
        })
    }
}

impl TryFrom<&SocketAddr> for SessionMemberLocation {
    type Error = anyhow::Error;

    fn try_from(socket: &SocketAddr) -> anyhow::Result<Self> {
        match socket.ip() {
            IpAddr::V6(addr) => Ok(Self {
                addr,
                port: socket.port()
            }),
            _ => Err(anyhow!("ipv4 not supported"))
        }
    }
}

impl ToString for SessionMemberLocation {
    fn to_string(&self) -> String {
        format!("[{}]:{}", self.addr, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSessionResponse {
    pub session_id: String,
}