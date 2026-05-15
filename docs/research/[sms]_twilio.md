# [SMS & Notifications] Twilio SMS

**Title**: Global SMS Notifications via Twilio
**Problem Statement**: Users with low English proficiency or in areas with poor internet rely heavily on SMS over email. They miss critical business updates without it.
**Research Report**:
- **Target Persona**: Business owners serving local communities where SMS is the primary communication channel, or businesses needing urgent transactional alerts.
- **Evaluation**: Twilio provides the most reliable global coverage. While slightly technical to set up initially, OHC can abstract the complexity.
- **Ease of Use**: Low for direct use, but High if OHC abstracts the API integration.
- **Pricing**: Cost is per message (fractions of a cent), very scalable.
- **Key Risks**: Carrier filtering (spam blocking), strict regulations (A2P 10DLC) requiring business registration which can block small users.
- **Compatibility**: Cloud integration is straightforward. Standalone requires the user to create their own Twilio account and provide keys.
**Design Doc**: OHC acts as the Twilio broker. Users simply toggle "Enable SMS notifications" and provide their phone number. OHC handles the API communication in the background.
**Implementation Prompt**: Create a notification preference setting for SMS. Acceptance criteria: users can toggle SMS on, and critical alerts are delivered to their phone.
**Priority**: P0
**Estimated Scope**: Medium
