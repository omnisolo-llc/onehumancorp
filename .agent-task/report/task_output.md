issue_title: "Implement Intelligent Distributed Locks for High-Concurrency Inventory Reservations"
issue_description: |
  **Title**: Implement Intelligent Distributed Locks for High-Concurrency Inventory Reservations

  **Problem Statement**:
  In high-traffic situations like product drops, flash sales, or viral events, the OHC system must guarantee that inventory is not oversold, even if multiple users attempt to purchase the exact same item simultaneously across different channels (e.g., online storefront and in-person POS). The traditional optimistic concurrency control or row-level locking on a central SQL database can become a bottleneck or result in confusing user errors (e.g., failed checkouts after entering payment info). A non-technical owner (like Priya) doesn’t know what a lock is—they just know they promised a customer an item that was actually sold to someone else three seconds earlier.

  **Research Report**:
  - **Shopify**: Handles high concurrency well with sophisticated queueing and caching infrastructure, often using Redis to manage checkout holds.
  - **Stripe**: Provides idempotency keys and locks, but their system relies heavily on the developer correctly implementing distributed locking in their custom backend.
  - **OHC Opportunity**: Implement a seamless, distributed lock mechanism (like Redis Redlock) that temporarily "reserves" an item the moment a customer begins the checkout process or a POS transaction is initiated. This prevents concurrent processes from modifying the same inventory item. AI agents (like "The Manager") can monitor these locks to detect abandoned carts and automatically release inventory back to the storefront.

  **Design Doc**:

  *Architecture diagram*:
  ```mermaid
  graph TD
      A[Customer Checkout / POS Tap] -->|Request Lock| B(Lock Manager)
      B --> C{Redis Redlock}
      C -->|Success| D[Hold Inventory]
      C -->|Failure| E[Show Graceful 'Out of Stock' Msg]
      D --> F[Finalize Payment & Ledger Update]
      F -->|Release Lock| C
      D -.->|Timeout / Abandoned| G[The Manager Agent]
      G -->|Release Lock| C
  ```

  *Key design decisions and why*:
  - **Redis Redlock**: Chosen for its robust distributed nature and speed, ideal for the high concurrency required during flash sales.
  - **Lock Keys**: Structured as `ohc:lock:{tenant_id}:inventory:{product_id}` to ensure strict isolation across tenants.
  - **Agent Monitoring**: Instead of just waiting for timeouts, "The Manager" agent actively monitors lock lifecycles, identifying trends (e.g., high abandonment rates) and suggesting interventions (e.g., sending a discount code via "The Ambassador").

  **Implementation Prompt**:

  **User-Facing Outcome**: During a busy sale, a customer tries to buy the last "Blue Ceramic Mug." They enter checkout, and the system reserves it for them. Another customer tries to buy it a second later and sees "Item temporarily reserved, please check back in 5 minutes." If the first customer abandons the cart, the item automatically becomes available again. The store owner never has to deal with angry customers who bought an out-of-stock item.

  **CUJ & Acceptance Criteria**:
  1. Set up a test scenario with 1 remaining stock for an item.
  2. User A initiates checkout. The system acquires a lock.
  3. User B attempts to initiate checkout for the same item. The system denies the request gracefully due to the existing lock.
  4. User A completes the purchase. The lock is released, and the central ledger is updated to 0.
  5. Provide unit and integration tests verifying the lock acquisition, lock denial, and lock release mechanisms.

  **Priority**: P1

  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
