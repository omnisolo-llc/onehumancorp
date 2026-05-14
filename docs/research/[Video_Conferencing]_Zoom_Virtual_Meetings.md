## 7. Video Conferencing
**Title**: Integrate Zoom for Automated Virtual Meetings
**Problem Statement**: Tutors, therapists, and consultants offer virtual sessions but struggle with manually creating Zoom links, sending them to clients, and remembering which link goes with which meeting.
**Research Report**:
- **Tool**: Zoom API (Server-to-Server OAuth or standard OAuth)
- **Problem it solves for which persona**: Automates the creation of virtual meeting rooms for service professionals offering online sessions.
- **Ease of Use**: Owner connects Zoom once. Every online booking automatically gets a unique Zoom link.
- **Pricing**: Free tier allows 40-min meetings. Pro tier ($15/mo) required for longer or more advanced features.
- **Key Advantages**: The most recognized video conferencing brand; clients already have it installed.
- **Integration Risks**: Zoom OAuth approval process is notoriously stringent for public apps; managing token lifecycles.
- **Environment**: Works well in Cloud via OAuth. Standalone mode might require Server-to-Server OAuth credentials.
**Design Doc**:
- **Trigger**: Client books a "Virtual Consultation" service via OHC scheduling.
- **Action**: OHC calls Zoom API to create a meeting and attaches the `join_url` to the calendar event and confirmation email.
- **User Interface**: Service settings have a "Location" dropdown where the owner can select "Zoom Meeting".
**Implementation Prompt**: Implement a Zoom OAuth integration. When a service marked as "Virtual via Zoom" is booked, automatically generate a Zoom Meeting link via the API and include it in the customer confirmation emails and owner's calendar event.
**Priority**: P2
**Estimated Scope**: Medium
