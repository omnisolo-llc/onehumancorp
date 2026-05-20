## [Email Marketing] MailerLite Integration
**Title**: Integrate MailerLite for Customer Campaigns
**Problem Statement**: Keeping customer lists synchronized between the CRM and the email marketing platform is a pain point. Users need an intuitive way to send campaigns.
**Research Report**:
- **Tool**: MailerLite
- **Target Persona**: Micro-businesses
- **Advantages**: Solves the immediate data-silo problem efficiently. MailerLite's intuitive UI and generous free tier make it an ideal partner for micro-businesses.
- **Risks**: Data syncing issues if the sync fails or errors.
- **Pricing**: Generous free tier.
- **Compatibility**: Cloud, Standalone (via outbound API calls).
**Design Doc**:
- User provides MailerLite API credentials to connect their account.
- Build a one-way contact sync from OHC to MailerLite.
- When new customers are added to OHC, they are automatically pushed to a designated MailerLite list.
**Implementation Prompt**: Implement an integration to connect to a MailerLite account via API. Create a scheduled or event-driven sync process to push OHC customer contacts to MailerLite, ensuring lists are kept up-to-date automatically.
**Priority**: P1
**Estimated Scope**: Medium
