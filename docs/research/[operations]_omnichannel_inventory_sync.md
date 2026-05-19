# [operations] Omnichannel Inventory Sync Agent ("The Merchant")

## Problem Statement
Small business owners like Priya (boutique owner) often sell across multiple channels: in-person, Instagram DMs, and an online storefront. Currently, keeping inventory in sync is a manual nightmare. If she sells her last vintage vase in-store, she often forgets to update her online stock, leading to embarrassing "out of stock" emails and lost trust. She needs an "invisible teammate" that keeps her stock levels accurate everywhere, automatically.

## Research Report
- **Market Context**: Shopify and Wix offer inventory management, but they often require expensive third-party apps (e.g., Stocky, Trunk) to sync effectively across non-integrated channels like Instagram DMs.
- **Competitor Gap**: Durable and 10Web have basic inventory but lack the proactive "agentic" layer that watches unstructured data (like DMs) to suggest inventory updates.
- **User Evidence**:
    - *Reddit (r/ecommerce)*: "I spend 2 hours every night just updating my Etsy and Shopify counts based on what sold at the craft fair today."
    - *Trustpilot (Shopify Review)*: "Oversold a product because the Instagram sync lagged. Customer was furious."

## Design Doc
### Architecture
- **Entity Relationship**: `InventoryItem` <-> `SalesChannel` <-> `ExternalEvent` (Webhook/DM).
- **Integration Points**: Meta Graph API (Instagram DMs), Stripe (POS/In-store), OHC Storefront Builder.
- **Agent Integration**: "The Merchant" agent listens to the `msgbus` for "order_created" or "message_received" events. It uses NLP to detect intent like "I want to buy the blue vase" in DMs.

### UI/UX Flow (375px Mobile)
1. **Low Stock Alert**: A glassmorphic card appears in the "Daily Briefing" showing a product with a "Low Stock" badge.
2. **Actionable Recommendation**: The card says: "You sold 3 vases on Instagram today. Should I update your online stock to 2 remaining?"
3. **1-Tap Approval**: A primary blue button [Confirm Sync] updates all connected channels instantly.

## Implementation Prompt
Implement the "The Merchant" agentic workflow. The system must:
1. Aggregate inventory counts from the internal store and connected "Connect Apps" (simulated).
2. Listen for incoming sales events (Stripe, Manual, or DM intent).
3. Automatically calculate projected stock levels.
4. Present a "Conflict Resolution" task in the Activity Feed when counts differ across channels.
5. Provide a "Low Stock" proactive notification with a pre-filled "Reorder" or "Update Count" action.

**Acceptance Criteria**:
- User can see a unified inventory view on mobile.
- Inventory is updated across "Online" and "Instagram" channels after a single tap.
- No technical jargon (e.g., use "Stock Level" not "Inventory SKU count").

## Priority: P0
## Estimated Scope: Medium
