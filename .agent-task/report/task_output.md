issue_title: "Implement Multi-Channel Inventory Sync & POS Capabilities"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations. The core issue is that current e-commerce platforms often lead to disjointed online and offline inventory synchronization, resulting in double-booking or out-of-stock scenarios. Without real-time, strongly consistent inventory locks and distributed sync mechanisms, managing multi-channel retail is extremely challenging.

  ## Research Findings & Competitive Analysis
  Our discovery mapping highlights the following:
  - **Shopify:** Robust POS and online features, but small to micro-SMEs find the disjointed inventory unmanageable without costly third-party add-ons.
  - **Square & Stripe Terminal:** Excellent for POS hardware, but they lack native, integrated agentic workflow automation that easily merges with the online catalog without deep configuration.

  **The OHC Edge:** OHC can implement a real-time, AI-augmented inventory locking system using distributed mechanisms (like Redis Redlock) to temporarily reserve stock during checkout alongside an offline-first POS synchronization strategy to reconcile sales seamlessly.

  ## Design Doc
  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** Uses row-level locks to maintain consistent source-of-truth counts.
  - **Distributed Locks:** Redis Redlock ensures temporary inventory reservation during checkout processes to avoid double sales.
  - **Offline/Local First POS Client:** Ensures continuous operations where the local mobile cache allows POS checkouts during network drops, queuing the transactions for eventual consistency sync.

  ### AI Agent Coordination
  - **Operations Agent (The Manager):** Reconciles inventory sync conflicts and sends low-stock alerts.
  - **Finance Agent (The Accountant):** Aggregates and correlates multi-channel transaction data for financial reporting.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[In-Store POS / Web Checkout] -->|Initiate Checkout| B(Redis Redlock: Reserve Item)
      B --> C{Lock Acquired?}
      C -- Yes --> D[Process Payment via Stripe Terminal/Checkout]
      D --> E[Finalize Sale & Release Lock]
      E --> F[Update PostgreSQL Ledger & Invalidate Caches]
      C -- No --> G[Trigger Operations Agent: Notify user of conflict]
      G --> H[Customer Success Agent: Prompt out of stock message]
  ```

  ### Mobile UX Flow
  - Provide a touch-optimized POS interface within the 375px bounds.
  - Interactive inventory adjustments with immediate optimistic UI feedback and quick rollback if synchronization fails.

  ## Implementation Prompt
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - **Step 2:** Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - **Step 3:** Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
