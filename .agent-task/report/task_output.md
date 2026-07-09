issue_title: "[research] Autonomous Agentic Kitchen Display System (KDS) & Offline-Tolerant Operations Hub"
issue_description: |
  ## Title
  Autonomous Agentic Kitchen Display System (KDS) & Offline-Tolerant Operations Hub

  ## Problem Statement
  Food operators and mobile merchants (like Fatima the Food Cart Operator) struggle with high-stress, fast-paced environments where internet connectivity is frequently unreliable. Traditional tablet Point-of-Sale (POS) and Kitchen Display Systems (KDS) fail gracefully during network drops, causing missed orders, double-booking, and chaotic fulfillment. Furthermore, these systems require manual tapping to update order status, which slows down service. The gap is an offline-tolerant, multilingual, agentic KDS that allows the owner to focus on fulfillment while the AI handles order triage, customer notifications, and background syncing.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Traditional POS/KDS (Square KDS, Toast):** Highly reliable on stable networks but suffer significantly in offline mode. Often require complex setup and are rigid in their workflow, demanding constant manual interaction. They lack intelligent language translation and proactive order management.
  - **Mobile-First Builders (Shopify POS, GoDaddy):** Provide basic POS features but lack dedicated, robust KDS interfaces tailored for food/beverage or high-volume pickup environments, especially offline.
  - **OHC Opportunity:** Implement an "Offline-First" KDS using local caching and eventual consistency, paired with "The Operations Agent". The system can function entirely offline, queuing state changes locally. When reconnected, it syncs intelligently. Crucially, the AI agent can auto-translate incoming orders to the operator's native language (e.g., Arabic) and auto-reply to customers about delays.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web App] -->|Order Placed| B(API Gateway)
      B --> C[Postgres Central Ledger]
      B --> D[Event Mesh]
      D --> E[The Operations Agent]
      C -->|Sync| F[Local Offline-Tolerant Data Store / SQLite]
      E -->|Translation & Routing| F
      F --> G[Mobile KDS UI / Tablet]
      G -->|Order Ready| F
      F -->|Background Sync| B
      E -->|SMS Notification| H[Customer]
  ```

  ### Mobile UX Flow (375px First & Tablet Adaptation)
  - **Home Screen (KDS Feed):** A high-contrast, large-typography queue of incoming orders. Critical information only: Order #, Items, Customer Name, Time Elapsed.
  - **Interaction:** Swipe right on an order card to mark "Ready", swipe left to trigger an "Agent Assist" (e.g., "Tell customer we are 10 mins late"). Touch targets are massive (≥ 60x60px) to accommodate fast, imprecise taps in a busy environment.
  - **Offline Indicator:** A subtle but clear top-bar indicator showing offline status and the number of queued sync actions.
  - **Visual Design:** UniFi/Apple-style clean hierarchy. Minimalist Glassmorphism used sparingly; prioritizing readability and contrast over complex styling.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Monitors the queue. If it detects a surge in orders, it can automatically toggle "Sold Out" or increase the estimated pickup time on the customer-facing storefront to prevent overload.
  - **Customer Success Agent (The Ambassador):** Translates incoming special requests into the operator's native language. When an order is marked ready, it drafts and sends the pickup notification to the customer.

  ### Key Design Decisions
  - **Local-First Architecture:** The KDS UI reads exclusively from a local data store (e.g., IndexedDB or SQLite in Tauri/Flutter). Writes are appended locally and background-synced to the server to guarantee sub-millisecond UI responsiveness and total offline tolerance.
  - **Agentic Triage:** The system shouldn't just display orders; it should manage the flow. If the operator is overwhelmed, the AI steps in to manage customer expectations.
  - **Multilingual Native Experience:** Zero configuration translation.

  ## Implementation Prompt
  **User-Facing Outcome:** As Fatima the food cart owner, I am serving customers in a busy park with a weak 3G connection. Orders placed online appear on my tablet in Arabic. I simply swipe to mark them ready. Even if my tablet loses connection, I can keep managing the queue, and the system automatically sends SMS pickup alerts to customers once the connection is restored, without me missing a beat.

  **CUJ & Acceptance Criteria:**
  1. A customer places an order via the simulated online storefront.
  2. The system translates any special notes to the operator's designated language (Arabic) using the AI Agent.
  3. The KDS UI (operating in simulated offline mode) displays the new order via local cache.
  4. The operator swipes the order card to mark it "Ready". The local state updates instantly.
  5. Network connection is restored. The system successfully background-syncs the "Ready" state to the central PostgreSQL ledger.
  6. The Customer Success Agent dispatches the "Order Ready for Pickup" notification to the customer.
  7. Provide Playwright E2E tests validating the offline-to-online sync flow, swipe interactions, and final customer notification state.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
