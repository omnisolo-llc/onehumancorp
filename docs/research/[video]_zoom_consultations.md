# Auto-Generate Consultation Links via Zoom

**Title**: Auto-Generate Consultation Links via Zoom
**Problem Statement**: Consultants and tutors need a way to automatically generate and send video meeting links when a client books a session, avoiding manual link creation.

**Research Report**:
- Zoom is the ubiquitous video conferencing tool globally recognized by almost all consumers.
- **Ease of Use**: Users connect their Zoom account once; meeting links are generated transparently.
- **Pricing**: Free tier limits meetings to 40 minutes, which users must be aware of. Pro tier covers longer sessions.
- **Reputation**: Highly reliable, though some users may prefer browser-based alternatives like Google Meet.
- **Cloud vs Standalone**: Works well in both modes via OAuth.
- **Key Advantages**: High consumer familiarity, robust connection quality.
- **Key Risks**: The 40-minute limit on free accounts might cut off user consultations unexpectedly.

**Design Doc**:
- Users connect their Zoom account in the "Integrations" tab via a standard OAuth flow.
- When configuring a service (e.g., "1-Hour Consultation"), they select "Zoom Meeting" as the location.
- Upon booking, OHC automatically calls the Zoom API to generate a meeting link, which is instantly added to the calendar invite and confirmation email sent to both parties.

**Implementation Prompt**: Create a Zoom integration that allows users to seamlessly connect their accounts and automatically generate unique meeting links for newly booked online services.

**Priority**: P2
**Estimated Scope**: Medium
