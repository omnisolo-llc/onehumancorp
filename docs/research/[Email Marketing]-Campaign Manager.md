# Integrated Email Campaign Manager

## Problem Statement
Exporting customer lists to external email marketing tools is tedious. Business owners want to send newsletters or promotions directly to their existing customer base without managing multiple platforms.

## Research Report
Evaluated tools for email campaigns integrated with customer lists.

- **Ease of Use**: High value for retention and promotions.
- **Pricing**: External tools like Mailchimp scale pricing based on list size, getting expensive quickly.
- **Risks**: Spam compliance (CAN-SPAM/GDPR), bounce handling, maintaining high deliverability.
- **Modes**: Cloud-based transactional email providers (SendGrid, AWS SES) are required; Standalone will need a configured SMTP provider.

## Design Doc
User selects segments from their OHC customer CRM. They use a simple WYSIWYG editor to draft an email. OHC queues the emails and sends them via an integrated email provider, tracking open rates and clicks, and displaying the analytics on the dashboard.

## Implementation Prompt
Build a simple email composer and a campaign dashboard showing open/click metrics. Integrate a sending queue that batches emails to an external SMTP service.

## Priority
P1

## Estimated Scope
Medium
