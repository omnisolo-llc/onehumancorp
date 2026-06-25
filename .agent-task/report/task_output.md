issue_title: "[Research] AI-Driven Offline-First POS Architecture for Intermittent Connectivity"
issue_description: |
  # Research Report: AI-Driven Offline-First POS Architecture for Intermittent Connectivity

  ## 1. Problem Statement
  Small business owners and operators, particularly in field services (like Carlos, the handyman) or mobile food carts (like Fatima), frequently operate in environments with poor or intermittent internet connectivity. Current cloud-first Point-of-Sale (POS) systems either fail completely without a network or provide a crippled "offline mode" that leads to data loss, double-booking of inventory, and broken reconciliation when connectivity is restored. This frustrates users and erodes trust in the platform, directly impacting their revenue.

  ## 2. Research Report
  Our analysis of the market reveals a critical gap. Traditional e-commerce giants (Shopify, BigCommerce) and modern website builders (Wix, Squarespace) excel in cloud connectivity but treat offline capability as an afterthought. Their POS applications require constant syncing to prevent inventory discrepancies. Even dedicated POS providers (Square) struggle with complex asynchronous reconciliation when multiple offline transactions conflict with online sales upon reconnection.

  AI-native tools have largely focused on website generation (Durable, 10Web) or online workflows, ignoring the physical reality of hybrid and field operators. OHC's mission to serve personas like Fatima and Carlos requires a robust, Offline-First architecture where AI agents seamlessly handle conflict resolution during intermittent sync events without burdening the operator.

  ## 3. Design Doc: High-level Architectural Design

  **Architecture Diagram:**
  ```mermaid
  sequenceDiagram
      participant Mobile as Mobile Client (Flutter)
      participant API as Backend API
      participant Agent as Operations Assistant (Gemini Pro)
      participant DB as Central Ledger (PostgreSQL)

      Note over Mobile: Offline Mode
      Mobile->>Mobile: Operator completes transaction
      Mobile->>Mobile: Log transaction with client_mutation_id (CRDT principles)
      Mobile-->>Operator: "Saved Locally" UI banner

      Note over Mobile,API: Network Reconnected
      Mobile->>API: POST /api/offline_sync (Batch Sync Request)
      API->>DB: Attempt Idempotent Upsert (using client_mutation_id)

      alt Conflict detected (e.g. oversold inventory)
          DB-->>API: Conflict Error
          API->>Agent: Analyze Conflict & Request Resolution
          Agent-->>API: Resolution Strategy / Triage Alert
          API->>DB: Apply Resolution (if auto-resolvable) or Flag for Review
      else No conflict
          API->>DB: Commit Transaction
      end

      API-->>Mobile: Sync Success / Resolution State
      Mobile-->>Operator: Update Sync Status UI & Inbox
  ```

  **Mobile UX Flow (375px first):**
  - **The "Truthful State" Banner:** A small, non-intrusive banner or icon on the mobile POS indicates the current sync state (Online, Offline - Logging locally, Syncing...). This sets clear expectations.
  - **Seamless Transaction Flow:** The checkout flow (cart, total, payment collection) remains identical regardless of connectivity. The operator taps "Charge," and if offline, the app immediately shows a "Saved Locally" confirmation, allowing them to serve the next customer instantly.
  - **Agentic Reconciliation Inbox:** When back online, the Operations Assistant provides a simple notification: "Synced 15 offline orders. No conflicts found." Or, if a conflict occurs (e.g., an item sold offline was also purchased online simultaneously), it presents a plain-language resolution prompt: "Item X oversold by 1. Should we refund the online order or restock from reserve?"

  **AI Agent Integration Points:**
  - **The Local Sentinel (Edge Agent):** A lightweight client-side logic module that queues transactions, applying deterministic timestamps and UUIDs to prevent duplication. It monitors network health to trigger sync bursts.
  - **The Reconciliation Arbiter (Operations Assistant):** When the offline sync batch hits the backend, this agent analyzes the payload against current state. It intelligently resolves simple conflicts (e.g., sequential inventory deductions) and flags complex ones (e.g., negative inventory) for human review via the Triage feed.

  **Key Design Decisions:**
  - **Eventual Consistency Model:** The central PostgreSQL ledger remains the single source of truth, but the mobile client operates on a locally cached, eventually consistent view using CRDTs (Conflict-free Replicated Data Types) principles for simple counters (like inventory).
  - **Idempotency is King:** Every offline transaction payload must include a unique transaction identifier to ensure retries during flaky network conditions do not result in double charges or double inventory deductions.

  ## 4. Implementation Prompt
  **Role:** Backend & Mobile Implementer
  **Outcome:** Enhance the offline synchronization workflow to robustly handle intermittent network bursts and integrate with an AI agent to resolve inventory conflicts arising from offline POS transactions.

  **CUJ (Critical User Journey):**
  1. Fatima (Food Cart Operator) is at a festival with zero reception. She processes 5 cash/pre-authorized tap orders via the OHC app.
  2. The app stores these locally and displays a reassuring "Saved Offline" status.
  3. Fatima gets a brief 3G connection. The app automatically pushes the batched synchronization request to the backend.
  4. The backend processes the batch idempotently. If one of the items she sold also sold online (causing negative inventory), the Operations Assistant intercepts it.
  5. Instead of an error crash, Fatima gets a notification in her Triage feed: "We synced your 5 offline orders. 'Spicy Chicken Wrap' oversold by 1. Action needed."

  **Acceptance Criteria:**
  - Enhance the data structures to handle more complex conflict scenarios (e.g., explicitly tracking previous known version/state).
  - Implement idempotent processing for offline synchronization to guarantee no double-counting of offline transactions even if the client retries the payload multiple times.
  - Create a lightweight test suite demonstrating that simultaneous offline deductions and online purchases of the same SKU result in a structured conflict object being generated rather than silently corrupting the ledger.

  ## 5. Priority & Scope
  **Priority:** P1 (High) - Critical for adoption by mobile-first field/hybrid operators.
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
