issue_title: "Implement Unified Multi-Channel Inventory Sync & POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with both physical and online presence (like Priya the boutique owner) struggle with disjointed inventory systems. When a customer buys an item in-store using a basic card reader, the online storefront often isn't updated instantly. This leads to double-booking the same item online, resulting in customer disappointment and manual refund administration. The business needs a unified, real-time inventory system that locks stock instantly across all channels, managed invisibly by an AI work assistant.

  ## Research Report
  - **Shopify:** Offers strong POS integration but often requires premium tiers or complex third-party apps for robust multi-location/multi-channel real-time locking. Their system is not fundamentally agent-driven.
  - **Wix/Squarespace:** E-commerce capabilities exist but local/in-person POS inventory synchronization is often eventual or manual, leading to high friction for fast-moving physical goods.
  - **Stripe Terminal/Square:** Excellent payment hardware, but lack the integrated AI agent layer to handle the resulting operational workflows (like updating an online storefront and notifying an online cart holder).
  - **OHC Opportunity:** By building an Agentic POS Architecture, OHC can instantly reserve inventory globally upon an in-store tap, preventing double-sells while the "Operations Agent" handles stock-outs and customer notifications seamlessly.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[POS Mobile App 375px] -->|Stripe Terminal Event| B(Omnichannel Gateway)
      C[Online Storefront] -->|Add to Cart| B
      B --> D{Redis Distributed Lock}
      D -->|Lock Granted| E[Central Inventory Ledger PostgreSQL]
      D -->|Lock Denied| F[Operations Agent]
      E --> G[Event Mesh]
      G --> F
      F -->|Notify Customer| H[Customer Success Agent]
      F -->|Update Catalog Cache| I[Storefront Edge Cache]
  ```

  ### Mobile UX Flow (375px)
  1. **POS Dashboard:** Clean UniFi-style cards with macOS Translucent Glass styling. Top card shows current active inventory.
  2. **Checkout Interaction:** Large (≥44x44px) touch targets for "Tap to Pay".
  3. **Visual Feedback:** Upon successful tap, a clear visual indicator shows the item is deducted from global inventory.
  4. **Agent Notification:** A lightweight toast/notification from the Operations Agent confirming "Online stock synced."

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Listens for successful POS transactions via the Event Mesh. Instantly invalidates the relevant Storefront Edge Cache and checks if the item is now out of stock. If out of stock, triggers a restock suggestion to the owner.
  - **Customer Success Agent (The Ambassador):** If an online user has the item in their cart during the POS transaction, this agent intercepts the checkout attempt with a graceful, human-like message: "I'm so sorry, someone just purchased our last one in-store!"

  ## Implementation Prompt
  **User-Facing Outcome:** As Priya, when I sell my last "Red Summer Dress" via tap-to-pay in my physical store, my online storefront immediately shows it as "Sold Out." If an online customer had it in their cart, they are gracefully notified, preventing a double sale, without me touching any software.

  **CUJ & Acceptance Criteria:**
  1. Set up a mock Stripe Terminal POS transaction for an item with `stock = 1`.
  2. Concurrently simulate an online checkout request for the same item.
  3. The system MUST apply a Redis Redlock during the POS transaction.
  4. The online checkout request MUST fail gracefully due to the lock/depleted stock.
  5. The PostgreSQL ledger MUST reflect `stock = 0`.
  6. The Operations Agent MUST generate a system event confirming the stock depletion.
  7. Provide Playwright E2E tests validating the POS checkout flow and the concurrent online cart rejection.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
