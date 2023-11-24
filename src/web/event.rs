use super::app::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/events/:id", get(self::get::event))
        .with_state(state.clone())
}

mod get {
    use crate::{
        golf::{BookingEvent, BookingGroup},
        web::app::AppState,
    };
    use askama::Template;
    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use std::sync::Arc;

    #[derive(Template)]
    #[template(path = "event.html")]
    struct EventTemplate {
        event: BookingEvent,
    }

    fn num_holes<'a>(group: &BookingGroup) -> &'a str {
        match group.holes() {
            Some(9) => "9",
            Some(18) => "18",
            _ => "Unknown",
        }
    }

    fn num_entries(group: &BookingGroup) -> usize {
        group.booking_entries.entries.len()
    }

    pub async fn event(
        state: State<Arc<AppState>>,
        Path(event_id): Path<u32>,
    ) -> impl IntoResponse {
        if let Ok(event) = state.golf_client.get_event(event_id).await {
            return EventTemplate { event }.into_response();
        }

        "Error finding event".into_response()
    }
}
