issue_title: "[Architecture] Autonomous Inventory Sync & Offline POS Engine"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  For multi-channel merchants like **Priya the boutique owner**, managing inventory across an online storefront and an in-person Point of Sale (POS) is a critical pain point. Currently, OneHumanCorp (OHC) lacks a strongly consistent, real-time synchronization architecture between the central database and offline-first edge clients. Without this, simultaneous online and in-store purchases lead to double-booking, overselling, and lost customer trust. The owner shouldn't have to manually reconcile inventory or use expensive third-party tools just to run a hybrid business.

  ## Research Report
  - **Current State in OHC:** OHC has basic inventory tracking, but lacks a robust distributed lock mechanism and offline-capable synchronization protocol to handle the "Tap-to-Pay vs. Web Cart" race condition.
  - **Competitor Analysis:**
    - **Shopify:** Offers POS and online integration, but often requires higher-tier plans or third-party apps for seamless, real-time inventory locking. Complex for micro-SMEs.
    - **Square:** Excellent POS hardware and offline capabilities, but lacks the unified, agentic automation OHC provides (e.g., automatically drafting a restock order when inventory dips).
    - **Stripe Terminal:** Provides the payment rails, but leaves the inventory sync logic entirely to the developer.
  - **The Gap:** A unified, zero-configuration system that instantly reserves stock during checkout (both online and offline) and uses AI agents to manage edge cases seamlessly.

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[Central Postgres Ledger] <-->|Eventual Sync| B(Offline-First POS Client)
      C[Online Storefront] -->|Checkout Attempt| D{Redis Redlock Service}
      B -->|Tap-to-Pay Transaction| D
      D -->|Lock Acquired: 15s-5m| A
      D -->|Lock Failed| E[Conflict Resolution]
      E --> F[Operations Agent]
      F -->|Push Notification| G[Owner Device]
      F -->|Stock Update| C
  ```

  ### 2. Core Components & Data Flow
  - **Redis Redlock Service:** Implements distributed locks. Key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`. A tap-to-pay transaction requests a short-lived lock (e.g., 15s), while a web checkout might hold a lock slightly longer.
  - **Central Ledger (PostgreSQL):** The ultimate source of truth. Uses row-level locks (`SELECT ... FOR UPDATE`) during final ledger mutation to ensure absolute consistency.
  - **Offline/Local-First Edge:** The mobile POS client caches inventory locally. If offline, it allows optimistic mutations (with explicit UI warnings) and queues events for reconciliation upon reconnection.

  ### 3. AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Monitors inventory levels. If an item hits zero, it automatically updates the online storefront to "Sold Out" and triggers a push notification to the owner suggesting a restock order.
  - **Customer Success Agent:** If an online cart is invalidated due to an in-store purchase, the agent drafts a personalized, apologetic email or SMS offering a substitute or backorder option.

  ### 4. Mobile-First UX Flow (375px Viewport)
  - **POS Interface:** Clean, high-contrast UI (macOS Translucent Glass style). Large touch targets (≥ 44x44px) for adding items to the cart.
  - **Real-time Indicators:** Subtle badges on items showing "1 left" that pulse when reserved by an online cart.
  - **Conflict UI:** If a race condition occurs and the POS loses, a graceful modal appears: "Item just sold online. Reserve next batch?"

  ## Implementation Prompt
  **Outcome:** Implement the Redis Redlock-based inventory reservation service and integrate it with a mock Terminal session endpoint to prove the race condition is solved.
  **Core User Journey (CUJ):**
  1. An online customer begins checkout for the last unit of Product X.
  2. Simultaneously, Priya attempts to process a tap-to-pay transaction for Product X in-store.
  3. The Redis Redlock service grants the lock to the first requester.
  4. The second requester receives a graceful "Item no longer available" error.
  5. The Operations Agent triggers a low-stock alert.
  **Acceptance Criteria:**
  - Create a Redis-backed distributed lock service using the `ohc:lock:{tenant_id}:inventory:{product_id}` pattern.
  - Implement a backend API endpoint simulating a POS checkout that attempts to acquire this lock before mutating the database.
  - Write unit tests demonstrating that concurrent requests for the last item result in only one successful transaction.
  - Write a Playwright E2E test simulating the race condition via the UI, verifying the correct error state is shown to the user who lost the race.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, inventory]
assignees: []
