issue_title: "Idempotent Payment Sessions and Verified Outcome States"
issue_description: |
  **Title**: Idempotent Payment Sessions and Verified Outcome States

  **Problem Statement**:
  Currently, the product logic in invoice generation (`F08` / `F12` audit findings) fabricates checkout-looking URLs, and AI agents can present simulation or unknown provider paths as confirmed outcomes. For a small business owner like Maya the baker or Carlos the handyman, this is disastrous. If an AI agent falsely tells a customer "Your deposit is paid and your time is booked" without real payment provider verification, the owner loses revenue, double-books time, and loses customer trust. The product must guarantee idempotent checkout sessions with real provider states (e.g., pending, paid, failed) and never hallucinate a verified business completion.

  **Research Report**:
  - *Context*: OneHumanCorp (OHC) is an operations team for solo professionals. The goal is to safely handle money and capacity invariants.
  - *Current Code Baseline*: Audit findings indicate `invoice.rs:45-48` and booking helpers fabricate checkout-looking URLs rather than bridging to a real provider session. Additionally, the system lacks exact authority, stale approval/revocation, and reconciliation checks on affected payment paths, meaning simulation can look like completion (F12).
  - *Market Comparison*: Systems like Shopify, Stripe Invoicing, and Wix explicitly separate draft invoices, active checkout sessions, and settled payments. They rely heavily on webhook-driven or polling-based provider reconciliation to move an invoice from "Pending" to "Paid."
  - *Target Segment*: Launch/Run stage solo professionals who use digital payments.

  **Design Doc**:
  - *Architecture diagram (Mermaid.js)*:
    ```mermaid
    sequenceDiagram
        actor Customer
        participant Agent as AI Agent (OHC)
        participant Engine as Booking/Invoice Engine (OHC)
        participant Provider as Payment Provider (Stripe)

        Customer->>Agent: "I want to book the service."
        Agent->>Engine: Request booking & checkout session
        Engine->>Engine: Atomic reservation (Budget/Capacity)
        Engine->>Provider: Create Idempotent Checkout Session
        Provider-->>Engine: Session URL & ID (Pending State)
        Engine-->>Agent: Provide REAL checkout URL
        Agent->>Customer: "Here is your payment link: [URL]"
        Customer->>Provider: Completes Payment
        Provider-->>Engine: Webhook (payment_intent.succeeded)
        Engine->>Engine: Reconcile and transition to PAID
        Engine-->>Agent: Payment Confirmed Event
        Agent->>Customer: "Your booking is confirmed!"
    ```
  - *Mobile UX Flow*:
    1. **Agent Chat / Screen (375px)**: Customer agrees to a service. Agent drops a clean, native-feeling "Pay Deposit" card inline in the chat.
    2. **Checkout Transition**: The card clearly displays a "Pending" state and explicitly directs to the Stripe/Provider checkout overlay (mobile-optimized).
    3. **Return & Receipt**: Upon return, the chat card dynamically updates to a green "Confirmed & Paid" state, serving as the truthful receipt.
  - *UI Wireframes*:
    - The transaction card uses macOS-style Translucent Glass materials (`rgba(255, 255, 255, 0.65)` with 30px blur in light mode) and clean Ubiquiti UniFi modular layout.
    - Buttons are tactile, high-contrast, and keyboard accessible.
    - Status indicators explicitly say "Awaiting Payment..." or "Payment Complete" using precise colors and icons (e.g., Lucide-react `Clock` vs `CheckCircle`).
  - *AI Agent Integration Points*:
    - Agents no longer construct URLs manually. They must call an internal `create_checkout_session` tool.
    - Agents cannot declare a booking as "Paid" unless the Engine emits a verified `PaymentReconciled` event.
    - The Engine acts as the definitive ledger. The agent reads the status, it does not determine it.

  **Implementation Prompt**:
  Implement a secure, idempotent payment session generation and state verification system.
  1. Remove fabricated URL logic in `invoice.rs` and booking helpers.
  2. Implement an idempotent `create_checkout_session` service path that integrates with the actual provider (or explicitly enters a safe "Draft/Pending" test-mode state if configured).
  3. Ensure that booking and invoicing records only transition to "Paid" via explicit, verifiable provider reconciliation (e.g., webhook payload processing).
  4. Ensure AI agent tools correctly report the explicit state of the invoice and are blocked from confirming a booking if the payment state is not verified.
  5. Include E2E Playwright tests simulating a user receiving a payment link, and the backend transitioning the invoice state via a mock provider reconciliation event.

  **Priority**: P0 (Critical - Blocking core business revenue invariants)

  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
