# Integrate Loops for Simplified Email Marketing

## Problem Statement
Small business owners know they should be sending newsletters or promotional emails to their customer base, but traditional tools (like Mailchimp or Klaviyo) are overly complex, bloated with features they don't need, and visually overwhelming. They need a simple, fast way to send a beautiful, plain-text or lightly styled email to their customer list without a steep learning curve.

## Research Report
**Tool**: Loops.so
Loops is a modern email marketing platform designed for simplicity and speed, popular among SaaS and modern small businesses.
- **Ease of use**: Extremely straightforward interface focused on writing the email rather than complex block builders.
- **Pricing**: Free tier up to 1,000 contacts, then scales fairly. Very competitive for small lists.
- **Reputation**: Growing rapidly, praised for excellent deliverability and clean design.
- **Environment**: Cloud-based API. For Standalone, OHC will need to proxy API calls securely or allow the user to input their own Loops API key.

## Design Doc
The integration will allow the user to sync their OHC customer contacts to Loops and trigger simple campaigns.
- **Trigger**: User connects their Loops account via API key in OHC settings.
- **Actions**: OHC will automatically sync new customer emails (from orders or signups) to a specific Loops audience list. OHC will also display basic metrics (open rate, click rate) for the latest sent campaign.
- **User View**: A "Marketing" tab that shows the total synced contacts and a quick summary of the last email sent (Subject, Date, Open Rate).

## Implementation Prompt
Create an "Email Marketing" settings section where the user can input their Loops API key. Implement a background job that syncs the OHC customer database to the Loops contacts list. In the marketing dashboard, use the Loops API to fetch and display the performance of the most recent email campaign. Ensure that if the API key is missing or invalid, the UI gracefully prompts the user to connect their account.

## Priority
P2

## Estimated Scope
Medium
