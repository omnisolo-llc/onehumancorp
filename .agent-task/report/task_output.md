issue_title: "AI-Powered Pre-Order & Multi-Lingual Real-Time Queue Management for Low-Bandwidth POS"
issue_description: |
  # Mission Queue Protocol: AI-Powered Pre-Order & Queue Management

  ## Problem Statement
  Food cart operators and street vendors (like Fatima) face rapid, concentrated demand spikes in environments with slow or flaky mobile data. They struggle to manage a mix of walk-up orders and pre-orders. Existing POS/e-commerce platforms require reliable internet connections and complex interfaces that are overwhelming for non-native English speakers. They need a system that effortlessly toggles item availability, manages pre-orders, notifies customers for pickup, and provides a daily printable order list, all while functioning seamlessly on a low-end Android device with intermittent connectivity.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Square POS:** Excellent for simple walk-up transactions, but offline mode is limited primarily to taking card payments; it lacks robust, offline-tolerant real-time queue coordination and multi-lingual AI assistance.
  - **Shopify POS:** Too heavy, requires reliable internet for inventory sync, and complex to configure for a simple food cart menu.
  - **WhatsApp Business:** Great for customer communication but lacks integrated menu management, payment processing, and queue tracking.
  - **OHC Opportunity:** Combine a truly offline-first, low-bandwidth mobile POS with an AI Operations Agent ("The Coordinator") that automatically handles customer pickup notifications via SMS/WhatsApp in the customer's preferred language. The system must support Arabic and English natively and handle "sold-out" toggles instantly even when offline, syncing via an event mesh when connectivity returns.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Web/WhatsApp] -->|Pre-Order & Pay| B(OHC Gateway)
      B --> C[PostgreSQL Ledger]
      B --> D[Event Mesh]
      D --> E{Operations Agent - The Coordinator}
      E -->|Translate & Notify| F[SMS/WhatsApp Twilio]
      D -->|Low-Bandwidth Sync| G[Mobile Client - Fatima]
      G -->|Local SQLite Cache| H[Offline Toggle Sold-Out]
      H -->|Sync on Reconnect| B
  ```

  ### Mobile UX Flow (375px First)
  - **Active Queue View:** A high-contrast, large-touch-target (min 44x44px) list of active orders. Color-coded for "Prep" and "Ready".
  - **Interaction:** Swiping an order right marks it "Ready" and triggers the Operations Agent to send a localized pickup SMS to the customer.
  - **Menu Toggle:** A simple "Menu" tab with large toggle switches next to photos to mark items "Sold Out". Toggles work instantly via local SQLite cache and sync optimistically.
  - **Printable Summary:** A one-tap button to generate a minimal, text-only daily order summary for offline use or Bluetooth printing.
  - **Localization:** Entire UI must be dynamically switchable between English and Arabic (RTL support) with zero latency.

  ### AI Agent Integration Points
  - **Operations Agent (The Coordinator):** Monitors the event mesh for "Order Ready" state changes. Looks up customer preferred language, uses Gemini to draft a friendly, localized pickup notification (e.g., "Your halal platter is ready for pickup!"), and dispatches it via the Twilio webhook.

  ### Key Design Decisions
  - **Offline-First Mutability:** "Sold-Out" toggles and order state changes must mutate local state first (SQLite/IndexedDB) to ensure the UI is never blocked by flaky 3G/4G connections.
  - **Asynchronous AI Notifications:** The owner doesn't draft pickup messages. They just swipe "Ready", and the AI handles the translation and dispatch asynchronously.
  - **Low-End Device Optimization:** No heavy animations. Use compressed WebP images and lazy loading for menu items.

  ## Implementation Prompt
  **User-Facing Outcome:** As a food cart operator (Fatima), I can see pre-orders pop up on my phone, tap one button to mark it ready (which automatically texts the customer in their language), and easily toggle items as "sold out" even if my phone connection drops.
  **CUJ & Acceptance Criteria:**
  1. Operator logs into the mobile app (375px) in Arabic.
  2. A simulated pre-order arrives via the backend API.
  3. Operator disconnects from network (simulated offline mode).
  4. Operator toggles "Chicken Shawarma" to "Sold Out" (UI updates instantly).
  5. Operator reconnects; the system syncs the "Sold Out" state to the backend.
  6. Operator swipes the new order to "Ready".
  7. The Operations Agent intercepts the state change, translates a pickup message, and triggers a mock SMS dispatch.
  8. Provide Playwright E2E tests verifying the offline toggle capability and the order state transition on a mobile viewport.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
