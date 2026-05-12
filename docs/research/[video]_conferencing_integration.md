**Title**: Auto-generating Video Links (Zoom & Google Meet)
**Problem Statement**: Service providers who offer online consultations or lessons (like Leo, the music tutor) manually create a Zoom or Google Meet link for every booked appointment and email it to the client. This is repetitive manual labor that often results in forgotten links and confused clients at the time of the meeting.
**Research Report**:
- **Zoom**: Proprietary videotelephony software. Dominant market player for video calls.
- **Google Meet**: Video communication service by Google. Highly integrated with Google Calendar.
- **Ease of Use**: Both have robust APIs for programmatically generating meeting links. Once authorized, OHC can create meetings on the user's behalf silently.
- **Pricing**: Both have free tiers (Zoom limits to 40 mins for >2 people; Meet is generally free for 1-on-1s). API access is usually included.
- **Reputation**: These are the two most trusted and widely used video conferencing platforms globally.
- **Cloud/Standalone**: Handled primarily via OAuth 2.0 flows, which work seamlessly in both environments.
**Design Doc**:
- **Trigger**: An appointment is booked via OHC's scheduling module (e.g., via Calendly integration).
- **Action**: OHC determines the user's preferred video provider, calls the Zoom/Meet API to create a scheduled meeting, and attaches the resulting `join_url` to the appointment record.
- **UI**: In the OHC appointment detail view, the location shows as "Online" with a clickable "Join Video Call" button for both the merchant and the client. The client receives the link in their automated confirmation email.
**Implementation Prompt**: Implement an OAuth flow for Zoom and Google Workspace. When a new appointment is created in OHC, intercept the event and conditionally call the Zoom or Google Meet API to generate a meeting link based on merchant settings. Store this link in the database and surface it in the appointment details UI and customer notification templates.
**Priority**: P1
**Estimated Scope**: Medium
