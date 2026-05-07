# Email Marketing: Listmonk

**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns

**Problem Statement**: Priya (Boutique Owner) wants to email her past customers when new stock arrives. External tools like Mailchimp are too complex and violate the Radical Simplicity rule. She needs an automated way to email customers natively within the OHC app.

**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- **Pricing**: Standard transaction fees apply.
- **Compatibility**: Cloud (Centralized SendGrid/SES account). Standalone (Centralized routing).

**Design Doc**:
- When a customer buys something, they are automatically added to the native OHC customer list with tags.
- The Marketing agent suggests campaigns natively in the UI.
- The user approves the AI-generated email, and OHC sends it via SendGrid/SES.
- The user sees open rates and clicks in the OHC Marketing dashboard.

**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
- **Priority**: P1
- **Estimated Scope**: Large
- **Acceptance Criteria**:
  - Marketing dashboard allows sending emails to segments.
  - Listmonk engine handles list management and dispatch.

**Strategy**: Self-host Listmonk to ensure data privacy and provide embedded marketing capabilities.
