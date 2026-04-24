# Email Marketing Tools

**Title**: Integrate Email Marketing capabilities (Resend, MailerLite)

**Problem Statement**:
Business owners (like Priya the Boutique Owner) want to notify their existing customers about new products, sales, or updates. They find tools like Mailchimp too complex and expensive. They need a simple way for the "Marketing & Advertising" agent to draft and send beautiful email blasts.

**Research Report**:
Evaluated Resend and MailerLite.
- **Resend**: Developer-first email sending API.
  - *Ease of Use*: For the developer, amazing. For the user, it requires OHC to build the entire email builder UI.
  - *Pricing*: Very generous free tier (3,000 emails/mo).
  - *Reputation*: Excellent deliverability and modern API.
- **MailerLite**: A complete marketing platform.
  - *Ease of Use*: Good, but pushes the user out of OHC to use their drag-and-drop builder.
  - *Pricing*: Free up to 1,000 subscribers.
- **Recommendation**: Use Resend for the infrastructure, and have OHC's "Marketing & Advertising" AI agent generate the HTML content for the emails. This keeps the user entirely within the OHC ecosystem.

**Design Doc**:
- **Trigger**: User asks the AI Agent to "Send an email to all past customers about the summer sale."
- **Action**: The Marketing AI agent drafts the email copy and generates a simple HTML template using OHC Premium design tokens. The user reviews the draft in the OHC app. Upon approval, the backend uses the Resend API to blast the email to the user's customer list.
- **User Experience**: A chat-like or simple form interface where the user describes the campaign. The AI does the layout. The user clicks "Approve and Send". They can see basic stats (open rate) in their dashboard.

**Implementation Prompt**:
Integrate the Resend API for bulk email sending. Create a backend service that compiles an OHC customer segment (e.g., "all past purchasers") and dispatches the emails. Build a UI in Flutter where the user can view an AI-generated email preview and approve it. Handle bounce and complaint webhooks from Resend to maintain list hygiene.

**Priority**: P1
**Estimated Scope**: Medium
