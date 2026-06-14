issue_title: "[Research] Distributed Lock and Caching Architecture for Unified Multi-Channel POS & Inventory Sync"
issue_description: |
  **Title**: Distributed Lock and Caching Architecture for Unified Multi-Channel POS & Inventory Sync

  **Problem Statement**:
  Small business owners like Priya (boutique owner) require seamless inventory tracking between their online stores and in-store operations. The current OneHumanCorp (OHC) platform lacks a real-time, strongly consistent inventory locking mechanism to prevent double-booking or out-of-stock scenarios when simultaneous online and offline purchases occur. The absence of a robust distributed sync protocol holds back multi-channel merchants and violates the core OHC value proposition of invisible AI handling operational complexity.

  **Research Report**:
  - **Market Mapping & Discoveries**: Competitors like Shopify dominate e-commerce and handle POS but are overly complex for micro-SMEs, often requiring costly third-party tools to synchronize online and offline stock perfectly. Square and Stripe Terminal have robust POS hardware but lack built-in agentic workflows to automatically re-engage customers when inventory issues arise or automatically trigger restock suggestions based on unified data. Wix and Squarespace have basic native tools but fall short on proactive conflict resolution.
  - **The OHC Opportunity**: We can differentiate by directly integrating a distributed lock mechanism (Redis Redlock) and a central ledger (Postgres) that the Operations Agent monitors invisibly. This eliminates the "app tax" and provides real-time consistency.
  - **Technical Findings**: OHC uses Postgres for central truth and Redis/Valkey for caching. The architecture requires a `Redlock` pattern implemented via Redis to reserve inventory during an active checkout or a fast tap-to-pay transaction. Additionally, local-first optimistic UI updates for mobile POS clients (with eventual backend reconciliation) are necessary for the desired 375px native mobile experience.

  **Design Doc**:

  **Architecture diagram**:
  ```mermaid
  graph TD
      A[Online Checkout] --> B{Redis Redlock Reservation}
      C[In-Store POS / Terminal] --> B
      B -->|Lock Acquired| D[Proceed to Payment]
      B -->|Lock Failed| E[Ops Agent: Graceful Out-of-Stock Notice]
      D --> F[Finalize Payment]
      F --> G[Commit to PostgreSQL Central Ledger]
      G --> H[Ops Agent: Trigger Restock / Sync Notifications]
  ```

  **UI wireframes or screen flow description (375px first)**:
  - **Inventory Item View**: Shows large product image, current centralized stock count, and large call-to-action buttons for manual restock and manual adjustments. Touch targets are at least 44x44px.
  - **POS Checkout View**: Large numpad for entry, clear list of items in cart. When an item is added, a fast visual indicator confirms the item is locked for checkout.
  - **Agent Feed Notification View**: An actionable glassmorphism card appears dynamically when stock hits 0, prompting a 1-tap "Order Restock" action.

  **Mobile UX flow**:
  1. The user (Priya) is in POS mode on her 375px mobile screen.
  2. She taps an item to add it to the cart (generous ≥ 44x44px touch targets).
  3. Under the hood, a fast (15s) Redis lock is acquired.
  4. If an online user tries to add the exact same item, the Operations Agent intercepts and displays a friendly "Item just sold out in-store!" message.
  5. Payment processes via Stripe Terminal, committing the change to Postgres.
  6. A background card is added to the Agent Feed: "Red Dress sold out. Would you like to draft a restock order?"

  **AI agent integration points**:
  - **The Manager (Operations Agent)**: Monitors stock. Notifies customers of sync failures, reconciles conflicts, and prompts the owner for restock plans based on velocity.
  - **The Accountant (Finance Agent)**: Correlates POS data with online purchases for unified reporting.

  **Key design decisions and why**:
  - **Redis Redlock over pure DB locking**: Redis is used for fast reservations during checkout because it allows setting a TTL that automatically unlocks if a client disconnects or an online customer abandons their cart. This prevents stuck inventory locks.
  - **Eventual Consistency for offline sales**: Ensures the in-store POS continues to function even if mobile data drops out, preventing sales loss. Syncing occurs silently when network is restored.

  **Implementation Prompt**:
  Implement the Redis Redlock inventory reservation service and integrate it into the multi-channel checkout flow (both online and mobile POS). Extend the underlying data models to support this lock-and-commit pattern with strict `tenant_id` isolation. Extend the Operations Agent capabilities to handle lock failures (generating graceful out-of-stock messages) and to trigger restock notifications in the Agent Feed upon successful inventory depletion. Ensure all backend capabilities support optimistic UI updates for a 375px mobile client. The final acceptance criteria is a working CUJ where an in-store transaction securely reserves an item, preventing an online cart from double-booking. Do not prescribe specific database schemas, API contracts, or function signatures.

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
