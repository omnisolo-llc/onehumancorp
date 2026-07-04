issue_title: "Implement Unified Mobile Tap-to-Pay & Hybrid Checkout Architecture"
issue_description: |
  # Unified Mobile Tap-to-Pay & Hybrid Checkout Architecture

  ## Problem Statement
  Currently, the OneHumanCorp platform lacks a deeply integrated, omni-channel checkout solution that caters to both online and in-person operations. Personas like Priya (boutique operator selling online and in-store) and Fatima (food cart operator with offline pre-orders) suffer from a fragmented experience. They need to manage online demand without losing control of in-store operations (like tap-to-pay). The gap lies in the absence of a unified point-of-sale (POS) and checkout architecture that handles both digital and physical transactions seamlessly from a mobile device (375px viewport), without requiring external, disconnected apps.

  ## Research Report
  - **Shopify & Wix**: Shopify excels with its POS system and online checkout, but separates the two into different apps, creating overhead for simple operators. Wix has a simpler integrated POS but lacks deep agentic orchestration for offline-tolerant operations.
  - **Square**: Square is the standard for in-person tap-to-pay but its e-commerce offering is often secondary to the physical terminal.
  - **OHC Opportunity**: OHC can differentiate by providing a unified checkout model where an online cart and an in-person tap-to-pay transaction share the exact same underlying architecture, ledger, and agentic workflows. When an online lead transitions to an in-person sale, the transition should be invisible to the owner.
  - **Technical Gap**: The current system needs a `TerminalSession` and `HybridCheckout` data model, integrated directly into the `tenant` schema with row-level security. We must implement a Tap-to-Pay SDK wrapper within the Flutter PWA/Native app, backed by our Go API.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      App[Flutter Mobile App - 375px] --> CheckoutUI[Unified Checkout UI];
      CheckoutUI --> TapToPay[Native Tap-to-Pay SDK];
      CheckoutUI --> OnlinePayment[Digital Payment Gateway];
      TapToPay --> GoAPI[Go API Server];
      OnlinePayment --> GoAPI;
      GoAPI --> Stripe[Stripe Terminal / Payments API];
      GoAPI --> Postgres[(Postgres - Unified Ledger)];
      GoAPI --> AgentQueue[AI Job Queue - Follow-ups & Sync];

      subgraph Multi-Tenant Boundary
      Postgres
      AgentQueue
      end
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Screen 1 (Active Cart/Order)**: The owner sees a clean, translucent glass UI card summarizing the customer's order. A large, high-contrast action button labeled "Collect Payment" anchors the bottom.
  - **Screen 2 (Payment Method)**: Tapping "Collect Payment" slides up a bottom sheet. Options: "Tap to Pay (Phone)", "Send Payment Link", "Cash".
  - **Screen 3 (Tap to Pay Active)**: If "Tap to Pay" is selected, the screen transitions to the native tap-to-pay modal. Background blurs to maintain focus.
  - **Screen 4 (Success & Agent Handoff)**: Upon success, a crisp checkmark animation plays. The system immediately shows "Receipt sent to Priya. Inventory updated." An AI agent automatically queues a follow-up task if this is a high-value customer.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Triggers immediately post-transaction to update the daily revenue summary.
  - **Operations Assistant**: Deducts inventory in real-time. If stock is low, queues a restock alert for the owner.
  - **Customer & Relationship Assistant**: Drafts a thank-you note or digital receipt, ready for the owner to approve or auto-sends based on rules.

  ### Key Design Decisions
  - **Unified Checkout Model**: Both physical and digital checkouts create a standard `CheckoutSession` entity in the database.
  - **Zero Trust & RLS**: Every `CheckoutSession` is strictly bound to `tenant_id` with Postgres Row-Level Security enabled.
  - **Offline Tolerance**: In-person transactions must gracefully queue or rely on the underlying payment SDK's offline capabilities if network connectivity drops, ensuring Fatima never loses a sale at her cart.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the "Unified Mobile Tap-to-Pay & Hybrid Checkout Architecture".
  - **User Journey**: As Priya (boutique owner), I want to build a cart on my mobile phone (375px) for an in-store customer and select "Tap to Pay" to process their credit card directly on my device, instantly updating my unified online/offline inventory and daily revenue.
  - **Requirements**:
    - Implement the UI flow starting from the active order screen to the payment selection bottom sheet.
    - Provide a unified service layer in Go to handle the `CheckoutSession` lifecycle for both digital and physical tap-to-pay scenarios.
    - Integrate the UI with a simulated Tap-to-Pay native bridge for testing (representing Stripe Terminal integration).
    - Ensure the UI adheres to the macOS-style Translucent Glass materials and UniFi-style card layouts.
    - Verify the flow completely on a 375px viewport.
  - **Acceptance Criteria**: E2E Playwright tests must verify the checkout flow from cart creation to simulated payment success and subsequent inventory update, running against the local Docker Compose stack.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
