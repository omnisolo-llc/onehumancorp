### 7. Google Meet (Video)
**Title**: Google Meet Integration for Video Conferencing

**Problem Statement**:
Service-based businesses (tutors, consultants) need to generate video call links for online appointments. Manually creating meetings and copying links into emails is tedious and prone to errors. They need meeting links to be auto-generated when a client books an online service.

**Research Report**:
- **Tool**: Google Meet (via Google Workspace API).
- **Ease of Use**: Ubiquitous. Most users already have a Google account. Joining requires no software installation for clients.
- **Pricing**: Included with free Google accounts or Google Workspace subscriptions.
- **Reputation**: Highly reliable, universally recognized, and trusted.
- **Compatibility**: Works well via OAuth. In Cloud mode, OHC manages the OAuth app. In Standalone mode, users may need to set up their own Google Cloud project for OAuth, which adds friction.

**Design Doc**:
- **Trigger**: An online appointment is booked or created.
- **Action**: OHC authenticates via Google API, creates a calendar event with a Meet link, and saves the link to the appointment record.
- **User Interface**: When creating an event, the user checks a box "Add Google Meet video conferencing". The generated link is displayed in the event details and sent to the client.
- **Integration Flow**: "Sign in with Google" button in the Integrations settings to grant calendar access.

**Implementation Prompt**:
Integrate Google Meet API to auto-generate video conferencing links for scheduled appointments. Add a "Sign in with Google" OAuth flow to authorize calendar access. When an online meeting is scheduled, automatically provision a Google Meet link and attach it to the OHC appointment record and notification emails.

**Priority**: P1
**Estimated Scope**: Medium
