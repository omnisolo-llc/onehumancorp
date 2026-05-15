use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarList {
    pub kind: String,
    pub etag: String,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    #[serde(rename = "nextSyncToken")]
    pub next_sync_token: Option<String>,
    pub items: Vec<CalendarListEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarListEntry {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub summary: String,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "summaryOverride")]
    pub summary_override: Option<String>,
    #[serde(rename = "colorId")]
    pub color_id: Option<String>,
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    #[serde(rename = "foregroundColor")]
    pub foreground_color: Option<String>,
    pub hidden: Option<bool>,
    pub selected: Option<bool>,
    #[serde(rename = "accessRole")]
    pub access_role: String,
    pub primary: Option<bool>,
    pub deleted: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventList {
    pub kind: String,
    pub etag: String,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub updated: Option<DateTime<Utc>>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "accessRole")]
    pub access_role: Option<String>,
    #[serde(rename = "defaultReminders")]
    pub default_reminders: Option<Vec<EventReminder>>,
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    #[serde(rename = "nextSyncToken")]
    pub next_sync_token: Option<String>,
    pub items: Vec<Event>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub kind: Option<String>,
    pub etag: Option<String>,
    pub id: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "htmlLink")]
    pub html_link: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "colorId")]
    pub color_id: Option<String>,
    pub creator: Option<EventCreator>,
    pub organizer: Option<EventOrganizer>,
    pub start: Option<EventDateTime>,
    pub end: Option<EventDateTime>,
    #[serde(rename = "endTimeUnspecified")]
    pub end_time_unspecified: Option<bool>,
    pub recurrence: Option<Vec<String>>,
    #[serde(rename = "recurringEventId")]
    pub recurring_event_id: Option<String>,
    pub transparency: Option<String>,
    pub visibility: Option<String>,
    #[serde(rename = "iCalUID")]
    pub i_cal_uid: Option<String>,
    pub sequence: Option<i32>,
    pub attendees: Option<Vec<EventAttendee>>,
    #[serde(rename = "attendeesOmitted")]
    pub attendees_omitted: Option<bool>,
    #[serde(rename = "extendedProperties")]
    pub extended_properties: Option<EventExtendedProperties>,
    #[serde(rename = "hangoutLink")]
    pub hangout_link: Option<String>,
    #[serde(rename = "conferenceData")]
    pub conference_data: Option<ConferenceData>,
    pub gadget: Option<EventGadget>,
    pub anyone_can_add_self: Option<bool>,
    pub guests_can_invite_others: Option<bool>,
    pub guests_can_modify: Option<bool>,
    pub guests_can_see_other_guests: Option<bool>,
    pub private_copy: Option<bool>,
    pub locked: Option<bool>,
    pub reminders: Option<EventReminders>,
    pub source: Option<EventSource>,
    pub attachments: Option<Vec<EventAttachment>>,
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCreator {
    pub id: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub self_: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventOrganizer {
    pub id: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub self_: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventDateTime {
    pub date: Option<String>, // yyyy-mm-dd format
    #[serde(rename = "dateTime")]
    pub date_time: Option<DateTime<Utc>>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttendee {
    pub id: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub organizer: Option<bool>,
    pub self_: Option<bool>,
    pub resource: Option<bool>,
    pub optional: Option<bool>,
    #[serde(rename = "responseStatus")]
    pub response_status: Option<String>,
    pub comment: Option<String>,
    #[serde(rename = "additionalGuests")]
    pub additional_guests: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventExtendedProperties {
    pub private: Option<std::collections::HashMap<String, String>>,
    pub shared: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConferenceData {
    // We can flesh this out more if needed, but it's large
    #[serde(rename = "createRequest")]
    pub create_request: Option<CreateConferenceRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConferenceRequest {
    #[serde(rename = "requestId")]
    pub request_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGadget {
    pub type_: Option<String>,
    pub title: Option<String>,
    pub link: Option<String>,
    pub icon_link: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub display: Option<String>,
    pub preferences: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReminders {
    #[serde(rename = "useDefault")]
    pub use_default: Option<bool>,
    pub overrides: Option<Vec<EventReminder>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventReminder {
    pub method: String,
    pub minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSource {
    pub url: Option<String>,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventAttachment {
    #[serde(rename = "fileUrl")]
    pub file_url: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    #[serde(rename = "iconLink")]
    pub icon_link: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyRequest {
    #[serde(rename = "timeMin")]
    pub time_min: DateTime<Utc>,
    #[serde(rename = "timeMax")]
    pub time_max: DateTime<Utc>,
    #[serde(rename = "timeZone")]
    pub time_zone: Option<String>,
    #[serde(rename = "groupExpansionMax")]
    pub group_expansion_max: Option<i32>,
    #[serde(rename = "calendarExpansionMax")]
    pub calendar_expansion_max: Option<i32>,
    pub items: Vec<FreeBusyRequestItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyRequestItem {
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyResponse {
    pub kind: String,
    #[serde(rename = "timeMin")]
    pub time_min: DateTime<Utc>,
    #[serde(rename = "timeMax")]
    pub time_max: DateTime<Utc>,
    pub groups: Option<std::collections::HashMap<String, FreeBusyGroup>>,
    pub calendars: std::collections::HashMap<String, FreeBusyCalendar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyGroup {
    pub errors: Option<Vec<FreeBusyError>>,
    pub calendars: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyCalendar {
    pub errors: Option<Vec<FreeBusyError>>,
    pub busy: Vec<TimePeriod>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeBusyError {
    pub domain: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePeriod {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}
