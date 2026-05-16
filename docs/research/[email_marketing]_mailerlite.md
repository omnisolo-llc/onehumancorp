# Scout: Tool Integration Research

## Email Marketing
**Title**: Integrate MailerLite for Simple, Beautiful Newsletter Campaigns
**Problem Statement**: Small businesses want to send visually appealing newsletters and automated welcome emails but find tools like Mailchimp overly complex, bloated, and expensive as their list grows.
**Research Report**:
- MailerLite focuses on simplicity and clean design. It is consistently rated as one of the easiest email marketing tools for beginners.
- It includes powerful automation (e.g., welcome sequences) without the steep learning curve.
- Pricing: Excellent free tier up to 1,000 subscribers and 12,000 emails/month. Very cost-effective paid plans.
- Compatibility: Robust API for syncing contacts. Works perfectly in Cloud mode. Standalone mode can utilize API keys.
**Design Doc**:
- In the "Customers" tab, users see a toggle to "Enable Email Marketing (MailerLite)".
- Upon connecting, OHC automatically syncs the customer list (name, email, purchase history tags) to MailerLite.
- "The Ambassador" AI can draft plain-text newsletter content, which the user can then stylize and send directly from the embedded MailerLite interface or via an API-driven simplified UI.
**Implementation Prompt**: Create a bi-directional sync between OHC's customer list and MailerLite. Ensure that when a customer makes a purchase, they are added to the appropriate MailerLite segment. Allow users to trigger automated welcome sequences configured in MailerLite.
**Priority**: P1
**Estimated Scope**: Medium