issue_title: "Architecture & Gap Analysis: Agent-Driven Hybrid Point-of-Sale (POS) & Centralized Inventory Lock"
issue_description: |
  # Mission Queue Protocol: Architectural Gap Analysis - Agent-Driven POS & Centralized Inventory Lock

  ## Problem Statement
  Current e-commerce platforms (like Shopify or Wix) rely on complex companion apps and manual syncing for Point-of-Sale operations, penalizing non-technical small business owners (like Priya the Boutique Owner) who sell both in-store and online. They frequently suffer from "double booking" because online inventory and in-person tap-to-pay are not aggressively locked in real-time. Traditional POS software ignores the AI assistant workflow, forcing the owner to manually update stock levels and run separate analytics.

  ## Research Report
  - **Shopify/Wix Desktop Reliance:** Complex operations require a return to the desktop; syncing is sometimes delayed without premium plugins.
  - **Link-in-Bio tools (Linktree/Stan):** Superb mobile-first design (375px), but completely lack robust physical inventory management or tap-to-pay POS support.
  - **OHC Missing Capability:** A strongly consistent, centralized distributed locking mechanism (e.g., Redis Redlock) tied to a mobile-first POS UI and unified AI Operations Agent. We must prevent an online user from buying an item the very moment an in-store transaction reserves it.

  ## Design Doc
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    sequenceDiagram
        actor POS as Priya (Mobile POS)
        actor Online as Customer (Web)
        participant API as OHC API
        participant Redis as Redis Redlock
        participant DB as PostgreSQL Ledger
        participant Agent as Operations Agent

        POS->>API: Tap-to-Pay "Red Dress"
        API->>Redis: Acquire Lock (15s) `ohc:lock:tenant_id:inventory:red_dress`
        Redis-->>API: Lock Acquired
        Online->>API: Add "Red Dress" to Cart
        API->>Redis: Check Lock `ohc:lock:tenant_id:inventory:red_dress`
        Redis-->>API: Lock Busy
        API-->>Online: Graceful "Item just sold out in-store"
        API->>DB: Deduct "Red Dress" Stock
        API->>Redis: Release Lock
        Agent->>API: Observe Low Stock
        Agent-->>POS: Feed Card: "Red Dress sold out. Draft restock order?"
    ```
  - **Mobile UX Flow (375px First):**
    - Owner opens the "Operations Feed".
    - Taps an item to initiate an in-store sale.
    - System instantly engages the Redlock (15 seconds for POS, 5 mins for online carts).
    - If successful, proceeds to Stripe Terminal interface. Touch targets >= 44x44px.
    - Post-sale, an "Operations Agent" card appears: "Red Dress sold out. Draft restock order?"
  - **AI Agent Integration Points:**
    - *Operations Agent* monitors the Postgres lock/ledger and proposes restock drafts.
    - *Customer Success Agent* intercepts conflicting online carts with graceful "Item just sold out in-store" messages.

  ## Implementation Prompt (For Implementer Agents)
  **Objective:** Build a Redis Redlock-backed centralized inventory locking service and a mobile-first POS flow.
  **Persona:** Priya (Boutique Owner)
  **CUJ:**
  1. Priya taps "Sell In-Store" for "Red Dress" on a 375px mobile UI.
  2. The backend acquires a Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) for 15 seconds.
  3. An online buyer trying to add "Red Dress" to their cart simultaneously is blocked with a graceful error.
  4. The in-store transaction finalizes, the stock drops to 0 in PostgreSQL.
  5. The Operations Agent generates a feed card asking if Priya wants to draft a restock order.

  *Acceptance Criteria:*
  - Fully responsive on 375px viewport (no horizontal scroll).
  - Backend must enforce strong consistency using Redis distributed locks.
  - 100% test coverage for the locking mechanism.
  - UI mockups/components reflect premium Translucent Glass styling.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
