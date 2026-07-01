issue_title: "Implement AI-Assisted Offline-First Inventory Management for Remote Ops"
issue_description: |
  # Research Report: Offline-First Inventory Management for Remote Operations

  ## 1. Problem Statement
  Operators like Fatima (food cart) or Jun (location manager) often work in environments with poor or non-existent mobile data connectivity. Existing cloud-first POS and inventory systems (like Square or standard Shopify POS) become sluggish or completely unresponsive offline, leading to lost sales, inaccurate stock counts, and frustrated customers. Furthermore, when they regain connectivity, the sync process is often manual or prone to conflicts, requiring technical intervention that owners don't have time for. They need a system that works flawlessly offline and uses AI to seamlessly resolve conflicts and predict stock needs once back online.

  ## 2. Research Report
  - **Market Context**: Cloud-native POS systems dominate, but they struggle in the real world of pop-up shops, rural service routes, and busy street food carts.
  - **Competitor Gaps**:
    - *Square*: Has an "offline mode" but it's limited, primarily focused on queuing payments, and frequently results in inventory desyncs.
    - *Shopify POS*: Offline mode is basic; conflict resolution requires manual owner intervention.
  - **The OHC Opportunity**: Build a truly local-first mobile architecture. Leverage the Operations Agent to handle the complex, tedious work of conflict resolution (e.g., two offline devices sell the same last item) when connectivity is restored, notifying the owner only if a critical business decision is needed.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App - Flutter] --> B(Local SQLite Cache)
      B --> C{Sync Engine - Offline/Online}
      C -->|Online| D[OHC API Gateway]
      D --> E[PostgreSQL Database]
      D --> F[Operations Agent]
      F -->|Conflict Detected| G[AI Resolution Logic]
      G -->|Resolved| E
      G -->|Needs Owner Input| H[Action Queue -> Mobile Feed]
  ```

  ### Data Model & Sync Strategy
  - **CRDTs (Conflict-Free Replicated Data Types)**: Utilize CRDTs for inventory counts to allow mathematical merging of offline sales without explicit lock contention.
  - **Local Persistence**: Flutter app uses local SQLite (e.g., via `sqflite` or similar robust local store) as the primary read/write source.
  - **Sync Queue**: All mutations (sales, restocks) are queued locally and pushed via background tasks when network health is verified.

  ### AI Integration Points
  - **Operations Agent (Conflict Resolution)**: If a true conflict occurs (e.g., physical stock negative), the agent analyzes the discrepancy, drafts a stock adjustment, and suggests a supplier order, presenting a simple "Approve Adjustment" card to the owner.
  - **Predictive Restocking**: Analyzes offline sales velocity once synced to predict future needs.

  ### Mobile UX Flow (375px)
  1. **Status Indicator**: Clear, non-intrusive UI element showing "Offline (Changes Saved)" vs. "Online".
  2. **Inventory Screen**: Loads instantly from local cache. Sales can be completed with zero latency.
  3. **Owner Feed**: When back online, if the AI resolved a conflict, a card appears: "Adjusted 2 missing cupcakes from offline sales. [View Details]".

  ## 4. Implementation Prompt
  **User-Facing Outcome**: As Fatima, I can sell my last 10 halal platters at a busy, disconnected festival without the app freezing. When I get home to Wi-Fi, the app syncs instantly. If there was a counting error, the AI assistant suggests the fix instead of showing me a scary red error screen.
  **CUJ & Acceptance Criteria**:
  1. Implement a local-first caching layer in the Flutter app for Inventory items.
  2. Create a background sync queue that captures inventory mutations.
  3. Develop a robust conflict resolution endpoint in the backend that utilizes CRDT principles for safe merging.
  4. Integrate the Operations Agent to detect and handle non-mergeable conflicts, generating an Action Card for the owner feed.
  5. Provide Playwright E2E tests simulating an offline state, completing a transaction, restoring connectivity, and verifying the correct backend sync and Agent action.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
