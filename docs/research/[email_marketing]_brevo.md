# [email_marketing] Brevo (formerly Sendinblue) Integration

## Problem Statement
Small business owners need an effective way to engage their customer base with newsletters, promotional offers, and transactional emails. Managing email lists and campaigns outside of their primary CRM (OHC) leads to data silos and disjointed marketing efforts. Integrating Brevo into OHC allows business owners to seamlessly sync their contacts and trigger marketing campaigns directly from their unified business platform.

## Research Report
### Overview
Brevo is a comprehensive marketing platform offering email marketing, SMS marketing, and marketing automation. It is well-suited for small to medium-sized businesses due to its competitive pricing structure, which is based on email volume rather than contact count.

### Ease of Use
The integration should allow business owners to connect their Brevo account via an API key or OAuth. The primary user experience within OHC will be segmenting their OHC customer list and pushing those segments to Brevo lists. OHC should also display high-level campaign metrics (open rates, click rates) from Brevo.

### Reputation
Brevo is highly regarded for its deliverability, feature set (including transactional emails and automation workflows), and value for money, especially compared to competitors like Mailchimp.

### Pricing
Brevo offers a generous free tier (300 emails/day) and paid plans starting around $25/month. This makes it an attractive option for budget-conscious small businesses.

### Environment
Works in Cloud.

### AI Integration
High potential. AI could assist in drafting email subject lines and body content based on the campaign goals, optimizing send times based on historical engagement data, and segmenting the customer list dynamically.

## Design Doc
1.  **Connection:** User connects their Brevo account via "Integrations" -> "Marketing" -> "Connect Brevo".
2.  **Contact Sync:** OHC provides a "Sync to Brevo" action on the Customer List view. This pushes selected contacts (or segments) to a specific list within Brevo.
3.  **Campaign Management:** OHC displays a simplified view of recent Brevo campaigns and their core metrics (Sent, Opened, Clicked).
4.  **Transactional Triggers:** OHC can utilize Brevo's transactional email API to send system emails (e.g., order confirmations, appointment reminders) configured by the business owner.

## Implementation Prompt
Implement a contact synchronization integration with Brevo. The user should be able to connect their Brevo account via an API Key. Once connected, add a "Sync to Brevo" button to the CRM/Customer List view. When clicked, the selected contacts should be added to or updated in a specific Brevo list (allowing the user to select the destination list). Additionally, pull high-level metrics for the 5 most recent email campaigns from Brevo and display them on a marketing dashboard within OHC.

## Priority
P2 (Medium) - Important for growth-focused businesses, but secondary to core communication channels.

## Estimated Scope
Medium
