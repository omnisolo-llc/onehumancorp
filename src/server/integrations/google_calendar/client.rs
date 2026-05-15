use crate::integrations::google_calendar::models::{
    CalendarList, Event, EventList, FreeBusyRequest, FreeBusyResponse,
};
use reqwest::{Client, Error, Method, RequestBuilder, Response, StatusCode};
use std::time::Duration;

const BASE_URL: &str = "https://www.googleapis.com/calendar/v3";
const MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone)]
pub struct GoogleCalendarClient {
    client: Client,
    access_token: String,
}

impl GoogleCalendarClient {
    pub fn new(access_token: String) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_default(),
            access_token,
        }
    }

    fn request(&self, method: Method, url: &str) -> RequestBuilder {
        self.client
            .request(method, url)
            .bearer_auth(&self.access_token)
    }

    async fn execute_with_retry(
        &self,
        mut request_builder: RequestBuilder,
    ) -> Result<Response, String> {
        // Since RequestBuilder doesn't implement Clone natively, we have to rely on try_clone
        let mut retries = 0;

        loop {
            // Need to clone the builder for retries
            let req = request_builder
                .try_clone()
                .ok_or("Failed to clone request builder for retry")?;

            let result = req.send().await;

            match result {
                Ok(response) => {
                    let status = response.status();
                    if status.is_success() {
                        return Ok(response);
                    } else if status == StatusCode::TOO_MANY_REQUESTS
                        || status.is_server_error()
                    {
                        if retries >= MAX_RETRIES {
                            return Err(format!(
                                "Request failed after {} retries: HTTP {}",
                                MAX_RETRIES, status
                            ));
                        }
                        retries += 1;
                        let delay = 2_u64.pow(retries) * 100; // Exponential backoff
                        tokio::time::sleep(Duration::from_millis(delay)).await;
                    } else if status == StatusCode::UNAUTHORIZED {
                        return Err("Unauthorized: Invalid or expired access token".to_string());
                    } else {
                        return Err(format!("Request failed with HTTP status: {}", status));
                    }
                }
                Err(err) => {
                    if retries >= MAX_RETRIES {
                        return Err(format!(
                            "Request failed after {} retries: {}",
                            MAX_RETRIES, err
                        ));
                    }
                    retries += 1;
                    let delay = 2_u64.pow(retries) * 100;
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                }
            }
        }
    }

    pub async fn get_calendar_list(&self) -> Result<CalendarList, String> {
        let url = format!("{}/users/me/calendarList", BASE_URL);
        let req = self.request(Method::GET, &url);
        let response = self.execute_with_retry(req).await?;
        response
            .json::<CalendarList>()
            .await
            .map_err(|e| format!("Failed to parse calendar list: {}", e))
    }

    pub async fn get_events(
        &self,
        calendar_id: &str,
        time_min: Option<&str>,
        time_max: Option<&str>,
    ) -> Result<EventList, String> {
        let url = format!("{}/calendars/{}/events", BASE_URL, calendar_id);
        let mut req = self.request(Method::GET, &url);

        if let Some(min) = time_min {
            req = req.query(&[("timeMin", min)]);
        }
        if let Some(max) = time_max {
            req = req.query(&[("timeMax", max)]);
        }

        let response = self.execute_with_retry(req).await?;
        response
            .json::<EventList>()
            .await
            .map_err(|e| format!("Failed to parse event list: {}", e))
    }

    pub async fn create_event(&self, calendar_id: &str, event: &Event) -> Result<Event, String> {
        let url = format!("{}/calendars/{}/events", BASE_URL, calendar_id);
        let req = self.request(Method::POST, &url).json(event);
        let response = self.execute_with_retry(req).await?;
        response
            .json::<Event>()
            .await
            .map_err(|e| format!("Failed to parse created event: {}", e))
    }

    pub async fn get_freebusy(&self, request: &FreeBusyRequest) -> Result<FreeBusyResponse, String> {
        let url = format!("{}/freeBusy", BASE_URL);
        let req = self.request(Method::POST, &url).json(request);
        let response = self.execute_with_retry(req).await?;
        response
            .json::<FreeBusyResponse>()
            .await
            .map_err(|e| format!("Failed to parse freebusy response: {}", e))
    }
}
