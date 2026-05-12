# OHC Tool Integration: Resend for Email Marketing

## Title
Implement Resend Integration for Automated Email Campaigns

## Problem Statement
Small business owners struggle to engage existing customers with promotions or updates because traditional email marketing tools are too complex, expensive, or disconnected from their customer database.

## Research Report
- **Tool Evaluated:** Resend
- **Why Resend?** Modern, developer-friendly API with high deliverability rates. Easier to integrate than legacy platforms like Mailchimp or SendGrid.
- **Ease of Use:** Business owners can use simple templates within OHC. Resend handles the complex delivery infrastructure invisibly.
- **Pricing:** Generous free tier (up to 3,000 emails/month); affordable pay-as-you-go thereafter.
- **Reputation:** Rapidly growing favorite among developers; excellent deliverability reputation.

## Design Doc
- **Trigger:** A business owner creates an email broadcast (e.g., "Holiday Sale") and selects a customer segment.
- **Action:** OHC compiles the email content and recipient list, then queues jobs to send individual emails via the Resend API.
- **User View:** A "Marketing" tab where owners can draft simple rich-text emails and hit "Send to All Customers" without managing external lists.

## Implementation Prompt
Integrate the Resend API for outbound marketing emails. Create a marketing interface where merchants can draft emails and select recipient segments from their existing OHC customer list. Implement a backend worker queue to handle bulk sending via Resend, ensuring rate limits are respected and delivery statuses (sent, bounced) are tracked and displayed back to the merchant.

## Priority
P2

## Estimated Scope
Medium
