# Email Marketing: Customer Campaigns

## Title
Integrated Email Campaigns for Customer Lists

## Problem Statement
Business owners want to send newsletters, promotions, or updates to their customers but find Mailchimp or Klaviyo too complex and expensive. They want a simple way to email the customers they already have in the OHC system without exporting CSVs.

## Research Report
- **Tools Evaluated:** Resend, SendGrid, Amazon SES, Postmark.
- **Ease of Use:** Resend has the best developer experience and modern APIs. SendGrid is legacy but proven. Small business owners just want a simple text/image editor, not complex drag-and-drop HTML builders.
- **Pricing:** Resend ($20/mo for 50k emails). SES is cheapest ($0.10/1k) but hard to set up.
- **Reputation:** Postmark has the best deliverability. Resend is very popular for modern SaaS.
- **Cloud vs Standalone:** Requires cloud infrastructure for reliable SMTP delivery and DKIM/SPF handling. Standalone mode might need an OHC cloud relay for sending.

## Design Doc
- **Trigger:** User selects a group of customers in the CRM and clicks "Send Email Campaign".
- **Action:** User writes a subject and message in a simple editor. OHC dispatches emails via API (e.g., Resend).
- **User View:** A simple editor with basic formatting. A dashboard showing open and click rates for past campaigns.

## Implementation Prompt
Build a simple email campaign tool integrated directly with the OHC customer list. Users should be able to write an email, select recipients, and send. Track basic metrics like opens. Focus on a clean, simple editor (like Notion) rather than a complex HTML layout builder. The sending infrastructure should be abstracted away from the user.

## Priority
P2

## Estimated Scope
Medium
