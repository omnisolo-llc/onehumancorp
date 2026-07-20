issue_title: "Implement No-Code Visual Workbuddy Workflow Builder & Sub-Agent Orchestration"
issue_description: |
  # Research & System Design Report: No-Code Visual Workbuddy Workflow Builder & Sub-Agent Orchestration

  ## 1. Problem Statement
  Small business owners like **Maya (baker)**, **Carlos (handyman)**, and **Priya (boutique owner)** have highly customized, localized business operations that do not fit into rigid, generic SaaS software boxes. For example, Maya wants an automated flow where: "When an Instagram DM inquiry containing 'vegan' arrives, check my calendar for the delivery date, draft a custom cake proposal, generate a payment link for a 50% deposit, and text me a summary for approval."

  Currently, setting up such multi-step automations requires complex external tools like Zapier or Make, which have a steep learning curve, require API keys, lack multi-tenant data context, and fail the "grandmother test". Solopreneurs need a premium, intuitive **No-Code Visual Workbuddy Workflow Builder** embedded directly inside OHC. They need a system where they can easily drag and drop (or conversationally ask the AI to construct) visual blocks that coordinate AI departments, tools, and human-in-the-loop checkpoints directly from their mobile phones.

  ---

  ## 2. Competitive Analysis & Market Research
  We investigated the low-code and automated workflow builders of leading platforms:

  ### Comparative Analysis Matrix
  | Feature / Platform | Shopify Flow | Zapier / Make | OHC Visual Workbuddy (Target) |
  |---|---|---|---|
  | **Target User** | Medium-Large Merchants | Technical Integrators / Developers | Non-technical Solopreneurs & Operators |
  | **Visual Editor** | Desktop-Heavy Flowchart | Complex Multi-Window Mapper | **Mobile-First Glassmorphism (375px)** |
  | **AI Integration** | None (Static Rule-Based) | External OpenAI connections | **Native, Context-Aware AI Departments** |
  | **Multi-Tenancy** | Single shop silo | Direct credentials required | **Strict Row-Level Isolation (SPIRE/OIDC)** |
  | **Setup Time** | > 30 minutes | > 1 hour | **< 2 minutes (AI-assisted generation)** |
  | **Offline Resilience**| None (Cloud-Only) | None (Cloud-Only) | **Local execution & offline checkpoint sync** |

  ### Key Findings
  1. **Cognitive Overload:** Traditional systems overwhelm users with developer terms like "webhook payload", "JSON path", "auth headers", and "variables mapping". OHC must abstract these variables into clean, high-level context bindings (e.g., "The Customer", "The Order Price").
  2. **The "Handoff" Gap:** Existing workflow tools do not handle Human-in-the-Loop approval sequences elegantly. If a workflow needs approval, it usually breaks or stops without a native mobile notification or easy 1-tap resolution. OHC's unique advantage is the unified activity feed.

  ---

  ## 3. High-Level System Architecture & Design
  We design a resilient, secure, and multi-tenant workflow engine that extends the current `WorkflowExecutor` with persistent state management and edge synchronization.

  ### Data Model Invariants (Mermaid.js ERD)
  ```mermaid
  erDiagram
      TENANT ||--o{ WORKFLOW_DEFINITION : configures
      WORKFLOW_DEFINITION ||--o{ WORKFLOW_INSTANCE : triggers
      WORKFLOW_INSTANCE ||--o{ STEP_CHECKPOINT : records
      WORKFLOW_DEFINITION ||--o{ NODE_DEFINITION : contains
      WORKFLOW_DEFINITION ||--o{ EDGE_DEFINITION : connects

      TENANT {
          string tenant_id PK
          string organization_name
      }
      WORKFLOW_DEFINITION {
          string workflow_id PK
          string tenant_id FK
          string name
          boolean is_active
          timestamp created_at
      }
      NODE_DEFINITION {
          string node_id PK
          string workflow_id FK
          string node_type
          json config
      }
      EDGE_DEFINITION {
          string edge_id PK
          string workflow_id FK
          string source_node_id
          string target_node_id
      }
      WORKFLOW_INSTANCE {
          string instance_id PK
          string workflow_id FK
          string status
          json current_state
          timestamp started_at
          timestamp updated_at
      }
      STEP_CHECKPOINT {
          string checkpoint_id PK
          string instance_id FK
          string node_id
          string state_snapshot
          timestamp created_at
      }
  ```

  ### Zero-Trust & Isolation Boundary
  Every workflow definition and instance is isolated at the database level using `tenant_id` Row-Level Security (RLS) in PostgreSQL.
  - **SPIFFE/SPIRE Integration:** Each execution step enqueued into the background worker mesh is cryptographically signed with the tenant's SPIFFE identity, ensuring that background workers executing LLM prompt templates or executing tools cannot access or mutate cross-tenant state.

  ### Orchestration Flow (Sequence Diagram)
  ```mermaid
  sequenceDiagram
      autonumber
      participant Trigger as Omnichannel Inbox
      participant Engine as Workflow Orchestration Engine
      participant Queue as AI Job Queue (SKIP LOCKED)
      participant Worker as Worker Thread (SPIRE Auth)
      participant UI as OHC Mobile App (375px)

      Trigger->>Engine: Event: "New Vegan DM from Maya's IG"
      Engine->>Engine: Resolve Active Workflow Definition & Bind Context
      Engine->>Engine: Create Workflow Instance (State: PENDING)
      Engine->>Queue: Enqueue step jobs with Tenant SPIFFE identity
      Queue->>Worker: Dequeue (SKIP LOCKED) & Execute Shards
      Worker->>Engine: Save Checkpoint Snapshot
      Engine-->>UI: Push Human-in-the-Loop Notification (Approval Required)
      UI->>Engine: 1-Tap "Approve Quote"
      Engine->>Queue: Enqueue remaining execution phases (Synthesis/Fulfillment)
  ```

  ---

  ## 4. Mobile-First UX Design (375px Viewport)
  The workflow builder must run beautifully on mobile, eliminating heavy drag-and-drop mechanics in favor of a **Linear Block Stack with Intelligent Connector Overlays**.

  ### Screen Flow & Interaction Mechanics
  1. **Conversational Setup:** Carlos says: "I want to text a customer an automated review request 1 day after their service is completed." The AI instantly assembles the flow and previews it as a vertical stack of translucent cards.
  2. **The Layout (Premium Glassmorphism):**
     - Each node is represented as a glassmorphism card with distinct color tokens:
       - `🔵 Blue` for Triggers (e.g., "New Payment", "New DM").
       - `🟣 Purple` for AI Departments (e.g., "The Accountant", "The Writer").
       - `🟢 Green` for Actions/Tools (e.g., "Send SMS", "Generate Invoice").
       - `🟠 Orange` for Human-in-the-Loop Checkpoints (e.g., "Require Owner Approval").
     - A simple, vertical line with high-contrast connect points joins the cards. Touch targets for connectors are enlarged to `48x48px` to facilitate thumb edits.
  3. **Haptic Feedback:** Native haptic vibrations accompany connecting cards, deleting edges, or toggling a workflow state.

  ---

  ## 5. AI Department Coordination
  The visual workflow engine coordinates three primary specialized AI departments:
  - **Operations Dept ("The Manager"):** Decodes scheduling constraints, queries local inventories, and coordinates fulfillment steps.
  - **Customer Success Dept ("The Ambassador"):** Reviews customer context, maintains tone-of-voice memory guidelines, and drafts localized messages.
  - **Finance Dept ("The Accountant"):** Oversees pricing rules, generates localized invoices, and processes secure Stripe deposit links.

  ---

  ## 6. Implementation Prompt for Implementer Agent

  ```text
  Build the No-Code Visual Workbuddy Workflow Engine and client API.

  1. Implement persistent workflow models in the multi-tenant PostgreSQL schema under strict row-level isolation using 'tenant_id'. Models must store WorkflowDefinitions, Nodes, Edges, WorkflowInstances, and step snapshots (checkpoints).
  2. Extend the Rust WorkflowExecutor to support resumable workflow executions. The executor must yield cleanly and save a STEP_CHECKPOINT when hitting a NodeType::HumanInLoop node, changing the status of the WorkflowInstance to 'awaiting_approval'.
  3. Implement an API endpoint under '/api/v1/workflow/resume' allowing the client to post the approved context to resume execution from the checkpointed node.
  4. Develop frontend components inside the Tauri desktop client adhering to premium macOS Translucent Glass styling, providing a beautiful vertical card layout for mobile breakpoints (375px) with no horizontal scroll.
  5. Enforce strict Zero-Trust isolation. Background workers executing workflow steps must verify the tenant identity claims using SPIRE workload authentication before completing any state operations or third-party tool calls.
  6. Deliver comprehensive E2E Playwright tests verifying the entire flow from initial event ingestion, automatic execution halting on human-in-the-loop nodes, and instant continuation after 1-tap approval.
  ```

  ---

  ## 7. Operational Parameters
  - **Priority:** `P1` (High)
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
