use axum::{
    body::Body,
    extract::{Path, State},
    response::{IntoResponse, Response},
    routing, Router,
};
use ureq::Agent;

use crate::constants::URL;

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .route("/:path", routing::get(img))
        .with_state(state)
}

async fn img(State(agent): State<Agent>, Path(path): Path<String>) -> impl IntoResponse {
    let mut icon = vec![];
    agent
        .get(&format!("{URL}/files/global/{path}"))
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut icon)
        .unwrap();

    Response::builder()
        .status(200)
        .header("Content-Type", "image/jpeg")
        .header("Cache-Control", "public, max-age=31536000, immutable")
        .body(Body::from(icon))
        .unwrap()
}
