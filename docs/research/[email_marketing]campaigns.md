# [Email Marketing] Campaign Management

## Title
Implement Integrated Email Marketing Campaigns

## Problem Statement
Small business owners often struggle to keep their customers engaged after a purchase or appointment. While they have a list of customer emails, exporting that list from their management software and importing it into a separate, complex email marketing tool is tedious. They need a straightforward way to send newsletters, promotions, or updates directly to their existing customer base without having to learn a complex new platform or manage duplicate contact lists.

## Research Report
### Mailchimp Evaluation
- **Overview:** Mailchimp (Intuit Mailchimp) is a leading marketing automation and email marketing platform.
- **Key Benefits for SMBs:**
  - **Brand Recognition:** Extremely well-known and trusted by small businesses.
  - **Template Quality:** Offers a wide variety of high-quality, easy-to-edit templates.
  - **Analytics:** Provides clear insights into open rates, clicks, and engagement.
- **Challenges/Risks:**
  - **Complexity:** Has grown into a massive platform; the sheer number of features can be overwhelming for a basic user who just wants to send a newsletter.
  - **List Management:** Strict rules around list sizes and unsubscribes; accidental duplication can lead to higher costs.
- **Ease of Use for Non-Technical Users:** The drag-and-drop editor is user-friendly, but the initial setup and list management can be confusing for true beginners. OHC should handle the list synchronization entirely in the background.
- **Cloud vs. Standalone:**
  - **Cloud:** APIs allow seamless syncing of contacts and triggering campaigns.
  - **Standalone:** The standalone app can sync contacts to the Mailchimp cloud via API, keeping the local database as the source of truth.
- **Pricing Estimate:** Free tier available for small lists. Paid plans start around $13/month and scale with the number of contacts.

## Design Doc
- **Integration Trigger:** A new "Marketing" tab on the dashboard with an option to "Connect Mailchimp".
- **Actions Taken:**
  - OHC automatically syncs the "Customers" list to a designated Mailchimp audience.
  - When a customer opts out in OHC or Mailchimp, the status is synced bidirectionally.
  - Users can view high-level campaign stats (sent, opened) directly within OHC.
- **User Experience:**
  - The business owner clicks "Connect", logs in, and their customer list is immediately synced.
  - They see a "Create Campaign" button in OHC that opens Mailchimp's editor.
  - Simple Mode: Basic sync and stats view. Advanced Mode: Ability to segment customers in OHC (e.g., "VIPs") and sync those as specific tags to Mailchimp.

## Implementation Prompt
Create an integration with Mailchimp that removes the friction of contact management for business owners. Build a simple OAuth flow to connect their account, and automatically sync their OHC customer list to Mailchimp in the background. Display basic campaign performance metrics (last sent, open rate) on a new "Marketing" dashboard within OHC. Ensure that the sync respects customer opt-out preferences seamlessly.

## Priority
P2

## Estimated Scope
Medium