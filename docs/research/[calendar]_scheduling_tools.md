# Calendar & Scheduling Tools

**Title**: Integrate Scheduling and Calendar Sync (Cal.com, Acuity)

**Problem Statement**:
Service-based business owners (like Carlos the Handyman and Leo the Music Tutor) lose time going back and forth with clients to find meeting times. They need an easy way for customers to book time slots that automatically sync with their personal Google or Apple calendars, preventing double bookings.

**Research Report**:
We evaluated Cal.com and Acuity Scheduling.
- **Cal.com**: Open-source, developer-friendly scheduling infrastructure.
  - *Ease of Use*: Extremely easy. OHC can white-label the booking experience using Cal.com's API so the user never leaves the OHC app.
  - *Pricing*: Free tier available for individuals. Open-source nature aligns well with OHC's architecture.
  - *Reputation*: Highly respected, modern API, robust webhook support.
- **Acuity Scheduling**: Older, very feature-rich, but less developer-centric.
  - *Ease of Use*: Requires the user to manage a separate Acuity account. Harder to fully white-label within OHC.
  - *Pricing*: Starts at ~$16/mo. No robust free tier.
- **Recommendation**: Use Cal.com as the scheduling engine. It provides the necessary APIs to create a seamless, native-feeling booking experience within OHC.

**Design Doc**:
- **Trigger**: Business owner enables "Bookings" for a service in the OHC app and connects their Google/Apple Calendar via OAuth.
- **Action**: OHC provisions a managed Cal.com link/event type in the background. The OHC public storefront embeds the booking widget. When a booking occurs, Cal.com handles calendar conflict resolution and sync. OHC receives a webhook to trigger internal workflows (e.g., sending a confirmation email via the "Operations" agent).
- **User Experience**: The business owner sees their upcoming appointments in the OHC app dashboard. The customer sees a seamless booking calendar on the business's website.

**Implementation Prompt**:
Integrate Cal.com's API to provision managed booking links for users. Build a Flutter UI component that embeds the booking flow on the user's public OHC site. Create a dashboard view for the business owner to see their schedule. Implement webhook handlers to log bookings into the OHC database and trigger necessary agent follow-ups.

**Priority**: P0
**Estimated Scope**: Medium
