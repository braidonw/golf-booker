use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GolfEvent {
    #[serde(rename(deserialize = "bookingEventId"))]
    id: Option<u32>, // Not sure why this is not returned sometimes by the API
    event_date: Option<String>,
    event_status_code: Option<u32>,
    event_status_code_friendly: Option<String>,
    title: String,
    booking_resource_id: Option<u32>,
    redirect_url: Option<String>,
    first_manual_upload_results: Option<String>,
    is_lottery: Option<bool>,
    can_open_event: Option<bool>,
    has_competition: Option<bool>,
    matchplay_id: Option<u32>,
    event_type_code: Option<u32>,
    event_category_code: Option<u32>,
    event_time_code_friendly: Option<String>,
    booking_note_action: Option<String>,
    auto_open_date_time_display: Option<String>,
    availability: u32,
    is_ballot: bool,
    is_ballot_open: bool,
    is_results: bool,
    is_open: bool,
    is_female: bool,
    is_male: bool,
    is_matchplay: bool,
}
