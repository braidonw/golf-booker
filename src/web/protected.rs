use crate::users::AuthSession;
use askama::Template;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

#[derive(Template)]
#[template(path = "index.html")]
struct HomeTemplate<'a> {
    username: &'a str,
}

pub fn router() -> Router<()> {
    Router::new().route("/", get(self::get::index))
}

mod get {
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
}
