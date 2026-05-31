issue_title: "[Architecture] Omnichannel Tap-to-Pay Terminal Mesh"
issue_description: |
  # Architecture Brief: Omnichannel Tap-to-Pay Terminal Mesh

  ## Problem Statement
  Small business owners like Priya (Boutique Owner) and Fatima (Food Cart Operator) sell both online and in-person. Currently, they use separate systems (e.g., Square for in-person, Shopify for online), leading to out-of-sync inventory, fragmented customer profiles, and a disjointed understanding of their business health. When Priya sells the last red dress in-store, her online store needs to instantly show it as "Sold Out." When Fatima takes a pre-order online, it needs to pop up on her phone alongside walk-up orders. They need a unified system where their phone acts as a secure POS (Tap-to-Pay) and syncs instantly with the cloud without any complex network setup.

  ## Research Report
  *   **Square:** Excellent at in-person, but their online store offering is basic. Unifying the two often requires third-party bridges or manual reconciliation.
  *   **Shopify POS:** Very powerful, but expensive. Requires a separate app and often specific hardware.
  *   **Stripe Terminal (Tap-to-Pay):** Allows merchants to accept contactless payments directly on compatible iPhones/Androids without extra hardware.
  *   **OHC Differentiation:** By leveraging Stripe Terminal SDKs for Tap-to-Pay directly within the OHC mobile app, we eliminate the need for extra hardware. By backing this with our distributed state machine (KAIROS) and local SIPDB, we ensure that an in-person sale instantly and invisibly updates the central inventory ledger, triggering the AI Operations agent to update the storefront and the Finance agent to reconcile the ledger.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Merchant as Priya (OHC App on iPhone)
      participant Customer as Customer (Apple Pay)
      participant Stripe as Stripe Terminal API
      participant Sync as OHC SyncDaemon (Local)
      participant KAIROS as OHC KAIROS Orchestrator
      participant Ledger as Global Ledger (PostgreSQL)

      Merchant->>Customer: Presents phone for Tap-to-Pay
      Customer->>Merchant: Taps phone (NFC)
      Merchant->>Stripe: Processes Payment Intent via SDK
      Stripe-->>Merchant: Payment Success
      Merchant->>Sync: Record local transaction & inventory decrement
      Sync-->>Merchant: UI Updates Instantly (Optimistic)
      Sync->>KAIROS: Background sync: `ProcessInPersonSale` event
      KAIROS->>Ledger: Update unified inventory & ledger
      KAIROS-->>Sync: Acknowledge Sync
  ```

  ### Key Components & Invariants
  1.  **Mobile Tap-to-Pay Integration:** The OHC Flutter/React Native app integrates the Stripe Terminal SDK. This turns any compatible smartphone into a secure POS terminal.
  2.  **Offline-Capable State (SIPDB):** If Fatima is in a spotty network area, the app must still record the sale locally. The transaction is queued in the local SIPDB.
  3.  **Unified Inventory Ledger:** The backend uses row-level locking (or Redis redlocks) to ensure that concurrent sales (e.g., someone buying online exactly when Priya sells the item in-store) do not result in overselling.
  4.  **AI Department Triggers:**
      *   **Operations:** Decrements inventory. If stock hits zero, instantly updates the online storefront to "Sold Out".
      *   **Finance:** Records the transaction, identifying it as "In-Person" vs. "Online" for the daily briefing.
      *   **Customer Success:** If the customer opted for a digital receipt via email/SMS, the agent formats and sends it.

  ### Mobile UX Flow (375px First)
  *   **POS Mode Toggle:** A prominent, thumb-friendly toggle at the top of the Home screen switches the app from "Dashboard" to "Register" mode.
  *   **Fast Catalog Search:** A grid of top-selling items with large touch targets (>= 44px).
  *   **Tap-to-Pay Screen:** Utilizes the native OS overlay for NFC payment collection.
  *   **Success Confetti:** Premium micro-interactions upon successful payment to provide positive feedback.

  ## Implementation Prompt
  **To Implementer Agent:**
  Design the backend data structures and gRPC service definitions for the Omnichannel Tap-to-Pay Sync Engine. Define the necessary proto definitions for POS interactions and offline synchronization. Define the database schema for unified inventory that tracks stock levels across multiple locations (e.g., "Online", "In-Store"). Ensure the API is idempotent to handle retries from the mobile client if network connectivity drops during sync. Do not implement the client-side Stripe SDK integration, focus strictly on the backend APIs and data models required to support the architecture described in the design doc. Ensure 100% test coverage for the service layer logic.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
