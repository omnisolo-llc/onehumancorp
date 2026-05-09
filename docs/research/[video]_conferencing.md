## [Video] Issue Brief: Auto-Generated Meeting Links

**Title**: Scout 🔍: Zoom Integration for Automatic Meeting Links
**Problem Statement**: Tutors and consultants who offer online sessions waste time manually creating Zoom links and sending them to clients. They need a system that automatically generates a unique meeting link as soon as a booking is confirmed.
**Research Report**:
- **Tools Evaluated**: Zoom API, Google Meet API, Jitsi.
- **Evaluation**: Zoom is the most ubiquitous video conferencing tool. Integrating its API allows us to automatically provision meetings.
- **Ease of Use**: User clicks "Connect Zoom" in their settings and authorizes via OAuth. OHC handles the link generation automatically for all online services.
- **Pricing**: Zoom API requires the merchant to have a Zoom account (free or paid).
- **Cloud vs. Standalone**: Standard OAuth flow works well in both environments.
**Design Doc**:
- The user connects their Zoom account via an OAuth integration.
- When creating a service, they mark it as an "Online Meeting."
- Upon a successful booking, the backend calls the Zoom API to create a scheduled meeting.
- The generated join URL is saved to the appointment record and included in the confirmation emails/calendar invites sent to the user and the client.
**Implementation Prompt**: Create a Zoom OAuth integration. Hook into the appointment booking flow to detect online meetings, call the Zoom API to create a meeting instance, and store the resulting join URL. Update notification templates to include the link.
**Priority**: P1
**Estimated Scope**: Medium
