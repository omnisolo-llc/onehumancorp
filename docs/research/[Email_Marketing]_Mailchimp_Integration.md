# [Email Marketing] Mailchimp Integration

**Problem Statement**: Business owners collect customer emails but find it hard to keep their email marketing lists in sync with their actual customer database. They need new contacts added to OHC to automatically flow into their newsletter tool.

**Research Report**:
- **Target Persona**: Retailers, online stores, content creators.
- **Ease of Use**: Mailchimp is very popular. OAuth integration is standard.
- **Pricing**: Free tier available (up to a certain number of contacts/sends).
- **Reputation**: Very established, though pricing changes have frustrated some small users.
- **Cloud/Standalone**: Works in both via standard API calls.

**Design Doc**:
- **Trigger**: A new customer is added to OHC (e.g., via a purchase or contact form).
- **Action**: OHC pushes the contact information to a designated Mailchimp audience.
- **User View**: Business owner selects a Mailchimp list in settings. They don't need to manually export/import CSVs anymore.

**Implementation Prompt**: Create a Mailchimp integration that allows the business owner to authenticate and select an audience. Automatically sync new contacts created in OHC to the selected Mailchimp audience.

**Priority**: P1
**Estimated Scope**: Medium
