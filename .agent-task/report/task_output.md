issue_title: "Implement Multi-Channel POS Architecture"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya (boutique owner) requires seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases.

  ## Competitive Analysis
  - **Shopify/Shopify POS:** Offers strong multi-channel inventory syncing but is highly complex and typically requires expensive apps or higher-tier plans for micro-SMEs to fully utilize. The setup paralysis is significant for non-technical users like Priya.
  - **Square:** Provides robust POS hardware and software, but lacks integrated, agentic workflow automation that can dynamically respond to inventory changes across channels, forcing owners to manually intervene when stock discrepancies occur.
  - **OHC Differentiation:** OHC will provide an "Invisible AI Automation" layer. We will use a distributed lock (Redis Redlock) for immediate inventory reservation during checkout and eventual consistency for offline sales, all managed by the Operations Agent.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      A[Mobile POS Client] -->|Eventual Consistency Sync| B(Central Ledger PostgreSQL);
      A -->|Reserve Lock| C(Redis Redlock);
      D[Online Customer Cart] -->|Check Availability| B;
      D -->|Reserve Lock| C;
      E[Operations Agent] -->|Monitor| B;
      E -->|Resolve Conflicts| C;
  ```

  ### Mobile UX Flow & UI Wireframes
  - **375px Viewport:** Ensure the POS interface operates flawlessly on a 375px viewport. Touch targets for inventory adjustment and checkout must be >= 44x44px.
  - **Flow:**
    1. Priya opens the POS view on her mobile app.
    2. She selects an item to sell.
    3. The system checks availability and applies a temporary Redis lock.
    4. If successful, the item is added to the cart; if not, an error message is displayed.
    5. Upon successful payment (e.g., tap-to-pay), the lock is released, and the central ledger is updated.
  - **Optimistic Updates:** Implement optimistic UI updates for inventory changes, with rollback capabilities if the Redis reservation fails.
  - **Empty States:** Provide clear empty states and loading indicators when the system is syncing.

  ### AI Agent Integration
  - **Operations Agent:** Monitors stock levels, handles sync conflicts, and triggers low-stock alerts and restock plans.
  - **Finance Agent:** Processes splits for Terminal transactions and correlates POS data.
  - **Customer Success Agent:** Updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.

  ### Key Design Decisions
  - **Redis Redlock:** Chosen for its distributed nature and high performance, making it suitable for short-lived, high-frequency reservations during checkout.
  - **Eventual Consistency for Offline Sync:** Ensures the POS remains functional even when internet connectivity drops, synchronizing with the central ledger once reconnected.

  ## Implementation Prompt
  Implement the OHC Unified Multi-Channel Inventory Sync & POS system. Follow the design outlined to integrate Redis Redlock for inventory reservation and build the backend and mobile-first interface for the Operations Agent to handle stock levels and conflicts. Ensure strong consistency for real-time inventory locking across all channels.

  ## Priority
  P1

  ## Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
