issue_title: "Agentic Tap-to-Pay Terminal Architecture for Mobile Point of Sale (mPOS)"
issue_description: |
  # Agentic Tap-to-Pay Terminal Architecture for Mobile Point of Sale (mPOS)

  ## Problem Statement
  Small business owners and operators (like Priya, the boutique owner, or Fatima, the food cart operator) need a seamless way to process in-person payments directly on their mobile devices (iOS/Android) without external hardware. Currently, OHC lacks an integrated, mobile-first Tap-to-Pay (mPOS) capability that ties directly into the centralized inventory, customer memory, and AI agent coordination systems. When an owner processes a transaction using a disconnected system (like a standard Stripe card reader or Square), the OHC ecosystem loses context—inventory isn't synced in real-time, the customer interaction isn't recorded by the Customer Assistant, and the Finance Assistant can't immediately correlate the sale with daily operations.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Stripe Terminal (Tap to Pay):** Allows compatible iPhones and Android devices to accept contactless payments directly. This is the industry standard for hardware-free mPOS.
  - **Shopify POS:** Offers a robust mobile POS system, but it's often too complex for simple setups and requires additional subscription tiers for full functionality.
  - **Square:** The dominant player in simple mPOS, but operates as a walled garden, disconnecting the payment from the broader agentic work assistance (like automatic follow-ups or smart inventory reordering).
  - **OHC Opportunity:** By integrating Stripe's Tap-to-Pay SDK directly into the OHC Flutter app, we can provide a frictionless, hardware-free checkout experience. Crucially, because it's native to OHC, every transaction immediately triggers the AI agents: Operations Agent locks inventory, Customer Assistant logs the interaction (if a known customer), and Finance Agent updates the daily summary. This provides an integrated experience that standalone POS systems cannot match.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile App (Owner)
      participant TTP as Tap-to-Pay SDK
      participant OHC as OHC Backend API
      participant Redis as Distributed Lock
      participant DB as PostgreSQL (Ledger)
      participant Agent as Operations/Finance Agents
      participant Stripe as Stripe API

      Owner->>OHC: Initiate Checkout (Cart)
      OHC->>Redis: Lock Inventory (ohc:lock:{tenant}:{product})
      OHC->>Stripe: Create PaymentIntent
      Stripe-->>OHC: PaymentIntent Secret
      OHC-->>Owner: PaymentIntent Secret
      Owner->>TTP: Present Card (Tap to Pay)
      TTP->>Stripe: Process Payment
      Stripe-->>TTP: Payment Success
      TTP-->>Owner: Success UI
      Owner->>OHC: Confirm Transaction
      OHC->>DB: Finalize Ledger & Inventory
      OHC->>Redis: Release Lock
      OHC->>Agent: Trigger Post-Sale Workflow
  ```

  ### Data Model & Invariants
  - **TerminalSession:** Entity to track active checkout sessions.
  - **Multi-Tenant Isolation:** Ensure all interactions with Stripe and the OHC backend strictly enforce `tenant_id` boundaries.
  - **Idempotency:** All payment and inventory deduction requests must include idempotency keys to handle flaky mobile networks gracefully.

  ### Mobile UX Flow (375px)
  1. **Cart Screen:** Owner adds items to the cart. A prominent "Charge $XX.XX" button is visible at the bottom (target > 44px).
  2. **Payment Method Selection:** User selects "Tap to Pay" (or it defaults if no card reader is connected).
  3. **Tap-to-Pay UI:** The native OS Tap-to-Pay sheet appears, instructing the customer to hold their card/device.
  4. **Success State:** A clean, translucent glass success screen appears, offering options to email/SMS a receipt, and returning to the main feed.

  ### AI Agent Integration
  - **Operations Assistant:** Automatically adjusts inventory based on the transaction. If an item drops below a threshold, it drafts a restock task in the owner's feed.
  - **Finance Assistant:** Instantly records the transaction and updates the daily revenue dashboard.
  - **Customer Assistant:** If the customer's email/phone is collected for the receipt, it links the transaction to their profile for future context.

  ### Key Design Decisions
  - **Hardware-Free First:** Prioritize Tap-to-Pay on iPhone/Android to minimize friction for new users. External readers (e.g., Stripe Reader M2) will be supported later.
  - **Optimistic Locking:** Use Redis Redlock during the checkout phase to prevent online sales from double-booking items currently being purchased in-store.

  ## Implementation Prompt
  **Outcome:** Implement the backend foundation and the AI agent coordination for the new Tap-to-Pay (mPOS) feature.
  **CUJ:** The owner selects items in the OHC mobile app and chooses "Tap to Pay". The system successfully creates a PaymentIntent, secures the inventory via Redis, and prepares the backend to handle the transaction confirmation and subsequent agent workflows.
  **Acceptance Criteria:**
  - Create the necessary API endpoints to initiate a Tap-to-Pay checkout session (interfacing with Stripe to create a PaymentIntent).
  - Implement Redis Redlock inventory reservation during the checkout initialization.
  - Design the `TerminalSession` database schema with strict multi-tenant constraints.
  - Implement the webhook/confirmation endpoint to finalize the transaction, update the PostgreSQL ledger, and trigger the Operations/Finance agents.
  - Ensure all critical operations are idempotent.
  - Add comprehensive unit tests and E2E Playwright tests simulating the backend flow. Do not implement the Flutter frontend SDK integration, focus on the backend capabilities that enable the frontend.

  ## Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
