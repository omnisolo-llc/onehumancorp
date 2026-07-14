issue_title: "OHC Architecture Design: Omnichannel Tap-to-Pay & Unified Inventory Sync"
issue_description: |
  ## 1. Problem Statement
  Small business owners like Priya (boutique owner) and Fatima (food cart operator) operate seamlessly between online demand and in-person transactions. Currently, OHC lacks a unified Point-of-Sale (POS) and Tap-to-Pay capability that natively integrates with online inventory and the AI work assistant feed. When an in-person sale occurs, the system must immediately reflect the transaction in the unified revenue dashboard, deduct inventory, and trigger necessary operational agents (e.g., restocking alerts, receipt generation) without requiring the owner to switch contexts or use a secondary terminal.

  ## 2. Research Report
  ### Market Context & Competitor Analysis
  - **Stripe Terminal & Shopify POS:** Both leverage native mobile SDKs to enable Tap-to-Pay directly on iOS/Android devices without additional hardware. Shopify tightly couples this with their unified inventory backend.
  - **Square:** Dominates physical POS by providing immediate, offline-tolerant transaction states and syncing to a centralized inventory.
  - **OHC's Gap:** While OHC supports Stripe Checkout Sessions for online links, it lacks the architecture to handle physical Tap-to-Pay (NFC) via the Flutter mobile client, coupled with real-time inventory decrementing and offline-tolerant sync via PowerSync.

  ## 3. Design Doc: Tap-to-Pay & Inventory Architecture

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      actor Priya
      participant Flutter Client (375px)
      participant Stripe Terminal SDK
      participant OHC API (gRPC)
      participant Ledger & Inventory (Postgres RLS)
      participant Operations Agent

      Priya->>Flutter Client: Initiates In-Person Sale
      Flutter Client->>OHC API: Request PaymentIntent (Tap-to-Pay)
      OHC API-->>Flutter Client: Return client_secret
      Flutter Client->>Stripe Terminal SDK: Collect Payment (NFC)
      Stripe Terminal SDK-->>Flutter Client: Payment Success Token
      Flutter Client->>OHC API: Confirm Transaction & Sync Inventory
      OHC API->>Ledger & Inventory: Decrement Stock, Record Revenue (Tenant Scoped)
      Ledger & Inventory-->>Operations Agent: Trigger low-stock alert if needed
      Operations Agent-->>Flutter Client: Update Work Feed (Real-time)
  ```

  ### Mobile UX Flow (375px First)
  1. **Cart & Checkout:** The user adds products from the catalog to the cart via large, touch-friendly grid items (44x44px min).
  2. **Payment Modality:** A prominent, translucent glass-styled "Tap to Pay" floating action button appears.
  3. **NFC Interaction:** The native OS Tap-to-Pay sheet overlays. It is fully operable on a 375px screen without horizontal scrolling.
  4. **Confirmation & AI Feed:** Upon success, a receipt is auto-emailed (via Customer Agent), and the owner's primary feed reflects the new revenue and any low-stock warnings instantly.

  ### AI Agent Integration Points
  - **Operations Assistant:** Monitors inventory thresholds post-transaction and queues restock tasks in the unified feed.
  - **Finance Assistant:** Reconciles the Tap-to-Pay transaction instantly against the daily summary ledger.
  - **Customer Assistant:** Matches the card's last-4 or associated email (if requested) to the customer record, updating their LTV.

  ### Key Design Decisions
  - **Native Tap-to-Pay:** Utilize Stripe Terminal SDK in the Flutter frontend, avoiding physical dongles.
  - **Zero-Trust Isolation:** All API endpoints handling physical transactions must enforce strictly authenticated `tenant_id` context via SPIFFE/SPIRE.
  - **Optimistic UI Updates:** The Flutter app uses PowerSync to instantly reflect inventory decrements locally, reconciling with the backend asynchronously to handle slow networks (e.g., Fatima's food cart).

  **Estimated Scope**: Large

  ## 4. Implementation Prompt
  **Implementer Agent Task:**
  Create the backend foundation for in-person sales and Tap-to-Pay integration.
  - **CUJ:** A non-technical owner (e.g., boutique operator) selects an item on their mobile app and clicks "Tap to Pay" to process an in-person transaction, resulting in updated inventory and revenue.
  - **Requirements:**
    1. Define the data model for `pos_transactions` and `inventory_ledgers` ensuring strict PostgreSQL RLS by `tenant_id`.
    2. Create the backend API routes (REST/gRPC) to handle PaymentIntent generation specifically for Stripe Terminal.
    3. Ensure that when a POS transaction is confirmed, it atomically decrements inventory and records the payment.
    4. Provide E2E Playwright tests verifying the checkout flow UI (mocking the hardware NFC interaction) and the correct data persistence.
    5. Maintain the Premium Translucent Glass styling on all new UI components for the 375px mobile viewport.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
