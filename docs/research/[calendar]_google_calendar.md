## [Calendar] Google Calendar API Integration
**Title**: Native Calendar Sync for Automated Booking
**Problem Statement**: Service providers like Carlos (Handyman) and Leo (Music Tutor) lose time going back and forth over email/text to find a time to meet. They need a way for customers to simply click a link, see available times, and book a slot directly synchronized with their existing Google Calendar or Apple Calendar, without confusing third-party scheduling tools like Calendly.
**Research Report**:
- **Strategy**: Direct Google Calendar API / CalDAV integration
- **Target Persona**: Carlos (Handyman), Leo (Music Tutor)
- **Advantages**: Zero configuration needed beyond logging in. Avoids confusing users with Calendly setups. Fully integrated into OHC's existing booking flow.
- **Risks**: Handling complex timezone logic internally.
- **Pricing**: Free API usage.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).
**Design Doc**:
- User goes to Sales dashboard and connects their Google account.
- OHC reads busy blocks directly from Google Calendar to calculate availability for predefined event types (e.g., "30-min Consultation").
- When a customer clicks to book, they use OHC's native booking widget.
- Upon successful booking, OHC pushes the event directly to Google Calendar and records the appointment in the Operations dashboard.
- **AI Integration**: The Operations Agent monitors the calendar and alerts the business owner if they have back-to-back appointments without buffer times, suggesting schedule optimizations.
**Implementation Prompt**: Create a native booking widget and Google Calendar OAuth integration. Calculate availability based on existing calendar events and sync new bookings directly to Google Calendar.
**Priority**: P1
**Estimated Scope**: Medium
