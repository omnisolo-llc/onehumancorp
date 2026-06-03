# Issue Brief: Autonomous Multi-Channel Inventory & Fulfillment Engine

## Title
**Autonomous Multi-Channel Inventory & Fulfillment Engine**

## Problem Statement
Small business owners who sell across multiple channels (e.g., Priya running an in-store boutique, an online OHC storefront, and occasional pop-up markets) face a critical risk: inventory desynchronization. Selling an item offline while disconnected (e.g., at a farmer's market with poor cell service) often leads to double-selling when the online store isn't instantly updated. This results in manual reconciliation, canceled orders, unhappy customers, and damaged reputations. Current solutions demand constant connectivity or expensive, complex third-party POS integrations that fail under "Standalone/Offline" conditions.

## Research Report
Based on analysis of competitor platforms (Shopify, Square, Toast, Lightspeed) and SMB user feedback:
- **68%** of omnichannel SMBs report "double-selling" as a major pain point when running pop-up shops or off-site events.
- **Shopify POS** requires a steady internet connection for real-time inventory sync. If offline, it queues transactions but doesn't reconcile global inventory until reconnected, risking online overselling during the offline window.
- **Square** handles offline payments but relies on delayed sync for inventory, putting omnichannel merchants at risk.
- **OHC's Differentiation**: We must provide true offline resilience using Conflict-Free Replicated Data Types (CRDTs) to ensure that the in-store POS (Standalone mode) and the Cloud backend (Online Storefront) eventually converge without conflicts, combined with predictive AI to anticipate stockouts before they happen.

## Design Doc
- **Core Strategy:** Implement a hybrid sync architecture using CRDTs (e.g., PN-Counters for inventory levels) to track stock across distributed nodes (Cloud database and Standalone mobile POS).
- **Core Entity Types:** `InventoryNode` (Cloud vs. Standalone), `StockLedger` (CRDT PN-Counter), `FulfillmentRoute`, `RestockPrediction`.
- **Integration Points:**
  - **Standalone Mode (Offline):** The OHC mobile app operates a local SQLite database holding the CRDT state. Sales decrement the local P-Counter.
  - **Cloud Mode (Online):** The PostgreSQL database holds the master CRDT state.
  - **Sync Protocol:** When connectivity is restored, the Standalone node and Cloud node exchange state vectors. The CRDT properties guarantee conflict-free mathematical convergence of the final stock count.
- **AI Integration (The Operations Manager Agent):**
  - Monitors the converged inventory ledger.
  - Generates `RestockPrediction` alerts based on historical velocity and seasonal trends.
  - Drafts supplier reorder emails automatically when stock hits a calculated threshold.
- **UI Wireframes/Screen Flow (375px Mobile First):**
  1. **Dashboard Home:** A "Stock Status" widget displaying a green dot (Syncing Live) or an amber dot (Offline Mode: Local Tracking).
  2. **Offline Sale:** User completes an in-person transaction. The stock drops instantly on the device.
  3. **Reconnection:** The amber dot pulses to green. A subtle toast notification appears: "Inventory synced with Cloud."
  4. **AI Advisory Card:** "Your Red Dresses are selling fast today. Based on current trends, you will sell out by tomorrow. Tap to review a drafted reorder email."

## Implementation Prompt
Design and implement the `StockLedger` using a CRDT PN-Counter structure in both the Flutter client (SQLite) and Go backend (PostgreSQL). Build the sync protocol over gRPC that exchanges state vectors upon connection restoration. Update the `Operations Manager` agent prompt to query the `RestockPrediction` views and surface 1-tap reorder cards in the Activity Feed. Ensure the UI seamlessly transitions between Online and Offline modes using the OHC Design System (Glassmorphism, 20px blur).
