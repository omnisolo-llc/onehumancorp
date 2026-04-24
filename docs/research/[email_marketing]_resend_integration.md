# Title: Automated Email Campaigns via Resend Integration

## Problem Statement
Business owners like Priya (The Boutique Owner) want to notify their past customers when new stock arrives or a sale is happening. They lack the time to design complex newsletters in Mailchimp and manage subscriber lists. They need their AI agent to draft and send beautiful, effective emails to their customer base automatically.

## Research Report
**Findings & Evaluation:**
- **Resend:** A modern, developer-first email API built for scale. It offers incredible deliverability, clean APIs, and React Email for building beautiful templates programmatically.
- **SendGrid / Mailgun:** Legacy alternatives. While powerful, their developer experience is dated, and managing templates via their UI is clunky compared to programmatic template generation.
- **Ease of Use:** Completely invisible to the business owner. They just tell the Marketing Agent: "Tell my customers about the summer sale."
- **Pricing:** Very affordable startup pricing; fits perfectly within our SaaS margins.
- **Cloud vs Standalone:** Fully supported in Cloud. Standalone users can provide their own Resend API key.

## Design Doc
**Integration with OHC:**
The Marketing Agent ("The Promoter") takes a prompt from the user (e.g., "Draft an email about our new summer collection"). The AI generates the copy and selects product images from the tenant's media library.
OHC compiles this into a beautiful React Email / HTML template matching the OHC Premium Token design system.
The OHC backend pulls the tenant's customer list from the Postgres database (filtering for marketing opt-ins) and dispatches the batch via the Resend API. Resend webhooks update OHC with open and click rates, which the Business Advisory Agent summarizes in the weekly report.

## Implementation Prompt
**User-Facing Outcome & Acceptance Criteria:**
- The business owner can ask their AI Marketing Agent to send an email to all past customers.
- The AI generates a beautiful, branded email template featuring the business's products and colors.
- The owner can review and approve the email in the OHC app before it sends.
- Emails are delivered reliably without ending up in spam.
- The owner receives plain-language analytics (e.g., "300 people opened your email, and 12 clicked the link!").

## Priority
P1

## Estimated Scope
Medium
