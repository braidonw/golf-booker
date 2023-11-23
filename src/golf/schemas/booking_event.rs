use serde::{Deserialize, Serialize};

use super::booking_group::{BookingGroup, BookingGroups};
#[derive(Debug, Deserialize, Serialize)]
pub struct BookingEvent {
    pub active: bool,
    pub id: u32,
    #[serde(rename(deserialize = "Date"))]
    pub date: String,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "lastModified"))]
    pub last_modified: String,
    #[serde(rename(deserialize = "lastModifierId"))]
    pub last_modifier_id: Option<u32>,
    #[serde(rename(deserialize = "BookingSections"))]
    pub booking_sections: BookingSections,
}

impl BookingEvent {
    pub fn get_booking_group(&self, id: u32) -> Option<(&BookingGroup, &BookingSection)> {
        for section in &self.booking_sections.sections {
            if section.booking_groups.groups.is_none() {
                continue;
            }

            for group in &section.booking_groups.groups {
                for g in group {
                    if g.id == id {
                        return Some((&g, &section));
                    }
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingSections {
    #[serde(rename(deserialize = "BookingSection"))]
    pub sections: Vec<BookingSection>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookingSection {
    #[serde(rename(deserialize = "lastModified"))]
    pub last_modified: String,
    #[serde(rename(deserialize = "lastModifierId"))]
    pub last_modifier_id: u32,
    #[serde(rename(deserialize = "@id"))]
    pub id: u32,
    pub active: bool,
    #[serde(rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "BookingGroups"))]
    pub booking_groups: BookingGroups,
}
