issue_title: "Implement Distributed Inventory Locks (Redis Redlock) for Multi-Channel Sync"
issue_description: |
  # Research Report: Implement Distributed Inventory Locks (Redis Redlock) for Multi-Channel Sync

  ## Problem Statement
  Currently, OneHumanCorp (OHC) is missing a robust distributed locking mechanism to prevent double-booking or overselling when a product is being purchased simultaneously in-store (POS) and online. For multi-channel business owners like Priya (Boutique Owner), this can lead to situations where the last item is sold to two different customers, causing frustration and requiring manual intervention.

  ## Research Report
  - **Market Context**: Platforms like Shopify handle this natively, but smaller merchants using disjointed tools (e.g., Stripe Terminal for in-store and a basic website for online) often face sync issues.
  - **The OHC Opportunity**: By implementing Redis Redlock during the checkout/reservation phase, OHC can guarantee consistency across all channels (online, POS, agent-driven sales) without forcing the owner to manually reconcile inventory.
  - **Competitive Edge**: True omnichannel inventory management requires distributed locks to handle temporary reservations (e.g., 5 mins for an online cart, 15 seconds for a tap-to-pay transaction) before final DB committal.

  ## Design Doc

  ### Architecture
  ```mermaid
  sequenceDiagram
      participant C as Customer (Online/POS)
      participant API as OHC API
      participant R as Redis (Redlock)
      participant DB as PostgreSQL (Ledger)
      participant OA as Operations Agent

      C->>API: Attempt Checkout (Product X)
      API->>R: Request Lock (ohc:lock:{tenant_id}:inventory:{product_id})
      alt Lock Acquired
          R-->>API: Lock Granted
          API->>DB: Check Inventory Levels
          alt In Stock
              API->>DB: Commit Transaction (Deduct Inventory)
              DB-->>API: Success
              API->>R: Release Lock
              API-->>C: Checkout Success
              API->>OA: Publish Event (Inventory Updated)
          else Out of Stock
              API->>R: Release Lock
              API-->>C: Out of Stock Error
          end
      else Lock Denied
          R-->>API: Lock Busy
          API-->>C: Item currently being purchased, try again
      end
  ```

  ### Mobile UX Flow
  - **POS (In-Store)**: When Priya taps to pay, the system attempts to acquire a short-lived lock (e.g., 15s). The UI shows a subtle loading spinner. If successful, payment proceeds.
  - **Online**: When a customer adds to cart or begins checkout, a longer lock (e.g., 5 mins) is acquired. If the item is locked by someone else, a graceful error message is displayed: "This item is currently being purchased by another customer. Please try again in a few minutes."

  ### AI Agent Integration
  - **Operations Agent**: Listens for inventory update events. If stock reaches zero, it can proactively suggest a restock order to the owner or update the storefront to reflect "Sold Out".

  ### Key Design Decisions
  - **Redis Redlock**: Using Redis for distributed locks ensures high performance and consistency across multiple API server instances.
  - **Lock Granularity**: Locks are applied at the `tenant_id` and `product_id` level to prevent widespread blocking.
  - **Fail-Safe**: If a lock cannot be released (e.g., server crash), the TTL ensures it eventually expires, preventing permanent deadlocks.

  ## Implementation Prompt

  **Feature Name**: OHC Unified Multi-Channel Inventory Sync & POS - Redis Redlock

  **Target Persona**: Priya the Boutique Owner

  **Outcome**: A robust locking mechanism that prevents double-booking across online and in-store channels, giving owners confidence in their inventory numbers.

  **Critical User Journey (CUJ)**:
  1.  Simulate two simultaneous checkout attempts for the same product with an inventory of 1.
  2.  The first request acquires the Redis lock and proceeds with the transaction.
  3.  The second request fails to acquire the lock and receives a "currently being purchased" error.
  4.  The first transaction completes, inventory is decremented, and the lock is released.
  5.  A subsequent request for the product receives an "out of stock" error.

  **Next Actions for Engineering**:
  1.  Implement the Redis Redlock algorithm (or use an existing Rust crate like `redlock` or a custom implementation using `redis` crate) in the backend.
  2.  Integrate the lock acquisition/release logic into the checkout and POS transaction flows.
  3.  Configure appropriate TTLs for different contexts (online cart vs. POS).
  4.  Write comprehensive unit and integration tests verifying the lock behavior under concurrent load.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
