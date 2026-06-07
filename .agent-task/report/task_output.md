issue_title: "Implement TerminalSession Schema and Stripe Terminal Sync"
issue_description: |
  **Problem Statement**
  Micro-SMEs and boutique operators (like Persona: Priya) struggle with disjointed POS and online inventory. Currently, the POS system lacks a clear `TerminalSession` schema and offline-sync reconciliation that bridges physical Stripe Terminal sales with the PostgreSQL central ledger, leading to double-booking and out-of-sync inventory across channels.

  **Research Report**
  Competitors like Shopify provide unified POS, but their implementation is complex and often requires additional plugins. For OHC, integrating a seamless, offline-tolerant `TerminalSession` structure is essential for syncing POS hardware transactions (like Stripe Terminal) with the central inventory ledger and Operations Agent. Currently, while `pos_offline_transactions` exists, there is no specialized schema representing a physical terminal session or its sync reconciliation state, which is necessary for robust real-world usage.

  **Design Doc**
  - **Data Schema:** Create a `pos_terminal_sessions` table in PostgreSQL with fields for `session_id`, `tenant_id`, `hardware_id` (Stripe Reader ID), `status` (active, paused, offline, reconciled), and `last_synced_at`.
  - **Architecture:** Add a new `TerminalSession` service in the Rust backend to manage the lifecycle of a terminal session.
  - **AI Coordination:** The Operations Agent will monitor the sync status of terminal sessions and notify the owner if a terminal has been offline for too long with pending sales.
  - **Mobile UX Flow:** The POS UI (375px) will show a "Terminal Status" indicator (Online/Offline/Syncing) and gracefully handle network drops by queuing transactions to the offline sync worker based on the active terminal session.

  **Implementation Prompt**
  Implement the `TerminalSession` data schema and service.
  1. Add a Goose migration for `pos_terminal_sessions` with proper tenant isolation (RLS).
  2. Implement gRPC endpoints for `StartTerminalSession`, `UpdateTerminalSessionStatus`, and `EndTerminalSession`.
  3. Ensure the mobile POS client (or UI tests) can gracefully indicate terminal connection state and link offline transactions to their respective `session_id`.
  4. Write E2E Playwright tests simulating an offline-to-online reconciliation flow for a terminal session.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
