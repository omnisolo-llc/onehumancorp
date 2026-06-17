issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Current e-commerce platforms struggle to seamlessly sync online and offline inventory in real-time. For small business owners like Priya (Boutique Operator), this leads to double-booking when an item is sold in-store via a POS terminal while simultaneously being checked out online.

  ## Market Mapping & Competitor Discovery
  Competitors like Shopify dominate e-commerce but their POS integration often falls out-of-sync without expensive third-party tools. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify operations for micro-SMEs effortlessly.

  ## Proposed Architecture Design
  ### Data Model & Sync Protocol (PostgreSQL & Redis Redlock)
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for inventory counts, utilizing row-level locking or optimistic concurrency.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during checkout to prevent double-booking. Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** Mobile POS client caches catalog data and employs eventual consistency to sync offline sales when network is restored.

  ### Mobile UX Flow & AI Agent Integration (375px)
  - **POS Interface:** Operates flawlessly on a 375px viewport with touch targets ≥ 44x44px. Optimistic UI updates for inventory changes.
  - **Operations Agent ("The Manager"):** Actively monitors stock levels, tracks orders, coordinates sync mechanisms, and triggers low-stock alerts.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data.
  - **Customer Success Agent ("The Ambassador"):** Updates storefront availability and notifies online customers if cart items are bought in-store.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock using Redis Redlock, preventing double-booking online, managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
