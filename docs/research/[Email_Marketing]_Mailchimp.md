**Title**: Mailchimp Integration for Seamless Customer Campaigns
**Problem Statement**: Business owners collect emails but don't know how to engage them. Exporting lists from their customer database to an email tool is tedious and often forgotten, leading to missed marketing opportunities.
**Research Report**: Mailchimp is highly recognizable and tailored for small businesses. It offers intuitive drag-and-drop templates and straightforward list management. It provides a generous free tier for new businesses. Open rate analytics are easy to understand.
**Design Doc**:
- **Trigger**: A new customer is added to OHC.
- **Action**: OHC automatically pushes the customer's email and name to a designated Mailchimp audience list.
- **User Experience**: The business owner sees a "Sync to Mailchimp" toggle. When active, their OHC customer list is always up-to-date in Mailchimp, ready for newsletters.
**Implementation Prompt**: Add a Mailchimp integration setting where the user can log in and select an audience list. Implement a one-way sync that automatically adds new OHC contacts to the selected Mailchimp list. Show a basic summary of the synced list size in the OHC dashboard.
**Priority**: P2
**Estimated Scope**: Medium
**Environment**: Works in both Cloud and Standalone modes.
