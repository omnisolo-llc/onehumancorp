issue_title: "Implement Universal Mobile POS & Tap-to-Pay with Agentic Inventory Sync"
issue_description: |
  ## Title: Implement Universal Mobile POS & Tap-to-Pay with Agentic Inventory Sync

  ## Problem Statement
  Priya, a boutique operator, currently uses a separate card reader and POS system in her store, and another system for her online sales. This causes "sold out" items to still appear online, and customer purchases made in-store are disconnected from their online profiles. Managing two systems is frustrating and causes overselling. Priya needs a unified, mobile-first Tap-to-Pay system right in her OHC app that instantly syncs inventory and customer records across all channels, without needing extra hardware.

  ## Research Report
  - **Shopify**: Offers Shopify POS, but it's a separate app, requires expensive hardware for full functionality, and isn't integrated perfectly into the core admin mobile app. Tap-to-Pay on iPhone exists but feels bolted on.
  - **Square**: Dominates physical POS but struggles with deep online integration and lacks an AI assistant layer to predict stockouts or draft reorder emails.
  - **Wix/Squarespace**: E-commerce first, POS second. In-person sales are clunky and often require third-party terminal integrations.
  - **OHC Opportunity**: By building Tap-to-Pay directly into the OHC mobile app, we eliminate the need for a secondary POS app. When an in-store purchase happens, the Operations Agent can instantly update omnichannel inventory, and the Finance Agent can update the daily revenue summary, providing Priya with real-time, unified visibility.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Mobile App - 375px] -->|Tap-to-Pay NFC SDK| B(Stripe Terminal/Tap-to-Pay)
      B --> C[Stripe Processing]
      C -->|Webhook/Callback| D[OHC API Gateway]
      D --> E{Event Mesh}
      E --> F[Ledger/Finance DB]
      E --> G[Omnichannel Inventory DB]
      E --> H[Operations Agent]
      H -->|Update| G
      H -->|Low Stock Alert| I[Action Required Queue]
      I --> J[Mobile App Feed]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Feed (Mobile):** A dedicated "Sell In Person" quick action on the main dashboard.
  - **Interaction:** Tapping opens a clean numpad or catalog view with large touch targets. User selects items or enters a custom amount.
  - **Checkout:** Tapping "Charge" triggers the native OS Tap-to-Pay interface (Apple/Google). No external dongle needed.
  - **Post-Transaction:** A success screen shows the updated daily total. A blurred glassmorphism card appears if inventory drops low, suggesting a reorder draft from the Operations Agent.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory levels. If a tap-to-pay transaction reduces an item's stock below the threshold, it drafts a supplier reorder email for the owner to approve.
  - **Finance Agent:** Immediately ingests the transaction into the real-time daily summary, ensuring the "plain-language daily performance summary" is perfectly accurate.

  ### Key Design Decisions
  - **Dongle-Free First:** Rely on native iOS/Android Tap-to-Pay capabilities via Stripe Terminal SDK to reduce physical friction and hardware costs.
  - **Single App Philosophy:** Do not build "OHC POS". The POS capability must live within the single owner work assistant app, accessible in one tap.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (like Priya), I can tap a button on my OHC mobile app, hold a customer's credit card to my phone, and instantly collect payment. The sale automatically deducts from my global inventory, and if the item is running low, my Operations Assistant adds a card to my feed suggesting I reorder it.

  **CUJ & Acceptance Criteria:**
  1. The UI provides a "Sell In Person" flow accessible from the main mobile dashboard.
  2. The flow integrates with the Stripe Terminal Tap-to-Pay capability (simulated in test environments) for dongle-free payment collection.
  3. A successful transaction immediately decrements the item's inventory in the unified database.
  4. If the inventory falls below a threshold, the Operations Agent generates a low-stock alert/reorder suggestion in the owner's Action Required feed.
  5. Provide Playwright E2E tests: A user logs in on a mobile-sized viewport, completes a simulated Tap-to-Pay transaction, and verifies the inventory update and AI alert on the dashboard.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []