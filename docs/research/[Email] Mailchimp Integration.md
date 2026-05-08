# Title: Mailchimp Integration for Customer Email Campaigns
## Problem Statement
Business owners collect customer emails through OHC when making sales or taking bookings, but have no easy way to send promotional emails (like holiday discounts or newsletters) without manually exporting CSVs and importing them into an email tool.

## Research Report
* **Tool:** Mailchimp Marketing API
* **What it does:** Manages subscriber lists, tags, and campaigns.
* **Ease of Use for Owners:** Mailchimp is an industry standard for non-technical users. The integration just needs a simple OAuth or API key connection.
* **Pricing:** Free for up to 500 contacts, which covers many new micro-businesses.
* **Cloud vs. Standalone:** Works seamlessly in both via standard OAuth or user-provided API key.

## Design Doc
* **Trigger:** Owner connects Mailchimp and toggles "Auto-sync customers".
* **Action:** OHC automatically adds new customers to a designated Mailchimp audience and tags them based on their purchase history (e.g., "Purchased Product X").
* **User Experience:** The owner never has to touch a CSV again. When they log into Mailchimp, their customer list is perfectly up to date, categorized, and ready for a newsletter.

## Implementation Prompt
Create a one-click Mailchimp sync feature. The owner should be able to authenticate with Mailchimp, pick an audience list, and see their existing OHC customer database upload automatically. Any new customer added to OHC must seamlessly appear in Mailchimp with appropriate tags.

## Priority
P2

## Estimated Scope
Medium
