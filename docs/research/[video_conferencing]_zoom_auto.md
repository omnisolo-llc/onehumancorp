# Title
Native Zoom Link Generation for Appointments

# Problem Statement
Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.

# Research Report
- **Tool:** Zoom API.
- **Target Persona:** Leo (Music Tutor) and other virtual service providers.
- **Advantages:** Standard OAuth connection process. Highly intuitive and globally recognized video platform.
- **Risks:** Zoom OAuth requires annual app review and compliance checks.
- **Pricing:** API is free for Zoom users, but requires the merchant to have a Zoom account.
- **Compatibility:** Cloud (OAuth). Standalone (Server-to-Server OAuth).

# Design Doc
- **Integration Trigger:** In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom".
- **User Flow:** User authenticates via Zoom OAuth to link their account.
- **Action Flow:** Upon a successful booking, OHC calls the Zoom API to create a meeting, retrieves the join URL, and embeds it in the calendar invite and confirmation email. The Customer Success Agent can follow up after the Zoom call ends to ask for a review or suggest booking the next session.

# Implementation Prompt
Build a Zoom integration that automatically creates meeting links for online service bookings. Users should be able to connect their Zoom account. When a customer books a service marked as "Online Meeting", the system must dynamically generate a Zoom link, store it with the booking, and share it with both the merchant and the customer.
- **Acceptance Criteria:** Merchant connects Zoom. Customer books online service. Unique Zoom link is generated and sent to both parties.
- **Priority:** P2
- **Estimated Scope:** Medium
