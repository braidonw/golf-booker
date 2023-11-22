use crate::users::AuthSession;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    username: &'a str,
}

#[derive(Template)]
#[template(path = "event.html")]
struct EventTemplate<'a> {
    event_id: &'a str,
}

pub fn router() -> Router<()> {
    Router::new()
        .route("/", get(self::get::index))
        .route("/events/:id", get(self::get::event))
}

mod get {
    use axum::extract::Path;

    use super::*;

    pub async fn index(auth_session: AuthSession) -> impl IntoResponse {
        match auth_session.user {
            Some(user) => HomeTemplate {
                username: &user.username,
            }
            .into_response(),

            None => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }

    pub async fn event(Path(event_id): Path<u32>) -> impl IntoResponse {
        EventTemplate {
            event_id: &event_id.to_string(),
        }
        .into_response()
    }
}
