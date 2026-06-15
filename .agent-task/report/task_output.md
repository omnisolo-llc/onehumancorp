issue_title: "[Research Report] OHC Universal Multi-Channel Inventory Sync & High-Performance Distributed POS Lock Architecture"
issue_description: |
  # Research Report: OHC Universal Multi-Channel Inventory Sync & High-Performance Distributed POS Lock Architecture

  ## Problem Statement
  Small business owners with hybrid physical and online presence (like Priya the Boutique Owner) struggle with inventory synchronization. They often double-book items because online and in-store Point-of-Sale (POS) systems do not talk to each other in real-time, resulting in frustrated customers and manual reconciliation tasks for the owner. Current solutions in the market (e.g., Shopify) often require expensive third-party apps or high-tier plans to achieve this, increasing the "App Tax" and friction.

  ## Research Report
  Our competitive analysis (as detailed in `docs/business/market_research/[research]_ohc_centralized_inventory_pos.md` and `docs/business/market_research/ohc_smb_mobile_first_agentic_workflows.md`) reveals a critical gap. Platforms either offer simple but disjointed setups (Wix, GoDaddy) or powerful but complex monoliths (Shopify).

  OHC's unique value proposition is the "Invisible AI Automation." To achieve this for inventory, we need a robust backend architecture that automatically handles concurrent transactions across channels seamlessly, enabling our AI agents (like "The Manager") to proactively manage stock levels without user intervention.

  ## Design Doc: High-Performance Distributed Inventory Lock Architecture
  This architecture addresses the core problem by implementing a highly consistent, distributed locking mechanism and a unified central ledger.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant POS as Mobile POS Client (In-store)
      participant Web as E-commerce Storefront (Online)
      participant API as OHC API Gateway
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL (Central Ledger)
      participant Agent as Operations Agent

      Note over POS, Web: Concurrent checkout attempt for last item
      POS->>API: Initiate Checkout (Item ID: 123)
      Web->>API: Initiate Checkout (Item ID: 123)

      API->>Redis: Request Lock `ohc:lock:{tenant_id}:inventory:123` (POS)
      Redis-->>API: Lock Granted (POS)
      API->>Redis: Request Lock `ohc:lock:{tenant_id}:inventory:123` (Web)
      Redis-->>API: Lock Denied (Web)

      API-->>Web: Return 409 Conflict (Item sold out)
      Web->>Agent: Trigger 'Out of Stock' Flow

      API->>DB: Process POS Transaction & Deduct Inventory
      DB-->>API: Success
      API->>Redis: Release Lock `ohc:lock:{tenant_id}:inventory:123`
      API-->>POS: Checkout Complete
  ```

  ### Key Components & Data Model
  1.  **Central Ledger (PostgreSQL):** The source of truth. Contains the `inventory_items` table with strict constraints preventing negative quantities. Multi-tenant isolation enforced via RLS.
  2.  **Distributed Lock (Redis Redlock):** Utilized during the checkout phase to reserve inventory temporarily. The lock key format must be `ohc:lock:{tenant_id}:inventory:{product_id}` to ensure strict tenant isolation. Lock TTL should be configurable based on the channel (e.g., shorter for quick in-store POS, longer for online carts).
  3.  **Operations Agent Integration:** If a lock is denied (item sold out), the system should trigger an event for the Operations Agent. The agent can then automatically notify the online user, update storefront availability, or draft a restock order for the owner.

  ### Mobile UX Flow (375px First)
  - The mobile POS interface must feel instant. When Priya taps a product to add to the cart, the UI should optimistically update while the API attempts to secure the Redis lock in the background.
  - If the lock fails (e.g., an online user just bought it), a clear, non-technical error toast appears: "This item was just purchased online."
  - Touch targets for the checkout flow must remain ≥ 44x44px.

  ### Multi-Tenancy & Security Invariants
  -   **Zero Trust:** All requests must be authenticated and authorized.
  -   **Row Level Security (RLS):** All inventory and ledger tables in PostgreSQL MUST have strict RLS policies enabled, ensuring `tenant_id = current_setting('app.current_tenant', true)`.
  -   **Redis Isolation:** Lock keys must incorporate the `tenant_id` to prevent cross-tenant lock contention or unauthorized unlocking.

  ## Implementation Prompt (For Implementer Agent)
  **Feature Request:** Implement the Distributed Inventory Locking Service and Central Ledger Sync.

  **Target Persona:** Priya the Boutique Owner.

  **Outcome:** Priya can confidently use her OHC Mobile POS while her online store is active, knowing that the system will automatically prevent double-booking of items through a distributed locking mechanism.

  **Acceptance Criteria:**
  1.  **Distributed Lock Service:** Implement a Redis-based locking mechanism using the key pattern `ohc:lock:{tenant_id}:inventory:{product_id}`. The service must handle lock acquisition, renewal, and release.
  2.  **Inventory Checkout Flow:** Integrate the locking service into the core checkout flow API. Ensure that concurrent checkout attempts for the same limited stock item fail gracefully for the second request.
  3.  **Data Integrity:** Ensure the PostgreSQL schema for inventory prevents negative quantities at the database level and enforces RLS for tenant isolation.
  4.  **Testing:** Must include robust unit tests mocking the Redis behavior to simulate race conditions and ensure the lock prevents double-spending. Add a Playwright E2E test simulating a concurrent POS and Online checkout.

  *Note to Implementer: Do not prescribe specific function signatures or specific packages in this brief; focus on fulfilling the architectural invariants and user journey.*

  **Priority:** P1
  **Estimated Scope:** Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
