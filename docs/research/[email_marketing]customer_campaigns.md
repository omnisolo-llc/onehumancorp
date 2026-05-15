### Title
`[email_marketing]customer_campaigns`: Implement Email Campaigns via Resend

### Problem Statement
Reaching out to past customers with promotions or updates is crucial for repeat business, but complex tools like Mailchimp are overkill and expensive for simple announcements. Business owners need a straightforward way to email their customer list directly from the platform they already use to manage their business.

### Research Report
- **Tool**: Resend
- **Pros**: Developer-friendly, simple API, excellent deliverability, built-in React email templates.
- **Cons**: Newer player, fewer out-of-the-box marketing features compared to legacy providers.
- **Reputation**: Highly regarded in the developer community for its modern approach and ease of use.
- **Pricing**: Generous free tier (3,000 emails/month), then very affordable ($20/month for 50,000 emails).
- **Ease of Use for Non-Technical Users**: The user only interacts with a simple composer in OHC; the complexity of SMTP and domain verification is abstracted.
- **Modes Supported**: Cloud and Standalone (via API calls).

### Design Doc
- **Trigger**: The business owner selects a list of customers and clicks "Send Email Campaign".
- **Action**: The OHC API server formats the email and dispatches it via the Resend API, tracking delivery status.
- **User View**: A simple email composer with audience selection and a basic performance dashboard (open rates, bounces).

### Implementation Prompt
Integrate Resend to enable basic email marketing capabilities. Users should be able to select segments of their customer database and send batch emails. Implement a simple, foolproof email composer. Ensure the system handles bounce and complaint webhooks to automatically clean the user's mailing list and maintain high deliverability scores.

### Priority
P2

### Estimated Scope
Medium
