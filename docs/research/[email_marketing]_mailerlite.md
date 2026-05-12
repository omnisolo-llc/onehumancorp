# MailerLite Email Marketing Integration

## Problem Statement
Small business owners want to send newsletters or promotional emails to their customer base but find tools like Mailchimp too complex, expensive, and bloated. They need a simple, affordable way to design and send professional emails without needing a marketing degree.

## Research Report
MailerLite is an email marketing tool focused on simplicity and affordability.
- **Ease of Use**: Features a very intuitive drag-and-drop editor that is less overwhelming than competitors. It is designed for beginners.
- **Pricing**: Excellent free tier (up to 1,000 subscribers and 12,000 emails/month). Paid plans are very affordable compared to the industry standard.
- **Reputation**: Highly praised for its clean interface, excellent customer support, and value for money.
- **Environment**: Cloud-based. Integration in Standalone mode would require an internet connection to sync lists and trigger campaigns via their API.

## Design Doc
**Trigger**: Business owner navigates to "Marketing" and clicks "Create Email".
**Action**: User selects a template or uses a simple drag-and-drop builder to create an email. The recipient list is automatically synced from the OHC customer database.
**User Experience**: The business owner can quickly design a nice-looking email and hit send. They can view basic stats (open rate, click rate) directly within the OHC dashboard.

## Implementation Prompt
Integrate a simple email marketing capability allowing the user to draft an email using a basic visual editor and send it to their customer list. Automatically sync the OHC customer list with the email provider. Display basic campaign performance metrics (sent, opened) in the OHC UI.

## Priority
P1

## Estimated Scope
Medium
