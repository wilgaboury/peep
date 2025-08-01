use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::{get, patch, post}, Json, Router};
use bootstrap_common::{SessionMemberLocation, SessionMemberLocationSerde};
use tokio::{io::AsyncWriteExt, net::TcpStream, select, time};
use uuid::Uuid;
use std::{collections::{HashMap, HashSet}, sync::{Arc, RwLock}};

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
    members: HashSet<SessionMemberLocation>,
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

async fn create_session(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let session_id = Uuid::new_v4().to_string();

    let members = HashSet::new();
    {
        let mut map = state.sessions.map.write().unwrap();
        map.insert(session_id.clone(), Arc::new(RwLock::new(Session { members })));
    }

    Ok(session_id)
}

async fn get_session(State(state): State<AppState>, Path(id): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let session = state.sessions.map.read().unwrap().get(&id).ok_or(StatusCode::NOT_FOUND)?.clone();
    Ok(Json(session.read().unwrap().members.iter()
        .map(|member| SessionMemberLocationSerde::from(member))
        .collect::<Vec<_>>()))
}

async fn update_session(
        State(state): State<AppState>, 
        Path(id): Path<String>, 
        Json(input): Json<SessionMemberLocationSerde>
    ) -> Result<impl IntoResponse, StatusCode> {
    let input_member = SessionMemberLocation::try_from(&input).map_err(|_| StatusCode::BAD_REQUEST)?;

    println!("trying connection to {}", input_member.to_string());

    let mut stream = select! {
        stream_result = TcpStream::connect(input_member.to_string()) => { stream_result.map_err(|_| StatusCode::CONFLICT) },
        () = time::sleep(time::Duration::from_secs(5)) => { Err(StatusCode::CONFLICT) }
    }?;
    stream.shutdown().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let session = state.sessions.map.read().unwrap().get(&id).ok_or(StatusCode::NOT_FOUND)?.clone();
    let mut session = session.write().unwrap();
    session.members.insert(input_member);
    Ok(StatusCode::OK)
}