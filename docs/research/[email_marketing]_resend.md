# Issue Brief: Automated Transactional Emails

## Title
Implement Automated Transactional Emails for Small Business Owners

## Problem Statement
When a customer pays a deposit, they expect a receipt instantly. If it goes to spam, they panic and call the business, causing unnecessary stress for the owner.

## Research Report
Resend is a highly reliable service for sending essential platform emails.

**Persona Impact:** The business owner doesn't have to do anything. OHC ensures that beautiful, branded, and highly-deliverable emails are sent to their customers exactly when needed.

**Advantages:** Extremely high delivery rates. Ensures the business looks highly professional.

**Risks:** None for the user. OHC handles all complexity.

**Pricing Estimate:** Free for the user (OHC covers the infrastructure cost under the hood).

**Environment:** Works perfectly in both Cloud and Standalone modes.

## Design Doc
1.  **Branding Settings:** User uploads their logo and chooses a brand color in OHC settings. All outbound receipts automatically use this branding.
2.  **No Configuration:** There is no setup required from the user. It works by default.

## Implementation Prompt
Implement a robust transactional email system that automatically sends branded receipts and appointment confirmations to customers without any user configuration.

## Priority
P0

## Estimated Scope
Medium

### Unique Considerations
Transactional emails must be visually identical across all clients. The React Email templates used with Resend must be rigorously tested in Outlook, Gmail, and Apple Mail to ensure the small business's branding always looks impeccable.
