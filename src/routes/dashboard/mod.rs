use axum::{
    body::Body,
    extract::{Path, Request, State},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing, Router,
};
use axum_extra::extract::CookieJar;
use tera::Context;
use ureq::Agent;

use crate::{api::{EnsembleResponse, MeResponse}, constants::URL, templates::TEMPLATES};

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .route("/", routing::get(home))
        .route("/ensemble/:id", routing::get(ensemble))
        .layer(middleware::from_fn(token_check))
        .with_state(state)
}

async fn token_check(jar: CookieJar, request: Request, next: Next) -> Response {
    if jar.get("token").is_none() {
        return Response::builder()
            .status(302)
            .header("Location", "/")
            .body(Body::empty())
            .unwrap();
    }

    next.run(request).await
}

async fn home(State(agent): State<Agent>, jar: CookieJar) -> impl IntoResponse {
    let me_response: MeResponse = agent
        .get(&format!("{URL}/v2.1/auth/me"))
        .set("Token", jar.get("token").unwrap().value())
        .set("AppVersion", "2.2.0")
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    if me_response.data.is_none() {
        return Response::builder()
            .status(302)
            .header("Location", "/")
            .header("Set-Cookie", "token=; Max-Age=0; Path=/; HttpOnly")
            .body(Body::empty())
            .unwrap();
    }

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "dashboard/home.html",
                    &Context::from_serialize(me_response.data.unwrap()).unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}

async fn ensemble(
    State(agent): State<Agent>,
    Path(id): Path<String>,
    jar: CookieJar,
) -> impl IntoResponse {
    let ensemble_response: EnsembleResponse = agent
        .get(&format!("{URL}/v2.1/ensemble/{}", id))
        .set("Token", jar.get("token").unwrap().value())
        .set("AppVersion", "2.2.0")
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    if ensemble_response.data.is_none() {
        return Response::builder()
            .status(302)
            .header("Location", "/")
            .header("Set-Cookie", "token=; Max-Age=0; Path=/; HttpOnly")
            .body(Body::empty())
            .unwrap();
    }

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "dashboard/ensemble.html",
                    &Context::from_serialize(ensemble_response.data.unwrap()).unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}
