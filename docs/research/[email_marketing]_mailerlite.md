# [Email Marketing] MailerLite Integration

## Title
MailerLite Integration for Affordable Email Campaigns

## Problem Statement
Sarah the Boutique Owner wants to send weekly newsletters to her customer base but finds Mailchimp too expensive and complex. She needs an easy, affordable way to sync her OHC customer list to an email marketing tool.

## Research Report
- **Strategy**: Integration with MailerLite API.
- **Advantages**: MailerLite is very affordable, has a great drag-and-drop editor, and is tailored for SMBs.
- **Risks**: API rate limits. Ensuring robust one-way or two-way sync of subscriber status (unsubscribes).
- **Pricing**: Excellent free tier (up to 1,000 subscribers and 12,000 emails/month).
- **Ease of Use**: API key setup is straightforward. The tool itself is highly rated for simplicity.
- **Compatibility**: Works well in both Cloud and Standalone setups.

## Design Doc
- User enters their MailerLite API key in the OHC integrations dashboard.
- OHC automatically pushes new customer contacts to a designated MailerLite group.
- OHC updates subscriber status if a customer opts out via OHC.

## Implementation Prompt
Build a one-way sync to MailerLite using their REST API. Automatically add new customers to a specified MailerLite group and handle subscriber status updates to maintain list hygiene.

## Priority
P2

## Estimated Scope
Small
