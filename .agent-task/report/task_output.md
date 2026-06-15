issue_title: "Autonomous Pre-Order & Multilingual Operations Flow for Food Carts"
issue_description: |
  ### Title
  Autonomous Pre-Order & Multilingual Operations Flow for Food Carts

  ### Problem Statement
  Fatima (Food Cart Operator) needs to handle fast-paced pre-orders, manage daily menus with 1-tap "sold-out" toggles on slow mobile data, and coordinate pickups across language barriers. Existing systems (Square, Toast) are hardware-heavy and complex, while form builders lack inventory sync and payment.

  ### Research Report
  - **Market Context**: Square and Toast dominate food, but are often overkill for a single cart or pop-up. They require specific hardware or strong continuous connectivity.
  - **The OHC Opportunity**: Providing a low-data, mobile-first (375px) web interface that allows optimistic offline "sold out" toggles and AI-assisted translation for order notes.
  - **Competitor Gaps**: Square offline mode is mostly for payments, not real-time inventory toggling across multiple staff devices with AI translation.

  ### Design Doc
  - **Architecture**:
    - `products` table needs optimistic offline syncing for `available_quantity` or a quick `is_sold_out` flag.
    - The `ohc_offline_queue` (already present in the frontend) must be extended to handle `inventory_toggle` events reliably.
    - **AI Integration**: The Operations Agent intercepts incoming orders, translates customer notes (e.g., "no onions") into the operator's preferred language (e.g., Arabic), and automatically dispatches an SMS/WhatsApp notification when the operator marks the order "Ready".
  - **Mobile UX Flow (375px)**:
    - A "Kitchen View" Command Center: High-contrast, large touch targets (min 44x44px).
    - Left column: Active orders with translated notes.
    - Right panel: The day's menu with massive, 1-tap "Sold Out" toggles.

  ### Implementation Prompt
  **Feature Name**: Food Cart Daily Operations & Multilingual Agent
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima can view translated incoming orders on her low-end Android phone, mark items sold out instantly (even with spotty cell service), and notify customers for pickup with a single tap.

  **Next Actions**:
  1. Implement the "Kitchen View" mobile layout (375px) with large touch targets for orders and inventory toggling.
  2. Extend the frontend offline sync queue (`ohc_offline_queue`) to support optimistic UI updates for `inventory_toggle` mutations.
  3. Integrate the Operations Agent to listen for new orders and append translated order notes based on the tenant's language preferences.
  4. Ensure E2E Playwright tests cover the offline toggle behavior and order state progression.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
