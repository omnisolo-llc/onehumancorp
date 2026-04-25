# Issue Brief: Calendar & Scheduling Integration (Cal.com)

## Title
Implement Cal.com Calendar & Scheduling Integration for Service & Booking Businesses

## Problem Statement
Small business owners who offer services (like Leo the Music Tutor or Carlos the Handyman) struggle with the back-and-forth of scheduling appointments. They currently rely on manual calendar checks, DMs, or texts to coordinate times. This is inefficient, prone to errors, and creates friction that loses potential customers. They need a simple, professional way for customers to view their availability, book a slot, and automatically receive a calendar invite and video link (if applicable), without needing to understand the technical details of calendar APIs.

## Research Report
Based on an analysis of calendar and scheduling tools (Calendly, Acuity, Cal.com, Google Calendar API directly):

*   **Calendly / Acuity**: Industry standards, but they trap the user in their ecosystem. Integrating deeply into OHC so it feels native to the 375px mobile experience is difficult. They also have relatively expensive paid tiers for features like routing or custom domains.
*   **Direct API (Google Calendar / Outlook)**: Requires building a scheduling engine from scratch (timezone math, conflict resolution, booking UX), which is highly complex and error-prone.
*   **Cal.com**: Open-source, developer-friendly scheduling infrastructure. It offers a robust API and webhooks, handles timezone math perfectly, integrates with all major calendars (Google, Apple, Outlook), and auto-generates video links (Zoom/Meet/Cal Video).
    *   **Ease of Use for Non-Technical Users**: The user just needs to connect their primary calendar (e.g., Google Calendar). OHC can use the Cal.com API to abstract away the "Event Type" creation. The user simply says "I offer a 45-min guitar lesson," and OHC handles the Cal.com configuration behind the scenes.
    *   **Pricing**: Cal.com offers a very generous free tier for individuals, and their API/Platform pricing is competitive for a SaaS integration. It can be self-hosted, but leveraging their managed cloud service via API is optimal for OHC's multi-tenant architecture.
    *   **Cloud & Standalone Support**: Works seamlessly via API for Cloud. For Standalone desktop users, the integration works exactly the same as long as the user authenticates via OAuth.

**Conclusion**: Cal.com's API is the best fit. It allows OHC to provide a native-feeling booking experience on the storefront while offloading the complex scheduling logic to a specialized provider.

## Design Doc
### High-Level Integration Flow
1.  **Authorization**: The business owner connects their calendar (e.g., Google Calendar) via a simple OAuth flow on the OHC mobile dashboard. OHC securely stores the connection token.
2.  **Service Definition**: When the business owner creates a new "Service" in OHC (e.g., "Plumbing Estimate"), the OHC backend automatically provisions a corresponding Event Type in Cal.com via API, mapping OHC parameters (duration, price, video/in-person) to the Cal.com event.
3.  **Customer Booking UX**: On the public storefront, the customer sees an availability calendar. This UI is native to OHC (not an iframe), fetching available slots from the Cal.com API.
4.  **Booking Execution**: The customer selects a slot and confirms. OHC calls the Cal.com API to book the slot.
5.  **Event Handling**: Cal.com handles sending the calendar invites and generating meeting links. Cal.com sends a webhook to OHC (`booking.created`), triggering the OHC Operations Agent to record the booking in the OHC database and the Customer Success Agent to send any custom follow-ups.

### UI / UX (Mobile First)
*   **Business Owner Dashboard**: A "Calendar Sync" settings page with a simple "Connect Google Calendar" button.
*   **Storefront**: A sleek, bottom-sheet style date and time picker native to the OHC Premium design system. No external branding visible to the customer.

## Implementation Prompt
Integrate Cal.com for calendar scheduling and booking. Implement the OAuth flow for users to connect their calendars. When a user creates a Service offering in OHC, automatically create a corresponding event type via the Cal.com API. Build a native, mobile-first booking UI for the storefront that fetches availability and creates bookings via the API. Implement a webhook receiver to handle `booking.created` events, ensuring the local OHC database is updated and the Operations Agent is notified of new appointments. Ensure the integration supports both Cloud and Standalone modes by handling API authentication securely in both environments.

## Priority
P0

## Estimated Scope
Large
