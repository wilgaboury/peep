use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionMemberJson {
    pub addr: String,
    pub port: u16
}