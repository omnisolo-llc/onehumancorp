# Mailchimp Customer Engagement Integration

## Problem Statement
Small business owners often have lists of customer emails but lack a simple way to send professional-looking newsletters, promotions, or updates. They find enterprise email tools confusing and need a straightforward way to reach their customers to drive repeat business without worrying about spam laws or complex template design.

## Research Report
Mailchimp is a veteran in the email marketing space, known for its user-friendly approach for small businesses.
- **Ease of Use**: Features an intuitive drag-and-drop builder and plain-language analytics.
- **Capabilities**: Customer list management, attractive email templates, open/click tracking, and automated compliance (unsubscribe links).
- **Competitors**: Resend, SendGrid, Listmonk. While SendGrid is great for developers, Mailchimp is purpose-built for non-technical small business owners.
- **Reputation**: High trust and widespread adoption among SMBs.
- **Pricing**: Generous free tier (up to 500 contacts and 1,000 sends/month), making it risk-free for new businesses. Essentials plan starts around $13/month.
- **Deployment**: Exposes extensive REST APIs for audience management. Works seamlessly across Cloud and Standalone environments.

## Design Doc
OHC will synchronize its internal customer list with a Mailchimp Audience. When a new customer is added in OHC (e.g., via a purchase or booking), they are automatically synced to Mailchimp. The OHC dashboard will feature a "Marketing" tab that displays high-level stats (e.g., recent campaign open rates) pulled from Mailchimp's API. The actual email design and sending will continue to happen in Mailchimp's user-friendly interface.

## Implementation Prompt
Create a "Connect Email Marketing" option in the OHC settings that guides the user through Mailchimp OAuth. Once connected, automatically sync OHC contacts to Mailchimp in the background. On the dashboard, display a simple "Recent Email Performance" card showing the name of the last sent email, how many people opened it, and how many clicked. Use clear terms like "People who opened" instead of "Open Rate percentage".

## Priority
P2

## Estimated Scope
Medium
