issue_title: "Implement AI-Driven Offline-Tolerant Mobile Menu & Pre-Order System"
issue_description: |
  ## Problem Statement
  Fatima, a food cart operator with limited English proficiency and slow mobile data, struggles to manage daily menu availability and handle pre-orders. Legacy tools assume strong connectivity and desktop-based complex menu builders. When Fatima's connection drops during a busy lunch rush, she cannot toggle sold-out items, leading to frustrated customers placing orders for unavailable food.

  ## Research Report
  Our audit of competitor POS systems (Square, Toast) reveals they heavily depend on continuous cloud syncing for inventory toggles. While Square provides offline mode for card processing, offline menu management (e.g., toggling an item out of stock locally and having it auto-sync to the online pre-order page once connectivity resumes) is poorly handled. Link-in-bio tools entirely lack robust localized offline sync capabilities. OHC requires a local-first state architecture using PWA paradigms (Service Workers, IndexedDB) to queue operations when offline, ensuring non-technical operators never lose data or context.

  ## Design Doc
  **Architecture Diagram**
  ```mermaid
  graph TD;
    Client[Mobile PWA - 375px] -->|Offline Read/Write| LocalCache[IndexedDB + Service Worker]
    LocalCache -->|Background Sync API| API[OHC Edge API]
    API -->|Queue| Jobs[PostgreSQL SKIP LOCKED Queue]
    Jobs --> DB[Central Ledger PostgreSQL]
    Jobs --> Agents[Operations Agent]
    Agents -->|Push Alert| Client
  ```

  **Mobile UX Flow**
  - **Screen 1 (Home - 375px):** Today's Active Menu. Large, high-contrast toggle buttons (44x44px minimum) for "Available" vs "Sold Out". Translucent glass materials used for readability.
  - **Offline State:** A subtle top banner indicates "Offline - Changes saved locally". User can still toggle items.
  - **Reconnection:** System silently syncs. A toast confirms "Menu updated online".

  **AI Agent Integration Points**
  - **Operations Agent:** Monitors the background sync queue. If a pre-order arrives during the offline window for a newly sold-out item, the agent intercepts the order, drafts an apologetic multilingual SMS via the Ambassador Agent, and refunds the deposit automatically.

  **Key Design Decisions**
  - **Optimistic UI:** Menu toggles reflect instantly in the UI without waiting for network response.
  - **Local-First:** All menu reads load from IndexedDB.
  - **Multilingual Support:** UI strings are pre-cached in English and Arabic to support Fatima's workflow.

  ## Implementation Prompt
  **Objective:** Implement the offline-tolerant menu management screen for food cart operators, backed by a local-first synchronization queue.

  **CUJ:**
  1. Fatima opens the app on a 375px Android device (simulated offline).
  2. She toggles "Chicken Shawarma" to "Sold Out". The UI instantly updates and shows an "Offline, saved" indicator.
  3. The network is restored. The app automatically syncs the state.
  4. The Operations Agent verifies no conflicting pre-orders were placed during the offline window.

  **Acceptance Criteria:**
  - Build a responsive 375px UI for menu toggling using OHC Premium Tokens (translucent materials, clean hierarchy).
  - Implement IndexedDB/Service Worker caching so the screen loads and accepts writes without a network.
  - On reconnection, queue items must sync to the backend.
  - Unit and Playwright E2E tests must verify offline-to-online state transitions and agent conflict resolution on a simulated 375px viewport. No mock data in UI code.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
