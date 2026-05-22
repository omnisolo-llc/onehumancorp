# Issue Brief: Autonomous Local Presence & Directory Sync

## Title
[Growth] Autonomous Local Presence & Directory Sync

## Problem Statement
The "Inconsistent Info" Trap: Small business owners like Carlos (Handyman) or Fatima (Food Cart) often have outdated hours or phone numbers on Yelp, Bing, or Apple Maps while their Google profile is correct. 48% of SMBs have at least one significant error in their online listings. Manually updating 10+ directories is a low-value, high-frustration task that technical founders ignore until they lose a customer who showed up when they were closed. Users need an agent that treats their OHC profile as the "Source of Truth" and autonomously synchronizes it across the entire local search ecosystem.

## Research Report
- **Competitor Landscape**:
  - **Durable AI**: Offers basic directory listing recommendations on their higher tiers, but still requires significant manual intervention for some platforms.
  - **Yext / Semrush Local**: Powerful but extremely expensive ($500+/year), targeting mid-market rather than solopreneurs. They are seen as "enterprise bloat" by micro-merchants.
  - **Wix/Shopify**: Rely on third-party app store integrations which add to "Cost Creep."
- **User Evidence**: Reddit threads in r/smallbusiness often complain about "extortive" pricing from directory sync services and the "nightmare" of keeping Apple Maps updated.
- **Key Metric**: 76% of people who search on their smartphone for something nearby visit a business within a day. Inaccurate data is the #1 reason for lost local intent sales.

## Design Doc
### High-Level Architecture
- **Source of Truth**: The `Organization` and `Location` entities in the OHC backend.
- **Agent Action**: "The Publicist" Agent.
  - Monitors the "Business Profile" section for any changes (Hours, Phone, Address, Photos).
  - Uses a Teammate Mesh integration with Local Discovery APIs (Google Business, Yelp, Bing, Apple Maps).
  - Performs a weekly "Audit Scan" to detect if external listings have been modified by third parties (e.g., "suggested edits" by users).
  - Autonomously submits corrections to external platforms.
- **Sync Status**: A simplified "Local Health" widget on the dashboard.

### Mobile UX Flow (375px First)
1. **Setup**: "Connect your Google Business Profile." (OHC handles the rest).
2. **Alert**: Push notification: "I noticed your hours on Yelp don't match your store. I've sent an update to fix it. 🛠️"
3. **Dashboard Card**: "Your business is visible and accurate on 12 local platforms. (Score: 100%)"

## Implementation Prompt
Implement the "Autonomous Local Presence Sync" engine. Create a background worker that monitors changes to the tenant's business profile and triggers "The Publicist" agent to synchronize these details with Google Business Profile and Yelp APIs. Build a "Local Health" dashboard card for the Tauri mobile UI that displays the sync status of major directories using simple green/red indicators. Ensure all API interactions are handled via the Teammate Mesh with proper tenant isolation.

## Priority
P2

## Estimated Scope
Medium
