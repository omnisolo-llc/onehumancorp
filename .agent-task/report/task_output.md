issue_title: "Universal Tap-to-Pay Terminal Integration with Agentic Offline-First Sync"
issue_description: |
  ## Track 1: Architectural Gap & Scaling Discovery

  **Problem Statement**:
  Small business owners with physical locations (like Priya the boutique owner or Fatima the food cart operator) rely on in-person sales just as much as online demand. Currently, OHC handles online checkout (Stripe Checkout) but lacks a cohesive, native in-person tap-to-pay architecture. Competitors like Shopify POS and Square dominate this space, but their solutions are separated from their AI assistant capabilities and often require purchasing expensive proprietary hardware.

  Our non-technical owners need to take payments instantly using their existing smartphones (Tap to Pay on iPhone/Android) while keeping inventory synchronized. Without an offline-first POS integration, OHC risks losing any owner who operates in the physical world.

  **Competitor Analysis**:
  - **Shopify POS**: Powerful but requires a separate app, additional fees for advanced features, and a steep learning curve for staff.
  - **Square**: Industry standard for physical POS, but its e-commerce capabilities lag behind.
  - **Stripe Terminal**: Excellent developer API but no native out-of-the-box UI for SMBs.

  **The OHC Differentiator**:
  OHC must integrate Stripe Terminal natively within our existing Flutter mobile app, using the owner's phone as the reader (Tap to Pay). Crucially, the "Operations Agent" must invisibly handle offline resilience: if the internet drops at a food cart, the app continues to accept payments and the Agent seamlessly syncs and reconciles inventory when the connection is restored.

  ## Track 2: Selected Architecture Deep Dive (Design Doc)

  **High-Level Architecture & Data Model**:
  - **Frontend (Flutter)**: Integrates the Stripe Terminal SDK for Tap to Pay on iOS/Android.
  - **Backend (Go + PostgreSQL)**: Implements Stripe Terminal Connection Tokens and Location Management.
  - **Data Schema**:
    - `TerminalSession`: Tracks the active POS session, operator ID, and location.
    - `OfflineTransactionQueue`: A local database on the mobile device to queue transactions when network is lost.
    - `InventoryLock`: Redis Redlock pattern to reserve inventory globally while a physical payment is processing.

  **Architecture Diagram**:
  ```mermaid
  graph TD
      A[OHC Mobile App - 375px] -->|Tap to Pay SDK| B(Stripe Terminal)
      A -->|Offline Mode| C[(Local OfflineTransactionQueue)]
      A -->|Online Sync| D{OHC API Gateway}
      D --> E[Stripe Payment Intents API]
      D --> F[Operations Agent]
      F --> G[(PostgreSQL Ledger)]
      F --> H[(Redis Inventory Redlock)]
  ```

  **AI Agent Integration Points**:
  - **The Manager (Operations Agent)**: Monitors the `OfflineTransactionQueue`. When connectivity is restored, it automatically batches the synchronization to the PostgreSQL ledger, resolving any potential inventory conflicts (e.g., if an item was sold online while offline).
  - **The Accountant (Finance Agent)**: Instantly logs tap-to-pay revenue in the daily summary and matches Stripe payouts to physical sales batches.
  - **The Ambassador (Customer Success Agent)**: If a customer provides an email or phone number for an electronic receipt, the agent links the transaction to their existing profile and triggers a post-visit review request.

  ## Track 3: Technical Integrity & Mobile-First Review

  **Mobile UX Flow (375px First)**:
  1. Priya opens the OHC app and switches to "Storefront" mode. The layout is optimized for high-speed operation on a 375px width screen with touch targets exceeding 44x44px.
  2. She taps an item from her visual catalog. A large "Charge $X.XX" button appears at the bottom.
  3. Tapping "Charge" invokes the native iOS/Android Tap to Pay overlay.
  4. The customer taps their card/phone. The app confirms payment with a satisfying haptic feedback and visual "Paid" token.
  5. In the background, the Operations Agent clears the Redis inventory lock and updates the PostgreSQL ledger.
  6. **Offline Scenario**: If Priya loses signal, the app continues to function. The "Charge" button works, logging the transaction locally. A small, non-intrusive banner reads "Offline - Will sync automatically".

  **Zero Trust & Security**:
  All Terminal communication uses short-lived, single-use connection tokens requested securely from the OHC API via SPIFFE/SPIRE authenticated microservices. No PCI-sensitive data touches OHC servers.

  ## Track 4: Implementation Prompt

  **Feature Name**: Universal Tap-to-Pay Integration with Agentic Offline-First Sync

  **Target Persona**: Fatima the Food Cart Operator, who operates in areas with spotty 5G coverage.

  **Outcome**: Fatima can accept credit cards directly on her Android phone using Tap to Pay. Even when her connection drops, she can continue ringing up customers. The OHC Operations Agent handles the deferred synchronization and inventory updates in the background.

  **Acceptance Criteria**:
  - Implement the backend logic to generate Stripe Terminal connection tokens.
  - Build the Flutter UI for the POS checkout flow, ensuring it works perfectly on a 375px viewport with large touch targets.
  - Implement a local persistence layer in the Flutter app to queue transactions when offline.
  - Extend the Operations Agent to ingest the batched offline transactions and resolve inventory state securely using PostgreSQL row-level locks.
  - 100% unit test coverage for the backend sync logic and automated Playwright/Flutter Driver E2E tests for the POS checkout flow.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []