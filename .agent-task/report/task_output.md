issue_title: "OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners with multi-channel operations (online + in-store) often face disjointed inventory management. When an item is sold in-store, it is not immediately reflected online, leading to double-booking and out-of-stock scenarios. Current competitors like Shopify offer extensive POS capabilities but are often too complex for micro-SMEs, while Square and Stripe Terminal provide robust POS hardware but lack integrated, agentic workflow automation.

  ## Research Report
  - **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  - **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking occurs during simultaneous online and offline purchases.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Online Customer
      participant Priya as Priya (POS)
      participant OperationsAgent as Operations Agent
      participant Redis as Redis (Redlock)
      participant CentralLedger as Central Ledger (PostgreSQL)

      Priya->>Redis: Request Lock for "Red Dress" (15s)
      Redis-->>Priya: Lock Granted
      Customer->>OperationsAgent: Attempt Checkout for "Red Dress"
      OperationsAgent->>Redis: Check Lock Status
      Redis-->>OperationsAgent: Locked
      OperationsAgent-->>Customer: "Item just sold out"
      Priya->>CentralLedger: Finalize Sale, Deduct "Red Dress"
      CentralLedger-->>Priya: Success
      Redis->>Redis: Release Lock
      OperationsAgent-->>Priya: "Red Dress sold out. Would you like to draft a restock order?"
  ```

  ### Mobile UX Flow
  - The POS interface is designed for a 375px viewport.
  - Touch targets for inventory adjustment and checkout are ≥ 44x44px.
  - Optimistic UI updates are used for inventory changes, with rollback capabilities if the Redis reservation fails.

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ## Implementation Prompt
  Implement the Unified Multi-Channel Inventory Sync & POS system.

  **User-facing Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a reservation to the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the central ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria:**
  - When an in-store order starts, the item must be reserved across all channels.
  - Online checkouts for the same item must gracefully fail or show "out of stock" during the reservation period.
  - Finalizing the in-store sale permanently deducts the item count.
  - The Operations Agent notifies the owner (Priya) about the low stock status.
  - The offline client securely reconciles with the central ledger upon coming back online.

  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []