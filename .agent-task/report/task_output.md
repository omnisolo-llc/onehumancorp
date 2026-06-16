issue_title: "[Research] Distributed Point-of-Sale (POS) & Terminal Integration Architecture"
issue_description: |
  # Research Report: Distributed Point-of-Sale (POS) & Terminal Integration Architecture

  ## Problem Statement
  Small business owners with physical presence (like Priya the Boutique Owner, Carlos the Field Service Owner, or Fatima the Food Cart Operator) need the ability to collect payments in-person smoothly. Currently, OHC lacks a robust, natively integrated, mobile-first Point-of-Sale (POS) architecture capable of seamlessly syncing offline sales with online inventory, handling physical payment terminals (e.g., Stripe Terminal), and coordinating with AI agents to reconcile data and follow up with customers. The lack of offline-tolerant distributed POS sync forces owners to use separate tools, creating disjointed inventory and accounting.

  ## Research Report
  Our competitive analysis indicates that giants like Shopify and Square excel in POS but lack integrated agentic intelligence.
  - **Square** dominates the hardware/software integration but remains siloed from broader business operations unless complex integrations are used.
  - **Shopify POS** is robust but primarily built around their e-commerce engine, and its setup can be complex for micro-SMEs.
  - **The Gap for OHC:** We need to provide a unified Omnichannel Cart that operates perfectly on a 375px mobile screen, handles flaky networks, syncs inventory in real-time using distributed locks, and allows physical card taps via Stripe Terminal—all orchestrated by the OHC Operations and Finance agents.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as OHC Mobile App (375px)
      participant POSClient as POS Offline/Local Sync Client
      participant StripeReader as Stripe Terminal Hardware
      participant OHCAPI as OHC API Gateway
      participant Redis as Redis Redlock (Inventory)
      participant DB as PostgreSQL (Ledger)
      participant OpsAgent as Operations Agent

      Owner->>POSClient: Build Cart & Tap "Charge"
      POSClient->>Redis: Lock Inventory (ohc:lock:{tenant_id}:inventory:{cart_items})
      Redis-->>POSClient: Lock Acquired
      POSClient->>StripeReader: Initialize Payment Intent (Terminal SDK)
      StripeReader-->>POSClient: Payment Success
      POSClient->>OHCAPI: Finalize Order & Sync (Idempotent)
      OHCAPI->>DB: Update Ledger & Deduct Inventory
      OHCAPI->>Redis: Release Lock
      OHCAPI-->>POSClient: Sync Confirmed
      OHCAPI->>OpsAgent: Trigger Order Processing Workflow
  ```

  ### Mobile UX Flow
  1. **Omnichannel Cart View (375px):** Large, touch-friendly grid of products. The owner taps items to add to the cart.
  2. **Payment Selection:** A bottom sheet slides up with options: "Tap to Pay (Terminal)", "Cash", "Send Invoice Link".
  3. **Terminal Flow:** If "Tap to Pay" is selected, the app communicates with the Stripe Terminal SDK. The UI shows a clear, animated "Present Card" state.
  4. **Offline Tolerance:** If the network drops, the app queues the transaction locally (for cash/offline methods) or guides the user to reconnect for live card processing.
  5. **Post-Transaction:** A success card appears with agent suggestions (e.g., "Draft email receipt?", "Add to VIP list?").

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors the POS sync. If an offline sync pushes an inventory item below a threshold, it drafts a restock task in the owner's feed.
  - **Finance Agent:** Automatically categorizes the POS payment, reconciles the Stripe Terminal payout, and updates the daily revenue summary.
  - **Customer Assistant:** If a customer profile is attached to the POS transaction, it drafts a "Thank you for visiting the store" follow-up.

  ## Implementation Prompt
  **Implementer Objective:** Design and implement the foundational backend architecture and mobile-first frontend components for the OHC POS & Terminal Integration.

  **Critical User Journey (CUJ):**
  1. Priya (boutique owner) opens the OHC app and navigates to the POS screen on her mobile device (simulated 375px width).
  2. She adds a physical product to the cart.
  3. She selects "Tap to Pay" to initiate a Stripe Terminal transaction.
  4. The backend securely generates a Stripe Terminal connection token and creates a PaymentIntent.
  5. The UI displays the connection state and waits for the mock/simulated reader to confirm the payment.
  6. Upon success, the system finalizes the order, ensures inventory is correctly locked/deducted using Redis, and records the transaction in the PostgreSQL ledger.

  **Acceptance Criteria:**
  - Create the backend data models and gRPC/REST APIs necessary to manage POS carts, Terminal connection tokens, and PaymentIntents.
  - Implement a mobile-first (375px) React/Flutter UI component for the POS cart and payment flow.
  - Ensure the backend uses idempotent processing for payment finalization.
  - Integrate Redis distributed locking to prevent inventory overselling during the checkout process.
  - Must include E2E Playwright tests verifying the POS flow using the repository's test mock Stripe configurations.
  - All UI elements must pass the "grandmother test" (large touch targets, zero technical jargon).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
