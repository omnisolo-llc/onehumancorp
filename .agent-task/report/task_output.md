issue_title: "Implement Offline-Tolerant Pre-Order & Pickup System for Low-Bandwidth Environments"
issue_description: |
  ## Title
  Implement Offline-Tolerant Pre-Order & Pickup System for Low-Bandwidth Environments

  ## Problem Statement
  For food and beverage operators like Fatima (food cart operator), consistent internet connectivity is rarely guaranteed. Traditional e-commerce and POS systems require constant network access to update menus, track inventory, or process orders. Fatima needs a pre-order and pickup flow that works even on a slow connection, provides offline-tolerant read paths (like sold-out toggles and order lists), handles multiple languages (Arabic + English) seamlessly, and alerts her immediately on a low-end Android device when a new order comes in.

  ## Research Report
  Our analysis of existing platforms shows a significant gap for micro-food vendors operating in low-bandwidth or offline environments:
  - **Shopify**: Requires continuous connectivity for its POS and admin interfaces. Offline mode is limited and complex to set up. Multilingual support requires third-party apps, adding cost and load time.
  - **Square Online / Weebly**: Good for restaurants but struggles with highly intermittent connections. Mobile experiences can be heavy for low-end devices.
  - **Wix / Squarespace**: Not optimized for low-end devices or offline-first operations. Setup is desktop-heavy and not conducive to quick toggles from a mobile phone.
  - **GoDaddy**: Very limited capabilities for real-time inventory toggling or offline queue management.
  - **Industry Data**: Non-technical food vendors experience high abandonment rates if they miss orders due to network drops. Progressive Web Apps (PWAs) with local caching and optimistic UI updates are industry standards for these environments (e.g., modern offline-first React/Flutter architectures).

  ## Design Doc
  ### Mobile UX Flow (375px)
  1. **Menu View**: A highly compressed, lazy-loaded photo menu. Users can select items and set a pickup time.
  2. **Vendor Dashboard**: A minimalist, high-contrast order list displaying pending and completed orders.
  3. **Offline Actions**: Fatima can toggle an item as "Sold Out" even without an active connection. The UI optimistically updates and queues the mutation to sync once the network is restored.
  4. **Multi-language**: A simple tap switches the UI between English and Arabic without reloading the page.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the sync queue. If an order comes in while the device is offline, the agent queues the notification and sends a resilient SMS/WhatsApp alert as a fallback. It also auto-translates customer special requests into Fatima's preferred language.
  - **Customer Success Agent**: Drafts automated pickup reminders to customers and manages expectations if there is a known delay.

  ### Key Design Decisions
  - **Offline-First Storage**: Utilize local storage (e.g., IndexedDB or local SQLite) for the active daily order list and menu state, ensuring zero-latency access.
  - **Optimistic UI Updates**: All critical actions (sold out toggles, order completion) apply locally first, then sync in the background.
  - **Compressed Assets**: Images are compressed to WebP and cached aggressively to reduce data payload for low-end mobile users.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant C as Customer (Web)
      participant OHC as OHC Server
      participant Agent as Operations Agent
      participant F as Fatima (Mobile POS)

      C->>OHC: Places pre-order & sets pickup time
      OHC->>OHC: Save order to DB
      OHC->>Agent: Trigger 'New Order' Event
      Agent->>Agent: Translate order notes to Arabic
      Agent-->>F: Push Notification / SMS Fallback
      Note over F,OHC: Connection Lost
      F->>F: Toggle "Sold Out" locally (Optimistic UI)
      Note over F,OHC: Connection Restored
      F->>OHC: Sync "Sold Out" mutation queue
      OHC-->>C: Update public storefront availability
  ```

  ## Implementation Prompt
  **Target Persona**: Fatima the Food Cart Operator

  **Outcome**: Fatima can manage her daily menu, toggle items as sold out offline, and view her daily order list on a low-end Android phone without worrying about network drops. Customer orders are automatically translated to her preferred language.

  **Critical User Journey (CUJ)**:
  1. Fatima logs into the OHC mobile app (375px view) at the start of her day to view the pre-order list.
  2. The network drops. Fatima notices she is out of chicken. She taps "Sold Out" on the Chicken Platter item.
  3. The UI immediately reflects the change locally without a loading spinner.
  4. The network reconnects 10 minutes later. The app silently syncs the "Sold Out" state to the server.
  5. A customer places an order with a special note in English.
  6. Fatima receives a notification, and the order appears on her list with the note auto-translated to Arabic by the Operations Agent.

  **Acceptance Criteria**:
  - The "Sold Out" toggle must work instantly in offline mode and sync when online.
  - The layout must fit comfortably on a 375px viewport with large touch targets (44x44px minimum).
  - Implementation must include E2E Playwright tests simulating offline network conditions and verifying the optimistic UI sync behavior.
  - Do NOT prescribe specific database schemas, API endpoints, or function signatures.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
