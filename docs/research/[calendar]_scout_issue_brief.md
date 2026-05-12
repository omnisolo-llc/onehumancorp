# Calendar & Scheduling Research Brief

## Title
Automated Client Scheduling and Calendar Sync

## Problem Statement
Small business owners spend countless hours playing "email ping-pong" trying to find a suitable meeting time with clients. Manual scheduling leads to double-booking, missed appointments due to timezone confusion, and lost productivity. They need a seamless way for clients to book available slots that automatically syncs with their existing calendars (Google, Outlook).

## Research Report
### Market Context
The scheduling software market is mature, dominated by tools like Calendly and Acuity. However, many small businesses still find these tools disconnected from their primary CRM or invoicing systems. A tightly integrated scheduling tool reduces context switching.

### Tool Evaluations

#### 1. Calendly
- **Ease of Use:** Very high. Industry standard for booking links.
- **Pricing:** Free basic tier; Pro is $12/user/month.
- **Capabilities:** Excellent timezone handling, multi-calendar sync, Zoom integration.
- **Reputation:** Unquestionably reliable, but branding is strong and removing it costs extra.

#### 2. Cal.com
- **Ease of Use:** High. Open-source alternative to Calendly.
- **Pricing:** Free for individuals.
- **Capabilities:** Extensive API, webhooks, self-hosting options.
- **Reputation:** Developer-friendly, modern, rapidly growing.

#### 3. Acuity Scheduling (Squarespace)
- **Ease of Use:** Moderate. Very feature-rich.
- **Pricing:** Starts at $16/month.
- **Capabilities:** Deep customization, payment collection at booking, class scheduling.
- **Reputation:** Preferred by wellness professionals and salons.

### Recommended Direction
Instead of building a scheduling engine from scratch (which involves complex timezone and daylight savings logic), OHC should integrate deeply with an open API like Cal.com or build a lightweight wrapper around Google/Microsoft Calendar APIs for native syncing.

## Design Doc
### Trigger & Action
1. **Trigger:** Business owner shares a booking link or embeds it on their site. Client selects a time.
2. **Action:** The system checks real-time availability against the owner's connected calendar. Upon booking, a calendar event is created, and an email confirmation (with an auto-generated meeting link) is sent to both parties.
3. **User View:** The business owner sees upcoming appointments in their OHC dashboard. They can configure working hours, buffer times, and meeting durations.

### Environment Support
- **Cloud Mode:** Standard OAuth flows for Google/Microsoft.
- **Standalone Mode:** Requires user to provide API credentials or use a local CalDAV sync for native calendar apps.

## Implementation Prompt
Implement a "Booking Page" feature that allows users to connect their Google Calendar.
- The user sets their availability (e.g., Mon-Fri, 9 AM - 5 PM).
- A public booking page is generated showing available slots, respecting existing events on their Google Calendar (no double-booking).
- When a client books, an event is automatically added to the owner's Google Calendar.
- The UI must allow configuring meeting duration (e.g., 30 mins, 60 mins).
- Acceptance criteria include successfully booking a time slot from the public page and seeing it appear in the connected Google Calendar immediately.

## Priority
P0 (Critical)

## Estimated Scope
Medium

### Extended Calendar Integration Analysis
#### Security & Privacy
Handling calendar events exposes highly sensitive data about the user's personal life. The integration must implement robust scopes, requesting only what is needed (e.g. read/write for specific events rather than full mailbox access). Privacy policies must be completely transparent regarding how calendar data is processed.

#### Reliability & Timezones
Timezone synchronization is notoriously difficult. If the client books in UTC+2 but the business owner operates in UTC-5, the integration must flawlessly translate event boundaries. Edge cases around daylight savings boundaries must be carefully addressed. Tests should specifically mock dates across timezone transitions.

#### Custom Buffer Times
A crucial feature for consultants or service professionals is travel time or prep time. The scheduling logic must support pre- and post-meeting buffers to ensure back-to-back bookings do not cause cascading delays.

#### Future Extensibility
While starting with Google Calendar, the data model must be generic enough to easily plug in Outlook Calendar or Apple iCloud calendars in the future without schema migrations.

### User Persona Match
- **Fatima (Boutique Owner):** Low value. She mostly manages physical inventory, not meetings.
- **Carlos (Consultant):** High value. He lives by his calendar and needs clients to book his consultation slots directly.

### Competitive Benchmarking
Compared to simply embedding a Calendly widget, a native OHC scheduler provides the advantage of automatic invoicing integration. A booked appointment can immediately generate an invoice, reducing the friction to get paid.

### Conclusion
A native, deeply integrated calendar synchronization engine is a foundational component for service-based businesses, greatly enhancing their productivity and professional appearance.
