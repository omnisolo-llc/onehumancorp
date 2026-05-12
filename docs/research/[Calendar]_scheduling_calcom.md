**Title**: Calendar & Scheduling Integration via Cal.com

**Problem Statement**:
Small business owners who rely on appointments (like Leo, a music tutor) struggle with the back-and-forth of scheduling. Manually finding times that work, sending Zoom links, and managing reschedules leads to double-booking and wasted time. They need a way to let clients book directly into their available slots without giving up control of their calendar.

**Research Report**:
Cal.com is an open-source, API-first scheduling infrastructure.
- **Ease of Use for Non-Technical Users**: Very high. Cal.com provides customizable React components ("Atoms") that can be embedded directly into the OHC platform. Users don't need to leave OHC to manage their schedules.
- **Features**: Supports syncing with Google Calendar, Outlook, and Apple Calendar. Automatically handles timezone conversions and can generate conferencing links (Zoom, Meet, Cal Video).
- **Reputation & Reliability**: Highly respected in the developer community as a modern, customizable alternative to Calendly.
- **Pricing**: Cal.com has a generous free tier for individuals (unlimited event types). For OHC, we could leverage their Platform API (though currently under restructuring) or their standard OAuth flow.
- **Cloud vs Standalone**: Cal.com can be self-hosted. In Standalone mode, OHC could theoretically bundle a lightweight version or connect to the user's own Cal.com instance, but realistically, connecting to the managed Cal.com cloud via OAuth is easiest for both modes.

**Design Doc**:
- **Trigger**: User goes to a new "Scheduling" tab in OHC and connects their existing calendar (Google/Apple). They define their working hours and event types (e.g., "30 Min Guitar Lesson").
- **Action**: OHC uses Cal.com Atoms to render the booking flow. When a customer visits the business's public OHC page, they see the Cal.com embed to select a time.
- **User View**: The business owner sees upcoming bookings in their OHC dashboard. Customers receive automated calendar invites with auto-generated video links.
- **Architecture**: OHC will act as an OAuth client to Cal.com. We will embed Cal.com UI components for the booking interface and listen to Cal.com webhooks for `booking.created` and `booking.rescheduled` to update OHC's internal state.

**Implementation Prompt**:
Integrate Cal.com scheduling into the platform. Provide an interface for the business owner to define availability and services. Embed a public booking widget on the business's customer-facing page. Ensure that when a booking is made, both the customer and business owner receive confirmation, and the event appears in the OHC dashboard.

**Priority**: P1 (high)
**Estimated Scope**: Medium
