issue_title: "Architectural Design: Offline-Tolerant Mobile Tap-to-Pay POS"
issue_description: |
  ## Title: Architectural Design: Offline-Tolerant Mobile Tap-to-Pay POS

  ## Problem Statement
  For physical-presence owner/operators like **Fatima (Food Cart Operator)** and **Priya (Boutique Operator)**, checkout speed and reliability are business-critical. Fatima operates in areas with spotty mobile data, and Priya needs to seamlessly accept in-person payments without purchasing clunky proprietary hardware. Currently, OHC lacks a robust, offline-tolerant point-of-sale (POS) capability that natively supports "Tap to Pay" on smartphones. When network connectivity drops, owners cannot process transactions or update inventory, leading to lost sales and operational chaos. They need a 375px-optimized, mobile-first POS that works seamlessly offline and utilizes native device capabilities to accept payments, while the AI assistant handles inventory reconciliation and revenue tracking in the background.

  ## Research Report
  **Market & Competitor Analysis:**
  - **Square:** The incumbent in physical POS. Excellent offline mode and tap-to-pay support, but disconnected from a holistic AI-driven digital assistant.
  - **Shopify POS:** Offers offline cash transactions, but requires a massive, separate mobile app footprint. Card payments often stall during network degradation.
  - **Stripe Terminal:** Provides native "Tap to Pay on iPhone/Android" SDKs, eliminating the need for dongles.
  - **Wix/GoDaddy:** Basic mobile apps that heavily rely on constant internet connections; no true edge-synced offline capabilities.

  **OHC Capability Gap:**
  OHC needs an offline-first data synchronization layer (e.g., PowerSync + SQLite) paired with Stripe Terminal SDK to allow operators to process orders, queue payments securely, and sync inventory asynchronously. The platform must transition from simple online checkouts to a unified omni-channel edge architecture.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile Client 375px
          UI[Flutter POS UI]
          Terminal[Stripe Terminal SDK - Tap to Pay]
          LocalDB[(SQLite Local Cache)]
          SyncEngine[PowerSync Client]
      end

      subgraph OHC Cloud
          SyncGateway[PowerSync Service]
          Backend[OHC Go Backend]
          DB[(PostgreSQL Tenant DB)]
          JobQueue[AI Job Queue]
      end

      subgraph AI Agents
          FinanceAgent[Finance & Decision Agent]
          OpsAgent[Operations Agent]
      end

      UI --> Terminal
      UI --> LocalDB
      LocalDB <--> SyncEngine
      SyncEngine <-->|WebSocket/HTTP| SyncGateway
      SyncGateway <--> DB
      Backend <--> DB
      Backend <--> JobQueue
      JobQueue --> FinanceAgent
      JobQueue --> OpsAgent
  ```

  ### Mobile UX Flow (375px)
  1. **Quick Catalog View:** Large, high-contrast, touch-friendly grid (44x44px minimum touch targets) for Fatima to quickly tap menu items.
  2. **Offline Status Indicator:** A subtle translucent glass pill at the top indicating "Offline Mode - Queuing Orders" when network drops.
  3. **Checkout Action:** A persistent, sticky bottom sheet with the total amount. Tapping initiates the "Tap to Pay" modal via native OS integration.
  4. **Transaction Queue:** If offline, the app securely stores the transaction intent (for cash/deferred) or captures offline-approved micro-transactions, syncing automatically when connectivity is restored.

  ### AI Agent Integration Points
  - **Finance & Decision Assistant:** Monitors the edge-sync queue. Upon reconnection, it reconciles the batch of transactions, producing a daily "Tap-to-Pay Volume vs. Online" plain-language summary for Priya.
  - **Operations Assistant:** Automatically adjusts inventory counts post-sync. If Fatima's food cart sells out of an item offline, the Ops Agent immediately toggles the item to "Sold Out" on the digital storefront upon reconnection, preventing double-selling.

  ## Implementation Prompt
  **Target Persona:** Fatima (Food Cart Operator), Priya (Boutique Operator).
  **Outcome:** The owner can open the OHC app, tap products to build a cart, and accept a customer's contactless payment directly on their phone using Tap to Pay, even in low-data environments.
  **CUJ:**
  1. Open OHC Mobile App on a 375px viewport.
  2. Navigate to the "Point of Sale" tab.
  3. Add an item to the cart and proceed to checkout.
  4. Initiate Tap to Pay (mocked for testing).
  5. Simulate an offline state: the order queues successfully.
  6. Reconnect to network: the order syncs, inventory updates, and the Finance Agent logs the revenue.

  **Acceptance Criteria:**
  - Implement the POS Cart UI using OHC Premium Tokens (translucent materials, large touch targets).
  - Integrate a local SQLite queue to store offline cart checkouts.
  - Ensure 100% unit test coverage for the offline queue logic.
  - Write Playwright E2E tests validating the cart assembly, offline queuing, and online synchronization flow without any mock data directly in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
