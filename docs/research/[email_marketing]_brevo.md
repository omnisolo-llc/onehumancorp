# [Email Marketing] Brevo Integration

## Title
Integrate Brevo for Automated Email Campaigns & Customer Sync

## Problem Statement
Priya (Boutique Owner) wants to email her past customers when new stock arrives, but she doesn't know how to export lists and manage campaigns across disjointed systems. She needs an automated, simple way to re-engage customers with simple newsletters without violating spam laws or wrestling with complex marketing suites.

## Research Report
- **Strategy**: Integrate with Brevo API for audience management and simple email campaigns.
- **Target Persona**: Priya (Boutique Owner), Leo (Music Tutor).
- **Advantages**: Generous free tier (up to 300 emails/day), easy drag-and-drop editor, strong API for a one-way contact sync from OHC. Simplifies email marketing and avoids complex Mailchimp functionality.
- **Risks**: Stricter account approval processes for new accounts to prevent spam.
- **Pricing**: Free tier up to 300 emails/day. Paid tiers start around $25/mo.
- **Ease of Use**: High. The merchant benefits from straightforward list syncing and clean templates.
- **Compatibility**: Cloud (OAuth). Standalone (API Key).

## Design Doc
- **Integration with OHC**:
    - OHC automatically syncs new customers (e.g. from bookings or physical sales) to a designated Brevo audience list via a one-way sync.
    - The "Promoter" AI agent can suggest email campaigns within OHC.
    - Event-based triggers in OHC (e.g., "Customer purchase") sync the contact to Brevo.
- **User View**: A "Marketing" tab in OHC showing simple "Connect Brevo" options.

## Implementation Prompt
Implement an integration to connect to a Brevo account via API/OAuth. Create a scheduled or event-driven sync process to push OHC customer contacts to Brevo automatically after they purchase or interact. Allow the AI Marketing agent to draft simple campaigns that the user can execute easily. Ensure compliance with unsubscribes automatically through the Brevo integration.

## Priority
P1

## Estimated Scope
Medium
