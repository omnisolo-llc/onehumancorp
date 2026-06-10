issue_title: "[Architecture] Autonomous Offline-First Tap-to-Pay POS Engine"
issue_description: |
  ## Problem Statement
  Priya (boutique owner) and Carlos (field service owner) often operate in areas with poor or no internet connectivity (e.g., inside concrete buildings, rural service areas). They need to process in-person payments (Tap-to-Pay) and record sales/invoices seamlessly. If the system blocks them from taking payment or viewing inventory without a strong connection, they lose revenue. Current cloud-dependent checkout flows fail our core persona needs when network latency spikes or drops entirely.

  ## Research Report
  - **Competitor Landscape:**
    - Shopify POS and Square handle offline payments by securely caching authorized transactions locally and batch-syncing them when connectivity is restored.
    - Stripe Terminal SDK supports offline transaction staging.
  - **Market Gap:** Most generic AI platforms fail to integrate hardware SDKs (like NFC Tap-to-Pay) with a local optimistic mutation layer that automatically syncs via the core backend once online.
  - **Persona Fit:** Priya needs zero-friction checkout regardless of store Wi-Fi stability. Carlos needs to finalize invoices and take payment in a customer's basement.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as Owner (Mobile UI)
      participant POS as POS Engine (Local Store/SQLite)
      participant NFC as Tap-to-Pay SDK (Stripe/Hardware)
      participant Server as OHC Backend
      participant Agent as Sales/Operations Agent

      Owner->>POS: Initiates Checkout/Invoice Payment
      POS->>POS: Optimistically updates local ledger
      POS->>NFC: Request NFC Payment Authorization
      NFC-->>POS: Offline Auth Token / Crypto Cryptogram
      POS-->>Owner: Success UI (Translucent Glass)

      Note over POS,Server: Background Sync when Online
      POS->>Server: Batch Sync Offline Transactions
      Server->>Server: Validate & Commit to Postgres
      Server->>Agent: Trigger Post-Sale Workflows (Inventory, Receipts)
  ```

  ### Mobile UX Flow (375px)
  1. **Checkout Screen:** Large, bold typography displaying the total amount. A full-width, primary action button "Tap to Pay".
  2. **Payment Modal:** A slick, translucent modal pops up instructing the user to hold their card near the device.
  3. **Offline Indicator:** A subtle, non-intrusive badge indicates "Offline Mode" if there's no connection, ensuring the owner knows the state but is not blocked.
  4. **Success State:** Instant visual feedback (check mark with haptic vibration) confirming the transaction, even if staged locally.

  ### AI Agent Integration Points
  - **Sales Agent:** Monitors the background sync queue. Once an offline transaction syncs, it automatically updates inventory counts and drafts the email receipt.
  - **Operations Agent:** Flags discrepancies if offline transactions fail validation upon sync, creating a prioritized task in the owner's feed to resolve the payment issue.

  ### Key Design Decisions
  - **Optimistic UI:** Always show success instantly upon local hardware auth, deferring network consensus.
  - **Local Persistence:** Utilize the existing local SQLite SIPDB architecture for durable offline staging of transactions.
  - **Security:** Ensure that offline NFC auth tokens are securely encrypted at rest before sync.

  ## Implementation Prompt
  **Goal:** Implement the foundation for the Autonomous Offline-First Tap-to-Pay POS Engine.
  **CUJ:** Priya completes an in-person checkout via Tap-to-Pay on her mobile device while disconnected from the internet. The transaction is recorded locally and synced when she regains connectivity, triggering her inventory to update.
  **Acceptance Criteria:**
  1. Create the necessary schema/data models to support staged offline transactions (e.g., `OfflineTransactionQueue`).
  2. Implement the local mutation logic to optimistically update the checkout state.
  3. Build the background sync manager that attempts to flush staged transactions to the backend.
  4. Integrate the sync manager with the existing Agent event bus to trigger post-sale actions.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
