# [Video Conferencing] Zoom Integration

**Problem Statement**: Tutors, consultants, and online coaches manually create Zoom links and email them to clients for every session. They need this process automated so a link is generated as soon as a session is booked.

**Research Report**:
- **Target Persona**: Tutors, online coaches, remote consultants.
- **Ease of Use**: Zoom OAuth is standard.
- **Pricing**: Free tier has 40-minute limits. Paid tiers required for longer meetings.
- **Reputation**: Ubiquitous.
- **Cloud/Standalone**: Works via standard API calls.

**Design Doc**:
- **Trigger**: A new online meeting is scheduled in OHC.
- **Action**: OHC calls Zoom API to create a meeting and stores the join URL.
- **User View**: When scheduling an appointment, the owner checks "Make it a Zoom meeting" and the join link is automatically added to the calendar invite sent to the customer.

**Implementation Prompt**: Integrate Zoom so that when users schedule a virtual appointment or event, OHC automatically generates a Zoom meeting link and includes it in the invitation details.

**Priority**: P2
**Estimated Scope**: Medium
