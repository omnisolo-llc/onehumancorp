# [Video Conferencing] Zoom Auto-Links

**Title**: Zoom Integration for Online Consultations
**Problem Statement**: Service-based business owners waste time manually creating Zoom meetings and emailing links to clients before appointments.
**Research Report**:
- **Target Persona**: Tutors, coaches, and consultants offering remote sessions.
- **Evaluation**: Zoom is ubiquitous. The OAuth flow is standard. Free tier covers 40-minute 1:1 meetings which suits many small consultants.
- **Ease of Use**: High. Everyone knows Zoom.
- **Pricing**: Free basic tier; Pro starts at $15/mo.
- **Key Risks**: Token expiration and required re-authentication, handling meeting cancellations/reschedules correctly.
- **Compatibility**: Cloud supports OAuth well. Standalone requires Server-to-Server OAuth setup which is complex for laymen.
**Design Doc**: When a virtual service is booked, OHC automatically creates a Zoom meeting and attaches the join link to the appointment details and confirmation emails.
**Implementation Prompt**: Add a "Virtual Meeting" option to services that automatically generates a Zoom link upon booking. Acceptance criteria: booked services include a valid Zoom join link.
**Priority**: P2
**Estimated Scope**: Medium
