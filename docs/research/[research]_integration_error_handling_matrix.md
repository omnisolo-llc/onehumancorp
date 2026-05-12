# Deep Dive: Integration Error Handling & Recovery Matrix

## Executive Summary
This document serves as the third appendix to the Q3 Tool Integration Research report. It details the specific error handling strategies and user-facing recovery flows for the most common failure states encountered when integrating with third-party APIs. The goal is to ensure that when an integration fails, the system degrades gracefully and empowers the user to fix the issue without contacting support.

## 1. OAuth Token Expiration (The "Silent Killer")

**The Scenario:** A user connected their Google Calendar 6 months ago. The Refresh Token provided by Google has expired or was manually revoked by the user from their Google Account security dashboard.

**System State:** The OHC backend attempts to push a new booking to Google Calendar. The Google API returns an HTTP 401 Unauthorized.

**Recovery Flow:**
1.  **Backend Detection:** The `GoogleCalendarAdapter` intercepts the 401. It marks the specific `oauth_connection` record as `EXPIRED` in the database.
2.  **Task Queue Handling:** The background task (the event creation) is moved from the active queue to a `PausedQueue`. It is *not* marked as failed yet.
3.  **User Notification:** An urgent, high-visibility banner is displayed across the top of the OHC dashboard for that user: "Action Required: Your Google Calendar connection has expired. Appointments are not syncing."
4.  **User Action:** The user clicks a "Reconnect Now" button on the banner, which immediately triggers the standard OAuth consent flow.
5.  **Reconciliation:** Upon successful re-authentication, the OHC backend saves the new tokens, clears the `EXPIRED` flag, and automatically releases all tasks in the `PausedQueue` for that specific integration back into the active queue. The missed event is finally pushed to Google Calendar.

## 2. Rate Limiting (HTTP 429 Too Many Requests)

**The Scenario:** A user runs a massive email campaign, triggering 5,000 updates to Mailchimp simultaneously, exhausting the Mailchimp API rate limits for the OHC master API key.

**System State:** Mailchimp returns HTTP 429.

**Recovery Flow:**
1.  **Backend Detection:** The `MailchimpAdapter` intercepts the 429. It parses the `Retry-After` header provided by Mailchimp.
2.  **Circuit Breaking:** A global circuit breaker trips for the Mailchimp integration, halting all outbound traffic to that specific provider for the duration specified in the `Retry-After` header.
3.  **Task Queue Handling:** All tasks currently being processed are aborted and requeued with a delay equal to the `Retry-After` window.
4.  **User Notification:** In this scenario, the user is *not* notified immediately. Rate limits are an infrastructure concern, and exposing them causes unnecessary panic. The system handles it silently.
5.  **Reconciliation:** Once the circuit breaker resets, the queue resumes processing at a throttled rate (using a Token Bucket algorithm) to prevent immediately triggering the 429 again.

## 3. Webhook Delivery Failure (The "Missing Payment")

**The Scenario:** A customer successfully pays an invoice via Mercado Pago. Mercado Pago attempts to send a webhook to OHC, but the OHC Cloud Relay is experiencing a momentary network blip and returns a 503 Service Unavailable.

**System State:** The invoice remains "Pending" in OHC, despite the customer having paid.

**Recovery Flow:**
1.  **Provider Retries:** Mercado Pago (like Stripe and others) employs its own exponential backoff for webhooks. It will retry the webhook 1 minute later, then 5 minutes later, etc.
2.  **Idempotent Ingestion:** When the webhook eventually arrives and succeeds, the OHC `WebhookIngester` uses the unique Event ID provided by Mercado Pago as an idempotency key.
3.  **Proactive Polling (Fallback):** If the webhook is delayed by more than 15 minutes, the user might notice the discrepancy. The OHC Invoice UI must include a "Check Payment Status" button. Clicking this triggers a synchronous, direct API call to Mercado Pago (`GET /v1/payments/{id}`) to bypass the webhook system entirely and manually reconcile the invoice state.

## 4. Unmapped Provider Errors (The "Unknown Unknown")

**The Scenario:** A user tries to post to Instagram via Ayrshare. The post contains an image format that Instagram secretly stopped supporting yesterday. Ayrshare passes back an obscure, undocumented error code from Meta.

**System State:** Ayrshare returns HTTP 400 Bad Request with a payload like `{"error": {"code": 190, "message": "Unsupported format type 8"}}`.

**Recovery Flow:**
1.  **Backend Detection:** The `AyrshareAdapter` recognizes it's a 4xx error (user error, not retryable) but cannot map the specific code to a known, human-readable OHC error string.
2.  **Task Queue Handling:** The task is marked as `FAILED` and moved to the Dead Letter Queue.
3.  **User Notification:** The UI displays a notification: "Your social media post failed to publish. The provider returned an unrecognized error."
4.  **Diagnostic Capture:** The raw JSON error payload is captured and displayed in an "Advanced Details" accordion in the UI. This allows the user to potentially Google the error themselves or provide it directly to the OHC support team, vastly accelerating resolution time.
5.  **Telemetry:** The unknown error code is logged as an anomaly in DataDog. The engineering team reviews these anomalies weekly to add new specific mappings to the Data Normalization Layer, turning "Unknown Unknowns" into "Known Knowns" over time.
