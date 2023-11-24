use crate::users::AuthSession;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

use super::app::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(self::get::index))
        .route("/load-events", get(self::get::get_events))
        .with_state(state.clone())
}

pub mod get {
    use askama::Template;
    use askama_axum::IntoResponse;
    use axum::extract::State;
    use chrono::NaiveDate;

    use crate::golf::GolfEvent;

    use super::*;

    #[derive(Template)]
    #[template(path = "index.html")]
    struct IndexTemplate<'a> {
        username: &'a str,
    }

    #[derive(Template)]
    #[template(path = "partials/events_table_body.html")]
    struct EventsTableBodyTemplate {
        events: Vec<GolfEvent>,
    }

    pub fn format_date(date: &NaiveDate) -> String {
        date.format("%d/%m/%Y").to_string()
    }

    pub fn status_code(event: &GolfEvent) -> String {
        if let Some(status_code) = event.event_status_code_friendly.clone() {
            status_code.to_string()
        } else if let Some(status_code) = event.event_status_code {
            status_code.to_string()
        } else {
            "Unknown".to_string()
        }
    }

    pub async fn index(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => IndexTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn get_events(state: State<Arc<AppState>>) -> impl IntoResponse {
        state.golf_client.login().await.expect("Failed to log in");
        let events = state
            .golf_client
            .get_events()
            .await
            .expect("Failed to get events");

        EventsTableBodyTemplate { events }.into_response()
    }
}
