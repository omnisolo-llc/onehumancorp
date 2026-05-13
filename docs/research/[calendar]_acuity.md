# Title: Acuity Scheduling Integration for Automated Booking

## Problem Statement
Service-based businesses (salons, consultants, tutors) spend excessive time going back and forth with clients to schedule appointments. They need a way to let clients book available times directly, without double-booking. For a non-technical owner, setting up custom booking systems is daunting; they need a simple way to plug their existing calendar into a booking page that syncs with their OHC dashboard.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Manual scheduling via email/SMS is error-prone and time-consuming.
- **Competitors:** Calendly is popular, but Acuity Scheduling (owned by Squarespace) is heavily favored by service businesses because it handles payments, complex appointment types, and intake forms better out-of-the-box.
- **Acuity API:** Acuity offers a robust REST API and webhooks for appointment creation, cancellation, and rescheduling.
- **Reputation & Ease of Use:** Acuity is highly regarded for its deep customization. For the user, connecting it should be a simple OAuth flow.
- **Pricing:** Acuity starts around $16-$20/month.

**Key Advantages:**
- Solves the scheduling pain point completely.
- Acuity handles the heavy lifting of calendar sync (Google/Outlook/iCloud) and timezone math.

**Integration Risks:**
- Handling recurring appointments and complex rescheduling logic correctly in the OHC UI.

**Environment Support:**
- **Cloud:** Webhooks work perfectly.
- **Standalone:** Requires polling the Acuity API periodically or a relay, similar to other webhook-reliant integrations.

## Design Doc
**Trigger:**
User goes to "Integrations" -> "Calendar" and selects "Connect Acuity". They authorize OHC via OAuth.

**Action:**
OHC syncs the user's Acuity appointment types. The user can then generate booking links directly from OHC or embed the Acuity widget on their OHC-hosted storefront. Webhooks notify OHC when a new appointment is booked.

**User View:**
The business owner sees a "Bookings" tab in OHC that displays a read-only view of upcoming Acuity appointments. They can generate a booking link to send to clients via the unified inbox with one click.

## Implementation Prompt
Integrate Acuity Scheduling to provide read-only booking visibility and link sharing.
- Build an OAuth flow to connect an Acuity account.
- Ingest Acuity appointments via webhooks and display them in a "Bookings" dashboard within OHC.
- Provide a quick-action button in the Unified Inbox to insert the user's default Acuity booking link into a message.
- (Do not prescribe database models; focus on delivering the connection flow and the dashboard UI.)

## Priority
P2

## Estimated Scope
Medium
