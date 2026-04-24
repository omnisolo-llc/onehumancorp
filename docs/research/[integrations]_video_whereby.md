# 🔍 Scout: Whereby (Video Conferencing)

## Title
Integrate Whereby for Frictionless Embedded Video Consultations

## Problem Statement
Coaches, tutors (like Leo the Music Tutor), and consultants need to conduct online sessions. Zoom is powerful but requires customers to download an app, which causes friction and delays. They need a simple, browser-based video solution that can be embedded directly into their OHC portal.

## Research Report
**Whereby** provides an Embedded Meetings API that allows developers to integrate video calls directly into their own web applications via an iframe. It is completely browser-based (WebRTC) and requires no downloads or sign-ins for the end customer.

**Pros for Non-Technical Users:**
- Zero friction for customers: just click a link and join in the browser.
- Keeps the customer inside the OHC-branded experience (embedded).
- Very simple API compared to Zoom.

**Integration Risks:**
- Whereby's API is paid (priced per participant minute). This requires OHC to have a robust billing mechanism to pass costs to the tenant or include it in a premium tier.
- Video quality depends heavily on the client's browser and network; troubleshooting is harder than with a dedicated app like Zoom.

**Pricing:**
- Usage-based (e.g., $0.004 per participant minute).

**Environment Support:**
- Cloud-based. Standalone is theoretically possible but requires the user to set up a Whereby developer account, which is too technical.

## Design Doc
- **Integration:** OHC manages the Whereby API connection centrally.
- **Data Flow:** When a virtual service is booked, OHC calls the Whereby API to generate a unique room URL. This URL is saved to the booking record. When the appointment time arrives, the OHC frontend embeds the Whereby iframe using that URL.
- **Action:** The "Operations" agent automatically generates the room link upon booking. The "Customer Success" agent sends the branded link to the customer.

## Implementation Prompt
Integrate the Whereby Embedded API to generate dynamic video rooms for virtual bookings. Update the service creation UI to allow users to specify a service as "Virtual (Whereby)". When a booking is made, automatically create a Whereby room and store the host and participant URLs. Create a new frontend view in OHC that embeds the Whereby iframe, allowing the business owner and customer to conduct the meeting entirely within the OHC platform.

## Priority
P2

## Estimated Scope
Medium
