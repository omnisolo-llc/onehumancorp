## [Calendar] Issue Brief: Calendar Sync & Meeting Link Generation

**Title**: Scout 🔍: Google/Outlook Calendar Sync & Zoom Meeting Generation
**Problem Statement**: Service-based businesses (like tutors or consultants) waste time going back and forth with clients to schedule appointments. They need a way to let clients book available times directly and have meeting links (e.g., Zoom) auto-generated without manually managing calendar invites.
**Research Report**:
- **Tools Evaluated**: Cal.com, Google Calendar API, Zoom API.
- **Evaluation**: Cal.com is an excellent open-source infrastructure that handles complex scheduling logic (timezones, conflicts). Integrating Cal.com's core, along with Google Calendar API for sync and Zoom API for link generation, is the best path.
- **Ease of Use**: User clicks "Connect Calendar" (Google/Outlook) and "Connect Zoom". OHC handles the rest, generating a public booking page.
- **Pricing**: Cal.com has an open-source/self-hosted version. Google Calendar API is free for typical usage. Zoom API requires the user to have a Zoom account.
- **Cloud vs. Standalone**: Cal.com can be self-hosted, making it viable for both Cloud and Standalone modes. OAuth flows for Google/Zoom work in both, provided standalone users can route OAuth callbacks.
**Design Doc**:
- User connects Google/Outlook Calendar and Zoom via the Settings page.
- OHC presents a public booking widget on the user's storefront, powered by scheduling logic that checks real-time availability.
- When a customer books, OHC creates the event in the user's connected calendar and calls the Zoom API to generate a meeting link.
- Confirmation emails to the customer include the calendar invite and Zoom link.
**Implementation Prompt**: Integrate with Cal.com (or build native scheduling logic) to provide a public booking widget. Implement OAuth for Google Calendar and Outlook to sync availability. Implement Zoom OAuth to auto-generate meeting links upon booking. Ensure the booking flow handles timezone conversions and prevents double-booking.
**Priority**: P0
**Estimated Scope**: Large
