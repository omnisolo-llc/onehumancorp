## [Video] Microsoft Teams Integration
**Title**: Integrate Microsoft Teams for Video Conferencing
**Problem Statement**: Manually generating video links for B2B consultations is inefficient and looks unprofessional.
**Research Report**:
- **Tool**: Microsoft Teams (Microsoft Graph API)
- **Target Persona**: B2B service providers relying on the Microsoft ecosystem
- **Advantages**: Automatically generates Teams meeting links for new bookings, which is high value for specific B2B verticals.
- **Risks**: Microsoft Graph API OAuth can be complex and requires specific Active Directory permissions.
- **Pricing**: Included with Microsoft 365.
- **Compatibility**: Cloud, Standalone (via OAuth / Graph API).
**Design Doc**:
- User authenticates via Microsoft OAuth.
- For online service bookings, OHC calls the Microsoft Graph API to create an online meeting.
- The Teams meeting link is retrieved and embedded into calendar invites and customer confirmation emails.
**Implementation Prompt**: Create an OAuth integration with Microsoft Graph API. When a user books an online service, automatically generate a Teams meeting link via the API and include it in the booking confirmation details.
**Priority**: P2
**Estimated Scope**: Medium
