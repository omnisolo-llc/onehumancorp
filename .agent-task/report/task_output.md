issue_title: "[architecture] Implement Offline-First Omni-Channel POS & Tap-to-Pay"
issue_description: |
  **Title**: Implement Offline-First Omni-Channel POS & Tap-to-Pay

  **Problem Statement**:
  Business owners like Priya (Boutique Operator) and Fatima (Food Cart Operator) operate in environments where they need to process in-person payments securely, but they frequently encounter poor internet connectivity or need a seamless link between online inventory and in-person sales. Currently, OHC lacks a robust, offline-tolerant Point-of-Sale (POS) and Tap-to-Pay terminal architecture that can seamlessly sync transactions and inventory reductions with the cloud backend without losing data or stalling the checkout process on a 375px mobile screen.

  **Research Report**:
  - **Persona Needs**:
    - Priya needs to tap a customer's card and immediately see her online inventory drop by 1 to prevent double-selling.
    - Fatima needs her offline Android device to queue up payments and inventory changes during peak lunch hours (when cell service drops) and reconcile them automatically when the connection is restored.
  - **Competitor Analysis**:
    - *Shopify POS*: Relies heavily on constant connectivity for inventory sync, although it offers basic offline cash processing.
    - *Square*: Excels at offline mode for card processing (storing encrypted swipes/taps) and auto-syncing when back online, setting a high standard for small-business expectations.
  - **Architectural Gaps**:
    - The existing `src/server/services/pos` and `src/server/integrations/stripe` modules lack a unified local-first queueing mechanism to safely hold pending transactions and inventory deducts.
    - Our multi-tenant Postgres backend needs an idempotency and conflict-resolution layer to handle bulk POS syncs when devices come back online.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
        actor Owner
        participant MobileApp as OHC Mobile App (Flutter)
        participant LocalStore as Local SQLite/KV Queue
        participant Tap2Pay as Native Tap-to-Pay API
        participant CloudAPI as OHC Backend API
        participant DB as Multi-Tenant PostgreSQL

        Owner->>MobileApp: Add items to cart & select Tap-to-Pay
        MobileApp->>Tap2Pay: Process Payment (Tokenize)
        Tap2Pay-->>MobileApp: Encrypted Token
        MobileApp->>LocalStore: Enqueue Transaction & Inv Sync (Offline-First)
        MobileApp-->>Owner: Success (UI Update on 375px)

        loop When Network Available
            MobileApp->>CloudAPI: Flush local queue (Idempotent Sync)
            CloudAPI->>DB: Process payment & deduct inventory (Row-Level Security)
            DB-->>CloudAPI: Confirmed
            CloudAPI-->>MobileApp: Acknowledge sync
            MobileApp->>LocalStore: Clear synced items
        end
    ```
  - **Mobile UX Flow (375px Baseline)**:
    - **Cart Screen**: A clean, touch-friendly grid of items (min 44x44px targets). The total is sticky at the bottom.
    - **Checkout Modal**: Tapping "Pay" slides up a translucent glassmorphism bottom sheet with the total and a pulsating "Tap to Pay" target.
    - **Offline Indicator**: A subtle, non-intrusive indicator (e.g., an amber cloud icon) shows if the device is offline, but it *never* blocks the tap-to-pay workflow.
    - **Sync State**: A background progress indicator handles sync status transparently to the user.
  - **AI Agent Integration**:
    - *Finance & Decision Assistant*: Monitors the sync queue. If a transaction fails to reconcile after 24 hours, it drafts a plain-language summary for the owner: "One of yesterday's card taps couldn't be processed. Here's what to do."
    - *Operations Assistant*: Pauses online availability for items marked as sold offline until the global inventory is fully reconciled.

  **Implementation Prompt**:
  Implement the offline-tolerant Omni-Channel POS and Tap-to-Pay architecture.
  1. Add a durable local-first queueing mechanism in the mobile app layer to store pending checkout sessions and inventory reservations.
  2. Implement an idempotent `/api/v1/pos/sync` endpoint in the backend that processes these queued transactions, validates Stripe terminal tokens, and safely updates the Postgres inventory tables using the tenant's row-level security context.
  3. Ensure the mobile UX uses a 375px-optimized glassmorphism checkout flow that allows the user to continue operating without interruption even when the network drops.
  4. Integrate the Finance AI Assistant to detect sync failures and surface them as actionable work items in the owner's feed.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
