use axum::Router;
use ureq::Agent;

mod dashboard;
mod calendar;
mod global;
mod info;
mod error;
mod login;
mod public;

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .nest("/", public::get_router())
        .nest("/", login::get_router(state.clone()))
        .nest("/info", info::get_router())
        .nest("/global", global::get_router(state.clone()))
        .nest("/calendar", calendar::get_router(state.clone()))
        .nest("/dashboard", dashboard::get_router(state.clone()))
        .fallback(error::error_404)
}