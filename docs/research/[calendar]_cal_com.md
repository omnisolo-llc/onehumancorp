# Calendar & Scheduling: Cal.com

**Title**: Integrate Cal.com for Zero-Config Booking & Calendar Sync

**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.

**Research Report**:
- Cal.com is an open-source scheduling infrastructure. It handles timezone math, calendar conflict resolution, and booking pages out-of-the-box.
- **Pricing**: Included in OHC platform costs.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).

**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types.
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.

**Implementation Prompt**: Embed Cal.com's infrastructure so users can sync their personal calendars and provide a public booking widget on their storefront that prevents double-booking.
- **Priority**: P1
- **Estimated Scope**: Medium
- **Acceptance Criteria**:
  - Storefront provides a seamless booking widget.
  - Zero-config sync with personal calendars prevents double-booking.

**Strategy**: Leverage Cal.com's robust scheduling infrastructure to provide native booking functionality.
