# Title: Targeted Email Campaigns via ActiveCampaign
## Problem Statement
Business owners need to send newsletters and promotional emails to their customers, but managing separate customer lists in an external tool is tedious and leads to outdated contact information.

## Research Report
**Tool Evaluated:** ActiveCampaign
- **Ease of Use:** Moderate. Powerful but has a slight learning curve.
- **Pricing:** Starts at $29/month.
- **Reputation:** Top-tier for automation and list management.
- **Advantages:** Exceptional template quality, detailed open/click analytics.
- **Risks:** Might be overly complex for a brand-new, non-technical user.
- **Environment:** Supported in Cloud mode; Standalone can sync via API.

## Design Doc
OHC will connect to ActiveCampaign to sync the customer list in real-time. The OHC dashboard will feature a simplified email campaign trigger, allowing owners to send pre-built templates to their audience. Campaign analytics (opens, clicks) will be pulled back into OHC.

## Implementation Prompt
Create an integration with ActiveCampaign that automatically syncs the OHC customer list. Provide a simple interface for the business owner to view campaign performance metrics (like open rates) directly within OHC.

## Priority
P2

## Estimated Scope
Large
