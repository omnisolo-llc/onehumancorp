## [Email Marketing] Automated Customer Outreach
**Title**: Integrate Resend / MailerLite for Simple Email Campaigns

**Problem Statement**: Small businesses have a list of past customers but no easy way to send them updates, promotions, or newsletters. Enterprise tools like Mailchimp have become too bloated, expensive, and intimidating for simple use cases.

**Research Report**:
- **Persona Context**: Local shops, bakeries, and creators wanting to announce new products or seasonal sales to their existing customer base.
- **Solution Evaluated**: MailerLite for visual builders, Resend for developer-friendly transactional + marketing emails. MailerLite is vastly superior for non-technical users due to its intuitive drag-and-drop builder.
- **Ease of Use**: MailerLite is very accessible for non-designers. Resend requires more custom UI work from our side to make it usable for the end-user.
- **Advantages**: High deliverability, simple list management, robust analytics (open rates, clicks).
- **Risks**: Spam compliance (CAN-SPAM/GDPR). Users might accidentally send spam and get their domains blacklisted.
- **Pricing Estimate**: MailerLite is free up to 1,000 subscribers, then starts at $9/month.
- **Cloud/Standalone Support**: Cloud-native SaaS integration. Standalone mode can bridge to these services using API keys provided by the user.

**Design Doc**:
- **Triggers**: User initiates a "New Campaign" or an automated flow (e.g., "Welcome Email" when a new customer is added).
- **Actions**: OHC syncs the local customer list to the email provider and triggers the send.
- **User Interface**: A "Marketing" tab where users can select segments of their customer list, write a plain-text or simple rich-text email, and click "Send". Analytics (opens/clicks) are displayed next to past campaigns.

**Implementation Prompt**:
Implement a simple email campaign tool. Users should be able to select contacts from their OHC customer list, compose an email with a subject and body, and send it. Show a basic summary of sent campaigns with open and click rates. Ensure users can easily provide their own API keys for the email service in Standalone mode.

**Priority**: P2
**Estimated Scope**: Medium
