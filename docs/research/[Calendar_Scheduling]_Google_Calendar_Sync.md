## 2. Calendar & Scheduling
**Title**: Integrate Google Calendar for Auto-Scheduling and Sync
**Problem Statement**: Small business owners spend hours doing "email ping-pong" to find a time to meet with clients. When they get booked, they often forget to manually update their personal calendar, leading to embarrassing double-bookings.
**Research Report**:
- **Tool**: Google Calendar API
- **Problem it solves for which persona**: Helps consultants, tutors, and salon owners automate booking and prevent double-booking.
- **Ease of Use**: Business owner clicks "Connect Google Calendar", completes OAuth, and it's done.
- **Pricing**: Free (included in Google Workspace / free Gmail accounts).
- **Key Advantages**: Ubiquitous usage; reliable conflict resolution; free.
- **Integration Risks**: Managing OAuth refresh tokens can be brittle; timezone complexities between client and owner.
- **Environment**: Works well in Cloud via standard OAuth. Standalone mode requires users to supply their own Google Cloud Project credentials.
**Design Doc**:
- **Trigger**: OHC booking widget checks available slots; client selects a time.
- **Action**: OHC creates an event on the owner's Google Calendar and sends a confirmation email to the client.
- **User Interface**: Owner sees a "Connect Calendar" button. Once connected, they can generate a booking link to share with clients.
**Implementation Prompt**: Create an OAuth flow for Google Calendar. Implement two-way sync: read free/busy times from the owner's calendar to power a public booking page, and create new calendar events when a client books a slot. Include basic timezone handling.
**Priority**: P1
**Estimated Scope**: Medium
