issue_title: "Unified Inventory Sync & POS Capability"
issue_description: |
  # Research Report: Unified Multi-Channel Inventory Sync & Local POS Capability

  ## Problem Statement
  Small business owners with hybrid operations (e.g., Priya running a boutique shop both online and in-store) suffer from split inventory states. Current platforms force them to manually reconcile online inventory with in-store Point-of-Sale (POS) purchases, leading to double-bookings or out-of-stock scenarios. A non-technical owner expects a sale made on their tap-to-pay terminal to instantly deduct the item from their online storefront without manual intervention.

  ## Research Report
  - **Market Gap:** Shopify and Wix require specific tier plans and often third-party bridge tools to keep physical POS inventory strictly synchronized with online catalogs. Square has strong POS but weaker unified AI orchestration.
  - **The OHC Opportunity:** By leveraging the existing `tenant_id` isolated PostgreSQL setup and combining it with Redis distributed locks (Redlock), OHC can provide strong consistency for fast-moving inventory. This prevents double-booking while an in-store customer is checking out. The Operations Agent ("The Manager") can automatically coordinate the reconciliation and trigger stock-replenishment flows.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant POS as Mobile POS (In-store)
      participant Redis as Redis (Redlock)
      participant PG as PostgreSQL (Ledger)
      participant Online as Online Storefront
      participant Agent as Operations Agent

      POS->>Redis: Lock Product_ID for 30s (Checkout Start)
      Redis-->>POS: Lock Granted
      Online->>Redis: Attempt Add to Cart (Product_ID)
      Redis-->>Online: Lock Denied (Item reserved)
      POS->>PG: Finalize Sale & Deduct Inventory
      PG-->>POS: Sale Confirmed
      POS->>Redis: Release Lock
      Agent->>PG: Detect Low Stock (Event Trigger)
      Agent-->>POS: Push Notification ("Red Dress sold out. Restock?")
  ```

  ### Mobile UX Flow (375px First)
  - **POS Interface:** A clean grid of catalog items with large tap targets (>= 44x44px).
  - **Cart Action:** Tapping an item immediately displays a visual "Reserved" indicator to prevent duplicate offline taps.
  - **Online Sync Alert:** If an online checkout attempts to claim the last item while an in-store lock is active, the online customer sees an optimistic "Item temporarily reserved, please try again in a moment" toast.
  - **Agent Intervention:** Following a finalized transaction that hits a zero-balance, a native notification card appears on the owner's dashboard prompting an AI-drafted restock order.

  ### AI Agent Integration Points
  - **The Manager (Operations Agent):** Subscribes to inventory deduction events. If an item reaches `stock <= threshold`, the agent drafts a purchase order or supplier message and surfaces an "Approve Restock" task in the owner's triage feed.

  ### Key Design Decisions
  - **Redis Redlock:** Chosen for its speed to handle temporary inventory reservations during checkout without writing persistent row locks to PostgreSQL for transient states.
  - **PostgreSQL Ledger:** Remains the ultimate source of truth, enforcing multi-tenant boundaries (`tenant_id` RLS).
  - **Optimistic Concurrency:** Web clients will use optimistic UI updates, reverting safely if the lock acquisition fails.

  ## Implementation Prompt
  **User-Facing Outcome:** Priya the boutique owner can tap a product on her mobile POS, process the payment, and have the online stock update instantly. An online shopper attempting to buy the exact same final item during Priya's physical checkout will be gracefully prevented from over-purchasing.
  **CUJ & Acceptance Criteria:**
  1. Implement a Redis-backed locking service for inventory items (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  2. Integrate the lock acquisition into the backend checkout flow (both POS and Web).
  3. Ensure PostgreSQL transactional integrity when finalizing the inventory deduction.
  4. Write Playwright E2E tests simulating a race condition: a POS client and a Web client attempting to purchase the last available item simultaneously. Ensure only one succeeds and the other receives the correct UI denial state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
