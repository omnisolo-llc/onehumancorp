issue_title: "[Omnichannel Inventory Sync] Architect Redis Redlock for POS/Online Concurrency"
issue_description: |
  # Research Report: Omnichannel Tap-to-Pay and Inventory Sync Engine Architecture

  ## Problem Statement
  Small business owners like Priya (Boutique Operator) need seamless inventory tracking between their online storefronts and in-store point-of-sale (POS) systems. Currently, OneHumanCorp (OHC) lacks a strongly consistent, real-time inventory locking mechanism that operates across multiple channels. Without this, simultaneous online and offline (Tap-to-Pay) purchases can result in double-booking and overselling of limited inventory, leading to negative customer experiences and manual reconciliation work for the owner.

  ## Research Findings
  Our research into competitor platforms (Shopify, Wix, Square) reveals that while Square excels at POS, its online inventory synchronization can lag. Shopify requires expensive POS Pro subscriptions for advanced inventory features. OHC's differentiation lies in providing enterprise-grade, zero-configuration consistency for small operators out of the box.

  **The Gap:** OHC needs a robust, low-latency distributed locking mechanism that temporarily reserves inventory during checkout (both online and via mobile POS) before final database commit.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant MobilePOS as Mobile POS (375px)
      participant OnlineCart as Online Cart (Browser)
      participant API as OHC API Gateway
      participant SyncEngine as Inventory Sync Engine
      participant Redis as Redis (Redlock)
      participant DB as PostgreSQL (Ledger)

      alt Simultaneous Checkout
          MobilePOS->>API: Tap-to-Pay Request (Product A)
          OnlineCart->>API: Checkout Request (Product A)

          API->>SyncEngine: Request Lock (Product A)
          SyncEngine->>Redis: Acquire Lock (ohc:lock:{tenant_id}:inventory:{product_id})

          alt Lock Acquired (Mobile POS)
              Redis-->>SyncEngine: Lock Granted
              SyncEngine-->>API: Proceed to Payment
              API->>DB: Deduct Inventory
              DB-->>API: Success
              API->>SyncEngine: Release Lock
              SyncEngine->>Redis: Delete Lock
              API-->>MobilePOS: Payment Success UI
          else Lock Denied (Online Cart)
              Redis-->>SyncEngine: Lock Failed (Timeout/In Use)
              SyncEngine-->>API: Inventory Unavailable
              API-->>OnlineCart: "Item just sold out!" UI
          end
      end
  ```

  ### Mobile UX Flow (375px)
  1.  **Priya (Boutique Owner)** uses the OHC mobile app as her POS. She adds an item to the cart and taps "Charge $50".
  2.  The app immediately requests an inventory lock.
  3.  **Optimistic UI:** The button transitions to a spinning state.
  4.  **Success:** If the lock is acquired and payment succeeds, a green checkmark appears.
  5.  **Failure:** If an online customer *just* bought the last item, an error toast appears: "This item just sold out online. Refreshing inventory." The item is removed from the POS cart.

  ### AI Agent Integration
  -   **Operations Agent:** Monitors inventory levels. If an item sells out due to an online or POS transaction, the Operations Agent immediately triggers an invalidation of the edge cache for that product page and alerts the owner if restocking is necessary based on historical velocity.

  ## Implementation Prompt
  Implement the Redis Redlock-based inventory reservation system and the corresponding API endpoints for the Unified Checkout engine.

  **Acceptance Criteria:**
  1.  A distributed locking mechanism is implemented using Redis (e.g., utilizing `redsync` in Go).
  2.  Lock keys must follow the pattern `ohc:lock:{tenant_id}:inventory:{product_id}`.
  3.  Locks must have configurable TTLs (e.g., 15 seconds for POS, 5 minutes for online carts).
  4.  The system must gracefully handle lock acquisition failures and return clear errors to the client.
  5.  Write E2E Playwright tests simulating concurrent checkouts for the same item to ensure one succeeds and one fails with an out-of-stock message.

  ## Priority & Scope
  -   **Priority:** P0 (Critical for Commerce Operations)
  -   **Estimated Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
