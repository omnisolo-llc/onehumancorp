issue_title: "Implement Inventory and POS Synchronization"
issue_description: |
  # Research Report: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. Priya, our boutique owner persona, requires seamless inventory tracking between her online storefront and in-store operations.

  ## Research Report
  Our research into competitor platforms like Shopify reveals strong POS capabilities but high complexity for micro-SMEs. Their inventory often falls out of sync between online and offline unless costly integrations are used. We propose a centralized inventory and distributed Point-of-Sale (POS) synchronization architecture tailored for OHC's AI agents to provide a zero-configuration, seamless experience.

  ## Design Doc
  - **Architecture:**
    - A Central Ledger in PostgreSQL will act as the single source of truth for inventory counts.
    - Redis Redlock will manage distributed locks to prevent double-booking during checkouts (e.g., 5 mins for online carts, 15s for rapid tap-to-pay).
    - An Offline/Local First POS Client will cache catalog data and use eventual consistency to sync offline sales when network connectivity is restored.
  - **AI Agent Integration:**
    - The Operations Agent ("The Manager") will monitor stock levels, trigger low-stock alerts, and suggest restocks.
    - The Finance Agent ("The Accountant") will process POS transactions and correlate them with online sales.
    - The Customer Success Agent ("The Ambassador") will update online storefront availability automatically.
  - **Mobile UX Flow:** Ensure 375px viewport compatibility. Use optimistic UI updates for inventory adjustments and checkouts with rollback support.

  ## Implementation Prompt
  Implement the backend synchronization protocol for the inventory and distributed POS system. Create the necessary PostgreSQL tables for the Central Ledger, integrate Redis Redlock for inventory reservations, and establish the API endpoints for the Offline/Local First POS Client to sync sales data. Ensure the Operations, Finance, and Customer Success AI agents are integrated to monitor and respond to inventory changes automatically. Validate the user journey for Priya, ensuring she can manage her boutique's inventory seamlessly across online and offline channels.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
