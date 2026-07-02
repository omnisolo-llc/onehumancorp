issue_title: "Architectural Gap: Offline-First Agentic Sync & Mutation Queue for Intermittent Connectivity"
issue_description: |
  ### Mission Queue Protocol

  **Title**: Architectural Gap: Offline-First Agentic Sync & Mutation Queue for Intermittent Connectivity

  **Problem Statement**:
  Our non-technical owners, specifically Carlos (Field Service) and Fatima (Food Cart), often operate in environments with flaky, intermittent, or completely absent network connectivity. Currently, if Carlos attempts to update an estimate in a client's basement with no cell service, or if Fatima taps "Sold Out" during a rush on a congested 3G network, the mutations risk being dropped or hanging the app. An owner's assistant must not fail silently or freeze when the network degrades. They need a system that truthfully queues their actions, reflects them locally immediately (Optimistic UI), and uses an AI agent to resolve any synchronization conflicts once connectivity is restored.

  **Research Report**:
  - **Competitor Analysis**: Square POS and Toast are industry leaders in offline reliability. Square allows businesses to continue taking payments and making catalog changes offline, syncing them transparently later. Linear and Notion (modern productivity tools) use a robust local-first mutation queue (e.g., using Replicache or custom SQLite implementations) to ensure instant UI responsiveness and delayed sync.
  - **Market Gap**: Small business tools (like Wix or standard web dashboards) typically break completely offline. OHC can differentiate by offering a "never-lose-work" guarantee, bringing enterprise-grade offline sync to the 375px mobile experience.
  - **Current OHC State**: The current architecture relies primarily on direct REST/gRPC calls for writes. While low-data mode handles lazy images, we lack a structured, local-first mutation queue for core operational writes (orders, estimates, inventory toggles).

  **Design Doc**:

  *Architecture Flow:*
  ```mermaid
  sequenceDiagram
      actor Owner (Fatima/Carlos)
      participant Flutter Mobile App
      participant Local Storage (SQLite/Isar)
      participant Mutation Sync Engine
      participant OHC Backend API
      participant AI Agent (Conflict Resolver)

      Owner->>Flutter Mobile App: Tap "Sold Out" (Offline)
      Flutter Mobile App->>Local Storage (SQLite/Isar): Save Mutation & Update Local State
      Flutter Mobile App-->>Owner: UI updates immediately (Optimistic)
      Note over Flutter Mobile App: Device regains connection
      Mutation Sync Engine->>OHC Backend API: Replay queued mutations
      OHC Backend API->>OHC Backend API: Detect concurrent modification?
      alt Conflict Detected
          OHC Backend API->>AI Agent (Conflict Resolver): Trigger conflict analysis
          AI Agent (Conflict Resolver)-->>OHC Backend API: Auto-resolve (e.g., prioritize "Sold Out")
      end
      OHC Backend API-->>Mutation Sync Engine: Sync Confirmed
      Mutation Sync Engine->>Local Storage (SQLite/Isar): Clear queued mutation
  ```

  *Mobile UX Flow (375px First):*
  1. The user taps a toggle (e.g., "Sold Out").
  2. The toggle switches instantly. A subtle, translucent indicator (using OHC Premium Tokens) appears at the top: "Syncing..."
  3. If offline, the indicator changes to "Saved Offline". The app remains fully functional for reading cached data and queueing more writes.
  4. Once online, the indicator spins briefly and disappears. If a conflict requires owner input (rare, AI handles most), an Inbox card appears in the Work Triage feed: "Review Inventory Sync Issue".

  *AI Agent Integration Points:*
  - **Conflict Resolver Agent**: Listens to the DLQ (Dead Letter Queue) or conflict queue on the backend. When a sync fails (e.g., a customer bought the last item online while Fatima was offline and marked it sold out), the agent analyzes the timeline, refunds the customer if necessary, and drafts a polite SMS explaining the stock-out, appearing in Fatima's Work Triage feed for 1-tap approval.

  *Key Design Decisions:*
  - **Local-First Writes**: All critical operations write to the local database first.
  - **UUIDv7**: Use sortable UUIDv7s generated on the client for all new records to avoid primary key collisions and maintain chronological order during sync.
  - **Agentic Conflict Resolution**: Instead of showing technical error popups ("HTTP 409 Conflict"), we route edge-case sync failures to an AI agent which translates the technical conflict into a business operations decision for the owner.

  **Implementation Prompt**:
  "Implement the Offline Mutation Sync Engine for the Flutter app and the corresponding AI Conflict Resolver endpoint on the Go backend.
  1. Frontend: Create a local SQLite-backed queue for actions (e.g., InventoryToggleMutation, EstimateUpdateMutation). Ensure the UI reflects changes optimistically. Add visual 'Offline' and 'Syncing' states using OHC Design Tokens.
  2. Backend: Create a conflict-aware ingestion endpoint that processes the mutation queue. If a version mismatch occurs, push the payload to the AI Conflict Resolver's job queue.
  3. AI Agent: Implement the agent workflow to evaluate the conflict, take corrective action (like drafting an apology if an order was taken for a sold-out item), and notify the owner via the Work Triage feed.
  Acceptance Criteria: A user must be able to turn off network connectivity, make 3 distinct inventory/estimate changes, turn connectivity back on, and see all changes propagate to the server without any blocking error modals."

  **Priority**: P1

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
