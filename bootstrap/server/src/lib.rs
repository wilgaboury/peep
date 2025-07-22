use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use common::{SessionMemberLocation, SessionMemberLocationSerde};
use uuid::Uuid;
use std::{collections::HashMap, sync::{atomic::{AtomicU64, Ordering}, Arc, RwLock}};

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

#[derive(Debug, Clone)]
struct AppState {
    sessions: Sessions
}

pub fn create_server_router() -> Router {
    Router::new()
        .route("/ok", get(ok))
        .route("/session", post(create_session))
        .route("/session/{id}", get(get_session))
        .route("/session/{id}", patch(update_session))
        .with_state(AppState { sessions: Sessions::new() })
}

async fn ok() -> impl IntoResponse {
    StatusCode::OK
}

async fn create_session(
        State(state): State<AppState>, 
        Json(input): Json<SessionMemberLocationSerde>
    ) -> Result<impl IntoResponse, StatusCode> {
    let member = SessionMemberLocation::try_from(&input).map_err(|_| StatusCode::BAD_REQUEST)?;
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
    Ok(Json(session.read().unwrap().members.keys()
        .map(|member| SessionMemberLocationSerde::from(member))
        .collect::<Vec<_>>()))
}

async fn update_session(
        State(state): State<AppState>, 
        Path(id): Path<String>, 
        Json(input): Json<SessionMemberLocationSerde>
    ) -> Result<impl IntoResponse, StatusCode> {
    let input_member = SessionMemberLocation::try_from(&input).map_err(|_| StatusCode::BAD_REQUEST)?;
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