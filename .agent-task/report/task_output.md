issue_title: "Implement Distributed Locks and Zero-Touch Inventory Sync for Multi-Channel POS"
issue_description: |
  **Mission Queue Protocol Report**

  **1. Title:** Implement Distributed Locks and Zero-Touch Inventory Sync for Multi-Channel POS

  **2. Problem Statement:**
  Priya, our boutique owner persona, sells items both in-store via Point-of-Sale (POS) and online. Currently, the system lacks real-time inventory locking. If Priya processes an in-store tap-to-pay transaction while an online user has the same item in their cart, the system allows double-booking, resulting in an out-of-stock angry customer online or a failed offline transaction.

  **3. Research Report:**
  Analysis of competitors like Shopify and Square shows they use distributed caching and ledger reconciliation to manage hybrid sales. Shopify requires robust POS and online sync, but for micro-SMEs, it can be too complicated. OHC needs to build a "Zero-Touch" sync layer utilizing Redis (for temporary distributed locks during checkout) and PostgreSQL (for the central ledger).

  **4. Design Doc (Architecture Design):**
  - **Architecture Diagram**:
    ```mermaid
    graph TD
    Client_Online[Online Client] --> LockService[Redis: Redlock]
    Client_POS[POS Terminal] --> LockService
    LockService --> CentralLedger[Postgres: products]
    ```
  - **Mobile UX Flow**: On a 375px screen, when an online user attempts checkout for a locked item, they see a clean toast: "Item reserved in another cart" instead of an error crash. POS operators see an immediate stock deduction.
  - **AI Integration Points**: The Operations Agent monitors `products.locked_quantity` and triggers auto-replenishment drafts or restock tasks when `available_quantity` drops near zero.
  - **Key Design Decisions**:
    - Use Redis for short-term reservation (`ohc:lock:{tenant_id}:inventory:{product_id}`) to hold items for 5 minutes online, 15 seconds POS.
    - Enhance `products` table with `locked_quantity` and `available_quantity` (handled in migration `127_inventory_lock_pos.sql`).
    - Expose a gRPC service for obtaining/releasing locks seamlessly within the checkout transaction flow.

  **5. Implementation Prompt:**
  As an implementer, build the Redis Redlock inventory reservation service and integrate it into the checkout flow. Ensure the POS interface operates flawlessly on mobile. Implement optimistic UI updates for inventory changes, with graceful rollback capabilities if the reservation fails. Add the gRPC service definition and Rust implementation for locking.

  **6. Priority:** P1 (High)
  **7. Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
