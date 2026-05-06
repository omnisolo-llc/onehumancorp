## [Email Marketing] Issue Brief

**Title**: Scout 🔍: Integrate Mailchimp for Customer Engagement
**Problem Statement**:
Business owners want to send newsletters and promotions but struggle to manually export their customer list from OHC and import it into an email tool.
**Research Report**:
- **Tool**: Mailchimp Marketing API
- **Evaluation**: Mailchimp is a popular email marketing tool. Integrating it allows automatic syncing of the OHC customer list, ensuring marketing campaigns always reach the right audience.
- **Ease of Use**: Simple OAuth connection. OHC handles the background syncing.
- **Pricing**: Generous free tier. Paid plans based on the number of contacts.
- **Cloud vs. Standalone**: Works in both modes via OAuth.
**Design Doc**:
- User connects their Mailchimp account.
- OHC automatically syncs new customers to a designated Mailchimp audience.
- The 'Marketing' agent can suggest campaign ideas based on customer data.
**Implementation Prompt**:
Implement Mailchimp API integration. Allow OAuth connection. Set up a background sync to push new OHC customers into a specified Mailchimp audience.
**Priority**: P2
**Estimated Scope**: Medium
