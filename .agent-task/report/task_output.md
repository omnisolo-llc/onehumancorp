issue_title: "OHC Unified Agentic Inventory Sync & POS Mobile Interface"
issue_description: |
  **Mission Queue Protocol Report**

  **Problem Statement:**
  Multi-channel small business owners (e.g., Priya, boutique operator) struggle to maintain synchronized inventory across online and offline (in-store tap-to-pay) sales. Legacy systems require expensive tier upgrades or fragile third-party integrations. Without a robust centralized inventory system integrated seamlessly with Point-of-Sale (POS) and Agentic automation, double-bookings are inevitable and the owner loses trust in the platform.

  **Research Report:**
  Analysis of competitors (Shopify, Wix, Square) reveals that true multi-channel consistency is a premium feature. Shopify requires "Shopify POS Pro" for advanced inventory features. Wix's POS hardware integrations are geographically limited. OHC's unique advantage is the "Operations Agent" which can monitor stock dynamically, manage distributed locks across the PostgreSQL central ledger and Redis cache during active checkouts, and autonomously suggest reorders. The key is treating an in-person POS transaction as a peer to an online cart checkout, unified via the identical agentic backend.
  *Competitor Compare:*
  - Shopify Sidekick: Reactive, tells you to check inventory.
  - Square AI: Good descriptions, but passive on stock levels.
  - OHC Operations Agent: Locks the item during a 15-second POS tap-to-pay window. If an online buyer tries to buy simultaneously, the agent blocks the online sale and offers them a backorder.

  **Design Doc:**
  *Architecture:*
  - `Central Ledger`: PostgreSQL `inventory_items` table with `tenant_id` and strict row-level security.
  - `Reservation System`: Redis Redlock (`ohc:lock:{tenant_id}:inventory:{item_id}`) to manage the checkout window.
  - `POS Client`: Flutter mobile application functioning locally, syncing to PostgreSQL.
  - `Agent Integration`: Operations Agent subscribes to `inventory_depleted` pub/sub events to draft restock orders.

  *Mobile UX Flow (375px):*
  - **POS Mode**: Owner selects "Sell in Person". A clean, large-touch-target grid of products appears.
  - **Cart**: Owner taps a product. The system instantly attempts a Redis Redlock. If successful, it's added to the local POS cart.
  - **Checkout**: Owner taps "Charge $XX". The app interfaces with Stripe Terminal SDK.
  - **Agent Feedback**: If the item was the last one, the Operations Agent pushes a non-intrusive bottom-sheet: "Item sold out. Draft restock order?"

  **Implementation Prompt:**
  *Target Persona:* Priya the Boutique Owner.
  *CUJ:* Priya is ringing up an in-store customer for the last "Blue Summer Dress". When she taps the dress on her 375px mobile POS view, the system applies a short-lived distributed lock. The transaction succeeds via the terminal. Concurrently, the Operations Agent detects the stock is now 0 and triggers a UI notification card to Priya asking to approve an automated restock email to her supplier.
  *Acceptance Criteria:*
  1. The POS interface must render perfectly on a 375px viewport with >=44px touch targets.
  2. Implement the Redis locking mechanism for inventory items.
  3. The Operations Agent must successfully detect a zero-stock event and generate a mock "Restock Approval" notification card.
  4. Full E2E Playwright coverage of the POS cart addition and lock application.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
