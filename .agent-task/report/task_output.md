issue_title: "OHC Unified Multi-Channel Inventory Sync & POS Implementation"
issue_description: |
  # Feature Name: OHC Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement
  Small business owners like Priya (boutique owner) require seamless inventory tracking between online (web/mobile) and in-store operations (tap-to-pay or card reader). Currently, OHC lacks a real-time, strongly consistent inventory locking and caching mechanism, as well as a robust distributed sync protocol for hybrid merchants. Without this, double-booking and out-of-stock scenarios occur during simultaneous online and offline purchases. The platform needs to support this natively so Priya doesn't have to bolt on third-party tools or manually coordinate between online and physical channels.

  ## Research Report
  Our competitive analysis shows that competitors like Shopify have massive app ecosystems, but managing online and offline inventory synchronization often relies on complex, expensive third-party apps for smaller merchants. OHC's unique advantage lies in providing this synchronization natively, backed by AI agents that actively monitor stock, prevent overselling, and proactively suggest restocking, all while staying invisible to the end user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      ClientOnline[Online Checkout] --> ReserveLock[Redis Redlock Inventory Reservation]
      ClientPOS[Stripe Terminal POS] --> ReserveLock
      ReserveLock --> CentralLedger[(PostgreSQL Central Ledger)]
      ClientPOSOffline[Offline POS Client] --> |Eventual Sync| CentralLedger
      CentralLedger --> OpsAgent[Operations Agent: Monitor & Alert]
      ReserveLock --> CustomerSuccessAgent[Customer Success Agent: Cart Adjustments]
  ```

  ### Mobile UX Flow
  1. Priya is logged into the OHC mobile app (POS mode) while an online customer browses her storefront.
  2. Priya processes an in-store sale for the last "Red Dress" using the Stripe Terminal integration (UI must be fully usable at 375px wide).
  3. The system transparently applies a 15-second Redis Redlock to reserve the item during the transaction.
  4. The online customer attempts to checkout the same "Red Dress" but gracefully receives an "Item just sold out" message, triggered by the Operations Agent.
  5. The POS transaction finalizes, updating the PostgreSQL ledger. The Operations Agent sends Priya a notification: "Red Dress sold out. Would you like to draft a restock order?"

  ### AI Agent Integration
  - **The Manager (Operations Agent):** Actively monitors stock levels across all channels. Tracks incoming orders, triggers low-stock alerts, and coordinates with the sync mechanism to reconcile conflicts and suggest restock plans.
  - **The Ambassador (Customer Success Agent):** Automatically updates online storefront availability and notifies customers if an item in their cart becomes unavailable due to an in-store purchase.
  - **The Accountant (Finance Agent):** Processes splits for Terminal transactions and correlates POS data with online purchases for unified financial reporting.

  ## Implementation Prompt
  Implement the "OHC Unified Multi-Channel Inventory Sync & POS" capability.

  1. Ensure the Redis Redlock inventory reservation service is fully integrated into both the online checkout flow and the Stripe Terminal tap-to-pay transaction flow, enforcing strict, real-time stock limits to prevent overselling.
  2. Refine the `TerminalSession` and offline POS sync data schemas to seamlessly handle eventual consistency and offline-sync reconciliation with the PostgreSQL central ledger when network connectivity is restored.
  3. Extend the Operations Agent to monitor real-time stock levels, handle inventory synchronization conflicts gracefully, and proactively trigger actionable low-stock push notifications to the owner.

  Your solution must include comprehensive unit and Playwright E2E tests verifying the end-to-end flow, explicitly ensuring that simultaneous POS and online purchases for the same final item correctly lock and gracefully handle the failure for the slower transaction without double-selling. Ensure the UI remains entirely mobile-first (375px target).

  ## Estimated Scope
  Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
