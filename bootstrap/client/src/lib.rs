use anyhow::anyhow;
use common::{SessionMemberLocation, SessionMemberLocationSerde, SessionMemberLocationsSerde, UpdateSessionResponse};
use reqwest::StatusCode;

#[derive(Clone)]
pub struct BootstrapClient {
    client: reqwest::Client,
    prefix: String
}

pub enum Security {
    Secure,
    Insecure
}

impl Security {
    fn to_protocol_prefix(&self) -> String {
        match self {
            Security::Secure => "https".to_string(),
            Security::Insecure => "http".to_string()
        }
    }
}

impl BootstrapClient {
    pub async fn new(location: String, security: Security) -> anyhow::Result<Self> {
        let prefix = format!("{}://{}", security.to_protocol_prefix(), location);
        let result = BootstrapClient {
            client: reqwest::Client::new(),
            prefix
        };

        let code = result.client.get(result.url("ok")).send().await?.status();

        match code {
            StatusCode::OK => Ok(result),
            _ => Err(anyhow!("Did not receive OK status code"))
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}/{}", self.prefix, path)
    }

    pub async fn create_session(&self, request: &SessionMemberLocation) -> anyhow::Result<common::CreateSessionResponse> {
        Ok(self.client.post(self.url("session")).json(&SessionMemberLocationSerde::from(request)).send().await?.json::<common::CreateSessionResponse>().await?)
    }

    pub async fn get_session(&self, session_id: &str) -> anyhow::Result<Vec<SessionMemberLocation>> {
        Ok(self.client.get(self.url(format!("session/{}", session_id).as_str())).send().await?.json::<SessionMemberLocationsSerde>().await?
            .iter()
            .map(|loc| loc.try_into())
            .collect::<anyhow::Result<_>>()?)
    }

    pub async fn update_session(&self, session_id: &str, request: &SessionMemberLocation) -> anyhow::Result<UpdateSessionResponse> {
        Ok(self.client.patch(self.url(format!("session/{}", session_id).as_str())).json(&SessionMemberLocationSerde::from(request)).send().await?.json().await?)
    }
}