### Title
Integrate Zoom for Auto-Generated Online Meeting Links

### Problem Statement
Service providers who offer digital products or consultations (like online music tutors or business consultants) waste time manually creating video conferencing links and pasting them into calendar invites for every new booking. They need a system that automatically generates a unique, secure video link as soon as a customer books a time slot.

### Research Report
**Tool Evaluated:** Zoom
**Overview:** Founded in 2011 by Eric Yuan, Zoom Communications is a dominant player in the videotelephony and online chat space. It saw explosive growth during the COVID-19 pandemic, becoming an essential tool for remote work and distance education. It reported $4.67 billion in revenue for 2025.
**Key Features & Advantages:**
- Extremely high brand recognition; most customers already have the app installed or know how to use it.
- Robust API allowing programmatic creation of meetings, webinars, and generation of join links.
- Supports varied meeting types (one-on-one, group classes, webinars).
**Risks:** Historically faced intense scrutiny over security and privacy ("Zoombombing", misleading encryption claims, data sharing). Though Zoom has settled lawsuits and significantly improved security (e.g., implementing true end-to-end encryption and better default passwords), the platform must ensure meetings generated via API enforce passwords or waiting rooms by default to protect OHC merchants.
**Ease of Use:** Very high. Users click a link to join.
**Pricing:** Freemium. Free 40-minute limit on group meetings; paid tiers for longer or larger meetings.
**Deployment:** Cloud-native.

### Design Doc
**Integration Trigger:** An online service booking is confirmed via the OHC scheduling flow (e.g., Leo the music tutor receives a new guitar lesson booking).
**Action:** The OHC Operations AI Agent triggers an API call to Zoom to create a meeting scheduled for the booked time, retrieves the join URL, and injects it into the calendar invite and confirmation emails.
**User Experience:**
- **Business Owner:** Connects Zoom in settings. When an online lesson is booked, the event on their calendar already contains the Zoom link. They just click it when it's time to start.
- **Customer:** Receives a booking confirmation email that explicitly includes the "Join Video Call" link and any required passcode.

### Implementation Prompt
Implement a Zoom API integration to automatically provision video meeting rooms for service-based bookings.

**Acceptance Criteria:**
1. Provide an OAuth 2.0 flow for merchants to connect their Zoom accounts to OHC.
2. When a service defined as "Online/Virtual" is booked and paid for, the backend must make an authenticated request to the Zoom API to create a scheduled meeting.
3. The integration MUST configure the meeting to have a waiting room or passcode enabled by default to prevent unauthorized access.
4. The resulting `join_url` must be saved to the booking record in the database and surfaced in the UI for both the merchant and the customer.

### Priority
P2

### Estimated Scope
Medium
