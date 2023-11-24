use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct GolfEvent {
    #[serde(rename(deserialize = "bookingEventId"))]
    pub id: u32, // Not sure why this is not returned sometimes by the API
    pub event_date: NaiveDate,
    pub event_status_code: Option<u32>,
    pub event_status_code_friendly: Option<String>,
    pub title: String,
    pub booking_resource_id: Option<u32>,
    pub redirect_url: Option<String>,
    pub first_manual_upload_results: Option<String>,
    pub is_lottery: Option<bool>,
    pub can_open_event: Option<bool>,
    pub has_competition: Option<bool>,
    pub matchplay_id: Option<u32>,
    pub event_type_code: Option<u32>,
    pub event_category_code: Option<u32>,
    pub event_time_code_friendly: Option<String>,
    pub booking_note_action: Option<String>,
    pub auto_open_date_time_display: Option<String>,
    pub availability: u32,
    pub is_ballot: bool,
    pub is_ballot_open: bool,
    pub is_results: bool,
    pub is_open: bool,
    pub is_female: bool,
    pub is_male: bool,
    pub is_matchplay: bool,
}
