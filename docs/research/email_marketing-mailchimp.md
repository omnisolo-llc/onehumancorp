# Send Newsletters to Customers via Mailchimp

**Problem Statement**
I have a list of past customers, but I don't have an easy way to send them updates about new products or sales. I need a simple tool to design and send professional emails to my customer list without needing design skills.

**Research Report**
Mailchimp is the industry standard for small business email marketing. It offers an intuitive drag-and-drop template builder, making it very accessible for non-technical owners. It tracks open rates and click rates natively. Pricing includes a free tier for up to 500 contacts, and paid plans start at $13/month. It handles spam compliance automatically. The integration relies on standard OAuth and APIs, fully supporting both Cloud and Standalone deployments.

**Design Doc**
Within the OHC customer list view, the business owner will see an option to 'Sync with Mailchimp'. When activated, OHC will keep the Mailchimp audience in sync with the OHC customer database. Users can see basic campaign statistics (like open rates) directly within OHC's analytics dashboard.

**Implementation Prompt**
Build a Mailchimp integration that syncs OHC contacts to a Mailchimp audience list. Provide a simple toggle to enable or disable the sync. On the dashboard, display the performance of the latest Mailchimp campaign. Acceptance criteria: Contacts added in OHC must appear in Mailchimp, and campaign stats must be successfully retrieved and displayed.

**Priority:** P2
**Estimated Scope:** Medium
