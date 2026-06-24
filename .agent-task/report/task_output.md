issue_title: "Offline-Tolerant Mobile POS & Agentic Background Synchronization"
issue_description: |
  ## Title
  Offline-Tolerant Mobile POS & Agentic Background Synchronization

  ## Problem Statement
  Mobile and field service owners (like Fatima with her food cart, or Carlos out on repair jobs) frequently operate in areas with spotty, slow, or nonexistent cellular data. When their connection drops, traditional SaaS platforms lock up, fail to load menus/services, and prevent the collection of orders or cash payments. If Fatima cannot toggle "sold out" offline or view her upcoming pickups when her data drops, her operation grinds to a halt. OHC needs a robust offline-first architecture that allows owners to seamlessly continue their operations without internet, using background agents to gracefully resync state once the connection returns.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square:** Offers robust "Offline Mode" for payments (queuing transactions for up to 24 hours), which is a key reason they dominate physical small businesses. However, their offline mode is mostly for payments, not complex inventory or agentic management.
  - **Shopify POS:** Has basic offline capabilities for cash transactions and viewing local inventory, but requires an active connection for full operations and syncing.
  - **Legacy web-based tools:** Completely fail when offline (browser connection errors).
  - **OHC Opportunity:** By utilizing an offline-first local database (e.g., SQLite via Flutter/PWA local storage) and background synchronization agents, OHC can guarantee that an owner can always see their menu, accept cash orders, queue card payments, and update local state. Once online, the "Sync Agent" handles the complex merge resolution against the central PostgreSQL database automatically.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Flutter Mobile App - 375px] --> B{Network State Monitor}
      B -->|Online| C[API Gateway]
      B -->|Offline| D[Local SQLite / Hive Cache]
      A --> D
      D --> E[Background Sync Queue]
      E -.->|Connection Restored| F[Data Synchronization Agent]
      F --> C
      C --> G[PostgreSQL & Tenant DB]
      F -->|Conflict Detection| H[Operations Agent]
      H -->|Resolve / Notify| I[Action Card to Unified Feed]
  ```

  ### UI Wireframes & Mobile UX Flow
  1. **Network Indicator:** A subtle translucent glass pill at the top of the 375px screen showing "Offline Mode - Changes saved locally" when connection drops.
  2. **Menu / Service Flow:** The user can continue to browse their locally-cached catalog, mark items as "sold out", and accept cash orders. Touch targets remain > 44px.
  3. **Queue Visibility:** A small indicator shows "3 orders queued for sync".
  4. **Connection Restored:** When online, a background progress bar briefly appears. If a conflict occurs (e.g., an item marked sold out locally was updated online by another device), the Operations Agent pushes an Action Card to the feed: "Merge conflict on Vegan Cake inventory. Use local or online version?"

  ### AI Agent Integration Points
  - **Data Synchronization Agent:** Invisibly manages the sync queue in the background. It structures the local mutations and pushes them via idempotent API calls.
  - **Operations Agent:** Monitors the sync process. If there are conflicting inventory states between the local device and the server, it prevents data loss by halting the merge and generating a clear, non-technical choice for the owner in the Agent Feed.

  ### Key Design Decisions
  - **Local-First Reads/Writes:** The UI should primarily read from and write to the local cache, making the app feel incredibly fast. The local cache then syncs with the server.
  - **Idempotent Sync:** All queued actions must include UUIDs and idempotency keys to prevent double-charging or double-counting orders if the network flickers during sync.
  - **No Technical Jargon:** The user should never see terms like "SQLite", "Merge Conflict", or "Idempotency". They only see "Offline Mode" and "Saved".

  ## Implementation Prompt
  **User-Facing Outcome:** The owner can put their phone in airplane mode, open the OHC app, see their active menu/services, record three cash transactions, toggle a menu item to "sold out", and turn airplane mode off. The app should smoothly sync the transactions and inventory change to the backend without blocking the UI or losing data.

  **Critical User Journey (CUJ):**
  1. Login to OHC.
  2. Disconnect internet (Airplane mode).
  3. UI reflects "Offline Mode" clearly.
  4. User records a new cash order and marks a product as sold out.
  5. Reconnect internet.
  6. The app syncs data in the background and updates the central server state, reflecting the new order and inventory status across other devices.

  **Acceptance Criteria:**
  - Establish local database (e.g. drift/sqlite or PWA IndexedDB).
  - Implement a queue for offline mutations.
  - Create the background sync agent logic.
  - Ensure UI components clearly indicate offline status without intrusive modals.
  - Ensure all layout adjustments fit within the 375px mobile constraint using OHC Premium Tokens.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
