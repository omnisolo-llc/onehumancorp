issue_title: "[Architecture] Distributed Agent Protocol and Teammate Mesh Coordination"
issue_description: |
  # Architecture Design: Distributed Agent Protocol and Teammate Mesh Coordination

  ## Problem Statement
  OneHumanCorp (OHC) is designed to be a Tencent Workbuddy-like work assistant that hides operational complexity behind a unified AI interface for business owners (e.g., Maya, Carlos, Priya). A core promise is that the "Assistant coordinates messages, customers, tasks, calendar, documents, payments, analytics, and agent work behind the scenes."

  Currently, while KAIROS orchestration provides a high-level task mesh and the standalone hybrid architecture supports local/cloud execution, there is a fundamental gap in how disparate AI agent departments (e.g., Customer Success, Operations, Sales) securely, deterministically, and privately communicate with one another to resolve complex multi-domain tasks.

  If a customer DMs Maya "Can I get a custom vegan cake delivered next Tuesday?" the Customer Success agent must draft the reply, but it must first ask the Operations agent to check delivery capacity and the Sales agent to check vegan cake inventory or pricing. Without a structured Agent Protocol and a robust Teammate Mesh, these agents operate in silos, leading to incomplete or hallucinated responses, or requiring the owner to manually piece information together, violating the "One Assistant" promise.

  ## Research Report
  - **Market Context:** Advanced AI ecosystems (like multi-agent frameworks AutoGen, CrewAI, or robust enterprise tools like Slack/Tencent Workbuddy) rely on explicit message passing, shared state spaces, and verifiable identities.
  - **Codebase Insights:** OHC already utilizes Redis Pub/Sub (`mesh:tasks`, `mesh:coordination`) and PostgreSQL/SQLite for state, plus SPIFFE/SPIRE for identity. The missing link is the standardized, multi-tenant aware payload structure (the "Protocol") and the formal handoff mechanisms (the "Mesh") between these specialized agents.
  - **Competitor Analysis:**
    - *Shopify Sidekick/Wix AI:* Often monolithic; one large LLM prompt context rather than true multi-agent delegation, limiting scale and depth.
    - *Tencent Workbuddy/DingTalk:* Excel at routing intents to specialized microservices, but often lack autonomous agentic negotiation before presenting options to the user.
  - **The Opportunity:** OHC must implement a formal Hybrid Agent Protocol (OHC-HAP) running over the Teammate Mesh. This allows agents to query each other, negotiate constraints (e.g., scheduling vs. inventory), and synthesize a single, coherent proposed action to the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant WorkTriage as Work Triage Agent
      participant Mesh as Teammate Mesh (Redis/Local Bus)
      participant OpsAgent as Operations Agent
      participant SalesAgent as Sales Agent
      participant Memory as AutoDream (PostgreSQL)
      actor Owner as Maya

      Customer->>WorkTriage: DM: "Custom vegan cake next Tuesday?"
      WorkTriage->>Memory: Retrieve past context (Customer preferences)
      WorkTriage->>Mesh: Broadcast Intent [CHECK_CAPACITY, CHECK_INVENTORY]

      par Operations Check
          Mesh->>OpsAgent: Request: Can we deliver next Tuesday?
          OpsAgent-->>Mesh: Response: Yes, 2 slots available.
      and Sales Check
          Mesh->>SalesAgent: Request: Vegan cake availability/price?
          SalesAgent-->>Mesh: Response: Yes, base price $50, deposit $25.
      end

      Mesh-->>WorkTriage: Aggregate Responses
      WorkTriage->>WorkTriage: Synthesize coherent draft & propose action
      WorkTriage-->>Owner: Feed: "Draft reply & payment link ready for vegan cake request."
      Owner->>WorkTriage: 1-Tap Approve
      WorkTriage-->>Customer: Sends finalized reply and deposit link.
  ```

  ### Core Components & Multi-Tenancy Rules
  1. **The Teammate Mesh (Message Bus):**
     - **Cloud:** Redis Streams (preferred over Pub/Sub for durability and replayability) or PostgreSQL `SKIP LOCKED` job queue.
     - **Local/Standalone:** Local background workers / in-memory queues in Go.
     - **Isolation:** Every message MUST be scoped by `tenant_id`. The mesh router drops any message lacking a valid tenant context.
  2. **The Agent Protocol (Payload Schema):**
     - A standardized structured JSON schema for inter-agent communication.
     - Fields: `message_id`, `tenant_id`, `sender_agent_id`, `target_department` (or specific `target_agent_id`), `intent` (e.g., `QUERY`, `ACTION_REQUEST`, `NEGOTIATION`), `payload` (domain-specific data), `context_id` (linking to the originating user request/session).
  3. **Zero Trust & Security:**
     - Agents authenticate to the Mesh using their SPIFFE/SPIRE identities.
     - Role-Based Access Control (RBAC): Only specific agents can authorize state mutations (e.g., Sales Agent authorizes payment requests, Operations Agent authorizes schedule bookings).

  ### Mobile UX Flow (375px First)
  - **The Owner's Feed:** The owner (Maya) sees a unified "Work Command Center".
  - **Card View:** A single, premium translucent card (Apple/Ubiquiti style) appears in the feed.
    - **Header:** "New Inquiry: Custom Cake (Instagram)"
    - **Body:** "Customer requested a vegan cake for Tuesday. Capacity and pricing confirmed."
    - **Draft Preview:** [Show abbreviated draft message]
    - **Actions:**
      - Large primary button (44x44px target): "Approve & Send Quote ($50)"
      - Secondary button: "Edit Draft"
      - Secondary button: "Decline"
  - **Behind the Scenes:** The complex multi-agent negotiation over the Teammate Mesh is entirely hidden from the owner unless they tap "View Agent Activity" (placed under an Advanced/Details toggle).

  ## Implementation Prompt (For Implementer Agents)
  **Objective:** Implement the foundational Teammate Mesh communication layer and Agent Protocol schema to allow specialized agents to collaborate on a single task.

  **Critical User Journey (CUJ):**
  1. An external inquiry (e.g., simulated webhook for an Instagram DM) is received by the backend for a specific `tenant_id`.
  2. The `Work Triage Agent` parses the inquiry and determines it needs information from both the `Operations Agent` (capacity) and `Sales Agent` (pricing).
  3. The Triage agent dispatches standardized messages over the Teammate Mesh.
  4. The Ops and Sales agents process the requests and reply via the Mesh.
  5. The Triage agent aggregates the responses, synthesizes a proposed reply and action, and persists this as a pending action in the database.
  6. The owner UI displays the synthesized proposal for 1-tap approval.

  **Acceptance Criteria:**
  - Define the Go structs for the Agent Protocol (Payload Schema).
  - Implement the Teammate Mesh transport layer in Go (abstracted interface with Redis Streams implementation for cloud).
  - Enforce strict `tenant_id` isolation at the mesh routing level.
  - Create a realistic unit or integration test simulating the exact multi-agent negotiation (Triage -> Ops & Sales -> Triage) described in the CUJ.
  - No specific UI implementation is required in this task, but the final synthesized proposal MUST be written to a PostgreSQL database table or API response structure that a Flutter frontend can easily consume as a feed card.

  **Priority:** P0
  **Estimated Scope:** Large

  ## Top 5 Codebase Anomalies Discovered
  1. The code base has mixed languages where the primary backend is currently partially implemented in Rust instead of the specified Go architecture.
  2. A legacy Next.js prototype exists in `src/ui/next` and a Tauri desktop app exists in `src/ui/tauri`, which contradicts the specified Flutter frontend architecture.
  3. Test dependencies include hardcoded credentials rather than properly segregated environment variables or vault-based secret injection.
  4. Build warnings exist in the Rust backend around `GrowthState` in `api/growth.rs` exposing private interfaces unintentionally.
  5. Documentation refers to a legacy Slint/Flutter UI that was removed, causing potential confusion on the target UI framework.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, core-platform]
assignees: []
