issue_title: "Implement Complete Point of Sale (POS) Workflow Architecture"
issue_description: |
  # Comprehensive Point of Sale (POS) Architecture Report & Issue Brief

  ## Research Report

  ### Market Analysis
  Current OHC POS capabilities are fragmented. We have rudimentary `pos.rs`, a disconnected `terminal_api.rs` (Stripe focus), offline sync mechanisms, and various partial models in `migrations/`. Competitors like Square and Shopify POS offer deeply integrated workflows:
  - **Shopify POS:** Seamless unification of online/offline inventory, variant handling, native card reader integration, offline mode with background sync, and rich staff permissions.
  - **Square:** The gold standard for mobile-first POS. Simple catalog access, variant selection, complex tax/discount rules, seamless tap-to-pay, and offline queuing.

  ### Findings in OHC Codebase
  - **Terminal API:** Exists (`src/server/api/terminal_api.rs`) but is heavily tied to Stripe Payment Intents. Needs broader integration with the unified checkout model.
  - **Inventory & Variants:** `product_variants` table exists (Migration 071). `cart_items` supports `variant_id` (Migration 120/121).
  - **Offline Sync:** Exists (`offline_sync.rs` and `sync_offline` in `terminal_api.rs`), but needs unification and solid error handling/conflict resolution.
  - **Session Management:** `pos_terminal_sessions` table exists (Migration 106).
  - **Square Provider:** Basic inventory access in `square/provider.rs`.
  - **Frontend:** There are existing POS flows documented in `e2e_pos_flow.md` but next.js UI routes need completion.

  ## Problem Statement
  For business owners like Priya (Boutique) and Carlos (Field Service), the physical checkout experience must be flawless, fast, and unified with their online catalog. The current OHC implementation has scattered backend pieces (Stripe intents, offline sync, basic cart) but lacks a cohesive, mobile-first, offline-capable Point of Sale architecture that ties catalog, variants, cart, and payment (Stripe Terminal/Square) together into a single "grandmother-test" approved flow.

  ## Design Doc

  ### High-Level Architecture (Hybrid POS)

  ```mermaid
  graph TD
      A[Mobile/Tablet App (Flutter/PWA)] -->|1. Fetch Catalog (Cached)| B(Edge Cache / Local SQLite)
      A -->|2. Add to Cart| C[Local Cart State]
      C -->|3. Initiate Payment| D{Connection State}
      D -->|Online| E[Stripe Terminal / Square API]
      D -->|Offline| F[Local Offline Queue]
      F -.->|Background Sync| G(Offline Sync API)
      E --> H(Payment Events / Ledger)
      G --> H
      A -->|4. Sync Session| I(Terminal Session API)
  ```

  ### Mobile UX Flow (375px First)
  1.  **Catalog View:** Grid/List of products. Translucent glass styling. High-contrast price tags.
  2.  **Variant Selection:** Tap product -> Bottom sheet opens for variant selection (e.g., Size/Color) and quantity.
  3.  **Cart Summary:** Sticky bottom bar showing total. Tap to expand full cart view.
  4.  **Checkout:**
      -   Select payment method (Tap-to-Pay, Card Reader, Cash).
      -   If Tap-to-Pay/Reader: Trigger native SDK (or simulated flow for web).
      -   Clear Success/Error state.
  5.  **Offline State:** Clear visual indicator (e.g., amber dot) if offline. Checkout flow remains identical, transactions queue locally.

  ### Key Design Decisions
  -   **Offline-First Read Path:** The catalog must be heavily cached locally (using PowerSync or standard local storage) to ensure zero-latency browsing.
  -   **Unified Cart Model:** The POS cart must mirror the online cart model (`cart_items` with `product_id` and `variant_id`), allowing a transaction started online to be finished in-store (omnichannel).
  -   **Terminal Abstraction:** The backend must abstract Stripe Terminal vs. Square Reader so the frontend just calls `/api/pos/intent` and handles the generic response.

  ## Implementation Prompt
  Implement the comprehensive POS checkout journey.
  1.  **Backend Unification:** Consolidate the POS API routes. Create a unified `/api/pos/checkout` endpoint that handles cart creation, variant validation, and initiates the payment intent (interfacing with `terminal_api.rs` logic). Ensure it handles offline sync payloads gracefully.
  2.  **Frontend POS Shell:** Build the mobile-first (375px) POS shell in `src/ui/next/src/app/pos/` (or designated directory). It must include the catalog grid, variant selector bottom sheet, and cart summary.
  3.  **Checkout Flow:** Implement the UI for selecting payment methods and handling the payment intent response. Ensure mock/simulated states are clearly handled for testing without physical hardware.
  4.  **E2E Verification:** Write Playwright E2E tests covering: browsing catalog, selecting a variant, adding to cart, and successful checkout (using mocked payment intent success).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
