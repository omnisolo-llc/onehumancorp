# [Email Marketing] Mailchimp Campaigns

**Title**: Mailchimp Integration for Customer Campaigns
**Problem Statement**: Business owners want to send newsletters and promotional offers to their customer list but find it difficult to export/import CSV files constantly.
**Research Report**:
- **Target Persona**: Small retail/service business owners who want to engage customers via email but lack technical marketing skills.
- **Evaluation**: Mailchimp offers a robust, user-friendly platform with excellent template quality and spam compliance. The free tier allows up to 500 contacts, making it ideal for small businesses just starting out.
- **Ease of Use**: High. Intuitive drag-and-drop builder.
- **Pricing**: Free tier up to 500 contacts; Paid starts around $13/mo.
- **Key Risks**: Sync latency with OHC contact list, handling of unsubscribe events (requires two-way sync to maintain compliance).
- **Compatibility**: Works well in Cloud environments via standard APIs. For Standalone, requires polling or webhook setup which might need extra user configuration.
**Design Doc**: Users sync their OHC customer list with Mailchimp with a single click. When a new customer is added in OHC, they are automatically subscribed to a designated Mailchimp audience.
**Implementation Prompt**: Build a sync toggle in customer settings to push new contacts to Mailchimp. Acceptance criteria: new OHC contacts appear in the user's Mailchimp audience automatically.
**Priority**: P2
**Estimated Scope**: Small
