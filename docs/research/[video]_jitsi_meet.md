# Video Conferencing: Jitsi Meet

**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons

**Problem Statement**: Leo (Music Tutor) manually creates a Zoom link for every new lesson and emails it to the student. This is prone to error and looks unprofessional. He needs links to be generated automatically natively when a lesson is booked, avoiding external meeting scheduling workflows.

**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.
- **Pricing**: Included in OHC platform costs.
- **Compatibility**: Cloud (Server-to-Server API). Standalone (Embedded WebRTC).

**Design Doc**:
- In the service creation flow, the user selects "Online Meeting" as the location and clicks "Connect Zoom" (label to be changed to Jitsi).
- Upon a successful booking, OHC automatically generates a Jitsi Meet join URL and embeds it in the calendar invite and confirmation email.
- The Customer Success Agent can follow up after the call ends.

**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
- **Priority**: P2
- **Estimated Scope**: Medium
- **Acceptance Criteria**:
  - Service creation allows selecting "Online Meeting".
  - Bookings automatically generate and embed Jitsi Meet links.

**Strategy**: Use Jitsi Meet's open-source WebRTC capabilities to provide embedded, login-free video conferencing.
