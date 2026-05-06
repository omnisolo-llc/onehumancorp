## [Video Conferencing] Auto-Generated Meeting Links
**Title**: Integrate Zoom / Google Meet for Virtual Consultations

**Problem Statement**: Tutors, therapists, and consultants who work remotely struggle with manually creating Zoom links for every meeting and emailing them to clients. They need meeting links to be generated automatically when a booking is made.

**Research Report**:
- **Persona Context**: Remote service providers, educators, and telehealth professionals.
- **Solution Evaluated**: Zoom API and Google Meet (via Google Calendar API). Google Meet is often preferred for its zero-install web experience.
- **Ease of Use**: Zero effort once connected. The link just appears on the calendar invite.
- **Advantages**: Eliminates manual copy-pasting of links. Professional appearance for the client.
- **Risks**: Zoom's OAuth approval process for public apps is stringent. Google Meet requires broader Google Calendar permissions.
- **Pricing Estimate**: APIs are generally included in the user's existing Zoom Pro ($15/mo) or Google Workspace subscription.
- **Cloud/Standalone Support**: Supported in both via OAuth (Cloud) or direct API integration (Standalone).

**Design Doc**:
- **Triggers**: A new appointment is scheduled through the OHC booking system.
- **Actions**: OHC requests a meeting link from the connected video provider and attaches it to the appointment details.
- **User Interface**: A "Video Conferencing" settings page to connect Zoom or Google. When viewing an upcoming appointment, a prominent "Join Meeting" button is displayed for both the business owner and the customer.

**Implementation Prompt**:
Enable automatic video meeting link generation for appointments. Allow users to link their Zoom or Google Meet accounts. When an appointment is created, automatically generate a meeting link and display a "Join Meeting" button in the appointment details view.

**Priority**: P2
**Estimated Scope**: Medium
