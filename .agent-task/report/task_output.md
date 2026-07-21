issue_title: "Implement Multi-Channel Inventory Real-time Synchronization and POS Support"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report investigates the current landscape of small business inventory management, specifically addressing the pain points of multi-channel (online + in-store) merchants. The objective is to design a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless, real-time experience for non-technical users.

  ## 1. Problem Statement
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## 2. Research Report
  Based on the current SMB ecosystem, the most critical pain points involve the manual reconciliation of cross-channel sales. Wix and Squarespace require third-party tools to effectively marry physical in-store tap-to-pay POS actions with online website inventories. For Priya, if she sells the last Red Dress in her store, she does not want to run to her iPad to update the website so an online buyer doesn't simultaneously buy it. OHC must fix this gap.

  ## 3. Design Doc
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      actor Priya (POS)
      participant Mobile POS
      participant Central Ledger (Postgres)
      participant Distributed Lock (Redis)
      participant Web Storefront
      participant Operations Agent

      Priya (POS)->>Mobile POS: Initiates Checkout for "Red Dress"
      Mobile POS->>Distributed Lock (Redis): Request Lock for "Red Dress" (15s)
      Distributed Lock (Redis)-->>Mobile POS: Lock Granted
      Customer->>Web Storefront: Clicks "Add to Cart" for "Red Dress"
      Web Storefront->>Distributed Lock (Redis): Request Lock for "Red Dress"
      Distributed Lock (Redis)-->>Web Storefront: Lock Denied
      Web Storefront-->>Customer: Shows "Item just sold out"
      Mobile POS->>Central Ledger (Postgres): Finalize Sale & Update Inventory
      Central Ledger (Postgres)-->>Mobile POS: Confirmed
      Central Ledger (Postgres)->>Operations Agent: Trigger low stock event
      Operations Agent->>Priya (POS): Push Notification: "Red Dress sold out. Draft restock order?"
  ```

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile UX Flow
  - **POS UI:** A 375px optimized layout for fast in-person checkout with touch targets >= 44x44px.
  - **Optimistic Updates:** Immediate UI reflection of inventory decrement upon tap-to-pay, with a fallback rollback if Redis locking fails.

  ## 4. Implementation Prompt
  **Title:** OHC Unified Multi-Channel Inventory Sync & POS
  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria for Implementer:**
  - Implement Redis locking for inventory checkout.
  - Ensure POS layout matches 375px responsive design.
  - Integrate AI Operations Agent low-stock notification.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
