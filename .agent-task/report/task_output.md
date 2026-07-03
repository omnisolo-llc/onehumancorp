issue_title: "[Platform] Distributed POS & Multi-Channel Inventory Locking with Redis Redlock"
issue_description: |
  # Research Report: Distributed Multi-Channel Inventory & POS Locking Architecture

  ## Problem Statement
  Small business owners (e.g., Priya the Boutique Operator) face a significant challenge when operating both an online storefront and an in-person Point-of-Sale (POS). Currently, OHC lacks a robust, real-time mechanism to prevent double-booking. When an in-store customer attempts to purchase the last available item via Stripe Terminal, an online customer can concurrently add the same item to their cart and checkout. This disjointed inventory management leads to overselling, customer dissatisfaction, and manual reconciliation work for the owner—violating the OHC core promise of radical simplicity.

  ## Research Report
  - **Competitive Analysis:** Shopify and Square offer inventory sync but often rely on expensive tier plans or external apps for true real-time locking. Their POS apps are separate silos from the online checkout until the transaction is fully finalized, leaving a window for race conditions.
  - **Market Demand:** OHC SMB Personas (Priya) require immediate, optimistic inventory locking across all channels (mobile POS and web storefront).
  - **Technical Gap:** The current PostgreSQL schema lacks a robust mechanism for distributed, high-concurrency reservation of inventory across edge locations before database commits.
  - **Reference Contexts:** `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md` outlines the need for a Central Ledger combined with a Redis-backed distributed locking strategy for eventual consistency and immediate reservation.

  ## Design Doc
  **Architecture Overview:**
  We must introduce a Redis Redlock-based distributed lock layer in `src/server/msgbus.rs` (or equivalent domain/interop layer) to handle temporary inventory reservations during checkout and POS tap-to-pay initiation.

  **Mermaid Diagram:**
  ```mermaid
  sequenceDiagram
      actor POS User (Priya)
      actor Online User (Customer)
      participant Redis (Redlock)
      participant PostgreSQL (Ledger)
      participant Operations Agent

      POS User->>Redis: Request Lock (15s) `ohc:lock:{tenant}:inventory:{product}`
      Redis-->>POS User: Lock Granted
      Online User->>Redis: Request Lock `ohc:lock:{tenant}:inventory:{product}`
      Redis-->>Online User: Lock Denied
      Online User->>Operations Agent: Trigger "Out of Stock" Alert
      Operations Agent-->>Online User: Notify Cart Update
      POS User->>PostgreSQL: Finalize Transaction & Deduct Inventory
      POS User->>Redis: Release Lock
  ```

  **Mobile UX Flow (375px):**
  1. Priya taps "Checkout" on her OHC mobile app for the "Red Dress".
  2. UI instantly shows a translucent "Securing Item..." state (Optimistic UI).
  3. If locked, the Stripe Terminal modal appears.
  4. Concurrently, if a user on their phone views the storefront, the "Red Dress" instantly grays out or is removed from the cart with an Agent message: "An in-store customer just grabbed the last one!"

  **AI Agent Integration Points:**
  - **Operations Agent:** Monitors Redis lock denials. When an online customer is blocked by an in-store lock, the Operations Agent drafts an alert to the user ("Item in cart became unavailable") and notifies the owner if stock hits zero.

  ## Implementation Prompt
  **Target Persona:** Priya (Boutique Operator)

  **CUJ & Acceptance Criteria:**
  1. Implement a distributed locking mechanism using Redis (Redlock pattern) for inventory reservations.
  2. The locking mechanism must integrate with the existing checkout and POS API endpoints.
  3. A temporary lock (e.g., 5-15 seconds for POS, longer for online carts) must prevent concurrent checkout of the same inventory item.
  4. The Operations Agent must receive a message bus event when a lock conflict occurs to notify affected online users.
  5. The POS UI (Flutter/Tauri) must gracefully handle lock acquisition failures (e.g., if the item was just sold online).
  6. E2E tests MUST simulate concurrent checkouts and prove the lock prevents double-billing/overselling.

  **Key Design Decisions:**
  - Do not alter the base PostgreSQL schema structure for this task; focus on the fast, ephemeral Redis reservation layer to protect the slower database commit.
  - The lock key must strictly follow the multi-tenant isolation pattern: `ohc:lock:{tenant_id}:inventory:{resource_id}`.

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
