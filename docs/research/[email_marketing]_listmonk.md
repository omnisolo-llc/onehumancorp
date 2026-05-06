# Scout: Tool Integration Research Q2 - Listmonk

**Title**: Integrate Listmonk for Embedded, No-Jargon Email Campaigns
**Problem Statement**: Priya the Boutique Owner wants to email her past customers when new stock arrives but finds Mailchimp confusing and expensive. She just wants to say "send this to everyone who bought last month."
**Research Report**:
- Listmonk is an open-source, self-hosted newsletter and mailing list manager.
- It is lightweight (Go + PostgreSQL), aligning perfectly with the OHC backend stack.
- Zero extra SaaS costs for OHC Standalone users; minimal scaling costs for Cloud.
- Simplifies list management and supports template-based sending without complex drag-and-drop builders.
**Design Doc**:
- Customer Success ("The Ambassador") tags customers automatically (e.g., "bought-shoes").
- Users type a plain-text prompt: "Draft an email about our new summer dresses."
- AI generates the HTML, Listmonk handles the reliable batch delivery, bounce tracking, and open rate analytics.
**Implementation Prompt**: Integrate Listmonk as the underlying email engine to allow users to trigger marketing emails to specific customer segments directly from the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium
