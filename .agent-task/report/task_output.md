issue_title: "Implement Redis Redlock Inventory Reservation for Multi-Channel POS"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report details the architectural design and required engineering tasks to implement a centralized inventory and distributed Point-of-Sale (POS) synchronization system for OneHumanCorp (OHC). Based on the investigation into small business inventory management pain points (specifically addressing the needs of multi-channel merchants like Priya), this design introduces a Redis Redlock-based inventory reservation mechanism coupled with a robust PostgreSQL central ledger, ensuring consistency between online and in-store sales channels.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  - **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. The existing PostgreSQL schema (`products`, `inventory_levels`, `inventory_transactions`) tracks static availability but lacks ephemeral reservations.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking.
    - The lock duration is dynamically tuned: 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions.
    - Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}` or `ohc:lock:{tenant_id}:inventory_variant:{variant_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### AI Agent Coordination
  - **Operations Agent ("The Manager"):** Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - **Finance Agent ("The Accountant"):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.
  - **Customer Success Agent ("The Ambassador"):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Mobile-First Implementation (375px First)
  - Ensure the POS interface operates flawlessly on a 375px viewport.
  - Touch targets for inventory adjustment and checkout must be ≥ 44x44px.
  - Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails (e.g. translucent glass styling to indicate locking states).

  ## 4. Implementation Prompt (Track 4)

  **Feature Name:** OHC Unified Multi-Channel Inventory Sync & POS

  **Target Persona:** Priya the Boutique Owner

  **Outcome:** A seamless inventory system where an in-store tap-to-pay purchase instantly reserves and deducts stock using Redis Redlocks, preventing an online customer from double-booking the same item, all managed invisibly by the Operations Agent.

  **Critical User Journey (CUJ):**
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration.
  3. The system applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but receives a graceful "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, the PostgreSQL ledger is updated, and the Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  **Acceptance Criteria for Implementer:**
  - Implement the Redis Redlock inventory reservation service in Rust and integrate it into the checkout/POS flow.
  - Define clear locking scopes using tenant isolation keys (`ohc:lock:{tenant_id}:inventory:{product_id}`).
  - Ensure the Operations Agent is hooked into lock failures to trigger automated alerts.
  - Write comprehensive unit tests for lock acquisition, timeout, and release.
  - Write a Playwright E2E test verifying the CUJ: simulating a concurrent checkout attempt while an item is locked by a POS session.
  - Maintain strictly 100% test coverage for new code.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
