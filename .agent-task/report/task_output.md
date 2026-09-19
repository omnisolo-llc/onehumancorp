issue_title: "Architectural Gap: Visual Workflow Orchestration Engine (No-Code Swarm Execution)"
issue_description: |
  Title: Build the Visual Workflow Orchestration Engine for Autonomous Agent Swarms

  Problem Statement:
  Business operators like Nora (agency principal) and Maya (home baker) struggle to coordinate multiple AI agents (e.g., Sales, Operations, Billing) across complex, multi-step processes. Currently, they lack a clear, visual way to see how tasks flow from a trigger (like an Instagram DM or form submission) to a completed action (like an interactive proposal or automated PO). They need a simple, no-code visual canvas to drag, connect, and observe automated workflows without writing scripts or understanding technical jargon.

  Research Findings:
  1. Shopify Flow: Offers a visual builder for simple e-commerce triggers, but lacks native multi-agent swarm coordination.
  2. Wix Automations: Easy to use for basic triggers, but rigid and doesn't support complex branching or natural language parsing.
  3. Squarespace: Minimal automation capabilities; relies heavily on third-party integrations like Zapier.
  4. Zapier/Make: Powerful but intimidating for non-technical users; exposes too many API details (JSON, headers) and breaks the Zero Complexity promise.
  5. Bento/Customer.io: Good visual canvas for marketing drips, but not generalized for full business operations (like scheduling or MRP).
  6. Industry Gap: None of the SMB market leaders provide a visual execution graph tailored for an autonomous AI workforce (agents acting as nodes).
  7. Operator Communities: Feedback from Reddit /r/smallbusiness and /r/agency indicates owners want set it and forget it workflows with a transparent dashboard to intervene if an agent gets stuck.
  8. Open Source Tooling: The market standard for building visual no-code editors is React Flow (MIT), which provides a robust, accessible foundation for 375px mobile and desktop canvases without reinventing the wheel.

  Design Doc (High-Level System Architecture):
  - Architecture Choices: Use React Flow on the frontend for rendering the visual graph. The backend orchestration should leverage a resilient job queue (e.g., PostgreSQL SKIP LOCKED or an open-source workflow engine like Temporal/Oban paradigm) for executing the nodes.
  - Agent Integration Points:
    - Swarm Agents are represented as specialized nodes on the visual canvas.
    - Triggers (e.g., New Inbox Message) activate the workflow.
    - Each node executes via the Universal Provider Facade, utilizing the Shared Local Services bundle for LLM inference.
    - Output edges pass structured semantic data to the next agent node.

  erDiagram
      WORKFLOW_GRAPH ||--o{ EXECUTION_NODE : contains
      EXECUTION_NODE }|--|| AGENT_WORKER : executes
      EXECUTION_NODE ||--o{ NODE_EDGE : connects_to
      WORKFLOW_GRAPH {
          string tenant_id
          string trigger_event
          boolean is_active
      }
      EXECUTION_NODE {
          string step_name
          string status
          json configuration
      }

  Mobile UX Flow (375px First):
  1. Trigger Screen: User taps New Automation. Selects a trigger from a simple list (e.g., When a customer emails).
  2. Canvas Screen: A touch-optimized vertical timeline or simplified graph view appears.
  3. Node Selection: User taps the + button below the trigger. A bottom-sheet slides up with available Agent Actions (e.g., Draft Proposal, Send Invoice).
  4. Configuration: Tapping an action opens a full-screen card to configure plain-English rules (e.g., If over 00, require owner approval).
  5. Live View: The canvas shows animated borders around nodes that are currently executing in the background.

  Implementation Prompt:
  As an Implementer, your task is to build the Visual Workflow Orchestration Engine UI and backend execution tracking.
  - Construct a mobile-responsive visual canvas using React Flow or similar UI primitives.
  - Create the CRUD endpoints for saving and loading WorkflowGraph and ExecutionNode models, ensuring strict PostgreSQL RLS by tenant_id.
  - Implement a visual node for at least one trigger (New Lead) and two actions (Draft Proposal, Notify Owner).
  - Do NOT use mock data. Wire the UI directly to real backend persistence. Ensure the UI gracefully handles loading states, empty states, and node execution failures.
  - Write E2E Playwright tests that simulate an owner creating a 2-step workflow from the UI.

  Priority: P0
  Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
