# Email Marketing - Mailchimp

## Problem Statement
Business owners want to send newsletters and promotional offers to their customer list but find full CRM tools too complex. They need a simple way to email past customers about new products or sales.

## Research Report
Mailchimp is widely recognized by small businesses.
- **Ease of Use**: Good drag-and-drop email builder, though the dashboard can sometimes be cluttered.
- **Pricing**: Free for up to 500 contacts and 1,000 sends/month. Paid starts at $13/month.
- **Reputation**: Well-established, strong deliverability.
- **Cloud/Standalone**: Cloud-based SaaS.

## Design Doc
- **Trigger**: A new customer makes a purchase or signs up on the OHC storefront.
- **Action**: OHC automatically adds the customer to a Mailchimp audience via API.
- **User View**: Business owner sees a "Marketing" tab where they can draft an email (using OHC's AI) and click "Send to all customers", which dispatches via Mailchimp.

## Implementation Prompt
Integrate Mailchimp for email marketing. Automatically sync OHC customer lists to a Mailchimp audience. Provide a simple interface in OHC for users to draft and send basic email blasts.
- Acceptance Criteria: Customers are auto-added to Mailchimp. User can send a text/image email to their list directly from the OHC dashboard.

## Priority
P1

## Estimated Scope
Medium

---
