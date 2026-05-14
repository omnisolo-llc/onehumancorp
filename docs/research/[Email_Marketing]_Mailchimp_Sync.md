## 3. Email Marketing
**Title**: Integrate Mailchimp for Customer Newsletters
**Problem Statement**: Local shops capture customer emails during checkout but have no easy way to send promotions or newsletters without exporting/importing CSVs to another tool, which they rarely have time to do.
**Research Report**:
- **Tool**: Mailchimp Marketing API
- **Problem it solves for which persona**: Allows retail shops and service businesses to send bulk promotional emails easily.
- **Ease of Use**: Well-known brand. Owner links their account. OHC automatically syncs new customer emails.
- **Pricing**: Free tier up to 500 contacts / 1000 sends per month. Then starts at $13/mo.
- **Key Advantages**: Industry standard, excellent template builder, reliable deliverability.
- **Integration Risks**: Strict spam compliance rules; API rate limits on free tiers.
- **Environment**: Cloud and Standalone supported (API key based).
**Design Doc**:
- **Trigger**: New customer added to OHC CRM.
- **Action**: OHC automatically adds the customer to a designated Mailchimp audience (list).
- **User Interface**: Owner sees a toggle in settings: "Sync new customers to Mailchimp".
**Implementation Prompt**: Build a background synchronization worker that pushes new customer contacts from the OHC CRM to a designated Mailchimp Audience using their API. Support OAuth or API key connection.
**Priority**: P2
**Estimated Scope**: Small
