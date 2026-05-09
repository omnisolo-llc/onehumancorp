# [video-conferencing] Automated Virtual Meeting Links

**Title:** Integrate Auto-Generated Video Conferencing Links

**Problem Statement:**
Consultants, tutors, and remote service providers spend manual effort generating Zoom or Google Meet links and sending them to clients after an appointment is booked. This manual step often causes confusion, lost links, and delayed meetings.

**Research Report:**
* **Tools Evaluated:** Zoom API, Google Meet (via Google Workspace API), Jitsi Meet.
* **Ease of Use:** Integrating these APIs allows OHC to automatically inject a meeting link into the calendar invite and confirmation email, making the process completely invisible to the business owner after initial setup.
* **Key Advantages:**
  - Zero manual link generation required.
  - Ensures the meeting host and attendee always have the correct, unique link.
  - Can enforce meeting passwords automatically for privacy.
* **Risks:**
  - OAuth token expiration requires robust refresh logic to ensure links are generated successfully in the background.
* **Pricing Estimate:** Free for basic usage (Google Meet, Jitsi); Zoom requires a pro account for certain API features.
* **Environment Support:** fully supported in Cloud mode. Standalone mode can connect to the API via external network requests.

**Design Doc:**
* **Trigger:** The business owner creates a "Virtual Service" and connects their Zoom or Google Workspace account via OAuth.
* **Actions:** When a customer books the "Virtual Service", OHC calls the respective API to create a scheduled meeting. OHC saves the join URL and includes it in the confirmation UI and notification systems.
* **User Experience:** The owner just looks at their schedule. At the time of the appointment, both the owner and the customer simply click the "Join Meeting" button inside the OHC dashboard or from their email.

**Implementation Prompt:**
Implement a video conferencing integration layer that allows merchants to link their Zoom or Google Meet accounts. Update the booking system so that when a virtual appointment is created, the system automatically requests a unique meeting link from the connected provider. Embed this link securely in the customer's appointment confirmation and the merchant's calendar view.

**Priority:** P2
**Estimated Scope:** Medium