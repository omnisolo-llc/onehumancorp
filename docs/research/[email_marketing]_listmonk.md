# Scout: Tool Integration Research Q2

## 3. Email Marketing
**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month."
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- It is lightweight, aligning perfectly with the OHC stack.
- Avoids extra third-party subscription costs for users while keeping platform operations simple.
- Simplifies list management and supports template-based sending without complex drag-and-drop builders.
**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the email content, and Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium
