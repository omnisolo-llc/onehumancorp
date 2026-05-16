# Title: Integrate Mailchimp for Customer Email Marketing

## Problem Statement
Small business owners collect customer emails but often fail to utilize them for marketing, promotions, or newsletters because setting up a standalone email platform is tedious. They need an automated way to sync their customer list from OHC into a marketing tool so they can easily send professional emails.

## Research Report
Mailchimp is one of the most popular email marketing platforms globally.
- **Ease of Use:** Excellent drag-and-drop template builder that requires zero coding. Campaign setup is guided and intuitive for non-technical users.
- **Pricing:** The free tier allows up to 500 contacts and 1,000 sends per month, which is perfect for new or very small businesses. Paid plans start around $13/month.
- **Reputation:** Extremely strong reputation; widely supported and highly reliable regarding spam compliance and deliverability.
- **Competitors:** Sendinblue (Brevo), Constant Contact, ConvertKit. While Brevo is cheaper at scale, Mailchimp's brand recognition and template quality make it the most requested tool by small business owners.
- **Cloud vs Standalone:** API access allows simple sync of contacts in both Cloud and Standalone modes.

## Design Doc
OHC will act as the source of truth for customer data, automatically pushing new contacts to a designated Mailchimp audience list.
- **Trigger:** A new customer is added to OHC (e.g., via a booking, purchase, or manual entry) and opts in to marketing.
- **Action:** OHC adds or updates the contact in the connected Mailchimp audience list via API.
- **User Interface:** A "Marketing" settings page where the user logs into Mailchimp via OAuth and selects a target Audience list. The user will see a simple "Sync Status" indicator. Actual email authoring will happen in Mailchimp.

## Implementation Prompt
Implement a Mailchimp integration that allows a user to connect their account via OAuth. Create a background sync process that automatically adds any new customer email (with marketing consent) to the selected Mailchimp audience list. Provide a settings UI to manage the connection and view sync history. Do not build an email editor in OHC; focus entirely on keeping the contact list in sync.

## Priority
P1

## Estimated Scope
Medium