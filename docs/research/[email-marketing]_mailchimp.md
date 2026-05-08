# Title: Mailchimp Integration for Email Campaigns

## Problem Statement
Small business owners often have a list of customer emails but lack a simple, integrated way to send promotional newsletters or announcements. Exporting CSVs from their store and importing them into another tool is tedious and leads to outdated lists.

## Research Report
Mailchimp is the standard for small business email marketing.
- **Ease of use:** Very intuitive drag-and-drop template builder.
- **Pricing:** Free tier available, then scales by contacts.
- **Reputation:** Excellent, high deliverability rates.
- **Key advantages:** High brand recognition, robust analytics, and excellent spam compliance features.
- **Risks:** The free tier has become more restrictive recently. Strict compliance requirements mean users can easily get banned if they import low-quality lists.
- **Environment:** Cloud works perfectly via API. Standalone works as well since it relies on outbound API calls to sync contacts.

## Design Doc
- User goes to "Marketing" and connects their Mailchimp account via OAuth.
- OHC automatically syncs the "Customers" list to a Mailchimp Audience.
- When a new customer is added in OHC, they are seamlessly added to the Mailchimp list (if they opt-in).
- User builds campaigns inside Mailchimp, but basic stats (Open Rate, Clicks) are displayed on the OHC dashboard.

## Implementation Prompt
Integrate Mailchimp API to provide contact syncing. Create an OAuth flow to connect the account. Implement a background job that listens for new customer creations in OHC and pushes them to the connected Mailchimp Audience. Fetch high-level campaign metrics to display on the home dashboard.

## Priority
P2

## Estimated Scope
Medium
