issue_title: "Implement Real-time Multi-Channel Inventory Sync & POS Capabilities"
issue_description: |
  **Issue Title:** Implement Real-time Multi-Channel Inventory Sync & POS Capabilities

  **Problem Statement:**
  Small business owners who sell both online and in-person (like Priya, the boutique operator) face significant pain points with inventory synchronization. They often experience double-booking (selling out an item online that was just sold in-store) and fragmented customer data because existing platforms (Shopify POS, Wix, Square) either require expensive third-party integrations, lack real-time synchronization capabilities, or operate passively. OHC must provide a seamless, real-time inventory locking and caching mechanism, alongside a robust distributed sync protocol for hybrid merchants.

  **Research Report:**
  Our research into the competitive landscape reveals that platforms like Shopify require third-party apps for robust inventory syncing across channels, while Square and Stripe Terminal provide robust POS hardware but lack the integrated, agentic workflow automation needed to unify business operations effortlessly. The gap in the market is an "invisible AI automation" system that proactively manages inventory conflicts and restock alerts.
  - **Shopify:** Complex setup, often requires third-party plugins for true omnichannel real-time sync.
  - **Square:** Great for in-person, but e-commerce integration can be fragmented.
  - **Wix/Squarespace:** Lack robust, real-time POS inventory locking.

  **Design Doc:**
  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  graph TD
      A[Mobile POS Client (375px)] -->|Sync offline sales| B(Distributed Sync Gateway)
      C[Online Storefront] -->|Checkout request| D{Redis Redlock}
      B --> D
      D -->|15-sec lock for POS, 5-min for online| E[PostgreSQL Central Ledger]
      E --> F(Operations Agent)
      F -->|Low stock alerts/Restock drafts| A
  ```

  **Mobile UX Flow (375px):**
  1.  Priya opens the OHC mobile app (POS mode).
  2.  She processes an in-store sale using Stripe Terminal integration.
  3.  A Redis Redlock temporarily reserves the item (e.g., 15 seconds) to prevent simultaneous online sales.
  4.  The system uses optimistic UI updates to instantly show reduced stock.
  5.  If an online customer tries to buy the same item during the lock, they receive a graceful "Item just sold out" message.
  6.  The Operations Agent sends Priya a notification (Action Card): "Red Dress sold out. Would you like to draft a restock order?" with "Approve" or "Dismiss" buttons.

  **AI Agent Integration Points:**
  -   **Operations Agent:** Actively monitors the PostgreSQL Central Ledger. Triggers low-stock alerts, resolves sync conflicts (using eventual consistency strategies if offline), and proposes restock orders via the Agent Feed.
  -   **Customer Success Agent:** Notifies online customers if an item in their cart becomes unavailable due to an in-store purchase.

  **Implementation Prompt:**
  As an implementer, your task is to build the centralized inventory and distributed Point-of-Sale (POS) synchronization architecture for OHC. You must implement the following user journey:
  1.  A merchant (Priya) processes an in-store sale via the mobile POS interface.
  2.  The system applies a distributed lock (Redis Redlock) during the transaction to prevent double-booking from the online storefront.
  3.  The central inventory ledger (PostgreSQL) is updated as the single source of truth.
  4.  The Operations Agent detects the stock change and, if stock is low/depleted, drafts a restock order and sends a push notification to the merchant for approval.

  You must ensure the UI is fully functional on a 375px wide screen with 44x44px touch targets. You must write Playwright E2E tests validating the locking mechanism and the AI agent's notification flow. Do not prescribe specific database schema details; design the entities (Ledger, Lock, etc.) to support this flow securely and multi-tenant isolated.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
