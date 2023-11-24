use super::app::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/events/:event_id/booking_groups/:group_id",
            get(self::get::group),
        )
        .with_state(state.clone())
}

mod get {
    use crate::{
        golf::{BookingEvent, BookingGroup, BookingSection},
        web::app::AppState,
    };
    use askama::Template;
    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use std::sync::Arc;

    #[derive(Template)]
    #[template(path = "event_group.html")]
    pub struct BookingGroupTemplate<'a> {
        event: &'a BookingEvent,
        section: &'a BookingSection,
        group: &'a BookingGroup,
    }

    pub fn format_date(date: &chrono::NaiveDateTime) -> String {
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }

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

        dbg!(&group);

        BookingGroupTemplate {
            event,
            section,
            group,
        }
        .into_response()
    }
}
