use crate::users::AuthSession;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

use super::app::AppState;

pub fn router(_state: Arc<AppState>) -> Router {
    Router::new().route("/", get(self::get::index))
}

pub mod get {
    use askama::Template;
    use askama_axum::IntoResponse;

    use super::*;

    #[derive(Template)]
    #[template(path = "index.html")]
    struct EventTemplate<'a> {
        username: &'a str,
    }

    pub async fn index(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => EventTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
