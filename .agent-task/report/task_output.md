issue_title: "Implement Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement
  Priya, a boutique owner, requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Research Report
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web Checkout] -->|Inventory Reservation Request| B(Redis Redlock)
      C[POS Tap-to-Pay] -->|Inventory Reservation Request| B
      B -->|Lock Acquired| D[Central Ledger (PostgreSQL)]
      D --> E[Operations Agent (The Manager)]
      E -->|Low Stock Alert / Conflict Resolution| F[Agent Feed]
  ```

  ### Mobile UX Flow
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.

  ## Implementation Prompt
  **User-Facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **CUJ & Acceptance Criteria:**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - **Step 2:** Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - **Step 3:** Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
