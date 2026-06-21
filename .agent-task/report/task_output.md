issue_title: "Integrate PowerSync for Bidirectional Local-to-Cloud State Synchronization"
issue_description: |
  # PowerSync Integration: SQLite to Postgres State Sync

  ## Problem Statement
  The OneHumanCorp (OHC) platform operates on a Hybrid Agentic OS model, which requires seamless synchronization between local single-user SQLite instances (Standalone Mode) and the multi-tenant PostgreSQL Cloud Gateway (Cloud-Native Mode). Currently, there is a capability gap in maintaining a robust, low-latency, and conflict-free bidirectional sync of RAG state, agent memory contexts, and POS inventory ledgers across these environments. Small business owners relying on the offline-first capabilities of OHC risk facing disjointed state synchronization when shifting from local/offline desktop environments or mobile POS solutions back to the multi-tenant cloud context.

  ## Research Report
  **Market Analysis:** Several tools exist for local-to-cloud synchronization, notably ElectricSQL and PowerSync.
  - **ElectricSQL:** Focuses on active-active replication but has had significant architecture shifts recently.
  - **PowerSync:** Provides robust SQLite-to-Postgres synchronization with out-of-the-box support for offline-first applications, robust conflict resolution, and high concurrency. It aligns perfectly with our need to sync local RAG contexts and POS ledgers with the Cloud Gateway over mutually authenticated TLS.

  **Platform Gap Analysis:**
  While OHC currently uses standalone SQLite instances for edge caching and POS local ledgers, there is no unified sync orchestrator connecting this local state back to the central multi-tenant PostgreSQL backend. This leads to fragmented AI agent memory where local workflows (like an air-gapped desktop interaction) do not propagate to the broader cloud-native swarm.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile/Desktop Edge Client
          App[OHC Mobile/Desktop App] --> LocalDB[(Local SQLite)]
          App --> LocalAgent[Local Embedded AI Agent]
          LocalDB --> PSClient[PowerSync Client SDK]
      end

      PSClient <-->|mTLS/WebSockets| PSService[PowerSync Service]

      subgraph OHC Cloud Services
          PSService --> PostgresDB[(Central OHC PostgreSQL)]
          PostgresDB --> CloudOrchestrator[OHC Cloud Orchestrator]
          CloudOrchestrator --> CloudSwarm[Cloud Agent Swarm]
      end
  ```

  ### Mobile UX Flow & AI Integration
  - **Offline/Local First Sync Pill:** When disconnected from the cloud, a subtle, premium glassmorphism pill appears on the 375px mobile UI showing "Offline Mode" and logging pending intents in the local SQLite database.
  - **Optimistic Reconciliation:** PowerSync will automatically reconcile the local SQLite changes with the central Postgres database upon reconnection without user intervention.
  - **AI Coordination (Operations & Finance Agents):** The Finance Agent subscribes to PowerSync events in the cloud to seamlessly reconcile ledger entries that have just synced from a disconnected mobile terminal, automatically generating a summary notification to the user upon success.

  ## Implementation Prompt
  **Role:** Implementer Agent

  **User-Facing Outcome:** Business owners using OHC in standalone mode or offline mobile POS can process tasks, adjust inventory, and trigger agent interactions without network dependency. When a connection is reestablished, all local actions instantly sync to the cloud without manual reconciliation.

  **CUJ & Acceptance Criteria:**
  1. Add PowerSync service orchestration to the Kubernetes/Helm deploy manifests, connected to the central OHC PostgreSQL database with Row-Level Security (RLS) enabled.
  2. Integrate the PowerSync client SDK into the Go standalone daemon (`src/server/orchestration/hybrid_sync/hybrid_sync.go` and `src/server/auth/powersync.go`).
  3. Establish bidirectional sync of a designated mock RAG context record or POS inventory item from SQLite to Postgres.
  4. Ensure zero UI blocking during background sync; all network status must use subtle visual indicators adhering to the macOS-style translucent glass aesthetic.
  5. Provide an E2E Playwright test simulating an offline state mutation that correctly propagates to the cloud Postgres instance upon network restoration.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
