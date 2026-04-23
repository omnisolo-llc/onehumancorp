# Integrate Google Calendar & Meet for Automated Booking

## Problem Statement
Small business owners who offer services or consultations (like Leo, the Music Tutor) currently lack a seamless way to schedule appointments online and automatically generate video conference links. When a customer books a time slot, the business owner must manually create a calendar event, generate a Google Meet link, and email the details to the customer. This manual process is time-consuming, prone to errors, and doesn't scale as the business grows. Non-technical users need an automated booking system that synchronizes their availability and instantly provisions video links without leaving the OHC platform.

## Research Report
### Tool Evaluated: Google Workspace APIs (Google Calendar & Google Meet)
Google Calendar is the industry standard for personal and professional scheduling, making it highly familiar to non-technical users. Google Meet is deeply integrated into Google Calendar; creating an event with conference data automatically provisions a Meet link.

- **Ease of Use for Non-Technical Users:** Very high. Users are already familiar with the "Sign in with Google" flow (OAuth 2.0). Once connected, the integration runs invisibly in the background.
- **Pricing:**
  - *Cloud Mode (Multi-tenant):* Free for standard API usage within Google Cloud's generous free tier (up to 1,000,000 Calendar API requests/day). The primary cost is managing OAuth tokens and refresh logic.
  - *Standalone Mode:* Free, but requires the user to provide their own Google Cloud OAuth Client ID, which adds friction for non-technical users unless OHC provides a proxy service or clear instructions.
- **Reputation & Reliability:** Excellent. Google's APIs are highly reliable with extensive documentation.
- **Key Advantages:** Ubiquity, built-in Meet link generation, robust handling of timezones and recurring events.
- **Risks:**
  - OAuth token expiration and revocation handling.
  - Sync conflicts (e.g., if a user deletes an event directly in Google Calendar, OHC needs to reflect that, likely via Webhooks/Push Notifications).
  - Standalone mode requires users to set up Google Cloud projects to get OAuth credentials, which is too complex for non-technical users.

## Design Doc
### User Experience
1. **Connection:** In the OHC "Operations" or "Settings" department, the user clicks "Connect Google Calendar". A standard Google OAuth popup appears.
2. **Configuration:** The user selects which of their calendars to check for conflicts (e.g., "Personal", "Work") and which calendar to add new OHC bookings to.
3. **Service Setup:** When setting up a service (e.g., "1-Hour Guitar Lesson"), the user toggles "Add Video Link".
4. **Customer Flow:** A customer books a slot on the user's public page. They immediately receive a confirmation email containing the date, time, and Google Meet link.
5. **Owner View:** The appointment appears in the OHC dashboard and automatically syncs to the owner's Google Calendar with the Meet link attached.

### Integration Architecture
- **Trigger:** A customer completes a booking flow in OHC.
- **Action:** OHC requests the user's saved OAuth credentials (handled securely). OHC calls the Google Calendar API to create an event, injecting the customer's email as an attendee and requesting `conferenceData` to generate a Meet link.
- **Webhook Sync:** OHC registers a webhook with Google Calendar to listen for event updates (e.g., cancellations or reschedules made directly in Google Calendar) and updates the OHC database accordingly.

## Implementation Prompt
**User-Facing Outcome:**
Business owners can connect their Google account to OHC. Once connected, any bookings made through their OHC storefront will automatically check their Google Calendar for conflicts, block out the booked time, and generate a Google Meet link if it's an online service. Both the owner and the customer receive calendar invites with the link.

**Acceptance Criteria:**
1. A user can authenticate with Google via a "Sign in with Google" button in the OHC dashboard.
2. Users can designate specific services as "Online Meetings".
3. When a customer books an online meeting service, an event is created in the owner's Google Calendar containing a valid Google Meet link.
4. The customer receives an email with the appointment details and the Google Meet link.
5. If an event is cancelled in OHC, the Google Calendar event is deleted.
6. The integration must be resilient to token expiration (auto-refreshing tokens).

## Priority
P1

## Estimated Scope
Medium
