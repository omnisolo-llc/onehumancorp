issue_title: "[Architecture] Omnichannel Mobile Tap-to-Pay & Unified Ledger"
issue_description: |
  ## Title: Implement Omnichannel Mobile Tap-to-Pay & Unified Ledger Architecture

  ## Problem Statement
  Small business operators like Priya (Boutique Operator) and Carlos (Handyman) require seamless, instant payment collection across physical and digital channels. Currently, they juggle separate tools: a clunky card reader for in-person sales (which often loses connection) and a separate system for online invoices. This fragmentation results in delayed payments, mismatched inventory, and broken financial reporting. They need a unified "Tap-to-Pay on Mobile" experience natively integrated with a Zero-Trust Immutable Ledger, allowing them to accept payments anywhere (offline-tolerant) while an AI agent instantly reconciles the revenue and updates inventory without any manual bookkeeping.

  ## Research Report
  **Market Gap & Competitor Audit:**
  - **Shopify POS:** Powerful but requires expensive proprietary hardware (card readers) and forces the owner into a complex Shopify ecosystem. It is not easily adaptable for field services (like Carlos).
  - **Square:** Dominates in-person but operates as a separate silo from many online custom storefronts.
  - **Stripe Terminal (Tap to Pay on iPhone/Android):** Provides the underlying API, but requires significant engineering to integrate into a cohesive, non-technical owner dashboard.

  **The OHC Opportunity:**
  OHC can leapfrog by embedding Stripe's "Tap to Pay" directly into the OHC Flutter/PWA shell. This eliminates the need for extra hardware. When combined with our Event-Sourced Immutable Ledger and Operations AI, a single tap instantly updates the universal ledger, deducts inventory, and sends a drafted thank-you note to the customer.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      OWNER_DEVICE ||--o{ POS_SESSION : initiates
      POS_SESSION ||--|| STRIPE_TERMINAL : bridges_to
      STRIPE_TERMINAL ||--|| TAP_TO_PAY : processes
      TAP_TO_PAY ||--o{ PAYMENT_INTENT : creates
      PAYMENT_INTENT ||--|| LEDGER_ENTRY : triggers
      PAYMENT_INTENT ||--|| INVENTORY_SYNC : triggers

      FINANCE_AI_DEPARTMENT {
          string role
          string trigger
      }
      PAYMENT_INTENT ||--o{ FINANCE_AI_DEPARTMENT : notifies
      FINANCE_AI_DEPARTMENT ||--o{ OWNER_FEED : pushes_summary
  ```

  ### Mobile UX Flow (375px first)
  1. **Dashboard:** Priya opens the OHC app. She taps a prominent, floating "Charge" button.
  2. **Amount & Item:** She enters an amount or selects an item from her inventory (e.g., "Vintage Scarf").
  3. **Tap to Pay:** The screen transitions to the native OS Tap-to-Pay overlay (using Stripe Terminal SDK). The customer taps their card on Priya's phone.
  4. **Instant Reconciliation:** A translucent glass success card appears: "Payment Approved. Ledger updated and inventory adjusted."
  5. **Offline Tolerance:** If offline (e.g., Carlos in a remote basement), the app securely caches the transaction intent and queues it for sync when connectivity returns, showing a "Payment Queued" state.

  ### AI Agent Integration Points
  - **Operations Agent:** Listens for the successful `PAYMENT_INTENT` event. Instantly decrements the associated product inventory across all channels (online and physical).
  - **Finance Agent:** Records the transaction in the immutable ledger and updates the daily revenue summary for the owner's feed.

  ### Key Design Decisions
  - **Hardware-less POS:** Leverage native Tap-to-Pay on iPhone/Android via Stripe Terminal to eliminate friction and hardware costs.
  - **Offline-Tolerant Queue:** Crucial for field service personas. Use local device storage (IndexedDB/SQLite) to queue transactions safely if the network drops.
  - **Strict Multi-Tenancy:** Ensure every POS session and ledger entry is strictly bound to the `tenant_id` via Row Level Security (RLS) in PostgreSQL.

  ## Implementation Prompt

  **User-Facing Outcome:**
  Deliver a hardware-less, Tap-to-Pay POS experience within the OHC mobile shell. The feature must allow the owner to accept an in-person payment using only their smartphone. Upon success, the payment must instantly reflect in the unified financial ledger and adjust inventory automatically.

  **Core User Journeys (CUJ):**
  1. The owner initiates a charge from the mobile app (375px optimized).
  2. The customer taps their credit card on the owner's device.
  3. The payment is processed (or queued if offline).
  4. The Finance AI agent automatically records the sale in the unified ledger and updates the daily summary feed.

  **Acceptance Criteria:**
  - Integrate Stripe Terminal SDK for native Tap-to-Pay (mocked for E2E tests).
  - Build the 375px-optimized POS UI with translucent glass materials.
  - Ensure the transaction successfully creates an immutable entry in the PostgreSQL ledger.
  - Implement offline queuing for POS transactions.
  - Write Playwright E2E tests validating the full Tap-to-Pay to Ledger flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
