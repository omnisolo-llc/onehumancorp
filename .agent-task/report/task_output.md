issue_title: "Architecture: Unified Offline-Tolerant Tap-to-Pay mPOS"
issue_description: |
  **Problem Statement**
  Owners like Priya (boutique), Fatima (food cart), and Carlos (handyman) operate in physical spaces where network connectivity is often flaky. They need to capture payments instantly without standalone hardware. They need the OHC mobile app to act as an offline-tolerant Point of Sale (mPOS) that uses modern "Tap to Pay on iPhone/Android" APIs via Stripe Terminal, deeply integrated with the unified work feed. Currently, OHC lacks a dedicated, robust mobile-native payment terminal architecture that handles spotty networks while updating AI agents invisibly.

  **Research Report**
  - **Competitor Analysis:**
    - **Shopify POS Go & Stripe Terminal:** Excellent offline catalog caching, fast tap-to-pay via NFC. Requires a separate POS app; we want it unified in the main OHC assistant.
    - **Square:** The gold standard for mPOS. Handles offline payments by queueing encrypted card data (catalog and cart creation works offline).
  - **Target Capability:** Provide "Tap to Pay" directly in the OHC mobile app using Stripe Terminal SDK, with an edge-cached catalog powered by local storage to ensure the owner can ring up customers even with poor connection.

  **Design Doc**
  *Architecture Diagram:*
  ```mermaid
  sequenceDiagram
    participant Owner
    participant App as Flutter App (Local Cache)
    participant Terminal as Stripe Terminal SDK
    participant Backend as OHC Backend
    participant Agent as Finance AI Agent

    Owner->>App: Open Catalog (Offline-Tolerant)
    App-->>Owner: Display Items (from Cache)
    Owner->>App: Add items to Cart & Tap to Pay
    App->>Terminal: Initiate PaymentIntent
    Terminal-->>Owner: Prompt NFC Tap
    Owner->>Terminal: Tap Card
    Terminal->>Stripe: Process Payment
    Stripe-->>App: Payment Success
    App->>Backend: Record Ledger Entry (Sync Queue)
    Backend->>Agent: Trigger Ledger Update
  ```

  *Mobile UX Flow (375px):*
  1. **Quick Charge**: Prominent card on the main assistant feed.
  2. **Cart Building**: Fast, cache-first list of products/services with 44x44px touch targets.
  3. **Payment Modality**: Translucent glass-styled bottom sheet sliding up, showing "Tap to Pay" (NFC).
  4. **Success State**: Clear translucent checkmark with an option to instantly "Text Receipt".

  *AI Agent Integration:*
  - **Work Triage:** Records the transaction instantly so the day's total updates.
  - **Customer Assistant:** Matches card fingerprint to an existing customer if known, proposing a loyalty tag.
  - **Finance Assistant:** Summarizes today's tap-to-pay vs. online sales.

  **Implementation Prompt**
  You are the Implementer agent. Your task is to build the foundational Offline-Tolerant Tap-to-Pay UI and local caching layer.
  1. Implement a local data cache for the `ProductCatalog` to allow offline cart creation.
  2. Build the 375px-optimized "Quick Charge" bottom sheet following the OHC Translucent Glass token system.
  3. Integrate the Stripe Terminal SDK hooks (stub the actual hardware calls in E2E tests).
  4. Create the offline sync queue mechanism to retry failed recordings to the OHC backend.
  Ensure 100% of the UI passes the "grandmother test" and is fully operational on a 375px viewport. Verify all UI elements using Playwright.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
