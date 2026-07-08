issue_title: "[Research] Autonomous Multi-Tenant AI Agent Architecture for OHC"
issue_description: |
  # Research Report: Autonomous Multi-Tenant AI Agent Architecture for OHC

  ## Executive Summary
  This report details an architectural design for integrating autonomous AI agents deeply into the OHC multi-tenant platform. By shifting from reactive prompt-based integrations to an asynchronous, event-driven agentic framework, OHC can deliver true "owner work assistant" capabilities where agents proactively coordinate tasks, monitor operations, and draft communications behind the scenes.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  The current SMB SaaS landscape features disjointed AI integrations (e.g., Shopify Sidekick, Wix Studio AI). While these tools offer prompt-based text/image generation or simple dashboard shortcuts, they lack persistent operational awareness. Emerging AI-native platforms (like Durable or Lindy.ai) provide localized autonomous task execution but struggle with complex multi-tenant data consistency (e.g., POS sync, inventory ledger locking). OHC's unique value proposition is merging the autonomous agency of platforms like Relevance AI with the robust multi-tenant constraints of an ERP.

  ## 2. Deep Dive Architecture Design (Track 2)
  ### The Event-Driven Agent Framework
  To support proactive agent behaviors, we introduce an Event-Driven Agent Framework centered around the `AgentJobQueue`.

  - **Event Ingestion:** All significant multi-tenant mutations (e.g., `OrderCreated`, `InventoryDepleted`, `CustomerMessageReceived`) are published to a structured PostgreSQL-backed event log, scoped strictly by `tenant_id`.
  - **Agent Job Dequeue:** The AI Job Queue leverages a PostgreSQL `SKIP LOCKED` pattern. Specialized agent workers (e.g., "Customer Success Agent", "Operations Agent") poll this queue for relevant events.
  - **Memory & Context:** Agents query a localized Redis cache and a semantic vector store (tenant-scoped) to retrieve historical customer interactions or operational policies before acting.
  - **Action Coordination:** When an agent decides to act (e.g., draft a quote, restock inventory), it uses Redis Redlock (`ohc:lock:{tenant_id}:{resource_type}:{resource_id}`) to ensure its actions do not conflict with simultaneous manual owner operations or other agent actions.
  - **Human-in-the-Loop:** All agent-generated mutations that affect external users (sending emails, modifying public store status) must initially be pushed to an "Owner Approval Feed" (the Assistant Shell) rather than executed directly, adhering to the "Owner Clarity" core value.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as OHC Mobile App
      participant DB as PostgreSQL (Central Ledger)
      participant Queue as AgentJobQueue (SKIP LOCKED)
      participant Agent as Operations Agent
      participant Redis as Redis (Redlock/Cache)

      App->>DB: Process In-Store Sale (Item X)
      DB-->>App: Confirm Sale
      DB->>Queue: Publish Event (InventoryDepleted: Item X)
      Queue->>Agent: Dequeue Event (Worker)
      Agent->>Redis: Request Lock (ohc:lock:tenant:inventory:X)
      Redis-->>Agent: Lock Granted
      Agent->>Agent: Analyze threshold (Stock < 2)
      Agent->>DB: Create AgentTask (Draft Restock Order)
      Agent->>Redis: Release Lock
      DB-->>App: Push Notification (New Task in Approval Feed)
  ```

  ### Data Model Enhancements
  - **`AgentEventLog`**: Tracks raw system events triggering agent evaluation.
  - **`AgentTask`**: Represents a specific unit of work scheduled for an agent. Includes `status` (pending, drafting, awaiting_approval, completed, failed).
  - **`AgentMemory`**: Semantic embeddings of past interactions or owner directives.

  ## 3. Mobile-First Integrity (Track 3)
  The "Owner Approval Feed" UI must be optimized for a 375px viewport. Agent drafts (quotes, emails, inventory adjustments) are presented as dismissible or actionable cards within the unified feed. Network latency during approval must be mitigated with optimistic UI updates that clearly demarcate "pending AI execution" states.

  ## 4. Implementation Prompt
  **Outcome:** The Operations Agent autonomously drafts a restock task when an item drops below the reorder threshold, presenting it to the owner in the Unified Feed for one-tap approval.

  **CUJ:**
  1. An inventory update event lowers "Blue T-Shirt" stock to 1.
  2. The `AgentJobQueue` routes this to the Operations Agent.
  3. The Operations Agent queries the tenant's supplier config and drafts a restock order.
  4. The owner opens the OHC app (375px screen) and sees a high-priority card in the Assistant Shell: "Blue T-Shirt is critically low. Drafted a restock order for 50 units."
  5. The owner taps "Approve" (≥ 44x44px touch target). The order is finalized and the ledger updated.

  **Acceptance Criteria:**
  - Introduce `AgentTask` schema with strict `tenant_id` RLS.
  - Implement the PostgreSQL `SKIP LOCKED` job worker for processing inventory events.
  - Build the mobile-first (Flutter/PWA) Assistant Shell feed component displaying pending `AgentTask` records.
  - Ensure the feed handles flaky networks gracefully.
  - Write full E2E Playwright tests covering the event trigger, feed display, and approval interaction.

  ## 5. Repository Anomalies List (Top 5)
  1. `bazel` toolchain wrapper is missing or `PATH` is incorrectly configured on the environment, rendering `bazel` uncallable.
  2. The E2E tests mention `playwright.config.ts`, but Playwright UI testing workflows seem disconnected from the primary Docker compose stack.
  3. Duplicate `commit_msg` and `commit_msg2` hooks in the root directory.
  4. The `deploy/docker-compose.yml` attempts to pull a specific `alpine:3.19` image layer that fails on extraction due to missing permissions (`etc/alternatives/.wh.pager.1.gz`), breaking the local testing workflow.
  5. Rust (`Cargo.toml`) and Go (`go.mod`) build files are co-located in `src/server/ohc`, which might complicate the Bazel build boundaries if not strictly defined.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
