# Native Integration of Zoom for Online Appointments

## Title
Native Integration of Zoom for Online Appointments

## Problem Statement
Tutors, consultants, and therapists need to manually create video meeting links and email them to clients after an appointment is booked. This manual process is prone to errors, looks unprofessional, and forces the business owner to juggle multiple apps. They need links to be generated automatically and seamlessly shared.

## Research Report
- **Strategy**: Native OAuth integration with the Zoom API to dynamically generate meeting links.
- **Target Persona**: Tutors, consultants, therapists, and any service business offering online sessions.
- **Advantages**: Zoom is the ubiquitous standard for video conferencing. Generating links via their API provides a frictionless experience for both the merchant and the client.
- **Risks**: The Zoom App Marketplace requires a stringent review process. Merchants must have their own Zoom accounts.
- **Pricing**: The API is free to use, but the merchant is subject to the limitations of their personal Zoom plan (e.g., 40-minute limit on free tiers).
- **Compatibility**: Compatible with both Cloud mode (via standard OAuth) and Standalone mode (via Server-to-Server OAuth or personal API keys).

## Design Doc
- User navigates to "Services" in OHC to create a new service offering.
- For location, they select "Online Meeting" and click "Connect Zoom".
- They complete the standard Zoom OAuth flow.
- When a customer books this service, OHC securely calls the Zoom API to generate a unique meeting ID and join link.
- The link is automatically embedded in the customer's confirmation email, calendar invite, and the merchant's OHC dashboard.
- **AI Integration**: The Customer Success Agent follows up automatically after the scheduled meeting time to ask the client for a review or to book the next session.

## Implementation Prompt
Build a native Zoom integration to automate video link generation for online service bookings. The merchant must be able to authorize OHC via Zoom OAuth. When an "Online" service is booked, the backend must dynamically generate a Zoom meeting, store the join URL, and deliver it to both the merchant and the customer.
- **Acceptance Criteria**: Merchant can authenticate with Zoom. Booking an online service automatically generates a unique Zoom link. Both merchant and customer receive the link via email/calendar invite.
- **Priority**: P2
- **Estimated Scope**: Medium
