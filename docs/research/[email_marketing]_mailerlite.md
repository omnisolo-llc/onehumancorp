## 3. Email Marketing: MailerLite

**Title:** Integrate MailerLite for Affordable SMB Email Campaigns
**Problem Statement:** Sending newsletters and promotional emails can be expensive and complicated. Owners need an easy way to blast updates to their customer list.
**Research Report:**
- **Tool evaluated:** MailerLite
- **What problem it solves for which persona:** Helps retail or service owners easily send marketing emails to their existing customers.
- **Ease of Use:** Extremely user-friendly drag-and-drop editor.
- **Pricing:** Free for up to 1,000 subscribers, then starts around $10/month.
- **Reputation:** Known for great deliverability and affordability compared to Mailchimp.
- **Advantages & Risks:**
  - *Advantages:* Cost-effective, clean UI, good automation features.
  - *Risks:* Strict approval process for new accounts might frustrate some users.
- **Cloud/Standalone Mode:** Works in both via standard REST API.
**Design Doc:**
- **Trigger:** A new customer purchases a product or signs up.
- **Action:** Customer email is synced to a MailerLite list. Business owner drafts an email in OHC that gets sent via MailerLite.
- **User View:** A 'Marketing' tab in OHC showing subscriber count, recent campaigns, and basic open rates.
**Implementation Prompt:**
Implement a sync mechanism that adds new customer emails to a third-party mailing list. Build a simple UI for the business owner to view their current subscriber count and trigger a pre-defined campaign template.
**Priority:** P1
**Estimated Scope:** Medium
