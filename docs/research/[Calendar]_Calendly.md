**Title**: Implement Calendly Sync for Automated Booking
**Problem Statement**: Scheduling appointments, consultations, or classes involves endless back-and-forth emails. Small business owners often double-book themselves because they manually copy appointments into their personal calendars.
**Research Report**: Calendly is the industry standard for scheduling. It seamlessly syncs with Google Calendar and Outlook to prevent double bookings. The user interface for the person booking is foolproof, and the setup for the business owner is simple. There is a robust free tier, with premium features starting at $10/mo.
**Design Doc**:
- **Trigger**: Business owner pastes their Calendly API key or authenticates via OAuth.
- **Action**: OHC automatically syncs Calendly events into the OHC internal calendar view and triggers reminder flows.
- **User Experience**: The business owner views their upcoming appointments directly inside OHC without needing to check Calendly, and client details are automatically added to the customer list.
**Implementation Prompt**: Create a "Schedule" tab in OHC that displays upcoming appointments fetched from Calendly. Provide a simple connection flow for the business owner to link their Calendly account. New bookings should automatically generate a customer profile in OHC.
**Priority**: P1
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.
