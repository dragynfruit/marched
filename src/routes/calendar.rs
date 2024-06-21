use std::io::BufReader;

use axum::{
    body::Body,
    extract::{Path, Query, State},
    response::{IntoResponse, Response},
    routing, Router,
};
use chrono::{Datelike, TimeZone, Utc};
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
    previous_month_length: i64,
    first_day_index: u32,
    month_length: i64,
    month_index: u32,
    year: i32,
    // days: Vec<CalendarDay>,
}

pub fn get_router(state: Agent) -> Router {
    Router::new()
        .route("/:google_id", routing::get(calendar))
        .with_state(state)
}

fn get_next_month(month: u32, year: i32) -> (u32, i32) {
    let next_month = if month == 12 { 1 } else { month + 1 };
    let next_year = if month == 12 { year + 1 } else { year };
    (next_month, next_year)
}

fn get_previous_month(month: u32, year: i32) -> (u32, i32) {
    let previous_month = if month == 1 { 12 } else { month - 1 };
    let previous_year = if month == 1 { year - 1 } else { year };
    (previous_month, previous_year)
}

fn get_days_in_month(month: u32, year: i32) -> i64 {
    let next_month = get_next_month(month, year);
    let first_day = Utc.with_ymd_and_hms(year, month, 1, 0, 0, 0).unwrap();
    let last_day = Utc
        .with_ymd_and_hms(next_month.1, next_month.0, 1, 0, 0, 0)
        .unwrap();
    last_day.signed_duration_since(first_day).num_days()
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
    let first_day = Utc
        .with_ymd_and_hms(
            query.year.unwrap_or(current_date.year()),
            query.month.unwrap_or(current_date.month0()) + 1,
            1,
            0,
            0,
            0,
        )
        .unwrap();
    let previous_month = get_previous_month(first_day.month(), first_day.year());

    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(Body::new(
            TEMPLATES
                .render(
                    "calendar/calendar.html",
                    &Context::from_serialize(CalendarMonth {
                        previous_month_length: get_days_in_month(
                            previous_month.0,
                            previous_month.1,
                        ),
                        first_day_index: first_day.weekday().num_days_from_sunday(),
                        month_length: get_days_in_month(first_day.month(), first_day.year()),
                        month_index: first_day.month0(),
                        year: first_day.year(),
                    })
                    .unwrap(),
                )
                .unwrap(),
        ))
        .unwrap()
}
