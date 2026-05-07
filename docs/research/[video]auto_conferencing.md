# Auto-Generating Video Conference Links

**Problem Statement:**
For business owners offering online services (tutoring, coaching, consultations), creating a Zoom link for every new booking is a manual, annoying chore. They often forget, leading to panicked messages right before the meeting starts: "Where is the link?"

**Research Report:**
- **Evaluated Tools:** Zoom API, Google Meet (via Google Calendar API).
- **Ease of Use:** Seamless. The business owner connects their account once, and links appear magically.
- **Pricing:** APIs are free to use, requiring only a standard Zoom or Google account.
- **Reputation:** Zoom and Google Meet are ubiquitous and highly reliable.
- **Cloud vs Standalone:** Fully supported in both modes via standard OAuth flows.

**Design Doc:**
- **Trigger:** A customer books an online appointment.
- **Action:** OHC calls the Zoom/Meet API to generate a unique meeting room link and attaches it to the calendar event and confirmation emails.
- **User Interface:** When setting up a service, the owner selects "Location: Online (Zoom/Meet)." The generated link is prominently displayed in the booking confirmation UI.

**Implementation Prompt:**
Integrate with the Zoom API or Google Meet to automatically generate video conference links for online bookings. When a client books an "Online" service, the system must automatically create a unique meeting link, embed it in the calendar invitation, and include it in the confirmation emails sent to both the business owner and the client.

**Priority:** P2
**Estimated Scope:** Small
