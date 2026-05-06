# Integrate Mailchimp for Customer Email Campaigns

## Problem Statement
After collecting customer contacts through bookings, sales, or inquiries, small business owners struggle to engage them effectively. Exporting lists to external tools is tedious and error-prone. They need a way to easily sync their OHC customer contacts to an email marketing platform to send newsletters, promotions, or updates without manual data entry.

## Research Report
**Findings & Data**: Mailchimp is a leading marketing automation and email marketing platform widely adopted by small to medium businesses.
**Ease of Use**: Mailchimp is known for its user-friendly interface, drag-and-drop template builder, and straightforward list management. For the OHC integration, the user simply connects their account to automate contact syncing.
**Pricing**: Offers a robust free tier (often up to a certain number of contacts/emails per month), making it highly accessible for new or very small businesses. Paid tiers scale with the contact list size.
**Reputation**: Strong reputation for deliverability, analytics, and compliance (handling unsubscribes, CAN-SPAM).

## Design Doc
**Integration flow**:
1.  **Connection**: The user authenticates their Mailchimp account via OAuth in the OHC integration settings.
2.  **Mapping**: The user selects a target Mailchimp Audience (list) where OHC contacts should be synced.
3.  **Synchronization**: Whenever a new customer is added in OHC (e.g., a new booking, a new sale), OHC automatically pushes the contact details (Name, Email) to the selected Mailchimp audience via API.
4.  **Campaigns**: The user logs into Mailchimp to design and send the actual email campaigns, utilizing the automatically updated contact list.

## Implementation Prompt
**User-Facing Outcome**: The user can connect their Mailchimp account to OHC. Once connected, any new customer email captured within OHC is automatically added to their specified Mailchimp mailing list.
**Acceptance Criteria**:
- OAuth flow to connect Mailchimp account.
- UI dropdown to select the destination Mailchimp Audience.
- Automatic background syncing of new OHC customer records to Mailchimp.
- Graceful handling of existing contacts (no duplicates).
- Works seamlessly in both Cloud and Standalone environments.

## Priority
P2

## Estimated Scope
Small
