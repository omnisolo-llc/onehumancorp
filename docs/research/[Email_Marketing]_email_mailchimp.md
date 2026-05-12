**Title**: Email Marketing Integration via Mailchimp

**Problem Statement**:
Small business owners want to engage their customers with newsletters, promotional offers, and automated campaigns, but managing a separate customer list in an external tool is tedious. They need a simple way to sync their OHC customer list with an email marketing platform to easily send professional, spam-compliant emails.

**Research Report**:
Mailchimp is one of the most popular and user-friendly email marketing platforms.
- **Ease of Use for Non-Technical Users**: Excellent. The platform is designed for small businesses. Setting up the integration via OHC would just require an OAuth login. Once connected, users can manage their campaigns in Mailchimp while OHC automatically keeps the audience list updated.
- **Features**: Offers campaign management, audience segmentation, automated email journeys, and robust analytics (open rates, click rates). Crucially, it handles unsubscribe compliance automatically.
- **Reputation & Reliability**: Industry leader in small business email marketing. Highly reliable API.
- **Pricing**: Generous free tier (up to 500 contacts and 1,000 emails per month), making it accessible for very small businesses. Paid tiers start around $13/month.
- **Cloud vs Standalone**: The integration uses standard REST APIs (Mailchimp Marketing API). Works identically in both Cloud and Standalone modes, as OHC pushes data to Mailchimp.

**Design Doc**:
- **Trigger**: User navigates to Settings > Integrations in OHC and clicks "Connect Mailchimp".
- **Action**: An OAuth 2.0 flow is initiated. Upon successful connection, OHC performs an initial sync of the customer list to a designated Mailchimp Audience. Subsequent customer additions or updates in OHC trigger API calls to keep the Mailchimp Audience in sync.
- **User View**: Business owners see a connected status for Mailchimp in OHC. They can view basic campaign stats within OHC, but will primarily use the Mailchimp interface to design and send emails.
- **Architecture**: OHC will act as an OAuth client. Implement a background job queue to handle syncing customer data (additions/updates/deletions) to the Mailchimp Marketing API (specifically the Lists/Audiences endpoints). Webhooks from Mailchimp could be used to sync unsubscribe events back to OHC.

**Implementation Prompt**:
Integrate the Mailchimp Marketing API to allow business owners to sync their customer lists. Implement an OAuth connection flow. Build a background synchronization mechanism to ensure that any new customers added in OHC are automatically pushed to the connected Mailchimp Audience, and any updates to customer details are reflected. Ensure unsubscribe events in Mailchimp are synced back to OHC.

**Priority**: P2 (medium)
**Estimated Scope**: Medium
