# Zoom Video Conferencing Integration

## Problem Statement
Small business owners who offer online consultations, tutoring, or virtual fitness classes struggle with the manual process of creating video meeting links. After a customer books a slot, the owner has to manually open Zoom, generate a meeting, copy the link, and email it to the customer. This process is prone to human error (forgetting to send the link or sending the wrong one), leading to frustrating customer experiences and delayed start times. They need meeting links to be automatically generated and attached to bookings without any manual intervention.

## Research Report
Zoom is the ubiquitous platform for video conferencing, heavily relied upon by remote-first and hybrid small businesses.
- **Benefits for Users:** Eliminates manual meeting creation. Links are generated instantly at the time of booking.
- **Ease of Use:** Highly familiar to end-customers. For the business owner, it's a one-time "Connect Zoom" OAuth setup.
- **Reputation:** Zoom is the industry standard for reliable video conferencing.
- **Pricing:** Zoom API access requires a Pro account or higher (approx. $15.99/mo). There are no additional per-meeting API charges for standard use cases.
- **Environment Compatibility:** Works seamlessly in both Cloud and Standalone modes via Server-to-Server OAuth or standard user OAuth flows.

## Design Doc
```mermaid
graph TD
    Client(Customer) -->|Books Online Consultation| BookingFlow[OHC Booking System]
    BookingFlow -->|Triggers Integration| OHC_Backend[OHC Backend]
    OHC_Backend -->|Requests Meeting| ZoomAPI[Zoom API]
    ZoomAPI -->|Returns Join URL| OHC_Backend
    OHC_Backend -->|Attaches Link to Booking| DB[(SIPDB / Postgres)]
    OHC_Backend -->|Emails/SMS Link| Client
    OHC_Backend -->|Syncs Calendar Event| OwnerCal[Google Calendar]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class BookingFlow,OHC_Backend,DB premium;
```

When a user books a service marked as "Virtual", OHC intercepts the booking event. It securely calls the Zoom API using the owner's OAuth token to create a unique meeting. The returned `join_url` is saved to the database, embedded in the confirmation email/SMS sent to the customer, and added to the calendar invite.

## Implementation Prompt
Integrate Zoom API for automatic meeting link generation during the booking flow.
- **User Outcome:** Business owners connect their Zoom account. When they create a "Virtual Service", customers booking it automatically receive a unique Zoom link in their confirmation, requiring zero manual work from the owner.
- **Acceptance Criteria:**
  - Secure OAuth2 integration for Zoom.
  - Ability to create meetings programmatically and extract the join URL.
  - Integration with the OHC booking and notification (email/SMS) systems.
  - Handle token refresh and expiration gracefully.

## Priority
P2

## Estimated Scope
Medium
