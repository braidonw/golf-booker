use super::app::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/events/:event_id/groups/:group_id", get(self::get::group))
        .with_state(state.clone())
}

mod get {
    use crate::{golf::BookingEvent, web::app::AppState};
    use axum::{
        extract::{Path, State},
        response::Html,
    };
    use std::sync::Arc;

    pub async fn group(
        Path((event_id, group_id)): Path<(u32, u32)>,
        state: State<Arc<AppState>>,
    ) -> Html<String> {
        let event: BookingEvent = state.golf_client.get_event(event_id).await.unwrap();
        let (group, section) = event.get_booking_group(group_id).unwrap();

        let mut ctx = tera::Context::new();
        ctx.insert("group", &group);
        ctx.insert("section", &section);
        ctx.insert("event", &event);

        let template = state
            .tera
            .render("group.html", &ctx)
            .expect("Failed to render group.html");

        Html(template)
    }
}
