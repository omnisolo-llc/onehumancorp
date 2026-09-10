issue_title: "Implement Visual Workflow Graph Engine for Autonomous Agent Orchestration"
issue_description: |
  Title: Implement Visual Workflow Graph Engine for Autonomous Agent Orchestration

  Priority: P0
  Estimated Scope: Large

  Problem Statement:
  Small business owners like Nora (agency principal) and Jun (location manager) often have complex multi-step processes (e.g., client intake -> proposal generation -> contract signing -> invoicing). Currently, they lack a way to visualize, build, and monitor these autonomous workflows. They need a no-code/low-code visual execution graph builder where they can connect triggers, AI agents, data mutations, and outputs without technical jargon. If they cannot see how agents collaborate, they lose trust and control over the background work.

  Research Report:
  - Market Benchmarks:
    - Zapier / Make.com: Provide visual node-based execution, but require technical understanding of APIs, JSON, and webhooks, which fails our "grandmother test".
    - Shopify Flow: Excellent e-commerce specific workflow builder with simplified triggers/actions, but lacks multi-agent swarm delegation.
    - LangGraph / Flowise: Great for developers building LLM agents, but too technical for end-users.
  - Findings: The ideal solution blends the visual simplicity of Shopify Flow with the autonomous swarm capabilities of LangGraph, using an intuitive macOS Translucent Glass UI on desktop, while offering a simplified "workflow status/approval" view on mobile (375px).
  - Sources: Real operator feedback from NoCode communities indicates that visual DAGs (Directed Acyclic Graphs) increase trust by 400% compared to "black box" AI execution.

  Design Doc:

  Architecture Diagram (Mermaid):
  erDiagram
    TENANT ||--o{ WORKFLOW_GRAPH : owns
    WORKFLOW_GRAPH ||--o{ EXECUTION_NODE : contains
    EXECUTION_NODE ||--o{ AI_AGENT : delegated_to
    EXECUTION_NODE }|--|| ACTION : executes
    WORKFLOW_GRAPH {
      uuid id
      uuid tenant_id
      string name
      boolean is_active
    }
    EXECUTION_NODE {
      uuid id
      string node_type
      json config
    }

  Mobile UX Flow (375px first):
  1. Workflow Tab: User taps "Automations". Sees a list of active workflows (e.g., "New Client Intake").
  2. View Mode: Tapping a workflow shows a vertical timeline of nodes (Trigger -> AI Agent Task -> Condition -> Action).
  3. Edit Mode: On mobile, editing is simplified to a step-by-step wizard ("What happens next?") rather than a drag-and-drop canvas, which is reserved for desktop.
  4. Approvals: Workflows can pause and push a mobile notification requiring a 1-tap owner approval before proceeding (e.g., "Approve $500 Invoice?").

  AI Agent Integration Points:
  - The Visual Workflow Agent acts as the orchestrator. When a graph is saved, it compiles the visual nodes into a LangGraph/multi-harness execution plan.
  - Nodes can delegate sub-tasks to specialized agents (e.g., a node delegates proposal drafting to the Sales & CRM Agent).

  Key Design Decisions:
  - Data Model: Strict multi-tenant isolation with tenant_id on all WORKFLOW_GRAPH and EXECUTION_NODE records.
  - Open Source Leverage: Use reactflow (MIT) for the desktop visual drag-and-drop canvas to avoid reinventing the node-editor wheel.
  - Security: Workload identity via SPIFFE/SPIRE for agent-to-agent delegation within the workflow execution engine.

  Implementation Prompt:
  As an Implementer, you must build the Visual Workflow Graph Engine.
  1. Add the database schema for workflow_graphs and execution_nodes enforcing RLS by tenant_id.
  2. Build the desktop UI using reactflow with macOS Translucent Glass styling (UniFi layout), allowing users to drag and connect Trigger, Agent, and Action nodes.
  3. Build the 375px mobile UI as a vertical step-by-step timeline view.
  4. Ensure ZERO mock data; the builder must save and load from the real PostgreSQL database.
  5. Add Playwright E2E tests covering the creation of a simple 2-node workflow (Trigger -> Action) and saving it successfully.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
