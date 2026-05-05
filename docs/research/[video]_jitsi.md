## [Video Conferencing] Issue Brief: Zero-Setup Online Lessons

**Title**: Scout 🔍: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**:
Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room natively integrated into his booking flow without needing a separate Zoom subscription or managing complex OAuth setups.

**Research Report**:
- **Tool**: Jitsi Meet.
- **Evaluation**: Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool. Requires no account for the student. Works natively in the browser and mobile.
- **Ease of Use**: Completely seamless integration with no technical setup required by the user. Zero extra effort. The system automatically provisions the link.
- **Advantages**: OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription. Deep native integration without third-party friction.
- **Risks**: Hosting Jitsi at scale requires infrastructure management. Relying on public servers might lead to poor performance.
- **Pricing**: Free and open-source.
- **Compatibility**: Perfect for both Cloud (hosted instance) and Standalone (can point to public server or local instance).

**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call natively.

**Implementation Prompt**:
Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses. When a service is marked as "online", ensure the booking system auto-generates a unique Jitsi Meet link and includes it in the customer's confirmation email and the business owner's dashboard.
- **Acceptance Criteria**: Service can be marked as "Online Meeting". Booking automatically generates a unique Jitsi URL. Both merchant and customer can join the video call via the generated link seamlessly.
**Priority**: P2
**Estimated Scope**: Small
