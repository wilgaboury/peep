use anyhow::anyhow;
use reqwest::StatusCode;

pub struct BootstrapClient {
    client: reqwest::Client,
    location: String
}

impl BootstrapClient {
    pub async fn create(location: String) -> anyhow::Result<Self> {
        let result = BootstrapClient {
            client: reqwest::Client::new(),
            location
        };

        let code = result.client.get(result.url("ok")).send().await?.status();

        match code {
            StatusCode::OK => Ok(result),
            _ => Err(anyhow!("Did not receive OK status code"))
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("https://{}/{}", self.location, path)
    }

    pub async fn create_session(&self, request: &common::SessionMemberLocation) -> anyhow::Result<common::CreateSessionResponse> {
        Ok(self.client.post(self.url("session")).json(&request).send().await?.json::<common::CreateSessionResponse>().await?)
    }

    pub async fn get_session(&self, session_id: &str) -> anyhow::Result<common::SessionMemberLocations> {
        Ok(self.client.get(self.url(format!("session/{}", session_id).as_str())).send().await?.json::<common::SessionMemberLocations>().await?)
    }

    pub async fn update_session(&self, session_id: &str, request: &common::SessionMemberLocation) -> anyhow::Result<common::UpdateSessionResponse> {
        Ok(self.client.patch(self.url(format!("session/{}", session_id).as_str())).json(&request).send().await?.json::<common::UpdateSessionResponse>().await?)
    }
}