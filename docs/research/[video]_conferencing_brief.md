# Title: Embedded Frictionless Video Consultations via Jitsi

## Problem Statement
Service-based businesses (tutors, consultants, therapists) struggle with getting clients into video calls. Clients often have the wrong app installed, forget their password, or can't find the link.

## Research Report
- **Tool Evaluated:** Jitsi (Jitsi Meet API / 8x8)
- **Benefits:** Open-source, web-native, requires zero downloads and zero logins for the end customer.
- **Ease of Use:** Unbeatable. It's just a web link that works instantly in any browser.
- **Pricing:** Free (if self-hosted) or cheap via Jitsi as a Service (8x8).
- **Cloud/Standalone:** Can be deeply embedded via iframe. In Standalone, users can technically run their own Jitsi server, or rely on public servers.

## Design Doc
1. **Trigger:** A customer books a virtual service via the OHC scheduling link.
2. **Action:** OHC automatically generates a unique Jitsi room URL.
3. **UI Outcome:** Both the business owner and the customer see a "Join Video Call" button in their respective portals/emails. Clicking it opens the video call directly inside the browser.

## Implementation Prompt
Integrate Jitsi to provide 1-click video consultations. When a virtual appointment is booked, automatically generate a unique meeting link. Build a UI where the business owner can start the meeting directly from their OHC dashboard, and the customer can join via a web link without needing to download any software.

## Priority
P2

## Estimated Scope
Medium
