use crate::users::AuthSession;
use axum::{http::StatusCode, routing::get, Router};
use std::sync::Arc;

use super::app::AppState;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(self::get::index))
        .with_state(state.clone())
}

pub mod get {
    use axum::{extract::State, response::Html};

    use super::*;

    pub async fn index(
        State(state): State<Arc<AppState>>,
        auth_session: AuthSession,
    ) -> Html<String> {
        match auth_session.user {
            Some(user) => {
                let mut ctx = tera::Context::new();
                ctx.insert("username", &user.username);
                let template = state
                    .tera
                    .render("index.html", &ctx)
                    .expect("Failed to render index.html");

                Html(template)
            }

            None => Html(StatusCode::INTERNAL_SERVER_ERROR.to_string()),
        }
    }
}
