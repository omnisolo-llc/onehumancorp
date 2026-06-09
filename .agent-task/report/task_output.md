issue_title: "[Research] Architect Offline-Tolerant Mobile Order Management for Food Cart Operators"
issue_description: |
  ## Title
  Offline-Tolerant Mobile Order Management for Food Cart Operators (Fatima Persona)

  ## Problem Statement
  Food cart operators, like Fatima, operate in fast-paced, high-stress environments often with spotty mobile data connections. They need a simple, reliable way to manage pre-orders, toggle item availability (sold-out status), and notify customers without relying on a constant, high-speed internet connection. Current solutions either fail completely offline or require complex, non-intuitive manual syncing.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square Point of Sale:** Offers an offline mode, but it's primarily focused on taking card payments offline (storing them to process later). It doesn't deeply integrate an AI assistant to manage order flow and customer notifications.
  - **Shopify POS:** Has basic offline capabilities but is heavy and requires a more traditional retail setup.
  - **OHC Opportunity:** Build a mobile-first, offline-tolerant order management system where the AI assistant intelligently caches expected daily orders, allows offline state changes (e.g., marking items sold out), and queues actions (like customer notifications) to sync when connectivity is restored, ensuring uninterrupted service for operators like Fatima.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] --> B{Local SQLite Database}
      A --> C[Offline Action Queue]
      B --> D[Sync Manager]
      C --> D
      D -->|Network Available| E[OHC API]
      E --> F[PostgreSQL]
      E --> G[Agent Event Mesh]
      G --> H[Operations Agent]
      H --> I[Customer Notification Dispatch]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Screen:** A clear, high-contrast list of pending pre-orders. A visible indicator shows online/offline status unobtrusively.
  - **Menu Management:** A simple list of menu items with large toggle switches for "Available / Sold Out."
  - **Interaction:** Fatima taps a toggle to mark an item "Sold Out" while offline. The app immediately updates the UI and saves the state locally.
  - **Sync:** When the network reconnects, the Sync Manager transparently uploads the state change, and the Operations Agent updates the public menu and potentially notifies customers who pre-ordered that item (if a substitution is needed).

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the sync queue. When an offline "Sold Out" event arrives, it updates the central inventory and triggers the Customer Success Agent if any active pre-orders are affected.

  ### Key Design Decisions
  - **Local-First Architecture:** The mobile app must read and write to a local database (SQLite) as the primary source of truth for the UI, ensuring zero latency and offline capability.
  - **Conflict Resolution:** Simple "last write wins" based on timestamps for menu item availability.
  - **Transparent Syncing:** The user shouldn't have to press a "sync" button. It should happen automatically in the background.

  ## Implementation Prompt
  **User-Facing Outcome:** As a food cart operator working in an area with bad cell reception, I can mark my daily special as "Sold Out" in the app. The app instantly updates for me. When I get back to a good connection, the system automatically updates my online menu without me doing anything extra.
  **CUJ & Acceptance Criteria:**
  1. Operator opens the app and loads the menu (online).
  2. Operator goes offline (network disabled).
  3. Operator toggles an item to "Sold Out". The UI updates immediately.
  4. The action is stored in the local offline queue.
  5. Operator comes back online (network enabled).
  6. The app automatically syncs the "Sold Out" state to the backend.
  7. Provide Playwright E2E tests: Simulate offline mode, perform the toggle action, verify local state, simulate online mode, and verify the backend state is updated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
