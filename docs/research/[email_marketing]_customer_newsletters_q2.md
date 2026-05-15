# [email_marketing] Issue Brief: Automated Customer Newsletters

**Title**: Simple Email Marketing via Resend
**Problem Statement**: As a boutique owner like Priya, I want to let my past customers know when the new summer collection arrives. I don't understand how to use complex tools like Mailchimp, and they are too expensive. I need a "one-click" way to tell my AI to send a beautiful email to everyone who has bought from me before.
**Research Report**:
- Evaluated Tools: Resend, SendGrid, Mailchimp API.
- Ease of Use: Resend has the most developer-friendly API and excellent email deliverability without complex setup. Mailchimp is powerful but heavily branded and expensive for the end-user if integrated directly.
- Pricing: Resend offers 3,000 free emails/month, which is perfect for our free-tier small businesses. SendGrid is also good but has a steeper learning curve for domain authentication.
- Reputation: Resend is highly regarded for transactional and simple marketing emails.
- Environment: Cloud-only. Requires domain verification for best results, though we can use a shared sender domain (e.g., `mail.onehumancorp.com`) for non-custom-domain users.
- Recommendation: Integrate Resend for sending outbound marketing and transactional emails.
**Design Doc**:
- **Integration Flow**: Invisible to the user. OHC provisions a Resend sub-domain or uses the primary domain.
- **Actions/AI Integration**: "The Promoter" (Marketing Agent) drafts an email based on Priya's prompt ("Tell customers about summer collection"). Priya reviews the draft in the app. Upon approval, OHC uses the Resend API to blast it to all customers tagged in the CRM.
- **User Interface**: A "Campaigns" section under the Marketing department. Shows a simple text box to instruct the AI, a preview of the generated email (Glassmorphism card), and an "Approve & Send" button. Basic stats: "Sent to 150 people", "Opened by 45".
**Implementation Prompt**: Build a simple Email Marketing sender using the Resend API. The Marketing Agent must be able to draft an HTML email template. The user must be able to review the draft and click "Send". The system will then iterate through the user's customer list and dispatch the emails. Acceptance criteria: emails are successfully delivered to test inboxes, open tracking (if available) is reported back to the UI, and the UI remains simple with zero technical jargon.
**Priority**: P2
**Estimated Scope**: Medium
