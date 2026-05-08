## [Video Conferencing] Issue Brief: Jitsi Meet Integration for Frictionless Consultations

**Title**: Scout 🔍: Integrate Jitsi Meet for No-Download Video Consultations
**Problem Statement**:
Service providers (tutors, consultants) need a reliable way to conduct video calls. Forcing clients to download apps or create accounts creates friction. They need instant, browser-based video meeting links attached to their calendar events.
**Research Report**:
- **Tool**: Jitsi Meet
- **Evaluation**: An open-source, fully encrypted video conferencing solution. Works directly in the browser with no account required for guests.
- **Ease of Use**: Zero friction for the end client. Very easy for the business owner.
- **Pricing**: Free to use public servers (meet.jit.si). Self-hosting is free but requires infrastructure.
- **Cloud vs. Standalone**: Excellent for both. Standalone users benefit from the privacy of open-source.
**Design Doc**:
- When a consultation is booked (e.g., via the Cal.com integration), OHC generates a unique Jitsi Meet URL.
- The URL is included in the calendar invite and confirmation emails.
- Both the user and the client click the link to join the meeting directly in their browser.
**Implementation Prompt**:
Integrate Jitsi Meet URL generation for booked appointments. Ensure that meeting links are automatically created and attached to calendar events. Embed a basic Jitsi iframe within the OHC dashboard so users can join meetings without leaving the platform, if desired.
**Priority**: P2
**Estimated Scope**: Small
