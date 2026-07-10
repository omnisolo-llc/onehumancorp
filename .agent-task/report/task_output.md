issue_title: "Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report investigates the current landscape of small business inventory management, specifically addressing the pain points of multi-channel (online + in-store) merchants. The objective is to design a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless, real-time experience for non-technical users.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  - **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ```mermaid
  erDiagram
      Tenant ||--o{ Product : has
      Product ||--o{ TerminalSession : synced_in
      TerminalSession ||--o{ PosOfflineTransaction : processes
      Product ||--o{ InventoryReservation : has_lock

      Product {
          uuid id PK
          uuid tenant_id FK
          string title
          int inventory_count
      }

      TerminalSession {
          uuid id PK
          uuid tenant_id FK
          uuid device_id
          string status
          timestamp last_synced_at
          int offline_changes_count
      }

      PosOfflineTransaction {
          uuid id PK
          uuid tenant_id FK
          uuid terminal_id FK
          int amount_cents
          string status
      }

      InventoryReservation {
          string lock_key PK
          uuid product_id FK
          timestamp expires_at
      }
  ```

  ```mermaid
  sequenceDiagram
      participant POS as POS Mobile Client
      participant API as OHC API
      participant Redis as Redis Redlock
      participant DB as PostgreSQL Ledger
      participant Agent as Operations Agent

      POS->>API: Process in-store tap-to-pay
      API->>Redis: Acquire lock ohc:lock:{tenant_id}:inventory:{product_id}
      Redis-->>API: Lock acquired (15s TTL)
      API->>DB: Process transaction & Deduct inventory
      DB-->>API: Success
      API->>Redis: Release lock
      API-->>POS: Success
      API->>Agent: Notify low stock / out of stock
      Agent-->>POS: Push notification: "Item sold out. Draft restock order?"
  ```

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile-First Implementation
  - Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Next Actions for Engineering:**
  - **Step 1:** Implement the Redis Redlock inventory reservation service and integrate it into the checkout flow.
  - **Step 2:** Refine the `TerminalSession` data schema to handle offline-sync reconciliation with the PostgreSQL central ledger.
  - **Step 3:** Extend the Operations Agent to monitor real-time stock levels, handle sync conflicts, and trigger low-stock push notifications.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
