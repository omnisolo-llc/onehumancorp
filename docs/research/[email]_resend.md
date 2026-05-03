# Email Marketing Integration (Resend)

## Title
Integrate Resend for Transactional and Marketing Emails

## Problem Statement
Small business owners like Priya (The Boutique Owner) need to communicate with their customers at scale, whether it's sending order confirmations or announcing new stock arrivals. Managing email infrastructure is too technical, and they need a reliable way to ensure their emails reach the inbox, not the spam folder.

## Research Report
- **Tool Evaluated**: Resend (Developer-first email API).
- **Benefits for OHC Users**: Ensures high deliverability for critical transactional emails (receipts, booking confirmations) and provides a clean API for marketing campaigns.
- **Ease of Use**: Invisible to the user. OHC handles the integration. The business owner just writes the content or lets the AI (The Promoter) draft it.
- **Pricing**: Generous free tier (3,000 emails/month), then pay-as-you-go. Very cost-effective for SMBs.
- **Reputation**: Exceptional developer experience, modern infrastructure, high deliverability rates.
- **Cloud vs. Standalone**: Primarily a cloud service. Ideal for OHC Cloud. Standalone might require SMTP configuration.

## Design Doc
- **User Experience**: The user types a message or uses the AI to generate a newsletter. They click "Send to All Customers". Transactional emails happen automatically.
- **Integration**: Use Resend API for sending emails. Handle webhooks for bounce/complaint tracking to maintain list hygiene. Integrate with the OHC customer database.
- **Triggers**: System events (order placed, booking confirmed) or manual campaign trigger.
- **Actions**: Send email via Resend API, log delivery status, update customer profile with engagement metrics.

## Implementation Prompt
Integrate the Resend API to handle both transactional and marketing emails for OHC users. Develop a unified service for sending emails that handles templating and delivery tracking. Ensure high deliverability. Acceptance criteria include successfully sending transactional emails (e.g., order confirmation) and marketing broadcasts to a customer segment, with bounce and delivery tracking recorded in the OHC system.

## Priority
P1

## Estimated Scope
Medium
