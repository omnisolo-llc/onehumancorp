## [Email Marketing] Issue Brief: Embedded, No-Jargon Email Campaigns

**Title**: Scout 🔍: Integrate Listmonk for Embedded Email Campaigns
**Problem Statement**:
Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month" directly from her OHC dashboard, without needing to learn a new tool.

**Research Report**:
- **Tool**: Listmonk.
- **Evaluation**: Listmonk is an open-source, self-hosted newsletter and mailing list manager. It is lightweight (Go + PostgreSQL), aligning perfectly with the OHC backend stack.
- **Ease of Use**: Extremely high for the end-user, as it is completely abstracted. They only interact with OHC's native interface.
- **Advantages**: Zero extra SaaS costs for OHC Standalone users; minimal scaling costs for Cloud. Simplifies list management and supports template-based sending without complex drag-and-drop builders.
- **Risks**: Requires managing email delivery reputation (or pairing it with SES/SendGrid).
- **Pricing**: Free and open-source.
- **Compatibility**: Perfect for Standalone (runs locally alongside OHC). Great for Cloud (hosted centrally).

**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the HTML natively in OHC.
- OHC passes the campaign to the underlying Listmonk engine.
- Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics, which are then displayed natively in OHC.

**Implementation Prompt**:
Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard. Abstract the Listmonk interface completely, exposing only the campaign creation and analytics natively in OHC.
- **Acceptance Criteria**: User can create an email campaign. AI can generate content. Emails are delivered via Listmonk. Open rates are tracked and displayed.
**Priority**: P2
**Estimated Scope**: Medium
