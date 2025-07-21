use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{collections::HashMap, net::{Ipv6Addr, SocketAddr}, sync::{Arc, RwLock}};
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
struct Sessions {
    map: Arc<RwLock<HashMap<String, Arc<RwLock<Session>>>>>
}

impl Sessions {
    fn new() -> Self {
        Sessions{ map: Arc::new(RwLock::new(HashMap::new())) }
    }
}

#[derive(Debug, Clone)]
struct Session {
    members: Vec<SessionMember>
}

#[derive(Debug, Clone, Copy)]
struct SessionMember {
    addr: Ipv6Addr,
    port: u16
}

impl SessionMember {
    fn to_json(&self) -> SessionMemberJson {
        SessionMemberJson { addr: self.addr.to_string(), port: self.port }
    }
}

#[derive(Debug, Clone)]
struct AppState {
    sessions: Sessions
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/session", post(create_session))
        .route("/session/{id}", get(get_session))
        .route("/session/{id}", patch(update_session))
        .with_state(AppState { sessions: Sessions::new() });

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], 80));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct SessionMemberJson {
    addr: String,
    port: u16
}

impl SessionMemberJson {
    fn maybe_to_struct(&self) -> Option<SessionMember> {
        let addr = self.addr.parse::<Ipv6Addr>().ok()?;
        Some(SessionMember {
            addr,
            port: self.port
        })
    }
}

#[derive(Debug, Serialize, Clone)]
struct CreateSessionResponse {
    id: String
}

async fn create_session(State(state): State<AppState>, Json(input): Json<SessionMemberJson>) -> Result<impl IntoResponse, StatusCode> {
    let member = input.maybe_to_struct().ok_or(StatusCode::BAD_REQUEST)?;
    let id = Uuid::new_v4().to_string();

    {
        let mut map = state.sessions.map.write().unwrap();
        map.insert(id.clone(), Arc::new(RwLock::new(Session {
            members: vec![member]
        })));
    }

    Ok(Json(CreateSessionResponse {
        id
    }))
}

async fn get_session(State(state): State<AppState>, Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let session = {
        let map = state.sessions.map.read().unwrap();
        let val = map.get(&id);
        if let Some(session) = val {
            session.clone()
        } else {
            return Err(StatusCode::NOT_FOUND)
        }
    };

    Ok(Json(session.read().unwrap().members.iter().map(|member| member.to_json()).collect::<Vec<_>>()))
}

async fn update_session() {
    
}