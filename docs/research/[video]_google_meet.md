# [Video] Google Meet Integration

**Title**: Integrate Google Meet for Automated Consultation Links

**Problem Statement**: Tutors and consultants waste time manually generating video links for their sessions and emailing them to clients.

**Research Report**:
- **Tool**: Google Meet (via Google Workspace API)
- **Target Persona**: Tutors, Consultants, Coaches.
- **Advantages**: Extremely ubiquitous, most users already have a Google account. Free to use for basic meetings.
- **Risks**: Requires robust Google OAuth integration and managing token refresh cycles.
- **Pricing**: Free for standard Google accounts.
- **Compatibility**: Cloud. Standalone.

**Design Doc**:
- As part of the Calendar integration, users authorize OHC to manage their Google Calendar.
- When a service requiring a video call is booked, OHC automatically attaches a Google Meet link to the calendar event.
- The link is sent to both the business owner and the customer in their confirmation messages.

**Implementation Prompt**: Enhance the Calendar integration to automatically generate and attach Google Meet links to scheduled appointments that require video conferencing.

**Priority**: P1

**Estimated Scope**: Small
