issue_title: "Implement Multi-Channel POS Sync and Row-Level Inventory Reservation Architecture"
issue_description: |
  # Architecture Design: Centralized Inventory & Distributed POS Synchronization

  ## Problem Statement
  OneHumanCorp (OHC) owners (like Priya the Boutique Operator) face critical synchronization failures when operating across online and in-store channels simultaneously. The current platform lacks a strongly consistent, real-time inventory locking mechanism and distributed Point-of-Sale (POS) synchronization architecture. Without it, double-booking and out-of-stock anomalies occur during concurrent physical tap-to-pay transactions and online purchases. Small business operators require an invisible, robust backend that gracefully manages these race conditions without requiring technical oversight.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Square:** These platforms manage multi-channel inventory using centralized ledgers, but they often struggle with real-time, sub-second reservation locking at the micro-merchant level without expensive, complex, higher-tier POS software. Their resolution logic is often reactive (canceling orders post-purchase) rather than proactive.
  - **Industry Standard (Stripe Terminal):** Employs robust client-side caching for offline resilience but relies on server-side idempotency and eventual consistency for inventory reconciliation.
  - **OHC Opportunity:** By building an integrated Redis Redlock-based inventory reservation system combined with a PostgreSQL central ledger relying on `SELECT ... FOR UPDATE` and Row-Level Security (RLS), OHC can deliver enterprise-grade inventory consistency invisibly. The AI Operations Agent ("The Manager") will autonomously oversee these locks, handle restock notifications, and communicate availability status to the Customer Success Agent ("The Ambassador").

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Online Storefront] -->|Checkout Attempt| C{Redis Redlock Reservation}
      B[Mobile POS - In Store] -->|Tap-to-Pay Transaction| C
      C -- Success --> D[(PostgreSQL Central Ledger)]
      C -- Failure / Conflict --> E[Operations Agent: The Manager]
      D --> F[Finalize Sale & Deduct Stock]
      E --> G[Trigger 'Item Sold Out' UX / Draft Restock Order]
      E --> H[Customer Success Agent: Notify Online User]
  ```

  ### Data Model & System Integrity
  1. **PostgreSQL Ledger:**
     - The ultimate source of truth. All tables must enforce multi-tenant isolation via `tenant_id` and Row-Level Security (RLS).
     - Tables required: `inventory_items` (with strict constraints ensuring `quantity >= 0`) and `transaction_logs`.
  2. **Distributed Locks (Redis):**
     - Implementation of a temporary inventory reservation mechanism (e.g., locking an item for 15 seconds during a fast in-store tap-to-pay, or 5 minutes for an online cart).
     - Lock Key Pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3. **Offline-First POS Client Synchronization:**
     - Mobile clients cache the catalog locally and queue finalized offline sales.
     - Upon network restoration, an asynchronous sync job resolves eventual consistency against the central ledger, with the Operations Agent managing any arising conflicts.

  ### Mobile UX & AI Integration
  - **Mobile Parity (375px):** The POS and inventory management interfaces must be flawlessly responsive on 375px viewports with touch targets ≥ 44x44px. Optimistic UI updates will visually confirm stock changes instantly, with graceful rollbacks if Redis reservations fail.
  - **AI Operations Agent:** "The Manager" constantly monitors the transaction stream. If an item sells out in-store while an online user is checking out, the Agent intercepts the failure, triggers a friendly "Item just sold out" notification to the online user, and pushes an actionable card to the owner's mobile feed suggesting a restock order.

  ## Implementation Prompt
  **User-Facing Outcome:**
  Priya is seamlessly processing an in-store sale for her last "Red Dress" on her mobile POS. Simultaneously, an online shopper tries to buy it. The online shopper gracefully receives an "Item just sold out" message, and Priya’s in-store transaction succeeds immediately. Priya later receives a push notification from the Operations Agent suggesting a restock.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Implement the Redis Redlock inventory reservation service and integrate it into the shared checkout/POS data access layer.
  2. Enforce strict PostgreSQL RLS and `SELECT ... FOR UPDATE` row-level locks on the `inventory_items` table.
  3. Create an E2E Playwright test simulating concurrent checkout attempts (one POS, one Online) for a product with a stock of 1. Verify that only one succeeds and the other receives a polite rejection.
  4. Implement the Operations Agent logic to trigger a push notification to the owner's feed when inventory hits zero.
  5. Ensure all data schemas explicitly prevent cross-tenant leakage.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
