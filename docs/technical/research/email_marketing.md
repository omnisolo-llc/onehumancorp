# [Email Marketing] Integrate Resend for Customer Campaigns

## Problem Statement
Boutique owners like Priya need to notify customers when new stock arrives. Managing a separate tool like Mailchimp is complex and requires importing/exporting CSVs of customer emails. They need an automated, beautiful way to send emails directly from their customer list in OHC.

## Research Report
**Evaluated Tool:** Resend
**Alternatives Considered:** SendGrid, Mailgun
**Pros:** Developer-friendly, extremely fast, excellent deliverability out-of-the-box. Built with modern React Email components in mind, making it easy to generate beautiful, mobile-responsive templates programmatically.
**Cons:** Newer player, fewer legacy features compared to SendGrid.
**Ease of Use for Non-technical Users:** The user simply clicks "Send Campaign" or "Generate Email". The AI and Resend handle the formatting, delivery, and open-rate tracking automatically.
**Pricing:** Generous free tier, then volume-based. Very affordable for SMBs.
**Deployment:** Cloud-native. Perfect for multi-tenant.

## Design Doc
**Integration with OHC:**
- **Trigger:** "The Promoter" agent schedules an email campaign, or the user clicks "Send Newsletter".
- **Action:** OHC generates the email HTML (using React Email or similar) and sends it via the Resend API to the filtered customer list.
- **AI Agent Interaction:** "The Promoter" drafts subject lines and email body text based on new inventory or seasonal events.
- **User View:** A "Marketing Campaigns" tab showing draft emails, sent emails, open rates, and click rates.

## Implementation Prompt
Integrate the Resend API to enable bulk and transactional email sending. Create a UI flow for users to select a customer segment and generate an email campaign. Ensure "The Promoter" AI can draft templates and that open/click events are tracked via Resend webhooks.

## Priority
P1

## Estimated Scope
Medium
