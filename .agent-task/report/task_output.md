issue_title: "Architecture Design: Mobile-First Offline-Tolerant Tap-to-Pay POS System"
issue_description: |
  # Architecture Design: Mobile-First Offline-Tolerant Tap-to-Pay POS System

  ## Problem Statement
  Priya (boutique owner) and Fatima (food cart operator) need to accept in-person payments securely and rapidly, often in environments with spotty cellular coverage. Current platforms either force owners into expensive proprietary hardware (Square) or fail gracefully when offline. OHC must provide a native mobile tap-to-pay integration (via Stripe Terminal SDK / NFC) that works beautifully on a 375px viewport, captures offline transaction intents securely, and seamlessly syncs with the unified AI Assistant queue once connectivity is restored, all without requiring technical configuration.

  ## Research Report
  Our analysis of Shopify POS, Stripe Terminal, and Square reveals that high-scale tap-to-pay capabilities require:
  1. **Edge-local State Management**: Transactions must proceed without blocking on network egress, capturing encrypted card data via native NFC/Terminal SDKs.
  2. **Multi-Tenant Idempotency**: Offline captures must guarantee exactly-once processing using strict `tenant_id` isolated idempotency keys.
  3. **Agent Handoff**: Once synced, the Operations Assistant updates inventory and the Finance Assistant logs the transaction, providing the owner with real-time plain-language summaries.
  4. **Low-Data Tolerance**: Webhooks and retry loops must utilize exponential backoff and WebP/compressed payload structures.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      A[Mobile Flutter App - 375px] --> B(Stripe Terminal SDK / NFC);
      B --> C{Network Status};
      C -- Online --> D[OHC Backend API gRPC/REST];
      C -- Offline --> E[Local Encrypted Queue SQLite];
      E --> F(Background Sync Worker);
      F --> D;
      D --> G[PostgreSQL / SKIP LOCKED Queue];
      G --> H(Operations Assistant Agent);
      G --> I(Finance Assistant Agent);
      H --> J[Inventory Update];
      I --> K[Ledger Update];
  ```

  ### Mobile UX Flow (375px First)
  1. **Home Shell**: Fatima taps the "Checkout" floating action button (FAB).
  2. **Cart/Order View**: A clear, translucent glassmorphism pane slides up showing the order total.
  3. **Payment Method**: Fatima selects "Tap to Pay".
  4. **NFC Modal**: A highly legible, large touch-target (44x44px minimum) screen prompts the customer to tap their card.
  5. **Confirmation & Offline State**: If offline, the screen instantly displays a green checkmark with a subtle "Pending Sync" icon. The AI agent queue takes over the rest.

  ### AI Agent Integration Points
  - **Work Triage**: Ingests successful syncs and clears pending states.
  - **Finance Assistant**: Automatically reconciles batched tap-to-pay transactions at end-of-day and flags anomalies.
  - **Operations Assistant**: Deducts sold items from local inventory models immediately and syncs globally.

  ### Key Design Decisions
  - **Local-First SQLite Queue**: Ensure no customer is turned away due to slow networks.
  - **Stripe Terminal SDK**: Avoid proprietary OHC hardware; leverage standard Android/iOS NFC capabilities.
  - **Zero Trust Isolation**: All local encrypted queues maintain strict `tenant_id` bounds.

  ## Implementation Prompt
  **Implementer Agent Objective**: Implement the Flutter Tap-to-Pay checkout flow and backend sync endpoint.
  - **User Journey (CUJ)**: A boutique owner adds an item to the cart, initiates a Tap-to-Pay session, the customer taps their card, and the transaction is recorded locally. Upon network restoration, the transaction syncs to the OHC backend, triggering inventory and ledger updates.
  - **Acceptance Criteria**:
    - The UI must perfectly fit a 375px width using OHC Premium Tokens (translucent glass styling, correct spacing).
    - Touch targets for checkout must be >= 44x44px.
    - An offline transaction must successfully queue locally and sync when the network is mocked as restored.
    - 100% unit test and Playwright E2E test coverage for the checkout flow.
    - Zero mock data in the final UI; all data must flow through the real SQLite/backend path.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
