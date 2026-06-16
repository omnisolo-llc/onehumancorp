issue_title: "Implement Unified Multi-Channel Inventory Sync & POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report details the architectural design and implementation plan for a robust Centralized Inventory and Distributed Point-of-Sale (POS) synchronization system for OneHumanCorp (OHC). The goal is to solve the critical pain point of inventory fragmentation for multi-channel merchants (like Priya, our persona). This architecture ensures real-time consistency between online storefronts and in-store POS terminals, powered by a robust Redis Redlock reservation mechanism and OHC's intelligent agent ecosystem.

  ## 1. Problem Statement
  Small business owners operating both online and physical storefronts face significant challenges with inventory synchronization. A common failure mode is "double-booking" or selling out an item simultaneously online and in-store because traditional platforms lack real-time synchronization without expensive add-ons.

  **Target Persona**: Priya (Boutique Owner).
  Priya needs seamless inventory tracking. An item sold via tap-to-pay in-store must instantly become unavailable online to prevent overselling.

  ## 2. Research & Competitive Analysis
  - **Shopify / Traditional Platforms**: While powerful, real-time omni-channel inventory sync often requires premium tiers or third-party apps, adding complexity and the "App Tax". Their POS systems are robust but lack agentic intelligence to handle edge cases proactively.
  - **Square**: Strong POS hardware integration, but historically weaker e-commerce integration without heavy configuration.
  - **OHC Differentiation**: We integrate inventory management deeply with AI Agents. The "Operations Agent" actively monitors locks, coordinates sync, and can proactively suggest actions (like reordering stock) rather than passively reporting a failure.

  ## 3. Architecture & Design Doc

  ### 3.1 Data Model & Sync Protocol (The "What")
  The system relies on a hybrid consistency model: optimistic locking for central persistence, and distributed locking for transient transaction states.

  - **Central Ledger (PostgreSQL)**: The ultimate source of truth. We will use robust transaction isolation and potentially row-level locking or optimistic concurrency (e.g., a `version` or `updated_at` check) when updating `Inventory` tables to ensure strong consistency on commit.
  - **Distributed Reservation Locks (Redis Redlock)**: During a checkout flow (online or POS), a temporary reservation is placed on the inventory item.
    - **Key Pattern**: `ohc:lock:{tenant_id}:inventory:{product_id}`
    - **Duration**: Tunable based on context (e.g., 5-10 minutes for an online cart, 15-30 seconds for an active POS transaction).
    - **Purpose**: Prevents another transaction from acquiring the same item while payment is processing.
  - **Offline/Local-First POS Client**: The POS interface caches the catalog. If a network disruption occurs, it logs transactions locally. Upon reconnection, it uses an eventual consistency reconciliation protocol to sync with the central ledger. (Note: Offline sales cannot guarantee preventing online oversell during the offline period, but the system will reconcile gracefully upon reconnect).

  ### 3.2 AI Agent Integration (The "Who")
  - **Operations Agent**: The core orchestrator. Monitors inventory levels, handles reservation timeouts (releasing locks), and detects discrepancies during offline sync reconciliation.
  - **Finance Agent**: Processes split payments for Terminal transactions and unified reporting.
  - **Customer Success Agent**: If an online customer attempts to buy an item that was just locked by a POS transaction, this agent generates a graceful "Item just sold out in-store" message and can offer alternatives or backorder options.

  ### 3.3 Mobile UX Flow (375px First)
  - **The POS View**: A clean, high-contrast interface designed for quick interactions. Large touch targets (>= 44x44px) for products.
  - **The Transaction Flow**:
    1. Tap product to add to cart.
    2. Tap "Charge" (initiates Redis Redlock on inventory).
    3. Stripe Terminal processes payment.
    4. Success -> PostgreSQL ledger updated, lock released/consumed.
  - **Offline State**: A clear, unobtrusive banner indicating "Offline Mode: Changes will sync when connected."

  ### 3.4 Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant User (POS)
      participant OHC Backend
      participant Redis (Redlock)
      participant Postgres (Ledger)
      participant Stripe Terminal

      User (POS)->>OHC Backend: Initiate Transaction (Item A)
      OHC Backend->>Redis (Redlock): Acquire Lock ohc:lock:tenant_id:inventory:item_A
      alt Lock Acquired
          Redis (Redlock)-->>OHC Backend: Lock Granted
          OHC Backend->>Stripe Terminal: Process Payment
          Stripe Terminal-->>OHC Backend: Payment Success
          OHC Backend->>Postgres (Ledger): Update Inventory Count (Commit)
          OHC Backend->>Redis (Redlock): Release Lock
          OHC Backend-->>User (POS): Transaction Complete
      else Lock Denied (Item A in online cart)
          Redis (Redlock)-->>OHC Backend: Lock Denied
          OHC Backend-->>User (POS): Error: Item currently reserved online
      end
  ```

  ## 4. Implementation Prompt for Engineer Agents

  **Feature**: Unified Multi-Channel Inventory Sync & POS Integration

  **Target Persona**: Priya the Boutique Owner

  **Outcome**: Priya can process an in-store sale using the POS interface, and the system instantly reserves the inventory via Redis, preventing online customers from double-booking the last item.

    **Estimated Scope**: Large
  **Acceptance Criteria & Next Actions**:
  1.  **Distributed Lock Service**: Implement a robust Redis-backed distributed locking mechanism (Redlock pattern) within the Go backend service layer. Provide clear APIs to `acquire`, `release`, and `renew` inventory locks based on `tenant_id` and `product_id`.
  2.  **POS Transaction Flow Integration**: Integrate the lock service into the POS transaction initiation flow. Before a Terminal payment is processed, the system MUST successfully acquire the Redis lock for the requested inventory items.
  3.  **PostgreSQL Reconciliation**: Ensure the backend securely finalizes the inventory deduction in the PostgreSQL database upon successful payment, and reliably releases the Redis lock regardless of transaction success or failure (using timeouts or explicit release on error paths).
  4.  **TerminalSession Schema Refinement**: Update the data schema to support `TerminalSession` states that accurately reflect online vs. offline synchronization statuses, enabling robust future offline reconciliation.

  *Note to Implementer: Do not hardcode specific DB columns in this phase; design the trait/interface for the locking mechanism so it can be cleanly injected into the POS and checkout service layers. Ensure strict tenant isolation on all lock keys.*
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
