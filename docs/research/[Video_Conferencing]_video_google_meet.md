**Title**: Video Conferencing Integration via Google Meet

**Problem Statement**:
Service providers like tutors or consultants need a reliable way to conduct online sessions. Manually generating video links and sending them to clients for every booking is repetitive and prone to errors. They need video links to be automatically generated and attached to calendar invites when a booking is made.

**Research Report**:
Google Meet (part of Google Workspace/Calendar) is ubiquitous and highly reliable for video conferencing.
- **Ease of Use for Non-Technical Users**: Very high, as most users already have a Google account. Clients joining the meeting simply click a link without needing to install specialized software.
- **Features**: Generates unique meeting links for calendar events. Supports screen sharing, recording (on paid tiers), and integrates perfectly with Google Calendar.
- **Reputation & Reliability**: Backed by Google's infrastructure, it is one of the most reliable video conferencing tools available.
- **Pricing**: Free for basic use (up to 60 minutes per meeting). Paid tiers are bundled with Google Workspace.
- **Cloud vs Standalone**: The API integration works seamlessly across both modes via OAuth. Standalone instances will need to be configured with a Google Cloud project ID and credentials.

**Design Doc**:
- **Trigger**: When a customer books a virtual appointment via the OHC scheduling feature.
- **Action**: OHC creates a calendar event using the Google Calendar API and sets the `conferenceData` property to request a Google Meet link.
- **User View**: The business owner connects their Google account in OHC settings. When a client books an online service, both the business owner and the client receive a calendar invite containing a unique Google Meet link.
- **Architecture**: OHC will require Google OAuth integration with Calendar scopes. The backend will need to construct calendar event payloads that explicitly request conference data generation (`createRequest`).

**Implementation Prompt**:
Integrate the Google Calendar API to automatically generate Google Meet video conferencing links for virtual bookings. Allow users to connect their Google accounts via OAuth. When an online service is booked, automatically create a calendar event with a Google Meet link and ensure the link is included in the confirmation emails sent to both the business owner and the customer.

**Priority**: P2 (medium)
**Estimated Scope**: Medium
