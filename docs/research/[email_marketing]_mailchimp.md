# Mailchimp Integration for OHC

## Problem Statement
Small business owners like Priya (Boutique Owner) want to send automated emails when new stock arrives or run promotional campaigns. While OHC has basic built-in tools, many businesses already have extensive mailing lists and campaigns set up in Mailchimp. Forcing them to migrate immediately or managing two separate customer lists creates friction and inconsistency.

## Research Report
- **Features & API Suitability**: Mailchimp provides a robust REST API (v3.0) for managing audiences (lists), campaigns, and reporting. It supports syncing contacts, adding tags, and triggering automations.
- **Pricing**: Freemium model. Free tier covers up to 500 contacts and 1,000 sends/month. Paid tiers scale with contact volume.
- **Ease of Use for Non-Technical Users**: High. OAuth connection to Mailchimp allows seamless contact syncing.
- **Cloud vs. Standalone**: Works well in both. API key or OAuth works for Cloud; API key can be configured locally for Standalone.
- **Advantages**: Industry standard, huge feature set, highly recognizable brand.
- **Risks**: Strict compliance rules (anti-spam, opt-out).

## Design Doc
- **Integration Point**: "The Promoter" (Marketing & Advertising) and "The Ambassador" (Customer Success).
- **Trigger**: User connects Mailchimp account via OAuth or API Key.
- **Action**: One-way or two-way sync of customer contacts. OHC tags customers based on purchase history (e.g., "purchased_vegan_cake"). "The Promoter" agent can suggest triggering a Mailchimp campaign based on these tags.
- **User View**: A sync status indicator on the Customer list. Ability to trigger an email blast directly from the OHC dashboard using Mailchimp as the delivery engine.

## Implementation Prompt
Implement a contact synchronization integration with Mailchimp. The feature must allow users to connect their Mailchimp account. Whenever a new customer makes a purchase or signs up via the OHC storefront, their contact details (email, name) must be automatically added to the designated Mailchimp audience. Furthermore, apply relevant tags (e.g., "customer", "newsletter") based on their interaction.

## Priority
P2

## Estimated Scope
Small
