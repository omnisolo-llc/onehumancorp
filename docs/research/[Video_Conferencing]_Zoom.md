**Title**: Zoom Integration for Auto-Generated Meeting Links
**Problem Statement**: Virtual service providers (tutors, consultants) waste time manually creating Zoom links for every booking and emailing them to clients. This often leads to wrong links being sent or clients losing the link.
**Research Report**: Zoom is universally recognized. Its API allows for instantaneous meeting creation. The free tier covers most 1-on-1 needs for 40 minutes, and Pro accounts are affordable. The join experience is frictionless for attendees.
**Design Doc**:
- **Trigger**: A virtual appointment is booked via OHC.
- **Action**: OHC calls the Zoom API to generate a unique meeting link.
- **User Experience**: The business owner connects Zoom. When a client books a service marked as "Virtual", OHC instantly generates a Zoom link, adds it to both parties' calendar invites, and displays it on the OHC appointment detail page.
**Implementation Prompt**: Add a Zoom integration option in the scheduling settings. When a new virtual booking occurs, automatically generate a Zoom meeting URL and embed it in the confirmation screen and notification emails. Display a "Join Meeting" button in the OHC dashboard for the business owner.
**Priority**: P1
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.
