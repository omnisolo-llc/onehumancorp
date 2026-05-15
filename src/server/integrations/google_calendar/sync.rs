use crate::integrations::google_calendar::client::GoogleCalendarClient;
use crate::integrations::google_calendar::models::{
    Event, EventDateTime, EventList, FreeBusyRequest, FreeBusyRequestItem, FreeBusyResponse,
};
use chrono::{DateTime, Utc};

pub struct CalendarSyncService {
    client: GoogleCalendarClient,
}

impl CalendarSyncService {
    pub fn new(access_token: String) -> Self {
        Self {
            client: GoogleCalendarClient::new(access_token),
        }
    }

    /// Fetches all events from a given calendar within a time range
    pub async fn fetch_events(
        &self,
        calendar_id: &str,
        time_min: Option<&str>,
        time_max: Option<&str>,
    ) -> Result<EventList, String> {
        self.client.get_events(calendar_id, time_min, time_max).await
    }

    /// Fetches FreeBusy information for a list of calendars
    pub async fn fetch_freebusy(
        &self,
        calendar_ids: Vec<String>,
        time_min: DateTime<Utc>,
        time_max: DateTime<Utc>,
    ) -> Result<FreeBusyResponse, String> {
        let items: Vec<FreeBusyRequestItem> = calendar_ids
            .into_iter()
            .map(|id| FreeBusyRequestItem { id })
            .collect();

        let request = FreeBusyRequest {
            time_min,
            time_max,
            time_zone: Some("UTC".to_string()),
            group_expansion_max: None,
            calendar_expansion_max: None,
            items,
        };

        self.client.get_freebusy(&request).await
    }

    /// Pushes a new booking to Google Calendar
    pub async fn push_booking(
        &self,
        calendar_id: &str,
        summary: &str,
        description: &str,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    ) -> Result<Event, String> {
        let event = Event {
            kind: Some("calendar#event".to_string()),
            etag: None,
            id: None,
            status: Some("confirmed".to_string()),
            html_link: None,
            created: None,
            updated: None,
            summary: Some(summary.to_string()),
            description: Some(description.to_string()),
            location: None,
            color_id: None,
            creator: None,
            organizer: None,
            start: Some(EventDateTime {
                date: None,
                date_time: Some(start_time),
                time_zone: Some("UTC".to_string()),
            }),
            end: Some(EventDateTime {
                date: None,
                date_time: Some(end_time),
                time_zone: Some("UTC".to_string()),
            }),
            end_time_unspecified: None,
            recurrence: None,
            recurring_event_id: None,
            transparency: None,
            visibility: None,
            i_cal_uid: None,
            sequence: None,
            attendees: None,
            attendees_omitted: None,
            extended_properties: None,
            hangout_link: None,
            conference_data: None,
            gadget: None,
            anyone_can_add_self: None,
            guests_can_invite_others: None,
            guests_can_modify: None,
            guests_can_see_other_guests: None,
            private_copy: None,
            locked: None,
            reminders: None,
            source: None,
            attachments: None,
            event_type: None,
        };

        self.client.create_event(calendar_id, &event).await
    }
}
