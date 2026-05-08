## [Email Marketing] Issue Brief: Listmonk Integration for Self-Hosted Newsletters

**Title**: Scout 🔍: Integrate Listmonk for Privacy-First Email Marketing
**Problem Statement**:
Small businesses need to send newsletters and promotional emails to their customer base, but commercial tools (Mailchimp, etc.) can become expensive as lists grow. Standalone OHC users also require a solution that keeps customer data entirely local and private.
**Research Report**:
- **Tool**: Listmonk
- **Evaluation**: A fast, standalone, self-hosted newsletter and mailing list manager.
- **Ease of Use**: The admin UI is clean, but initial setup (SMTP configuration) can be technical.
- **Pricing**: Open-source and free. User pays only for their SMTP provider (e.g., AWS SES, SendGrid).
- **Cloud vs. Standalone**: Perfect for Standalone (can be bundled). For Cloud, OHC would need to host a multi-tenant instance or run separate instances per tenant.
**Design Doc**:
- Users manage their subscriber lists within OHC's CRM.
- OHC syncs this list to Listmonk.
- Users compose emails in OHC (or directly in Listmonk's UI if embedded).
- Listmonk handles the high-throughput delivery via the configured SMTP server and tracks opens/clicks.
**Implementation Prompt**:
Integrate Listmonk as the default email marketing engine for Standalone OHC. Create a seamless sync between OHC's CRM contacts and Listmonk's subscriber lists. Provide a simplified UI within OHC for users to configure their SMTP settings and trigger campaigns.
**Priority**: P2
**Estimated Scope**: Large
