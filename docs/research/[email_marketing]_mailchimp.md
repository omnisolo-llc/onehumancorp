# Sync Customer Lists to Mailchimp for Marketing

## Problem Statement
Small business owners collect customer emails through their storefront, sales, and inbox interactions, but they struggle to leverage this data for marketing. They want to send newsletters or promotional blasts, but manually exporting CSVs from OHC and importing them into an email marketing tool is tedious and leads to outdated lists.

## Research Report
**Tool Evaluated:** Mailchimp Marketing API
- **Ease of Use:** Mailchimp is highly recognizable and user-friendly for non-technical users creating email campaigns. The integration just requires logging in.
- **Pricing:** Free tier up to 500 contacts and 1,000 sends/month. Paid plans start around $13/month. Very accessible for SMBs.
- **Reputation:** Long-standing industry leader. Excellent deliverability, robust template builder, and strong compliance tools (CAN-SPAM/GDPR).
- **Deployment:** Cloud mode is fully supported. Standalone mode works well since it relies on outbound API calls to Mailchimp's cloud.

## Design Doc
- **Trigger:** User connects Mailchimp via OAuth. They map an OHC customer segment (e.g., "All Customers" or "Recent Buyers") to a Mailchimp Audience.
- **Action:** A background sync process ensures that whenever a customer is added or updated in OHC, their details (email, name, tags) are pushed to the corresponding Mailchimp Audience. Opt-outs in Mailchimp are synced back to OHC.
- **User View:** A "Marketing Sync" settings page. The user just sees a toggle saying "Keep my Mailchimp list updated with my OHC customers."

## Implementation Prompt
Build a two-way synchronization between OHC's customer CRM and Mailchimp. Users should be able to authenticate their Mailchimp account and select which Audience to sync. Ensure that new customers in OHC are automatically added to Mailchimp, and that unsubscribe events in Mailchimp update the customer's marketing consent status in OHC.

## Priority
P1

## Estimated Scope
Medium