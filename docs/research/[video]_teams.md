## [Video] Microsoft Teams Integration
**Title**: Integrate Microsoft Teams for Automated Video Consultations
**Problem Statement**: Consultants and B2B small businesses need video conferencing integrated directly into their booking flows without manually generating and emailing links for every new meeting.
**Research Report**:
- **Tool**: Microsoft Teams (Graph API)
- **Target Persona**: B2B Consultants, Tutors
- **Advantages**: Extremely common in enterprise/B2B settings, included in standard Office 365 subscriptions.
- **Risks**: Graph API authentication can be notoriously complex.
- **Pricing**: Included with existing Microsoft 365 plans.
- **Compatibility**: Cloud (OAuth). Standalone (OAuth).
**Design Doc**:
- User authenticates their Microsoft account.
- When an appointment is scheduled in OHC, a Teams meeting link is automatically generated via the Graph API.
- The meeting link is attached to the calendar invite and sent to the client.
**Implementation Prompt**: Implement Microsoft Graph API OAuth. Add support for creating online Teams meetings and retrieving the join URL. Ensure this seamlessly integrates with the OHC calendar module.
**Priority**: P3
**Estimated Scope**: Large
