issue_title: "[architecture]_mobile_first_tap_to_pay_pos"
issue_description: |
  # Architecture Deep Dive: Autonomous Mobile-First Tap-to-Pay POS

  ## Problem Statement
  For in-person operators like **Priya (boutique owner)** and **Fatima (food cart operator)**, the disconnect between their physical point-of-sale and their online inventory is a critical source of lost revenue and operational headache. Existing solutions like Square force users into a separate ecosystem that doesn't natively talk to their AI workflows, while Stripe Terminal requires complex developer integration. They need a simple, mobile-first Tap-to-Pay experience integrated directly into the OHC assistant app, turning any 375px smartphone into a unified POS without extra hardware.

  ## Research Report
  - **Competitor Audit:**
    - **Square POS:** Industry standard for physical retail, but creates a data silo. Difficult to unify with automated AI social commerce or custom digital storefronts without heavy API wrangling.
    - **Stripe Terminal:** Offers Tap-to-Pay on iPhone/Android, but is a developer SDK, not a ready-to-use operator tool.
    - **Shopify POS:** Powerful but expensive and overwhelming for simple operators; overkill for Fatima's food cart.
  - **The OHC Opportunity:** By wrapping Stripe Terminal's Tap-to-Pay SDK within the OHC Flutter app, we can offer Priya and Fatima a zero-hardware POS. When a transaction occurs, the Operations Agent immediately deducts inventory, the Finance Agent logs the deposit, and the Customer Success Agent captures the customer's email for automated receipts and future AI marketing loops.

  ## Design Doc
  - **Mobile UX Flow (375px First):**
    - **Operator View:** A bold "Charge" button on the primary dashboard. Tapping it opens a numeric keypad or a quick-select catalog grid (optimized for Fatima's high-speed checkout).
    - **Tap-to-Pay Flow:** Entering the amount and tapping "Charge" triggers the native OS Tap-to-Pay sheet (Apple/Google).
    - **Post-Transaction:** A quick success screen offering "Text Receipt" or "Email Receipt". If the customer is recognized (via loyalty/card link), the receipt is sent automatically.
  - **Architecture Diagram (Mermaid):**
    ```mermaid
    sequenceDiagram
        actor Operator (Priya)
        participant Mobile App
        participant Stripe Terminal SDK
        participant OHC Backend
        participant Operations Agent

        Operator->>Mobile App: Selects item, taps "Charge"
        Mobile App->>Stripe Terminal SDK: Init Tap-to-Pay
        Stripe Terminal SDK->>Customer Card: Reads NFC
        Stripe Terminal SDK->>Mobile App: Payment Intent Success
        Mobile App->>OHC Backend: Confirm Transaction
        OHC Backend->>Operations Agent: Deduct Inventory
        OHC Backend->>Operator (Priya): Show Success UI
    ```
  - **AI Integration Points:**
    - **Operations Agent:** Deducts inventory in real-time. If stock drops below threshold, alerts the owner to reorder.
    - **Marketing Agent:** If a receipt is texted, includes a link to the online storefront and a 10% discount on the next digital order, bridging physical and digital acquisition.
  - **Key Design Decisions:**
    - Integrate Stripe Terminal SDK directly into the mobile client shell.
    - Design offline-tolerant queuing: if network is flaky (e.g., Fatima's food cart), store signed transactions locally and sync when back online (subject to Stripe's offline limits).

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Design and implement the Mobile-First Tap-to-Pay POS capability within the OHC ecosystem.
  1. Integrate the Stripe Terminal SDK (Tap-to-Pay on iPhone/Android) into the mobile client shell.
  2. Implement the backend gRPC services to issue ConnectionTokens and capture PaymentIntents, ensuring strict `tenant_id` isolation.
  3. Build the 375px checkout UI: numeric keypad, cart summary, and the "Present Card" flow, adhering to the Translucent Glass design tokens.
  4. Plumb the successful transaction event into the OHC Event Bus so the Operations Agent can adjust inventory and the Marketing Agent can trigger receipt workflows.
  5. **Acceptance Criteria:** Create an E2E Playwright test simulating an owner initiating a charge, mocking the Terminal SDK success response, and verifying that inventory is deducted in the backend.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
