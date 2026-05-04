# [Video Conferencing] Auto-Generated Meeting Links via Zoom API

**Title**: Implement Auto-Generated Video Conferencing Links via Zoom

**Problem Statement**:
Service providers who teach or consult online, like Leo (The Music Tutor), need an effortless way to generate video meeting links. Currently, when a student books a lesson, Leo has to manually open Zoom, create a meeting, copy the link, and email it to the student. This manual work limits his ability to scale his tutoring business. He needs the video link to be generated automatically the second a student books and pays for a slot.

**Research Report**:
I evaluated Zoom API, Google Meet API, and Daily.co.
- **Zoom API**: Universal brand recognition. Almost every student already has the Zoom app installed. The API for generating Server-to-Server (or OAuth-based) meeting links is very robust.
- **Google Meet API**: Requires tight coupling with Google Workspace, which can be restrictive for users who prefer Apple or Outlook calendars but still want standard video calls.
- **Daily.co**: Excellent for embedding video directly *inside* the OHC app (White-labeled WebRTC). However, most users (and their clients) strongly prefer the familiarity of native Zoom or Meet apps for 60-minute tutoring sessions.
- **Conclusion**: Zoom is the most expected and frictionless platform for end-users. Integrating the Zoom API allows OHC to automatically attach a unique video link to calendar events.

**Design Doc**:
- **Integration Point**: Resides within the "Operations" (The Manager) department, acting as an extension of the Calendar & Scheduling module.
- **Triggers & Flow**:
  1. The business owner links their Zoom account via an OAuth flow in OHC Settings.
  2. The owner creates a Service (e.g., "1 Hour Guitar Lesson") and toggles "Online Meeting".
  3. A student books the service via the OHC scheduling page.
  4. The OHC backend calls the Zoom API to generate a unique meeting ID and password.
  5. The Zoom link is injected into the Calendar invite (via Cronofy) and sent in the confirmation email (via Resend).
- **User View**: A simple toggle when creating a service: "Generate Zoom link automatically". A button on their upcoming appointments list: "Join Video Call".

**Implementation Prompt**:
Integrate the Zoom API to automatically generate unique video conferencing links for scheduled online services. Build an OAuth flow allowing the business owner to connect their Zoom account. Modify the service creation UI to include a toggle for "Online Meeting". When a customer successfully books an online service, the system must call the Zoom API to create a meeting, store the join link in the database, and automatically include it in the customer's confirmation email and calendar invite. Ensure the integration handles token refreshes securely and gracefully degrades if the API is temporarily unavailable.

**Priority**: P2
**Estimated Scope**: Medium
