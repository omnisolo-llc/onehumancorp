issue_title: "[research] Build Agentic Mobile-First POS & Tap-to-Pay Integration"
issue_description: |
  ## Title
  Agentic Mobile-First POS & Tap-to-Pay Integration

  ## Problem Statement
  Small business operators performing in-person sales (like Priya in her boutique, Carlos doing field repairs, or Fatima at her food cart) require a frictionless way to accept physical payments. Switching between an inventory app and a separate payment terminal app (or relying on clunky Bluetooth card readers) slows down operations, disrupts the customer experience, and leads to unsynchronized offline/online inventory data. They need a built-in POS that uses native Tap-to-Pay on their smartphones, seamlessly overseen by an AI Operations Assistant that instantly locks inventory and updates the central ledger without manual reconciliation.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify POS:** Offers excellent capabilities but requires downloading a separate POS app. Deep integration is available, but hardware card readers are often pushed, and true Tap-to-Pay on iPhone requires navigating complex permissions.
  - **Square:** The market leader in mobile POS, but Square creates a separate silo from the merchant's main online storefront unless they fully commit to Square Online (which is weak for robust e-commerce).
  - **Stripe Terminal (Tap-to-Pay):** Provides the underlying SDKs for frictionless mobile payments without extra hardware.
  - **OHC Opportunity:** By embedding Stripe Tap-to-Pay natively into the primary OHC mobile application shell (Flutter), the business owner handles online orders, messaging, and in-person POS from a single interface. The Operations Assistant ("The Manager") and Finance Assistant ("The Accountant") automatically ingest these transactions, instantly syncing omnichannel inventory and updating daily revenue summaries.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[OHC Mobile App - POS Mode] -->|Initiate Tap-to-Pay| B[Stripe Terminal SDK]
      B -->|Payment Intent Success| C[OHC API Gateway]
      C --> D{KAIROS Distributed Lock}
      D -->|Inventory Lock| E[PostgreSQL Central Ledger]
      D --> F[Event Mesh Pub/Sub]
      F --> G[Operations Agent The Manager]
      F --> H[Finance Agent The Accountant]
      G -->|Push Notification| I[Owner Feed: Low Stock Alert]
      H -->|Update| J[Daily Revenue Dashboard]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **POS Mode Access:** A prominent "Take Payment" floating action button (FAB) or bottom nav tab in the OHC mobile app.
  - **Cart Compilation:** A 375px-optimized visual catalog with 44x44px minimum touch targets for adding items. A large, sticky "Charge \$XX.XX" button at the bottom.
  - **Payment Flow:** Tapping "Charge" triggers the native iOS/Android Tap-to-Pay interface overlay.
  - **Success Screen:** A translucent glassmorphism success card indicating the payment was received, with 1-tap options to "Send Digital Receipt" via SMS/Email (handled by the Ambassador Agent).

  ### AI Agent Integration Points
  - **Operations Agent:** Upon a successful POS transaction, the agent monitors the inventory decrement. If stock falls below a threshold, it drafts a restock order or hides the item from the online storefront.
  - **Finance Agent:** Categorizes the in-person sale in the ledger, distinguishing it from online revenue, and factors it into the owner's daily plain-language performance summary.
  - **Customer Assistant:** If the customer's payment method is linked to an existing profile, the agent updates their omnichannel history and drafts a personalized follow-up or review request for the owner to approve.

  ### Key Design Decisions
  - **No External Hardware:** Rely entirely on native Tap-to-Pay (NFC) via Stripe Terminal SDKs to minimize setup barriers for users like Fatima and Carlos.
  - **Single App Shell:** POS is a mode within the primary OHC app, not a standalone application, preventing context switching.
  - **Optimistic Inventory Locking:** Use Redis Redlock for immediate inventory reservation during the transaction to prevent online overselling while the physical customer is tapping their card.

  ## Implementation Prompt
  **User-Facing Outcome:** As an owner (like Priya), I can ring up an in-store customer by tapping items in the OHC app and holding out my phone for them to tap their credit card. The payment clears instantly, my online inventory is immediately reduced, and my Finance Assistant logs the sale without me opening a second app or connecting a Bluetooth reader.

  **CUJ & Acceptance Criteria:**
  1. User navigates to the POS screen on the mobile app.
  2. User selects a product and taps "Charge."
  3. The Stripe Terminal SDK mock/test interface is invoked and returns a successful payment intent.
  4. The system updates the central PostgreSQL ledger and releases the distributed lock.
  5. The Finance Agent acknowledges the transaction in the event stream.
  6. E2E Playwright tests must verify the visual selection of items and the optimistic update of the cart total, strictly adhering to the 375px mobile layout constraints.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
