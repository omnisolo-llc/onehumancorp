issue_title: "Implement Terminal Tap-to-Pay Integration (Offline-Sync Compatible)"
issue_description: |
  # Architecture & Implementation Plan: Terminal Tap-to-Pay Integration

  ## Problem Statement
  Small business owners like Priya (Boutique) and Fatima (Food Cart) need to accept in-person payments (Tap to Pay) using their mobile devices. Current cloud-only POS solutions fail in unreliable networks. OHC needs a seamlessly integrated, offline-compatible POS system that uses Stripe Terminal to accept payments directly within the OHC mobile app, instantly updating inventory and the central ledger when online, and queuing transactions when offline.

  ## Research Report
  Our competitive analysis indicates that native Tap to Pay solutions (like Stripe Terminal's SDK) provide a critical advantage over external dongles by eliminating hardware barriers. The missing link in OHC is the connection between the OHC mobile frontend, the OHC backend, and the Stripe Terminal API, specifically handling connection tokens and payment intents that respect multi-tenant boundaries and the recently introduced `pos_terminal_sessions` offline-sync schema.

  ## Design Doc
  ### Mobile UX Flow (375px first)
  1. The user builds a cart on the POS screen and taps "Accept Contactless Payment".
  2. The OHC frontend fetches a secure Terminal Connection Token from the backend (`/api/v1/payments/terminal/token`).
  3. The frontend creates a Terminal PaymentIntent via the backend (`/api/v1/payments/terminal/intent`).
  4. (Frontend Simulator for tests): A "Simulate Tap" overlay appears.
  5. Upon success, the backend processes the sale, deducts inventory (respecting Redis Redlock if online), and updates the `pos_terminal_sessions` and ledger.

  ### AI Agent Integration
  *   **Operations Agent:** Monitors inventory depletion from POS sales and triggers restock workflows.
  *   **Finance Agent:** Reconciles offline transactions and payment splits.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant POS UI (Mobile)
      participant OHC Backend
      participant Stripe API

      POS UI (Mobile)->>OHC Backend: POST /api/v1/payments/terminal/token
      OHC Backend->>Stripe API: Request Connection Token
      Stripe API-->>OHC Backend: Token
      OHC Backend-->>POS UI (Mobile): Token

      POS UI (Mobile)->>OHC Backend: POST /api/v1/payments/terminal/intent
      OHC Backend->>Stripe API: Create PaymentIntent (card_present)
      Stripe API-->>OHC Backend: Client Secret
      OHC Backend-->>POS UI (Mobile): Client Secret
  ```

  ## Implementation Prompt
  Implement the backend and frontend components for the Universal Tap-to-Pay POS System.
  1.  **Backend APIs:** Ensure `/api/v1/payments/terminal/token` and `/api/v1/payments/terminal/intent` are fully implemented in `src/server/api/terminal_api.rs`, properly authenticating the tenant and using the `StripeClient` functions.
  2.  **Frontend POS:** Update `src/ui/tauri/src/ui/pos.html` to integrate with these endpoints. The "Accept Contactless Payment" button should trigger the token and intent creation flow. Include a "Simulate Customer Tap" button in the overlay for E2E testing.
  3.  **Data Integrity:** Ensure that the backend endpoints interact correctly with the `pos_terminal_sessions` table for session tracking and the `products` table for inventory deduction upon successful payment.
  4.  **Testing:** Add unit tests for the API endpoints and ensure a Playwright E2E test covers the POS checkout flow.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
