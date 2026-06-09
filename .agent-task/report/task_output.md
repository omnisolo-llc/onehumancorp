issue_title: "[Research] OHC Unified Multi-Channel Inventory Sync & POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## 1. Problem Statement
  Small business owners with multi-channel setups (e.g., Priya the boutique owner selling online and in-store) struggle with out-of-sync inventory. When selling the last unit of a product in-store, an online customer might simultaneously buy the same item online, leading to double-booking and negative customer experiences. Competitors either charge a premium for unified systems (Shopify with complex POS additions) or require manual reconciliation.

  ## 2. Research Report
  - **Market Context**: Legacy platforms like Shopify are complex and require technical knowledge to set up proper multi-channel inventory. Budget platforms lack the capability to sync offline and online sales in real-time.
  - **OHC Opportunity**: OHC can differentiate by providing an integrated, AI-driven Point-of-Sale (POS) and inventory management system that is simple to use and native to the platform. By leveraging Redis for distributed locks and the Operations Agent for background management, we can deliver real-time inventory consistency without user intervention.
  - **Key Gap**: Currently, OHC lacks a real-time, strongly consistent inventory locking mechanism for checkout and a robust distributed sync protocol for POS.

  ## 3. Design Doc
  ### Data Model (PostgreSQL & Redis)
  - **PostgreSQL**: Central source of truth (`products` table, specifically the `inventory_count` column).
  - **Redis Redlock**: Distributed locking mechanism used during the checkout process (both online and in-store). Key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **TerminalSession (PostgreSQL)**: To track offline sales made via the POS and sync them back to the central ledger asynchronously when the network is restored.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Storefront
      participant POS as Terminal POS
      participant Central_Ledger as Central Ledger (PG)
      participant Distributed_Lock as Distributed Lock (Redis)
      participant Agent as Operations Agent

      Customer->>Storefront: Add to cart
      Storefront->>Distributed_Lock: Acquire Lock (15s)
      Distributed_Lock-->>Storefront: Lock Acquired
      Storefront->>Central_Ledger: Reserve Inventory

      POS->>Distributed_Lock: Acquire Lock
      Distributed_Lock-->>POS: Lock Denied (Item reserved)
      POS-->>Customer: Item out of stock

      Storefront->>Central_Ledger: Complete Sale
      Distributed_Lock-->>Storefront: Release Lock
      Agent->>Storefront: Suggest Restock Action
  ```

  ### AI Integration
  - **Operations Agent ("The Manager")**: Monitors stock levels, handles sync conflicts, and triggers low-stock push notifications or automatically drafts restock orders.

  ### Mobile UX Flow (375px)
  1. **POS View (Owner)**: A mobile-first POS interface where the owner can process in-store sales using Stripe Terminal. The interface is optimized for 375px screens with large touch targets.
  2. **Online Customer View**: If an item is reserved by an in-store transaction, the online storefront instantly reflects "Item just sold out" for other customers.
  3. **Notification (Owner)**: After the sale, the owner receives a notification from the Operations Agent about the sold-out item and a suggested restock action.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS
  **Target Persona**: Priya the Boutique Owner
  **Outcome**: A seamless inventory system where an in-store tap-to-pay purchase instantly reserves stock, preventing online double-booking, managed invisibly by the Operations Agent.

  **Acceptance Criteria**:
  1. Implement a distributed lock mechanism (Redis Redlock) for inventory reservation during both online and in-store checkout.
  2. Enhance the existing data schema to support `TerminalSession` for robust offline-sync reconciliation with the PostgreSQL central ledger.
  3. Extend the Operations Agent to proactively monitor inventory levels, manage sync conflicts, and generate low-stock notifications.
  4. Ensure the POS interface and inventory management screens are fully functional and optimized for a 375px mobile viewport.
  5. Provide comprehensive automated test coverage, including unit tests for locking logic and Playwright E2E tests for the synchronized checkout flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
