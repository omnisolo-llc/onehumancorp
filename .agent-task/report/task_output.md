issue_title: "AI-Buffered Offline-First Operations for Field Workers"
issue_description: |
  # AI-Buffered Offline-First Operations for Field Workers

  ## Problem Statement
  Field service workers (like Carlos the Handyman) and mobile vendors (like Fatima the Food Cart Operator) frequently operate in areas with spotty or no cellular service (basements, remote job sites, crowded street festivals). Legacy tools fail completely when offline, locking up the UI, losing data, or blocking essential workflows like capturing a lead, accepting a cash payment, or logging task completion.

  ## Research Report
  - **Market Gap:** While modern PWAs and mobile apps have offline capabilities, most small business SaaS tools treat offline mode as a read-only afterthought or fail aggressively with "Network Error" modals.
  - **The OHC Opportunity:** We can introduce "Agentic Buffering". The user continues to interact with the Assistant locally. The Assistant logs the intent to a local queue, optimistic UI updates are applied instantly, and the business owner feels no friction.
  - **Competitive Differentiation:** Unlike Shopify or Wix POS which simply queue transactions, OHC can queue *agentic intents* (e.g., "Draft a quote for $500 for this basement repair") and have the cloud agents fulfill the heavy lifting the moment connectivity is restored.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Offline] -->|User Action/Voice| B(Local Intent Parser)
      B --> C[Local SQLite/IndexedDB Sync Queue]
      C -->|Optimistic Update| D[Local Mobile View]
      C -->|Network Restored| E[Sync Engine Worker]
      E --> F[OHC API Gateway]
      F --> G[Cloud Agents & PostgreSQL]
  ```

  ### Mobile UX Flow (375px First)
  - **Status Indication:** A subtle but clear "Offline Mode" translucent pill in the status bar.
  - **Interaction:** The user inputs a command or completes a task (e.g., marking a service complete).
  - **Agent Feedback:** The assistant immediately replies: "Saved locally. I'll sync this and send the customer a receipt when we have a connection."
  - **Queue Visibility:** A dedicated card appears in the Unified Agent Feed showing X pending sync actions.
  - **Resolution:** When connectivity is restored, the queue card animates to a "Synced" state and disappears.

  ### AI Agent Integration Points
  - **Local Assistant:** Handles lightweight, deterministic intents locally (Add task, record offline payment, save note) to provide immediate feedback.
  - **Cloud Operations & Finance Agents:** Upon sync, the cloud agents process the queued intents, resolve any conflicts (e.g., timestamp overlaps), and generate the complex outputs (e.g., drafting and emailing a PDF receipt).

  ### Key Design Decisions
  - **Intent Queuing over Data Syncing:** Instead of syncing raw database rows, we sync user intents. This allows the cloud LLM to flexibly handle the operation when online.
  - **Absolute Mobile Reliability:** The app must launch instantly and function even if the device has been in airplane mode for 24 hours.

  ## Implementation Prompt
  **User-Facing Outcome:** As Carlos the Handyman, I can open the app in a client's basement with zero bars, tap to complete the job, and log a cash payment. The app responds instantly. When I drive away and get cellular service, the system syncs smoothly and automatically emails the client their receipt.

  **Next Actions:**
  1. Implement a robust intent sync queue in the local storage layer (IndexedDB for Web/PWA, SQLite for native mobile).
  2. Implement an optimistic UI update mechanism for the Unified Agent Feed that clearly demarcates "pending sync" items.
  3. Create a background sync worker that flushes the local intent queue to the Rust API when network connectivity is detected.
  4. Ensure backend API idempotency for intent ingestion so duplicate syncs are safely ignored.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
