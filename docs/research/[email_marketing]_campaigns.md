**Title**: Integrated Email Marketing with Mailchimp and SendGrid
**Problem Statement**: Small businesses like Priya's boutique need to announce sales or new arrivals to their existing customer base. Currently, they have to manually export customer emails from their store and import them into an external tool, which is tedious and prone to errors. They need a simple way to send beautiful emails directly to their customer list.
**Research Report**:
- **Mailchimp**: A massive platform founded in 2001 (now owned by Intuit). It is synonymous with email marketing for small businesses.
- **SendGrid**: Founded in Denver and acquired by Twilio in 2018 for $2 billion. It is highly reliable for both transactional and marketing emails.
- **Ease of Use**: Mailchimp excels at drag-and-drop template builders for non-technical users. SendGrid is more developer-focused but highly reliable for API delivery.
- **Pricing**: Both offer free tiers. Mailchimp's pricing scales based on the number of contacts.
- **Reputation**: Both are industry standards. Mailchimp is better known by small business owners (the "grandmother test"); SendGrid is known by developers for high deliverability.
- **Cloud/Standalone**: Cloud mode can use webhooks for bounce/open tracking. Standalone mode can easily make API calls to trigger sends without needing a complex local mail server.
**Design Doc**:
- **Trigger**: Priya wants to send a "Summer Sale" announcement.
- **Action**: She selects a customer segment in OHC, picks a template, and clicks "Send." OHC uses the Mailchimp/SendGrid API to dispatch the emails.
- **UI**: A "Campaigns" tab in OHC. Users can see a list of past campaigns, open rates, and click rates. A "New Campaign" button opens a simplified composer where they can type a message or select a basic template, then select an audience (e.g., "All Customers" or "Recent Buyers").
**Implementation Prompt**: Create an Email Campaigns module. Allow the user to connect their Mailchimp or SendGrid account via API key or OAuth. Sync the OHC customer list with the provider's audience list automatically so the user never has to manually export/import CSVs. Build a simplified UI to draft an email and trigger a campaign send via the provider's API.
**Priority**: P2
**Estimated Scope**: Medium
