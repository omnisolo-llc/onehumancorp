## [Video Conferencing] Issue Brief: Auto-Generated Meeting Links

**Title**: Scout 🔍: Integrate Google Meet for Automated Online Lessons
**Problem Statement**:
For digital service providers like Leo (Music Tutor), manually creating Zoom or Google Meet links for every booked lesson and emailing them to the student is prone to human error (e.g., forgetting to send the link or sending the wrong one).
**Research Report**:
- **Tool**: Google Workspace API (Google Meet) or Zoom API.
- **Evaluation**: Google Meet is often preferred as it can be automatically attached to any Google Calendar event created during the booking process. Zoom requires a separate OAuth flow.
- **Ease of Use**: Zero extra effort if the user has already connected Google Calendar for availability syncing. The system automatically provisions the link.
- **Pricing**: Free if using the user's existing Google Calendar/Meet integration.
- **Cloud vs. Standalone**: Works natively in both.
**Design Doc**:
- When setting up a service, the user toggles "This is an online meeting".
- When a customer books the service, the OHC backend creates a Google Calendar event.
- The calendar event is configured to auto-generate a Google Meet conference link.
- The confirmation email sent to the customer includes this generated Meet link.
**Implementation Prompt**:
Extend the calendar booking flow to support online meetings. When a service is marked as "online", ensure the Google Calendar event creation request includes the conference data parameters to auto-generate a Google Meet link. Extract this link from the response and include it in the customer's confirmation email and the business owner's dashboard.
**Priority**: P1
**Estimated Scope**: Small
