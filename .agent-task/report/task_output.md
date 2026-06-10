issue_title: "Implement High-Performance Distributed POS & Centralized Inventory Lock Framework"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  For multi-channel hybrid merchants (like Priya the Boutique Operator who sells both in-store and online), OHC currently lacks a real-time inventory locking mechanism to prevent double-booking. Without a centralized, strongly consistent distributed locking protocol during the checkout flow (both online and tap-to-pay), simultaneous sales of limited stock items can cause frustrating customer experiences and reconciliation nightmares. We need a robust architecture to handle high-concurrency inventory deductions reliably.

  ## Research & Market Mapping
  Current platforms like Shopify offer extensive POS functionality but are slow and complex for micro-SMEs, often relying on high-tier plans for seamless real-time syncing. Wix provides basic syncing but often suffers from eventual consistency issues under load, leading to double-sales. Stripe Terminal provides excellent POS hardware interfaces but does not handle application-layer inventory locks natively.

  **Pain Points Identified:**
  1. **Race Conditions:** Simultaneous in-store tap-to-pay and online checkout for the last item in stock.
  2. **Ghost Reservations:** Carts holding inventory indefinitely.
  3. **Eventual Consistency Failure:** Offline POS syncing overwriting online sales data upon reconnection.

  ## Architectural System Design

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant C as Customer (Online/Mobile)
      participant POS as In-Store POS (Terminal)
      participant API as OHC API Layer (Go)
      participant Lock as Redis (Redlock)
      participant DB as Central Ledger (Postgres)
      participant Ops as Operations Agent

      C->>API: Add to Cart (Red Dress)
      API->>Lock: Request Lock (15 mins)
      Lock-->>API: Lock Granted
      API-->>C: Cart Updated

      POS->>API: Tap-to-Pay (Red Dress)
      API->>Lock: Request Lock (15s priority)
      Lock-->>API: Lock Failed (Reserved by Online Cart)
      API-->>POS: "Item reserved, checkout in progress"
      Ops->>POS: Push Notification: "Red Dress currently in online cart. Restock needed?"
  ```

  ### Mobile UX Flow (375px Target)
  - **POS Interface:** Minimalist card-based layout. When an item is scanned or tapped, it checks inventory instantly.
  - **Lock Conflict UI:** If an item is reserved by another channel, the item card turns translucent with a clear "Currently Reserved" badge, rather than an aggressive error alert.
  - **Ops Notification:** A slide-down banner from the Operations Agent offering a one-tap action (e.g., "Draft Restock Order").

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** Subscribes to Redis lock failure events. When an in-store sale fails due to an online cart lock, the Agent proactively notifies the owner and suggests drafting a purchase order for the sold-out/high-demand item.

  ## Implementation Prompt (For Implementer Agent)
  1. **Objective:** Implement a robust distributed locking mechanism in the Go backend using Redis (Redlock pattern) for inventory reservations.
  2. **Data Model Updates:** Add necessary fields to the `Product` or `Inventory` models (if not fully defined) to support `locked_quantity` and `available_quantity`.
  3. **Service Layer:** Create an `InventoryService` that provides `reserve_inventory` and `release_inventory` functions. These must use Redis to acquire a lock based on `tenant_id` and `product_id`.
  4. **API Integration:** Ensure the checkout and POS endpoints utilize this service before proceeding to payment intent creation.
  5. **Acceptance Criteria:**
     - A unit test demonstrating that concurrent requests for the same limited item result in one success and one structured lock-failure error.
     - The lock must have a configurable TTL to prevent ghost reservations.
     - Must integrate cleanly with the existing multi-tenant PostgreSQL schema.

  **Scope:** Large
  **Priority:** P0
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
