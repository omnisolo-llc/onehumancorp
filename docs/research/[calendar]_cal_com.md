# [Calendar] One-Click Booking with Cal.com

**Title**: Implement Cal.com for Automated Scheduling

**Problem Statement**:
Small business owners spend too much time going back and forth over email or text to find a time to meet with clients. They need a simple link they can share that shows their real availability and lets the client pick a time, automatically creating calendar events for both parties.

**Research Report**:
- **Evaluated Tools**: Cal.com, Calendly, Acuity Scheduling.
- **Findings**: Cal.com is an open-source alternative to Calendly that offers powerful scheduling features with a developer-friendly API and embeddable UI. Calendly is the market leader but is strictly proprietary.
- **Ease of Use**: Cal.com offers a very clean, modern interface. Setting up event types (e.g., "30 Min Consultation") is straightforward.
- **Pricing**: Cal.com is free for individuals and open-source (can be self-hosted), which is highly beneficial. Pro plans start at $15/user/month.
- **Cloud vs Standalone**: Cal.com works in Cloud mode via API/OAuth integrations. In Standalone mode, it could potentially be self-hosted locally alongside OHC or simply integrated via API to the user's existing Cal.com account.

**Design Doc**:
- **Trigger**: The user clicks "Set Up Booking Link" in OHC settings.
- **Action**: OHC authenticates with Cal.com (or provisions a sub-account). The user connects their Google or Outlook Calendar within the OHC flow.
- **User View**: The user gets a unique URL (e.g., `booking.ohc.com/mybusiness`) to share with clients. In the OHC dashboard, an "Upcoming Meetings" widget displays scheduled events synced from the booking system.

**Implementation Prompt**:
Integrate a scheduling solution into the OHC platform. Users must be able to generate a unique booking link that they can share with clients. The booking page should display the user's available time slots based on their connected personal calendar (Google or Outlook). When a client books a slot, an event must be created on the user's calendar. Show a list of upcoming bookings inside the OHC application dashboard.

**Priority**: P1
**Estimated Scope**: Medium
