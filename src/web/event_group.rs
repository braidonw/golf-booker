use axum::{routing::get, Router};

pub fn router() -> Router<()> {
    Router::new().route(
        "/events/:event_id/booking_groups/:group_id",
        get(self::get::event_group),
    )
}

mod get {
    use crate::golf::{BookingEvent, BookingGroup, BookingSection, GolfClient};
    use askama::Template;
    use askama_axum::IntoResponse;
    use axum::{extract::Path, Extension};
    use std::sync::Arc;

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

    #[derive(Template)]
    #[template(path = "event_group.html")]
    struct EventGroupTemplate {
        event: BookingEvent,
        section: BookingSection,
        group: BookingGroup,
    }

    pub fn event_group(
        golf_client: Extension<Arc<GolfClient>>,
        Path((event_id, group_id)): Path<(u32, u32)>,
    ) -> impl IntoResponse {
        if let Ok(event) = golf_client.get_event(event_id).await {
            if let Some((group, section)) = event.get_booking_group(group_id) {}
        }

        "Error finding event".into_response()
    }
}
