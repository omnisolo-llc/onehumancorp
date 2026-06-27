issue_title: "Offline-First Distributed Sync Architecture for Low-Bandwidth Operations"
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_description: |
  # Research Report: Offline-First Distributed Sync Architecture for Low-Bandwidth Operations

  ## Problem Statement
  Small business operators in fast-paced, sometimes outdoor environments (like Fatima the Food Cart Operator) frequently face unreliable or slow mobile networks. When connectivity drops, they still need to accept pre-orders, mark items as picked up, and toggle menu item availability. Currently, while there is backend support for `OfflineMutation` and `CrdtDelta` structures, the system lacks a cohesive, mobile-first UX (375px) that gracefully handles offline order ingestion and provides truthful pending states during intermittent connectivity. Without this, users experience frozen UIs, lost data, and ultimately, lost revenue during peak hours.

  ## Research Report (Track 1)
  **Market Findings & Competitive Analysis:**
  - **Square & Toast:** Both offer robust offline modes for their POS terminals, allowing continued card processing (up to a limit) and order taking. However, they rely on proprietary hardware or heavy tablet apps, not a lightweight mobile-first web/app experience.
  - **Shopify POS:** Offers offline cash transactions but struggles with complex offline inventory syncing without network access.
  - **Wix/Squarespace:** Primarily rely on constant connectivity. Their web-based builders do not offer a native offline-first operational mode.
  - **OHC Opportunity:** By leveraging our existing `CrdtDelta` and `OfflineMutation` backend capabilities and coupling them with a robust local-first frontend architecture (e.g., SQLite/IndexedDB in Flutter/PWA), OHC can provide an uninterrupted operational experience. The Operations Agent can autonomously reconcile state once connectivity is restored, completely abstracting the sync complexity from the owner.

  ## Design Doc (Track 2 & Track 3)
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Write| B(Local SQLite/IndexedDB)
      B --> C{Network Available?}
      C -->|No| D[Queue OfflineMutation]
      C -->|Yes| E[Sync Gateway API]
      D -.->|Network Restored| E
      E --> F[Operations Agent]
      F -->|Reconcile| G[PostgreSQL / CrdtDelta Table]
      F -->|Alert if Conflict| A
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The top bar displays a subtle "Network Health" indicator (e.g., a green dot for online, an amber "Offline - Changes Saved" badge when disconnected).
  - **Interaction:** Fatima taps a menu item to mark it "Sold Out". The UI updates instantly (optimistic UI). The change is written to local storage.
  - **Pending States:** Any order marked as "Picked Up" while offline shows a small syncing icon next to it.
  - **Restoration:** When connectivity returns, the pending icons disappear as the `OfflineSyncRequest` successfully processes. If a conflict occurs (e.g., an online pre-order came in for the sold-out item simultaneously), the Operations Agent triggers a "Triage" card in the feed asking Fatima how to handle it.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Acts as the reconciliation engine. When `OfflineMutation` payloads arrive, it compares the local timestamps with the `CrdtDelta` state. It resolves simple conflicts automatically (e.g., merging inventory counts) and escalates complex ones (e.g., double-booked limited inventory) to the owner's feed via plain-language prompts.

  ### Key Design Decisions
  - **Optimistic UI:** Every action in the operational dashboard must feel instant, regardless of network state.
  - **Truthful Status:** The user must clearly see what is synced and what is pending, building trust in the system's reliability.
  - **Agentic Reconciliation:** The burden of resolving sync conflicts is shifted from rigid database constraints to the AI, which can make contextual business decisions.

  ## Implementation Prompt
  **User-Facing Outcome:** As a food cart operator (Fatima), I can continue to toggle my menu availability and mark orders as picked up even when my 4G connection drops. The app feels instant, shows me that my changes are saved locally, and automatically syncs them when my connection returns, without me losing any data or having to understand "sync errors".

  **CUJ & Acceptance Criteria:**
  1. Fatima logs into the OHC mobile app (375px view).
  2. The network connection is simulated as offline.
  3. Fatima marks an order as "Picked Up" and toggles a menu item to "Sold Out".
  4. The UI reflects these changes instantly and displays an "Offline - Changes Saved" indicator.
  5. The network connection is restored.
  6. The app autonomously sends the `OfflineSyncRequest` with the queued `OfflineMutation`s.
  7. The backend processes the mutations, updates the `CrdtDelta` state, and the UI indicators clear, confirming synchronization.
  8. Provide Playwright E2E tests: A user interacts with the UI in an offline state (mocked via Playwright network routing), verifies the pending UI state, restores the network, and verifies the successful sync and UI update.

  **Priority**: P1
  **Estimated Scope**: Large
