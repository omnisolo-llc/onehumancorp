# [Calendar & Scheduling] Integration of Unified Booking & Calendar Synchronization

## Title
Implement Unified Booking and Calendar Synchronization for Service-Based Businesses

## Problem Statement
Service providers like Leo (Music Tutor) and Carlos (Freelance Handyman) need a seamless way to accept bookings without manually managing their schedule. Currently, they lose potential customers due to the friction of back-and-forth messaging to find a suitable time. Furthermore, if a customer books, the business owner has to manually create a calendar event and generate a Zoom/Meet link, risking double bookings or forgotten appointments. A non-technical user needs an automated, hands-off system where customers can view real-time availability, book a slot, and automatically receive a calendar invite and video link, all without leaving the OHC platform or requiring complex setup.

## Research Report
### Evaluated Tools
1. **Calendly API:**
   - **Pros:** Industry standard, robust APIs, excellent timezone handling, native Zoom/Google Meet integrations.
   - **Cons:** Paid tiers required for advanced API usage and multiple event types; branding can be intrusive on lower tiers.
   - **Ease of Use:** Extremely high for the end-user (customer), but API integration requires careful mapping of webhooks to OHC's architecture.
   - **Pricing:** ~$12-16/user/month for API access, which might be a barrier if OHC absorbs the cost, or require users to bring their own keys (adding friction).

2. **Acuity Scheduling (Squarespace):**
   - **Pros:** Highly customizable, great for complex appointment types and group classes (good for Leo).
   - **Cons:** API is less developer-friendly than Calendly; heavily integrated into Squarespace's ecosystem.
   - **Ease of Use:** Slightly steeper learning curve for configuration.
   - **Pricing:** Starts around $16/month.

3. **Cal.com (Open Source):**
   - **Pros:** Open-source, highly customizable API, white-labeling capabilities, excellent for platform integrations. Supports Google Calendar, Zoom, and Stripe out-of-the-box.
   - **Cons:** Younger product, slightly less brand recognition among everyday users compared to Calendly.
   - **Ease of Use:** Fantastic for platform integration; OHC can abstract away the complexity so the business owner just connects their Google account.
   - **Pricing:** Infrastructure pricing for platforms is very competitive; potentially free for self-hosting or per-booking micro-transactions.
   - **Hybrid Compatibility:** Since it's open source, it fits well with OHC's Standalone (local) and Cloud modes.

### Recommendation
**Cal.com Platform API** or building a bespoke **Google Calendar / Microsoft Graph API wrapper** tailored for OHC. For maximum simplicity and white-labeling, directly integrating Google Calendar API + Google Meet auto-generation is the most frictionless path for 80% of users, while falling back to Cal.com for complex routing.

## Design Doc
### Integration Flow
1. **Setup:** The business owner (e.g., Leo) navigates to the "Operations" department and clicks "Connect Calendar". They authenticate via OAuth (Google or Microsoft).
2. **Configuration:** They set their working hours (e.g., Mon-Fri 9 AM - 5 PM) and service durations (e.g., 30 min, 60 min).
3. **Customer Experience:** On the business's public OHC storefront, a "Book Now" button appears. Customers see a clean, OHC-styled calendar (Glassmorphism design) showing only available slots.
4. **Execution:** Upon selecting a slot and paying the required deposit (handled by Finance), the Operations Agent reserves the time, generates a Google Meet link, and sends a confirmation email to both parties.
5. **Conflict Management:** The system continuously syncs with the owner's external calendar to block out times if they manually add personal events.

## Implementation Prompt
**User-Facing Outcome:**
A business owner must be able to connect their existing calendar in under 3 clicks. Once connected, their OHC public storefront must automatically display a booking widget that reflects their real-time availability. Customers must be able to book a time, pay any required deposit, and immediately receive a calendar invite with an auto-generated video conferencing link (if applicable). The business owner's personal calendar must be automatically updated with the new booking.

**Acceptance Criteria:**
- Business owner can authenticate their external calendar (Google/Outlook) without technical jargon.
- Public booking widget correctly calculates availability based on predefined working hours and existing calendar events (timezone aware).
- Booking a slot immediately creates a calendar event on both the owner's and the customer's calendars.
- Auto-generation of video meeting links (e.g., Google Meet) for virtual services.
- The UI must adhere to the OHC Premium Token library (Glassmorphism, Outfit/Inter typography).
- The solution must gracefully handle offline/standalone mode (queueing the sync) and cloud mode.

## Priority
P1

## Estimated Scope
Large
