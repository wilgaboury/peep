use serde::{Deserialize, Serialize};

pub type SessionMemberLocations = Vec<SessionMemberLocation>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMemberLocation {
    pub addr: String,
    pub port: u16
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateSessionResponse {
    pub session_id: String,
    pub member_id: u64
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateSessionResponse {
    pub member_id: u64
}