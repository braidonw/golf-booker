use super::app::AppState;
use crate::golf::{BookingEvent, BookingGroup, BookingSection};
use askama::Template;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/events/:event_id/booking_groups/:group_id",
            get(self::get::group).post(self::post::book),
        )
        .with_state(state.clone())
}

#[derive(Template)]
#[template(path = "event_group.html")]
pub struct BookingGroupTemplate<'a> {
    event: &'a BookingEvent,
    section: &'a BookingSection,
    group: &'a BookingGroup,
    booking_message: Option<&'a str>,
}

pub fn format_date(date: &chrono::NaiveDateTime) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

mod get {
    use super::BookingGroupTemplate;
    use crate::web::app::AppState;
    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use std::sync::Arc;

    pub async fn group(
        Path((event_id, group_id)): Path<(u32, u32)>,
        state: State<Arc<AppState>>,
    ) -> impl IntoResponse {
        let event = &state
            .golf_client
            .get_event(event_id)
            .await
            .expect("Error finding event");

        let (group, section) = &event
            .get_booking_group(group_id)
            .expect("Error finding group");

        BookingGroupTemplate {
            event,
            section,
            group,
            booking_message: None,
        }
        .into_response()
    }
}

mod post {
    use super::BookingGroupTemplate;
    use crate::web::app::AppState;
    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use std::sync::Arc;

    pub async fn book(
        Path((event_id, group_id)): Path<(u32, u32)>,
        state: State<Arc<AppState>>,
    ) -> impl IntoResponse {
        if state.golf_client.login().await.is_err() {
            return "Error logging in".into_response();
        }

        if let Err(e) = state.golf_client.book(group_id).await {
            return e.to_string().into_response();
        }

        let event = &state
            .golf_client
            .get_event(event_id)
            .await
            .expect("Error finding event");

        let (group, section) = &event
            .get_booking_group(group_id)
            .expect("Error finding group");

        BookingGroupTemplate {
            event,
            section,
            group,
            booking_message: Some("Booking Successful"),
        }
        .into_response()
    }
}
