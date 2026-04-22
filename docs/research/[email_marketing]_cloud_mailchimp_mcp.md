# Mailchimp Integration
## Problem Statement
Small business owners, such as Priya the boutique owner or Carlos the handyman, need to easily build and manage email campaigns to reach their customers, offer discounts, or notify them of new stock/services, without needing technical email marketing skills.

## Research Report
**Tool**: Mailchimp
**Ease of use**: Very High. Provides a drag-and-drop builder, pre-built templates, and AI-powered content creation tools ideal for non-technical users.
**Pricing**: Includes a free tier (up to 500 contacts, 1,000 sends/month). Paid plans start around $13-$20/month depending on features.
**Reputation**: An industry leader in email marketing for small to medium businesses. Reliable delivery rates and a massive integration ecosystem.

## Design Doc
**Cloud Mode**: Integrate via Mailchimp Marketing API. OHC synchronizes customer data (email lists, purchase history, tags) with Mailchimp audiences.
**Standalone Mode**: While Mailchimp is a cloud service, local OHC instances can sync customer lists via the API when an internet connection is available.
**Triggers**: Customer makes a purchase, signs up for a newsletter on the OHC storefront, or a new business user creates an account.
**User Experience**: Business owner designs and schedules email campaigns from within the OHC dashboard or by jumping into the Mailchimp interface via SSO. Customer data is automatically kept in sync between OHC and Mailchimp.

## Implementation Prompt
Integrate Mailchimp into the OHC platform for robust email marketing capabilities.
**Acceptance Criteria**:
1. Business owners can connect their Mailchimp account via OAuth.
2. OHC automatically syncs customer lists, including basic tags (e.g., "Purchased Item X", "VIP"), to Mailchimp Audiences.
3. Users can trigger basic automated campaigns (e.g., Welcome Email, Abandoned Cart) from the OHC dashboard.
4. Provide a dashboard widget displaying basic email campaign performance (open rates, clicks) pulled from Mailchimp.

## Priority
P1

## Estimated Scope
Medium
