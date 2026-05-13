# Scout: Tool Integration Research Q2

## 7. Video Conferencing
**Title**: Embed Jitsi Meet for Zero-Setup Online Lessons
**Problem Statement**: Leo the Music Tutor currently has to manually create Zoom links, email them to students, and deal with students losing the link. He needs an automated, branded video room.
**Research Report**:
- Jitsi Meet is a fully open-source, WebRTC-based video conferencing tool.
- Requires no account for the student. Works natively in the browser and mobile.
- OHC can host a Jitsi instance (for Cloud mode) or point to public servers (for Standalone), saving users from needing a paid Zoom subscription.
- Completely seamless integration with no technical setup required by the user.
**Design Doc**:
- When a service is marked as "Online Meeting", OHC auto-generates a unique Jitsi URL (e.g., `meet.ohc.com/leo-guitar-session`).
- The link is automatically added to the calendar invite and the customer's dashboard.
- Users just click the link at the scheduled time to join the browser-based call.
**Implementation Prompt**: Integrate auto-generated Jitsi Meet links for bookings designated as "Online", providing a seamless, no-login video conferencing experience for service-based businesses.
**Priority**: P2
**Estimated Scope**: Small
