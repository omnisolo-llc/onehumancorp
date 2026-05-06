## [Video] Issue Brief

**Title**: Scout 🔍: Integrate Zoom for Automated Meeting Links
**Problem Statement**:
Consultants and tutors using OHC need to manually create Zoom links for every booking and send them to clients, which is prone to errors and forgotten links.
**Research Report**:
- **Tool**: Zoom API
- **Evaluation**: The Zoom API allows automatic creation of meetings. Integrating it with the scheduling feature ensures every booking automatically gets a unique Zoom link.
- **Ease of Use**: Users connect their Zoom account via OAuth.
- **Pricing**: Requires a Zoom Pro account for API access.
- **Cloud vs. Standalone**: Works via OAuth in both modes.
**Design Doc**:
- User connects their Zoom account.
- When a new appointment is scheduled, OHC calls the Zoom API to create a meeting.
- The Zoom link is saved to the appointment and automatically emailed/SMSed to the customer.
**Implementation Prompt**:
Integrate the Zoom API. Provide OAuth connection flow. Automatically generate a Zoom meeting when a new appointment is booked, and include the link in the confirmation notifications.
**Priority**: P1
**Estimated Scope**: Medium
