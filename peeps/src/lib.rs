use std::sync::Arc;

struct ClientState {
    
}

impl ClientState {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientState>
}

impl Client {
    pub fn new() -> Self {
        Self { inner: Arc::new(ClientState::new()) }
    }

    
}