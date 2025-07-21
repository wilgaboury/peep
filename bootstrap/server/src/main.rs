use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{collections::HashMap, env, net::{Ipv6Addr, SocketAddr}, sync::{atomic::{AtomicU64, Ordering}, Arc, RwLock}};
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

#[derive(Debug)]
struct Session {
    members: HashMap<SessionMemberLocation, u64>,
    next_id: AtomicU64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SessionMemberLocation {
    addr: Ipv6Addr,
    port: u16
}

impl SessionMemberLocation {
    fn to_json(&self) -> common::SessionMemberLocation {
        common::SessionMemberLocation { addr: self.addr.to_string(), port: self.port }
    }

    fn maybe_from_json(json: &common::SessionMemberLocation) -> Option<Self> {
        let addr = json.addr.parse::<Ipv6Addr>().ok()?;
        Some(SessionMemberLocation {
            addr,
            port: json.port
        })
    }
}

#[derive(Debug, Clone)]
struct AppState {
    sessions: Sessions
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let port = args.get(1).and_then(|port_str| port_str.parse::<u16>().ok()).unwrap_or(3000);

    // pass incoming GET requests on "/hello-world" to "hello_world" handler.
    let app = Router::new()
        .route("/version", get(version))
        .route("/session", post(create_session))
        .route("/session/{id}", get(get_session))
        .route("/session/{id}", patch(update_session))
        .with_state(AppState { sessions: Sessions::new() });

    // write address like this to not make typos
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await?;

    println!("listening on port {}", port);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

const version_num: u64 = 0;

async fn version() -> impl IntoResponse {
    Json(version_num)
}

async fn create_session(
        State(state): State<AppState>, 
        Json(input): Json<common::SessionMemberLocation>
    ) -> Result<impl IntoResponse, StatusCode> {
    let member = SessionMemberLocation::maybe_from_json(&input).ok_or(StatusCode::BAD_REQUEST)?;
    let session_id = Uuid::new_v4().to_string();

    let mut members = HashMap::new();
    members.insert(member, 1);
    {
        let mut map = state.sessions.map.write().unwrap();
        map.insert(session_id.clone(), Arc::new(RwLock::new(Session { 
            members,
            next_id: AtomicU64::new(1)
         })));
    }

    Ok(Json(common::CreateSessionResponse {
        session_id,
        member_id: 0
    }))
}

async fn get_session(State(state): State<AppState>, Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let session = state.sessions.map.read().unwrap().get(&id).ok_or(StatusCode::NOT_FOUND)?.clone();
    Ok(Json(session.read().unwrap().members.keys().map(|member| member.to_json()).collect::<Vec<_>>()))
}

async fn update_session(
        State(state): State<AppState>, 
        Path(id): Path<String>, 
        Json(input): Json<common::SessionMemberLocation>
    ) -> Result<impl IntoResponse, StatusCode> {
    let input_member = SessionMemberLocation::maybe_from_json(&input).ok_or(StatusCode::BAD_REQUEST)?;
    let session = state.sessions.map.read().unwrap().get(&id).ok_or(StatusCode::NOT_FOUND)?.clone();
    let mut session = session.write().unwrap();
    let maybe_member_id = session.members.get(&input_member).map(|v| *v);
    let member_id = maybe_member_id
        .unwrap_or_else(|| {
            let member_id = session.next_id.fetch_add(1, Ordering::Relaxed);
            session.members.insert(input_member, member_id);
            member_id
        });
    Ok(Json(common::UpdateSessionResponse {
        member_id
    }))
}