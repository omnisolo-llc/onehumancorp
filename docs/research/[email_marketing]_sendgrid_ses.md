## [Email Marketing] Native Email Campaign Manager
**Title**: Native Email Campaign Manager (SendGrid/SES)
**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.
**Research Report**:
- **Strategy**: Build a native email campaign manager utilizing a transactional email API (SendGrid or AWS SES)
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor)
- **Advantages**: Keeps the user within the OHC ecosystem. The Marketing agent can fully control the campaign without learning a third-party tool. No additional SaaS subscriptions required for the user.
- **Risks**: Requires building list management and unsubscribe logic internally.
- **Pricing**: Included in OHC platform costs (transactional API costs scale predictably).
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).
**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.
- **AI Integration**: The Marketing & Advertising Agent writes the subject lines, generates the copy, and tracks open/click rates to suggest the best times to send future emails.
**Implementation Prompt**: Build a native email campaign management system. Utilize SendGrid/SES for delivery. Allow the AI Marketing agent to create and queue email campaigns directly from the OHC database.
**Priority**: P1
**Estimated Scope**: Large
