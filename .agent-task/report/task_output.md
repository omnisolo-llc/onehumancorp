issue_title: "[Research] Autonomous Multi-Tenant Reservation & Inventory Sync"
issue_description: |
  # Research Report: Autonomous Multi-Tenant Reservation & Inventory Sync

  ## Problem Statement
  OneHumanCorp (OHC) users currently lack a unified, race-condition-free inventory reservation system. For a persona like Priya (boutique owner) or Leo (music tutor), double-bookings occur when an item is simultaneously checked out online and bought in person via POS.

  ## Research Report
  - Competitor platforms like Shopify require paid third-party apps for POS synchronization or complex webhooks.
  - OHC's value proposition is "Invisible AI Automation". Inventory synchronization should be zero-touch.
  - The current PostgreSQL schema needs robust `inventory_reservations` support tightly coupled with Redis distributed locks (Redlock).

  ## Design Doc
  - **Data Model:**
    - `inventory_reservations` table (PostgreSQL) tracking `tenant_id`, `product_id`, `quantity`, `expires_at`.
    - Redis-backed temporary locking (`ohc:lock:inventory:{tenant_id}:{product_id}`).

  ```mermaid
  erDiagram
      tenants ||--o{ inventory_reservations : "has many"
      products ||--o{ inventory_reservations : "reserved in"
      inventory_reservations {
          uuid id PK
          string tenant_id FK
          uuid product_id FK
          int quantity
          timestamp expires_at
      }
  ```

  - **AI Integration:**
    - The Operations Agent ("The Manager") should resolve conflicting reservations and alert the owner if stock is depleted.
  - **Mobile UX Flow (375px Target):**
    - The POS app must instantly deduct local cache and attempt a distributed lock. If lock fails (item bought online a second ago), show a clear "Item Just Sold Out" prompt.
    - Wireframe Description:
      - 375px wide viewport.
      - At checkout via POS, large tap-to-pay button (min 44px height).
      - On tap, optimistic UI updates the cart.
      - If reservation fails, a translucent glass bottom-sheet modal slides up: "Oops! Someone just bought the last one online." with a single "Acknowledge" button.

  ## Repository Issues Discovered
  1. No clear domain boundary separating external webhooks from internal domain models (`src/server/domain` vs `src/server/api`).
  2. The `onboarding_state` JSON blob lacks strong types in the Rust backend.
  3. No rate limiting middleware for public-facing agent endpoints.
  4. Redundant logging declarations across different domain services.
  5. The POS component lacks an explicit offline-sync reconciliation queue in the database schema.

  ## Implementation Prompt
  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona:** Priya the Boutique Owner
  **Outcome:** A seamless inventory system preventing double-booking across online and offline channels.

  **Next Actions for Engineering:**
  1. Implement `inventory_reservations` in PostgreSQL and Redis Redlock.
  2. Create a booking conflict resolution agent capability.
  3. Update `pos.html` to handle failed reservation states gracefully.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
