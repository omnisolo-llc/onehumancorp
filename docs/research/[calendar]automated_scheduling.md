### Title
`[calendar]automated_scheduling`: Implement Two-Way Google Calendar Sync

### Problem Statement
Business owners juggle consultations, classes, and personal appointments. Double-booking is a constant risk, and manually copying appointments from a booking page to their personal calendar is tedious. They need a system that automatically adds new bookings to their calendar and prevents customers from booking times when they are already busy.

### Research Report
- **Tool**: Google Calendar API
- **Pros**: Ubiquitous, highly reliable, excellent documentation. Supports real-time push notifications for changes.
- **Cons**: Requires Google Cloud project setup and OAuth verification, which can be daunting if not handled by the platform.
- **Reputation**: The gold standard for calendar integrations.
- **Pricing**: Free for standard usage limits (which are very high).
- **Ease of Use for Non-Technical Users**: Very intuitive. Users authenticate with their Google account and the system handles the rest.
- **Modes Supported**: Cloud and Standalone.

### Design Doc
- **Trigger**: The business owner clicks "Connect Google Calendar" and completes the OAuth flow.
- **Action**: The OHC API server fetches existing calendar events to block out busy times on the user's OHC booking page. New bookings made via OHC are pushed to the Google Calendar.
- **User View**: A "Calendar" or "Availability" settings page showing connected calendars and options to sync specific event types.

### Implementation Prompt
Create a two-way sync integration with Google Calendar. The system must read the user's availability to prevent double-booking on their OHC booking page. When a customer books a service, the event must be automatically created on the business owner's Google Calendar. The OAuth connection process must be straightforward, handling token refresh automatically in the background.

### Priority
P1

### Estimated Scope
Medium
