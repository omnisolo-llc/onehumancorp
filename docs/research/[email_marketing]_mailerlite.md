# Title: MailerLite Integration for Customer Campaigns

## Problem Statement
Small businesses need to send newsletters or promotional blasts (e.g., "Holiday Sale!") to their customer list. However, managing a customer database in one tool and an email marketing list in another leads to out-of-sync contacts and manual CSV exports. The owner needs a way to automatically sync their OHC customer contacts to an email marketing platform so they can easily send campaigns.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Exporting/importing CSVs is tedious and often forgotten, leading to stale mailing lists.
- **Competitors:** Mailchimp is the giant, but MailerLite is rapidly gaining market share among SMBs due to its much simpler interface, better deliverability, and generous free tier.
- **MailerLite API:** They offer a straightforward REST API for subscriber management (adding/updating/unsubscribing).
- **Reputation & Ease of Use:** MailerLite is famous for its clean, intuitive UI.
- **Pricing:** Free up to 1,000 subscribers, making it ideal for our target demographic.

**Key Advantages:**
- One-way sync solves 90% of the pain point without needing to build a full email editor in OHC.
- Generous free tier is a major selling point for micro-businesses.

**Integration Risks:**
- Handling unsubscribes correctly: if a user unsubscribes in MailerLite, that status must sync back to OHC to prevent future non-compliant messages.

**Environment Support:**
- **Cloud:** Full support.
- **Standalone:** Full support, as OHC pushes data outwards to MailerLite via API.

## Design Doc
**Trigger:**
User connects their MailerLite account via API key or OAuth in the "Marketing" settings.

**Action:**
The user maps OHC customer groups (e.g., "All Customers", "VIPs") to MailerLite groups. Whenever a new customer is added or updated in OHC, a background job pushes the update to MailerLite.

**User View:**
The user sees a toggle on customer profiles: "Sync to MailerLite". They can also view basic campaign stats (open rates) pulled from the MailerLite API directly on the OHC marketing dashboard, so they know how their emails are performing without switching apps.

## Implementation Prompt
Implement a contact synchronization integration with MailerLite.
- Create a connection interface for MailerLite (API Key).
- Build a background sync engine that automatically pushes new or updated OHC contacts to MailerLite.
- Implement a webhook receiver or polling mechanism to update OHC contact status if they unsubscribe via MailerLite.
- Display a summary of recent MailerLite campaigns (sends, opens, clicks) on the OHC Marketing dashboard.
- (Focus on the sync logic and UI presentation; do not mandate specific background job queues.)

## Priority
P1

## Estimated Scope
Medium
