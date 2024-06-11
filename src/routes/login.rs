use axum::{
    body::Body,
    extract::State,
    response::{IntoResponse, Response},
    routing, Form, Router,
};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};
use tera::Context;
use ureq::Agent;

use crate::{api::LoginResponse, constants::URL, templates::TEMPLATES};

#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginError {
    code: u16,
    message: String,
    error: bool,
}

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .route("/", routing::get(login_page).post(login))
        .with_state(state)
}

async fn login_page(jar: CookieJar) -> impl IntoResponse {
    if jar.get("token").is_some() {
        return Response::builder()
            .status(302)
            .header("Location", "/dashboard")
            .body(Body::empty())
            .unwrap();
    }

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .header("Cache-Control", "public, max-age=31536000, immutable")
        .body(Body::new(
            TEMPLATES.render("login.html", &Context::new()).unwrap(),
        ))
        .unwrap()
}

async fn login(State(agent): State<Agent>, Form(data): Form<LoginData>) -> impl IntoResponse {
    let mut hasher = Sha1::new();
    hasher.update(data.password);
    let password_hash = hasher
        .finalize()
        .to_vec()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let login_response: LoginResponse = agent
        .post(&format!("{URL}/v2.1/auth/login"))
        .send_form(&[("email", &data.email), ("password", &password_hash)])
        .unwrap()
        .into_json()
        .unwrap();

    if login_response.data.is_some() {
        return Response::builder()
            .status(302)
            .header(
                "Set-Cookie",
                format!(
                    "token={}; SameSite=Strict; Max-Age=31536000",
                    login_response.data.unwrap().token
                ),
            )
            .header("Location", "/dashboard")
            .body(Body::empty())
            .unwrap();
    }

    let login_error = LoginError {
        code: login_response.api_response.code,
        message: login_response.api_response.message,
        error: true,
    };

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "login.html",
                    &Context::from_serialize(&login_error).unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}
