issue_title: "Mobile-First Tap-to-Pay POS Sync & Inventory Architecture"
issue_description: |
  **Title**: Mobile-First Tap-to-Pay POS Sync Architecture

  **Problem Statement**:
  Small business owners like Priya (boutique operator) and Carlos (field service owner) operate heavily in-person and often on slower networks. They need seamless in-person tap-to-pay checkout that synchronizes instantaneously with their online inventory and bookings to prevent double-selling. Currently, OHC lacks a robust, offline-tolerant mobile POS synchronization architecture with optimistic UI and Redis-backed distributed locks for inventory reservation. Without this, in-person sales can clash with online orders, causing customer frustration and operational chaos.

  **Research Report**:
  - **Market Analysis**: Competitors like Square and Stripe Terminal provide excellent hardware and basic app checkout, but their integration into a broader, multi-channel business assistant is often disjointed or requires manual data reconciliation. Shopify POS is robust but overly complex and expensive for micro-SMEs. GoDaddy and Wix offer basic in-person payments but lack AI agent coordination for post-transaction workflows (e.g., automatic restock drafting, personalized follow-ups).
  - **Gap**: OHC's backend currently does not provide an optimistic, edge-cached, offline-tolerant tap-to-pay flow that guarantees inventory consistency via Redlock during the critical seconds of a transaction.

  **Design Doc**:
  - **Architecture Diagram (Mermaid.js)**:
    ```mermaid
    sequenceDiagram
      participant App as Mobile POS (Flutter)
      participant Redis as Redlock
      participant Stripe as Stripe Terminal
      participant DB as PostgreSQL Ledger
      participant Agent as Operations Agent

      App->>Redis: Request short-lived lock for inventory (15s)
      Redis-->>App: Lock granted
      App->>Stripe: Process Tap-to-Pay
      Stripe-->>App: Payment Success
      App->>DB: Commit transaction & deduct inventory
      DB-->>App: Success
      App->>Redis: Release lock
      DB->>Agent: Event: Inventory deducted
      Agent-->>Agent: Evaluate stock level & trigger restock alert if low
    ```
  - **Mobile UX Flow (375px first)**:
    1. Owner opens OHC app on phone.
    2. Taps "New Sale" or selects an existing booking/order.
    3. UI displays items/services with large touch targets (44x44px minimum).
    4. Taps "Charge" -> Prompts customer for Tap-to-Pay (native device integration).
    5. UI optimistically updates inventory counts locally to prevent double-tapping.
    6. Success screen shows payment confirmed, displaying updated stock level using OHC Premium Token styling (translucent materials, clean hierarchy).
  - **AI Agent Integration Points**:
    The Operations Agent acts as a background observer, monitoring the PostgreSQL ledger. Once a transaction is committed, it updates the agent's tenant-scoped memory and triggers a restock suggestion or online storefront update. The Customer Assistant can also draft a personalized "Thank You" receipt based on customer preferences.
  - **Key Design Decisions**:
    - Use Redis Redlock (`ohc:lock:{tenant_id}:inventory:{product_id}`) for short-lived, low-latency inventory reservation during tap-to-pay.
    - Implement optimistic local state updates in the Flutter mobile client to ensure a snappy 375px experience, even on slow networks.
    - Employ an eventual-consistency queue for offline mode, resolving conflicts asynchronously via the Operations Agent.

  **Implementation Prompt**:
  Implement the Mobile-First Tap-to-Pay POS Sync architecture. Start by setting up the Redis Redlock mechanism for inventory reservation during the checkout flow in the backend. Integrate the Stripe Terminal SDK for the tap-to-pay interface in the frontend app. Update the mobile UI (375px minimum viewport) to display optimistic inventory changes with clear, translucent glass styling. Finally, configure the Operations Agent to listen for inventory deduction events and generate owner alerts when stock runs low. The Critical User Journey (CUJ) must demonstrate a successful tap-to-pay transaction on a mobile device that instantly deducts inventory and triggers an agent restock notification.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
