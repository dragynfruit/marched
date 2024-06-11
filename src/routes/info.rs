use axum::{
    body::Body, response::{IntoResponse, Response}, routing, Json, Router
};
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::templates::TEMPLATES;

#[derive(Deserialize, Serialize, Clone)]
struct InstanceInfo {
    version: String,
    name: String,
    is_release: bool
}

fn get_info() -> InstanceInfo {
    InstanceInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        is_release: !cfg!(debug_assertions)
    }
}

pub fn get_router() -> Router {
    Router::new()
        .route("/", routing::get(info))
        .route("/json", routing::get(info_json))
}

async fn info() -> impl IntoResponse {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "info.html",
                    &Context::from_serialize(&get_info()).unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}

async fn info_json() -> Json<InstanceInfo> {
    Json(get_info())
}