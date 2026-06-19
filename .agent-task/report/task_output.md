issue_title: "Implement 'Agent Feed' Central Dashboard for Mobile-First Approval Workflow"
issue_description: |
  # Mission Queue Protocol: Implement 'Agent Feed' Central Dashboard for Mobile-First Approval Workflow

  ## Problem Statement
  Currently, OneHumanCorp (OHC) platform lacks a consolidated, mobile-first view of all the pending tasks, drafted responses, and automated actions prepared by different agents (e.g., Marketing, Operations, Advisory). Business owners like Maya (the baker) or Carlos (the handyman) need to review these proposals easily from their phones (375px screens) and approve or edit them quickly without navigating complex admin menus. A fragmented view leads to missed opportunities and "Agent Amnesia" where users forget to review the AI's drafts.

  ## Research Report
  - **Competitive Analysis**: Platforms like Shopify require accessing deep dashboard menus to approve drafts or check specific marketing automations. Wix's mobile experience is mainly a dashboard wrapper, lacking native agent-driven push notifications. Waiters and link-in-bio platforms excel in simplicity but lack agent coordination.
  - **OHC Gap**: We have background agents (via KAIROS orchestration) generating tasks and drafted contents in the `shared_tasks` and `autodream_memories` layers, but no single "Feed" that unifies these into actionable cards (Approve, Edit, Reject).
  - **Proposed Solution**: Build a new backend API that aggregates tasks/proposals from `shared_tasks`, `agent_feed_items`, and pending agent actions, and a corresponding mobile-first UI "Agent Feed" component.

  ## Design Doc
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      A[Agent Feed UI (Mobile First)] -->|GET /api/v1/feed| B(Feed Aggregator API)
      B --> C[(agent_feed_items)]
      B --> D[(shared_tasks)]
      A -->|POST /api/v1/feed/approve| B
      B -->|Transition State| E(KAIROS Orchestrator)
  ```

  ### Mobile UX Flow (375px)
  1. User opens the OHC mobile app.
  2. The home screen defaults to the "Agent Feed" view.
  3. The feed shows a vertical list of Action Cards. Each card clearly states the proposing agent (e.g., "The Promoter", "Operations"), a summary of the action, and a large "Approve" button.
  4. Example Card: "Operations: Maya, 3 cake orders need deposit confirmation. [Send Deposit Links]"
  5. Tapping the button transitions the task state via KAIROS and removes the card from the feed optimistically.

  ### AI Agent Integration
  Agents will continue generating records in `agent_feed_items` or `shared_tasks`. The Feed Aggregator simply serves these in a priority-sorted, unified timeline.

  ## Implementation Prompt
  Implement the Agent Feed functionality.
  - Create the backend API endpoint (`/api/v1/feed`) in Go that fetches and merges pending items from `agent_feed_items` and `shared_tasks` for the current tenant.
  - Implement the frontend Flutter component `UnifiedAgentFeed` that renders these items as actionable cards.
  - Ensure the layout is strictly constrained to mobile 375px widths, using OHC Premium Tokens (Glassmorphism).
  - Add integration with the KAIROS state machine to handle approvals/rejections directly from the feed.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
