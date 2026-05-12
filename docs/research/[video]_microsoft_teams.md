# Video Conferencing: Microsoft Teams

**Problem Statement:** Business owners offering online services (tutoring, consulting) need reliable, auto-generated video links for their appointments without manually creating meetings.

**Research Report:** Microsoft Teams is ubiquitous in corporate environments, but less common for micro-SMBs compared to Zoom or Meet. However, it's bundled with Microsoft 365, which many SMBs use.
- Ease of Use: Can be clunky for external guests joining via browser compared to Google Meet.
- Pricing: Included in Microsoft 365 subscriptions.
- Reputation: Enterprise-grade, highly secure.
- Cloud vs. Standalone: Cloud-based.

**Design Doc:**
- Upon booking confirmation in OHC, an API call to MS Graph generates a Teams meeting link.
- Link is included in the calendar invite and confirmation email.
- UI wireframes or screen flow description (375px first): Appointment detail view shows a prominent "Join Video Call" button.
- Mobile UX flow: Tapping the button deep-links to the Teams app or opens the mobile browser fallback.

**Implementation Prompt:** Integrate Microsoft Teams via the Graph API to auto-generate meeting links for online appointments scheduled through OHC.
- Acceptance Criteria: Unique Teams links are generated per appointment. Links are attached to the OHC booking record.

**Priority:** P3
**Estimated Scope:** Medium
