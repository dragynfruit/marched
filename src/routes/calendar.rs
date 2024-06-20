use std::io::BufReader;

use axum::{
    body::Body,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing, Router,
};
use chrono::{DateTime, Datelike, TimeZone, Utc};
use ical::IcalParser;
use serde::{Deserialize, Serialize};
use tera::Context;
use ureq::Agent;

use crate::templates::TEMPLATES;

#[derive(Deserialize)]
struct CalendarRequest {
    month: Option<u32>,
    year: Option<i32>,
    timezone: Option<i8>,
}

#[derive(Serialize)]
struct CalendarMonth {
    year: i32,
    month: u32,
    // days: Vec<CalendarDay>,
}

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .route("/:google_id", routing::get(calendar))
        .with_state(state)
}

async fn calendar(
    State(agent): State<Agent>,
    Path(google_id): Path<String>,
    Query(query): Query<CalendarRequest>,
) -> impl IntoResponse {
    let reader = agent
        .get(&format!(
            "https://calendar.google.com/calendar/ical/{google_id}/public/basic.ics"
        ))
        .call()
        .unwrap()
        .into_reader();

    let ical_reader = IcalParser::new(BufReader::new(reader));
    let _ = ical_reader.collect::<Result<Vec<_>, _>>().unwrap();

    let current_date = Utc::now();
    let month = Utc.with_ymd_and_hms(
        query.year.unwrap_or(current_date.year()),
        query.month.unwrap_or(current_date.month0()) + 1,
        1,
        0,
        0,
        0,
    ).unwrap();

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "calendar/calendar.html",
                    &Context::from_serialize(CalendarMonth {
                        year: month.year(),
                        month: month.month0(),
                    })
                    .unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}
