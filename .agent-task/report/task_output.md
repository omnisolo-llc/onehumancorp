issue_title: "Implement Distributed Inventory Locks with Redis Redlock for POS/Online Sync"
issue_description: |
  # Architecture Gap Identified: Centralized Inventory Synchronization

  **Problem Statement:**
  Small business owners with hybrid operations (online and in-store), like Priya the Boutique Owner, currently face double-booking and out-of-stock scenarios. When a simultaneous online and offline purchase occurs for the last item in stock, the platform fails to provide strong consistency. They require a centralized inventory mechanism that seamlessly syncs POS actions with the central ledger to prevent these collisions invisibly.

  **Research Report:**
  Analysis of competitors (Shopify, Square) reveals that their POS solutions often require complex configuration or lack robust real-time synchronization out of the box for smaller merchants. Referencing `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md`, we identified a gap where OHC's inventory lacks a real-time reservation system during checkout.

  **Design Doc:**
  - **Architecture:**
    - Utilize Redis Redlock for distributed inventory locks.
    - Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
    - Central ledger remains PostgreSQL using optimistic concurrency control.

    ```mermaid
    sequenceDiagram
        participant Customer as Customer (Online)
        participant POS as Priya (POS App)
        participant Server as OHC Server
        participant Redis as Redis (Redlock)
        participant Postgres as DB (Postgres)

        rect rgb(200, 220, 250)
            Note over Customer, Postgres: Concurrent Checkout Attempt
            Customer->>Server: Attempt Checkout (Item A)
            POS->>Server: Attempt Tap-to-Pay (Item A)

            par POS Lock Attempt
                Server->>Redis: Acquire Redlock (ohc:lock:tenant_id:inventory:A)
                Redis-->>Server: Lock Acquired (15s TTL)
                Server->>Postgres: Deduct Inventory (Item A)
                Postgres-->>Server: Success
                Server-->>POS: Payment Intent Successful
            and Online Lock Attempt
                Server->>Redis: Acquire Redlock (ohc:lock:tenant_id:inventory:A)
                Redis-->>Server: Lock Denied
                Server-->>Customer: Item Unavailable Error
            end
        end
    ```

  - **Mobile UX Flow:**
    - On the mobile POS interface (375px), when an item is added to the cart and the "Charge" button is pressed, the system attempts to acquire a short-lived Redis lock (e.g., 15s for POS).
    - If successful, the transaction proceeds.
    - If unsuccessful, the UI displays an optimistic update indicating the item is currently being purchased elsewhere.
  - **AI Agent Integration:**
    - The Operations Agent ("The Manager") will listen to lock exhaustion or zero-inventory events to automatically suggest restocking or update storefront availability.

  **Implementation Prompt:**
  As an Implementer agent, your task is to integrate Redis Redlock into the inventory management and checkout flows (both POS and Online).
  - The CUJ involves Priya attempting to sell the last "Red Dress" in-store while an online user simultaneously tries to buy it.
  - Implement the lock acquisition logic before processing the Stripe Payment Intent.
  - Ensure the mobile POS interface gracefully handles lock failures without crashing, providing clear feedback to the user.
  - Acceptance Criteria: A Playwright E2E test must demonstrate the prevention of double-booking under concurrent checkout attempts.

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
