issue_title: "[research] Implement Centralized Inventory & Distributed POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Executive Summary
  This report investigates the current landscape of small business inventory management, specifically addressing the pain points of multi-channel (online + in-store) merchants. The objective is to design a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless, real-time experience for non-technical users.

  ## 1. Market Mapping & Competitor Discovery
  Competitors like Shopify dominate the e-commerce space with extensive POS capabilities but often fail micro-SMEs due to complexity. Their inventory management can be disjointed—online inventory frequently falls out-of-sync with in-person sales unless costly third-party integration tools or higher-tier plans are employed. Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify the business operations effortlessly.

  ## 2. OHC Gap & Pain Point Identification
  - **Persona Focus:** Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader).
  - **The Gap:** Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## 3. Deep Dive Architecture Design

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** The ultimate source of truth for all inventory counts. We utilize row-level locking or optimistic concurrency control for critical updates.
  - **Distributed Locks (Redis Redlock):** A temporary inventory reservation system applied during the checkout process to prevent double-booking. The lock duration is dynamically tuned (e.g., 5 minutes for online carts vs. 15 seconds for rapid tap-to-pay transactions). Lock key pattern: `ohc:lock:{tenant_id}:inventory:{product_id}`.
  - **Offline/Local First POS Client:** The mobile POS client caches catalog data locally. It employs an eventual consistency mechanism to sync finalized offline sales and reconcile with the central ledger asynchronously when the network is restored.

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant Customer as Online Customer
      participant Storefront as OHC Web Store
      participant Central as OHC Central Ledger (PostgreSQL)
      participant Redis as OHC Redis Lock (Redlock)
      participant POS as OHC Mobile POS (Stripe Terminal)
      participant Cashier as Priya (Cashier)
      participant Operations as OHC Operations Agent
      participant Ambassador as OHC Ambassador Agent

      Customer->>Storefront: Add "Red Dress" to Cart
      Storefront->>Redis: Request Lock (5 min) `ohc:lock:tenant:inventory:red_dress`
      Redis-->>Storefront: Lock Granted
      Note over Storefront: Item locked for online checkout

      Cashier->>POS: Scan "Red Dress" for in-store sale
      POS->>Redis: Request Lock (15 sec) `ohc:lock:tenant:inventory:red_dress`
      Redis-->>POS: Lock Denied (Held by Online Cart)
      Note over POS: POS must handle lock contention (e.g., alert cashier)

      Note over Storefront,Customer: Online checkout expires (or customer abandons)
      Redis->>Redis: Lock Expires `ohc:lock:tenant:inventory:red_dress`

      Cashier->>POS: Retry Scan "Red Dress"
      POS->>Redis: Request Lock (15 sec) `ohc:lock:tenant:inventory:red_dress`
      Redis-->>POS: Lock Granted
      POS->>Central: Process Sale & Deduct Inventory (Sync)
      Central-->>POS: Success
      Redis->>Redis: Release Lock
      Central->>Operations: Inventory Updated Event
      Operations->>Ambassador: Notify low stock
      Ambassador-->>Storefront: Update Availability (Out of Stock)
  ```

  ### Mobile UX Flow
  1.  **Dashboard:** Priya sees a unified dashboard showing today's sales and current inventory alerts.
  2.  **POS Mode:** She switches to the "POS" tab, a highly optimized 375px mobile view.
  3.  **Cart Building:** She taps large (≥ 44x44px) product tiles or scans barcodes. As items are added, the system automatically checks real-time availability via Redis locks.
  4.  **Checkout:** She taps "Charge", activating the Stripe Terminal integration (or tap-to-pay). The short-lived inventory lock prevents online double-booking during this phase.
  5.  **Offline Support:** If offline, the UI clearly indicates "Offline Mode". The sale proceeds (if configured to allow offline sales), and the transaction is queued for synchronization.
  6.  **Resolution:** The Operations Agent monitors the sync queue. If an offline sale creates a negative balance upon syncing, it alerts Priya for manual reconciliation and automatically pauses online sales for that item.

  ### AI Agent Integration Points
  -   **The Operations Manager Agent:** Monitors the central ledger. If inventory drops below a threshold, it drafts a restock order for Priya's approval. It also handles the complex logic of resolving conflicting offline syncs.
  -   **The Customer Success Agent:** If an item is double-booked due to an edge case (e.g., prolonged offline mode), this agent drafts a personalized apology and alternative offer for the online customer.

  ## 4. Implementation Prompt

  **Objective:** Implement the core Centralized Inventory & Distributed POS locking mechanism.

  **Target Persona:** Priya the Boutique Owner

  **Critical User Journey (CUJ):**
  1.  Priya is logged into the OHC mobile app (POS mode).
  2.  An online customer adds the last "Red Dress" to their cart (acquiring a 5-minute Redis lock).
  3.  Priya attempts to process an in-store sale for the same "Red Dress".
  4.  The POS system gracefully handles the lock contention, informing Priya that the item is currently in an online cart.
  5.  (Alternative Path) Priya processes the in-store sale *first*. The item is deducted from the central ledger.
  6.  An online customer subsequently attempts to buy the dress but sees it as "Out of Stock".

  **Acceptance Criteria:**
  -   **Data Model:** Define the necessary entities for multi-tenant inventory tracking, offline sync queues, and POS transaction records in PostgreSQL. Ensure strict multi-tenant isolation.
  -   **Distributed Locking:** Implement the Redis Redlock mechanism for inventory reservation. Create distinct lock profiles for online carts (longer duration) vs. POS checkouts (shorter duration).
  -   **API Endpoints:** Create gRPC/REST endpoints for POS client interaction, including offline transaction sync and real-time inventory checks.
  -   **Mobile-First UI (Flutter/Tauri):** Build the POS cart and checkout UI, ensuring responsive 375px design, large touch targets, and clear offline/online status indicators.
  -   **Agent Hooks:** Implement the event publishing mechanism so the Operations Agent is notified of inventory changes and sync conflicts.
  -   **Testing:** MUST include comprehensive unit tests and full Playwright E2E tests verifying the locking logic and UI behavior under simulated network conditions (including lock contention). No mocked internal APIs.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
