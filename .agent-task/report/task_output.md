issue_title: "Implement Offline-First Sync using IndexedDB/PWA Service Workers"
issue_description: |
  # Research Report: Offline-First Reliability for Field Operators

  ## Problem Statement
  Field operators, like Carlos (the handyman) and Fatima (the food cart operator), frequently work in environments with poor or non-existent mobile data connectivity (e.g., inside customer homes, basements, or crowded street fairs). The current web-first implementation often leaves them stranded if a network drop occurs during critical actions like updating a service quote, checking inventory, or saving a booking.

  They need OHC to operate seamlessly offline, buffering their updates and instantly syncing them when connectivity returns, ensuring they never lose work or customer momentum due to spotty data.

  ## Research Report & Findings
  An audit of modern SMB tools (e.g., Square POS, Toast, Shopify Point of Sale) reveals that offline-first capabilities are non-negotiable for mobile operators. These platforms heavily leverage background sync to queue transactions.

  OHC currently lacks a robust PWA service worker with local state caching. By introducing IndexedDB via RxDB or a similar lightweight wrapper, combined with PWA service worker background sync, OHC can queue API writes (like booking creations or quote updates) locally and push them automatically upon network recovery.

  ## Architecture & Design Doc
  ### Proposed Architecture
  1. **Frontend PWA Layer:** Register a Service Worker with Workbox that intercepts critical API calls.
  2. **Local Storage (IndexedDB):** Create an offline write queue in the browser's IndexedDB.
  3. **Sync Engine (Frontend):** A singleton sync manager that listens to the `online` window event and PWA background sync events.
  4. **Backend Validation:** The backend endpoints must support idempotency (using unique client-generated UUIDs) to prevent duplicate processing when the sync manager retries buffered requests.

  ### Mobile UX Flow
  - 375px Viewport: A subtle but clear "Offline Mode" status pill appears in the top header.
  - When a user performs an action (e.g., "Save Quote"), the UI immediately reflects success and shows an "Updating in background" toast, never blocking the UI.
  - Upon reconnection, a subtle "Changes Synced" notification flashes, updating the local state seamlessly.

  ### AI Agent Integration
  - Agents attempting to fetch context from offline-created records must seamlessly read from the local IndexedDB projection if the record hasn't yet reached the remote database.

  ## Implementation Prompt
  Implement an offline-first sync architecture for the OHC frontend PWA.
  1. Add a Service Worker that registers a background sync queue for critical API mutations.
  2. Implement a local IndexedDB buffer (e.g., using `idb`) to store pending requests when `navigator.onLine` is false.
  3. Update the UI to show an "Offline Mode - changes saved locally" indicator when disconnected.
  4. Ensure backend mutations for at least one critical path (e.g., Quote creation/updating) process idempotency keys safely to handle delayed duplicate syncs.
  5. Provide end-to-end Playwright tests simulating offline states to prove the quote saves locally and syncs upon reconnection.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
