# Embedded Video Conferencing for Online Consultations

## Problem Statement
Service providers who offer online classes, telehealth, or consultations struggle with sending meeting links, dealing with expired links, or customers forgetting how to join. They need a video conferencing solution that is seamlessly embedded into their booking process, where the customer just clicks 'Join Meeting'.

### Target Personas
- **Tariq, online math tutor: Needs unique, secure meeting links for each student to prevent 'Zoombombing'.**
- **Nina, life coach: Wants to run group workshops with breakout rooms directly from her OHC portal.**
- **Dr. Smith, telehealth provider: Requires HIPAA-compliant video streams with no local recording without consent.**

## Research Report
We conducted a comprehensive analysis of the available tools in the market to solve this specific challenge for small businesses.

### Competitive Tool Analysis

#### Zoom API
- **Ease of Use**: Medium. Requires OAuth setup.
- **Pricing Model**: $14.99/month Pro plan required for API access.
- **Market Reputation**: Global standard for video meetings.
- **Key Advantages**: Everyone knows how to use it; highly reliable audio/video.
- **Identified Risks**: Users must install the Zoom app; browser experience is inferior.
- **Architecture Compatibility**: Cloud API.

#### Daily.co
- **Ease of Use**: High for developers.
- **Pricing Model**: 10,000 free minutes/month.
- **Market Reputation**: Developer-first WebRTC platform.
- **Key Advantages**: Can be completely embedded in the OHC UI via iframe. No app downloads required.
- **Identified Risks**: Brand recognition is zero among consumers.
- **Architecture Compatibility**: Cloud API.

#### Google Meet
- **Ease of Use**: High.
- **Pricing Model**: Included with Google Workspace.
- **Market Reputation**: Ubiquitous for Gmail users.
- **Key Advantages**: Seamless integration if the user is already syncing Google Calendar.
- **Identified Risks**: Guest experience can be tricky if they don't have a Google account.
- **Architecture Compatibility**: Cloud API.

#### Jitsi Meet
- **Ease of Use**: High.
- **Pricing Model**: Free (Open Source).
- **Market Reputation**: Leading open-source video conferencing solution.
- **Key Advantages**: Can be fully self-hosted for OHC Standalone. Exceptional privacy.
- **Identified Risks**: Scaling a Jitsi server for large group calls requires significant infrastructure expertise.
- **Architecture Compatibility**: Both Cloud and Standalone (Self-hosted).

### Market Context
The virtual services market boomed post-2020. Providing friction-free joining experiences increases client retention.

## Design Doc
When a business owner creates a service in OHC marked as 'Virtual', OHC will automatically generate a unique video meeting link upon every booking using the integrated provider's API. The OHC booking confirmation page and emails will feature a prominent 'Join Video Call' button. For advanced integrations (like Daily.co), the video call can be embedded directly within an OHC iframe.

### Security & Compliance
Meeting links must be unique per session with randomized passwords to prevent unauthorized access.

### Resilience Strategy
If the video API fails to generate a link during booking, generate a placeholder and implement a background retry mechanism to update the booking later.

## Implementation Prompt
Create a virtual meeting generation flow. When a user books a service flagged as 'Virtual', automatically request a meeting link from a video provider API (use a mock service if necessary). Store this link with the booking record. Display a 'Join Call' button on the customer's booking confirmation page that activates exactly 5 minutes before the scheduled time.

### Acceptance Criteria
- [ ] Booking a virtual service generates a unique video link.
- [ ] Customer email includes the dynamic join link.
- [ ] Host dashboard shows the upcoming meeting with a 'Start Meeting' button.
- [ ] Links are unique and not reused across different bookings.

## Priority
P2

## Estimated Scope
Medium

## Extended Architectural Considerations

When implementing video, developers must consider the implications for both the multi-tenant Cloud deployment of OHC and the self-hosted Standalone mode.

In Cloud mode, API rate limiting is a shared concern. A sudden spike in activity from one tenant must not exhaust the API quota for the entire platform. This necessitates a robust queueing system, such as RabbitMQ or AWS SQS, to process outbound requests and ingest incoming webhooks efficiently.

In Standalone mode, the business owner might not have the technical expertise to configure complex OAuth apps or webhook receivers. The UI must guide them through this process with extreme clarity, perhaps utilizing a proxy service maintained by OHC to simplify the webhook routing to dynamic IP addresses typical of self-hosted setups.

Furthermore, data privacy is paramount. Any PII (Personally Identifiable Information) synced from video tools must be encrypted at rest within the OHC database. Retention policies should automatically purge transient data (like raw webhook payloads) after successful processing to minimize the attack surface.

The user interface must remain mobile-first. Small business owners operate primarily from their smartphones. Therefore, the settings pages, dashboards, and daily interaction elements designed for this integration must be fully responsive and pass the 'Grandmother Test' for usability.

By carefully considering these architectural, security, and usability constraints, we can deliver an integration that not only functions reliably but empowers the user to grow their business without friction.
