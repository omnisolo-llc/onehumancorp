issue_title: "Offline-Tolerant Agentic POS & Tap-to-Pay System"
issue_description: |
  **Mission Queue Protocol Brief: Offline-Tolerant Agentic POS & Tap-to-Pay System**

  ## Problem Statement
  Physical and mobile operators (Priya the boutique owner, Fatima the food cart operator, Carlos the handyman) operate in environments with flaky network connectivity and need to accept in-person payments seamlessly. Currently, small business platforms require bolting on complex third-party POS hardware or using separate apps that don't sync with the central AI assistant's inventory or customer context. When Fatima loses cellular data at her food cart, she cannot process transactions, leading to lost revenue and frustrated customers. A non-technical owner needs an invisible, zero-setup Tap-to-Pay system on their existing phone (iOS/Android) that remains functional offline and automatically reconciles when connectivity is restored, all while the AI agent tracks inventory and customer preferences.

  ## Research Report
  - **Market Context**: Stripe Terminal provides Tap-to-Pay on iPhone/Android, but requires significant integration effort. Square dominates the SMB POS space due to its offline mode and easy hardware, but it lacks deep, autonomous AI integration for customer follow-ups and inventory prediction. Shopify POS is robust but heavily tied to their e-commerce ecosystem, often requiring expensive hardware add-ons for full capability.
  - **Competitive Gaps**:
    - *Square*: Excellent offline capabilities, but the system is passive; the owner must manually pull reports and initiate customer re-engagement.
    - *Shopify POS*: Complex setup for a simple food cart or handyman; hardware dependency.
    - *Wix/Squarespace*: Weak native in-person POS offerings; relies on Stripe integrations that are often clunky and not truly offline-first.
  - **The OHC Opportunity**: By natively integrating Stripe Tap-to-Pay directly into the OHC mobile app (Flutter) with an offline-first SQLite synchronization engine (PowerSync), OHC can turn any owner's smartphone into an enterprise-grade, AI-powered POS without extra hardware. The AI assistant can seamlessly bridge the gap between offline transactions and online inventory/customer data.

  ## Design Doc
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[OHC Mobile App - Flutter] --> B(Local SQLite DB - PowerSync)
      A --> C{Network Available?}
      C -- Yes --> D[Stripe Terminal SDK]
      C -- No --> E[Offline Transaction Queue]
      D --> F[Stripe API]
      E --> B
      B --> G[PowerSync Service]
      G --> H[(PostgreSQL - Multi-Tenant)]
      H --> I[AI Operations Agent]
      H --> J[AI Finance Agent]
      I --> K[Inventory Reconciliation]
      J --> L[Daily Revenue Summary]
  ```

  ### Mobile UX Flow (375px)
  1. **Checkout Initiation**: From the OHC Assistant Feed, the owner taps a floating "New Sale" button (44x44px minimum touch target).
  2. **Cart/Amount Entry**: A clean, high-contrast numpad appears. The owner enters the amount or selects items from a visual catalog (optimized for Fatima's fast-paced food cart).
  3. **Tap-to-Pay Overlay**: The native OS Tap-to-Pay interface slides up. If offline, the UI clearly indicates "Offline Mode - Payment Queued" using a subdued translucent glass status pill, ensuring the owner knows the transaction is safely stored.
  4. **Instant Receipt & AI Action**: Upon completion, the screen shows a success checkmark. The AI Customer Assistant immediately drafts a digital receipt SMS/email if the customer is recognized via a loyalty tap or previous interaction.

  ### AI Agent Integration Points
  - **Operations Agent**: Listens to the transaction sync queue. When offline transactions hit the central PostgreSQL database, it automatically adjusts inventory levels and resolves any overselling conflicts intelligently.
  - **Finance Agent**: Aggregates both online and in-person Tap-to-Pay transactions to generate plain-language daily summaries (e.g., "You made $450 at the cart today, mostly from the new chicken special. 3 transactions synced after you came back online.").
  - **Customer Assistant**: Associates the payment method with existing customer profiles to build a unified view of their online and offline purchasing habits, drafting personalized follow-ups.

  ### Key Design Decisions
  - **Offline-First Synchronization**: Utilizing PowerSync (local SQLite to central Postgres) ensures that product catalog reads and transaction writes are instantaneous and durable, regardless of cellular coverage.
  - **Zero-Hardware Dependency**: Leveraging native iOS/Android Tap-to-Pay SDKs removes the barrier to entry. No dongles or card readers to pair, charge, or lose.
  - **Translucent Glass & UniFi Layouts**: The POS interface uses high-contrast typography and clear visual hierarchy (cards) to remain legible outdoors in bright sunlight.

  ## Implementation Prompt
  **Target Persona**: Fatima the Food Cart Operator, Carlos the Handyman
  **CUJ**: Fatima needs to quickly ring up a customer for a $12 meal, accept a contactless card payment by tapping it to her Android phone, and have the transaction recorded and inventory updated—even if her food cart is in a cellular dead zone.

  **Next Actions**:
  1. **Data Model**: Extend the `Transaction` and `Order` models in PostgreSQL to support `offline_queued` states, `sync_timestamp`, and `terminal_id`. Implement strict Row-Level Security (RLS) based on `tenant_id`.
  2. **Mobile Integration**: Integrate the Stripe Terminal SDK for Tap-to-Pay within the Flutter application.
  3. **Offline Sync**: Implement the PowerSync local-first data layer in the Flutter app to cache the product catalog and queue offline transactions.
  4. **Agent Coordination**: Create the event listeners for the Operations and Finance agents to process synced transactions, update inventory, and generate the daily plain-language summary.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
