# 🔍 Scout: MailerLite (Email Marketing)

## Title
Integrate MailerLite for Simple, Beautiful Email Campaigns

## Problem Statement
Small business owners know they should be building an email list to retain customers, but tools like Mailchimp have become overly complex and expensive. They need a straightforward way to automatically capture emails from OHC checkouts and send visually appealing newsletters or promotional offers without needing a degree in digital marketing.

## Research Report
**MailerLite** is an email marketing tool known for its clean interface, simplicity, and excellent deliverability. It is significantly easier to use for beginners compared to its larger competitors and has a very generous free tier.

**Pros for Non-Technical Users:**
- Extremely intuitive drag-and-drop editor.
- Generous free tier (up to 1,000 subscribers and 12,000 emails/month).
- Built-in landing page and form builders.
- Easy automation workflows (e.g., welcome emails).

**Integration Risks:**
- Strict approval process for new accounts to prevent spam, which might cause friction during onboarding.
- API limits need to be respected when syncing large customer lists from OHC.

**Pricing:**
- Free up to 1,000 subscribers. Paid plans start at $10/month.

**Environment Support:**
- Cloud-based. Standalone requires the user to manage their own API keys.

## Design Doc
- **Integration:** The user connects their MailerLite account via OAuth.
- **Data Flow:** Every time a customer makes a purchase or signs up on the OHC storefront, their email and name are automatically synced to a specific MailerLite group via the API.
- **Action:** The "Marketing & Advertising" agent can use the MailerLite API to draft campaigns based on new OHC products and save them as drafts in MailerLite for the user to review and send.

## Implementation Prompt
Integrate the MailerLite API to automatically sync customer data. Create a background worker that listens for new customer creation events in OHC and pushes that data to the connected MailerLite account, ensuring they are added to a designated "OHC Customers" group. Provide a settings UI to manage the connection and select which group to sync to. Implement a feature where the AI Agent can draft a newsletter campaign using the MailerLite Campaigns API and save it as a draft.

## Priority
P1

## Estimated Scope
Medium
