# Scout 🔍: Integrate Whereby for Frictionless Video Consultations

## Problem Statement
Leo (Music Tutor) needs a simple way to meet students online for lessons. Traditional tools like Zoom or Microsoft Teams require app downloads, account creation, or can be clunky to join. He wants a "browser-first" experience where his students just click a link and are in the meeting immediately, without any technical friction.

## Research Report
- **Tool**: Whereby (Embedded or Meetings).
- **Target Persona**: Leo (Music Tutor), Consultants, Online Coaches.
- **Evaluation**: Whereby is known for its beautiful, high-quality, browser-based video meetings. No downloads or logins are required for guests.
- **Ease of Use**: Highest in class. The UX is premium, clean, and fits the OHC "Radical Simplicity" aesthetic perfectly.
- **Pricing**: Free for 1-on-1 meetings. Pro plans for teams or larger rooms are affordably priced for solo entrepreneurs.
- **Reputation**: High. Well-regarded for privacy and ease of use.
- **Cloud vs. Standalone**: Compatible with both. Cloud can handle room creation via API; Standalone can use permanent room links.

## Design Doc
- **Automation**: When a service is marked as "Online", a unique Whereby room link is automatically generated for each booking.
- **Communication**: The link is included in the confirmation email and the customer's dashboard.
- **Native Feel**: The video room can be embedded directly into the OHC dashboard via an iframe, so the merchant never leaves the app.

## Implementation Prompt
Integrate Whereby for online services and appointments. Automatically generate meeting room links using the Whereby API when a booking is confirmed. Provide a simple "Join Meeting" button in both the Merchant and Customer dashboards.
- **Acceptance Criteria**: Online bookings automatically get a Whereby link. Both parties can join from the OHC dashboard. No app download required for the customer.
- **Priority**: P2
- **Estimated Scope**: Small
