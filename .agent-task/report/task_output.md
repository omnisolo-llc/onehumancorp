issue_title: "AI-Powered Autonomous Yield Management & Pre-Order System"
issue_description: |
  # Research Report: AI-Powered Autonomous Yield Management & Pre-Order System

  ## Problem Statement
  Food cart operators and high-volume, low-SKU vendors (like Fatima the Food Cart Operator) struggle with dynamic inventory, pre-order management, and real-time communication. They face sudden demand spikes, unpredictable supply (e.g., selling out of a popular item halfway through lunch), and language barriers with customers. Existing point-of-sale systems (like Square) manage payments well but are passive; they don't predict demand, dynamically throttle incoming pre-orders based on remaining capacity, or proactively notify customers when a highly requested item is restocked. This leads to frustrated customers waiting in long lines and lost revenue for the operator.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square POS:** Excellent for in-person transactions and basic online ordering. However, it requires the operator to manually update "sold out" statuses. During a rush, operators cannot update their online menu fast enough, leading to pre-orders for items they no longer have.
  - **Toast:** Robust for traditional restaurants but overkill, too expensive, and too complex for a single-operator food cart.
  - **Wix/Shopify:** E-commerce platforms not optimized for hyperlocal, immediate fulfillment. They lack offline-tolerant workflows necessary for areas with spotty mobile data (like street corners).
  - **OHC Opportunity:** By introducing an AI-Powered Yield Management system, OHC can predict when an item will sell out based on current trajectory, autonomously throttle new pre-orders for that item, and seamlessly integrate translation for the operator and customer.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Pre-Order Web App] -->|Order Request| B(Edge API Gateway)
      B --> C{Yield Management Engine}
      C -->|Check Real-time Capacity| D[Central Inventory Ledger PostgreSQL]
      C -->|Approve/Throttle| E[Order Queue]
      E --> F[Operations Agent The Manager]
      F -->|Update Local Sync| G[Local First POS Client Fatima's Phone]
      F -->|Translate & Notify| H[Customer Success Agent The Ambassador]
      H --> I[Customer SMS/WhatsApp]
      G -->|Finalize Sale| D
  ```

  ### Mobile UX Flow (375px First)
  - **Customer View:** A hyper-optimized, low-bandwidth web menu (PWA). If an item is selling fast, a dynamic tag ("Selling fast! Only 5 left") appears. If sold out, it greys out immediately without requiring a page refresh.
  - **Operator View (Fatima's Dashboard):** A high-contrast, offline-tolerant interface. Focuses on the "Next 10 Orders" list. Large, distinct buttons for "Mark Ready" and "Delay 5 mins". The interface defaults to her preferred language (e.g., Arabic), while customer interactions are handled in their respective languages via The Ambassador.
  - **AI Integration Point:** The Operations Agent monitors the velocity of sales in the `Central Inventory Ledger`. If velocity indicates an item will sell out within 15 minutes, it automatically applies a temporary throttle to the pre-order system and alerts Fatima via a non-intrusive push notification.

  ### Key Design Decisions
  - **Offline-First Synchronization:** The POS client on the operator's phone MUST be fully functional without an internet connection, queuing state changes and syncing via eventual consistency when connectivity is restored.
  - **Predictive Throttling:** Shifting from reactive "out of stock" to predictive yield management to prevent overselling during rushes.
  - **Agentic Translation:** The Customer Success Agent transparently translates inbound requests (e.g., custom dietary notes) into the operator's native language, and translates the operator's status updates back to the customer.

  ## Implementation Prompt
  **User-Facing Outcome:** As Fatima, I can set my daily starting inventory. During the lunch rush, the OHC system automatically notices I'm selling out of chicken faster than expected, updates my online pre-order page to show low stock, and eventually marks it "sold out" without me ever touching my phone. I receive orders translated into Arabic, and my customers get English updates.

  **CUJ & Acceptance Criteria:**
  1. Operator sets initial inventory for "Chicken Platter" (e.g., 50 units) in the local POS client.
  2. A simulated burst of 45 pre-orders and in-person sales occurs over a short period.
  3. The Yield Management Engine detects the high velocity and automatically updates the Edge API to reflect "Low Stock" and applies rate limiting to new pre-orders.
  4. Once inventory reaches 0, the system automatically rejects further pre-orders and updates the PWA menu.
  5. The Ambassador agent successfully translates a customer's order note from English to Arabic for the operator's view.
  6. E2E Playwright test verifies the local-first POS functionality: operator can mark an order "ready" while offline, and the system syncs to PostgreSQL when the connection is restored.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []