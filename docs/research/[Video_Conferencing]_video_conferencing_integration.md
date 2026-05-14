# [Video Conferencing] OHC Tool Integration Research Brief: Auto-Generated Meeting Links

## Title
Auto-Generating Zoom/Meet Links for Online Lessons or Consultations

## Problem Statement
Small business owners who offer online services (like tutoring, consulting, or remote fitness classes) spend too much time manually generating video conference links and emailing them to clients after an appointment is booked. This manual process is inefficient and often leads to confusion or missed meetings if the link isn't sent promptly.

## Research Report
Video conferencing integration is critical for remote services. The two dominant players are Zoom and Google Meet, both offering robust features for programmatic link generation.

**Evaluated Tools:**

1. **Zoom (zoom.us)**
    *   **Focus:** Enterprise and SMB video communications.
    *   **Pros:** Ubiquitous, highly reliable. Supports features like waiting rooms and recordings.
    *   **Cons:** Requires users to have an account and authenticate.
    *   **Pricing:** Free basic tier (40 min limit), Pro starts at $15.99/mo.
    *   **Modes:** Cloud and Standalone.

2. **Google Meet (meet.google.com)**
    *   **Focus:** Integrated video conferencing within Google Workspace.
    *   **Pros:** Seamless integration if the user already uses Google Calendar.
    *   **Cons:** Deeply tied to the calendar ecosystem; generating standalone links without calendar events can be trickier.
    *   **Pricing:** Included in Google Workspace (starts at $6/mo).
    *   **Modes:** Cloud and Standalone.

3. **Whereby (whereby.com)**
    *   **Focus:** Browser-based video meetings with a focus on embedding.
    *   **Pros:** Extremely easy to embed directly into OHC without requiring the user or client to download software.
    *   **Cons:** Less brand recognition.
    *   **Pricing:** Free tier available, Pro starts at $14.99/mo.
    *   **Modes:** Cloud and Standalone.

**Recommendation:**
Given that we are also recommending a dedicated scheduling tool (which natively handles video integrations), the best approach is to leverage those existing video conferencing capabilities where possible.

If OHC needs direct video conferencing integration independent of scheduling, the industry standard should be supported first, followed by ecosystem-tied solutions. An embed-focused option is a strong alternative if we want to offer deeply embedded video calls directly within the OHC platform UI.

## Design Doc
**Integration Approach: Video Integration (Independent of Scheduling)**

1.  **Authentication:**
    *   Business owner navigates to OHC settings and connects their video conferencing account via standard authorization flows.
    *   OHC stores the necessary tokens securely.

2.  **Meeting Generation (Trigger):**
    *   When an Appointment is created in OHC, OHC checks if the service type requires a video link.
    *   If yes, OHC uses the stored tokens to request the creation of a scheduled meeting from the external service.

3.  **Client Notification (Action):**
    *   The external service returns the meeting URL.
    *   OHC saves the URL to the Appointment record.
    *   OHC automatically sends an email/SMS to the client containing the appointment details and the video link.

## Implementation Prompt
**Objective:** Implement video integration for auto-generating meeting links.

**Acceptance Criteria:**
1.  Implement an authorization flow to authenticate users with the external video service and store their tokens securely.
2.  Create an integration client to interact with the external service for creating meetings.
3.  Add logic to the Appointment creation process: if the service requires video, call the integration client to generate a link.
4.  Store the generated video link in the Appointment model and include it in customer notifications.

## Priority
P2

## Estimated Scope
Medium
