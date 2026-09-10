issue_title: "Implement Visual Workflow Graph Engine for Cross-Department Automation"
issue_description: |
  ## Problem Statement
  OmniSolo aims to empower solo founders with autonomous AI agents across 10 pillars. However, owners currently lack a non-technical, visual way to orchestrate multi-step, cross-department workflows (e.g., triggering a marketing email sequence when an inventory low-stock alert is resolved). Without a low-code/no-code Visual Workflow Orchestration engine, users cannot see or modify the execution graph connecting triggers, AI agents, data mutations, and outputs, resulting in a black-box experience that violates our "Owner Clarity" engineering value.

  ## Research Report
  - **Market Context**: Industry-standard automation platforms like Zapier, Make (Integromat), and n8n provide visual node-based editors.
  - **Competitor Analysis**: Tools like Shopify Flow and GoDaddy offer basic trigger-action rules, but lack multi-agent swarm orchestration.
  - **Operator Pain Points**: Solo operators (like Nora, the agency principal, and Jun, the location manager) need transparent oversight of automated workflows to trust AI agents with their business operations.
  - **Open Source Solutions**: React Flow (MIT) is the industry standard for building node-based graphical interfaces in React/Next.js. For the backend execution engine, Temporal (MIT) or a native Rust DAG executor can manage stateful, retriable workflows.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      WorkflowGraph ||--o{ ExecutionNode : contains
      ExecutionNode ||--o{ Edge : connects
      WorkflowGraph {
          uuid id
          string tenant_id
          string trigger_event
          boolean is_active
      }
      ExecutionNode {
          uuid id
          string agent_department
          json config
      }
  ```
  ### Mobile UX Flow (375px First)
  - **List View**: A clean, thumb-friendly list of active workflows with toggle switches.
  - **Editor**: A simplified mobile view showing linear execution steps (Step 1 -> Step 2), avoiding complex 2D canvas navigation on small screens.
  - **Desktop Experience**: Full 2D interactive canvas using React Flow for drag-and-drop node connections.
  ### AI Agent Integration
  - The Visual Workflow Agent translates natural language requests ("Remind me when stock is low") into a structured ExecutionGraph.
  - The graph acts as the coordination layer, dispatching jobs to specialized swarm agents (Marketing, Logistics, etc.) via the PostgreSQL/Redis job queue.
  ### Key Design Decisions
  - **Multi-Tenant Isolation**: Every `WorkflowGraph` must strictly enforce `tenant_id` at the database level (PostgreSQL RLS).
  - **Frontend Library**: Utilize `reactflow` for the desktop canvas to avoid reinventing complex zoom/pan/drag logic.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Objective**: Build the visual workflow graph builder UI and execution engine.
  **CUJ**: A user (like Nora) logs in, navigates to "Automations", clicks "New Workflow", and visually connects a "Client Intake Form" trigger to a "Draft Proposal" agent node and a "Send Email" action node.
  **Acceptance Criteria**:
  1. Interactive node-based canvas (desktop) and linear list (mobile) using OHC Premium Tokens (translucent glass).
  2. Integration with existing agent swarm job queues.
  3. Strict multi-tenant isolation via `tenant_id`.
  4. 100% Playwright E2E coverage for the workflow creation CUJ.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
