issue_title: "Implement Invisible Autonomous Agent Workflows (Market Gap Fix)"
issue_description: |
  # Research Report: Invisible Autonomous Agents

  ## Problem Statement
  Small business owners like Maya (the baker) and Carlos (the handyman) are overwhelmed by manual tasks and operational friction. Existing tools require them to act as IT admins. We need to implement the core invisible autonomous agents identified in the OHC SMB Market Report to resolve the initial setup paralysis and save them time via automated actions.

  ## Competitive Analysis
  - **Shopify:** Requires complex third-party plugins for automations like cart recovery or auto-replies, adding costs and cognitive load.
  - **Wix:** Automations require manual setup of "If This Then That" rules that small business owners rarely configure correctly.
  - **Squarespace:** Very basic built-in automations; primarily design-focused.
  - **OHC Advantage:** Zero-configuration, out-of-the-box autonomous agents that run in the background (e.g., auto-reply to DMs, cart recovery) without requiring users to build the logic.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Source: Message/Order/Abandoned Cart] --> B[OHC AI Job Queue (PostgreSQL SKIP LOCKED)]
      B --> C[AI Workflow Engine (Go/Redis)]
      C --> D[Autonomous Agent Department (Operations/CS/Marketing)]
      D --> E[Outcome: Auto-Reply, SEO Update, Follow-up]
  ```

  ### AI Agent Integration Points
  - **Job Queue Listener:** A Go service that listens to the `OHC AI Job Queue` and routes tasks to the appropriate AI agent department based on event type.
  - **Context Retrieval:** Before acting, agents query the tenant's memory graph (PostgreSQL/ChromaDB) to understand customer context and business rules.
  - **Action Execution:** Agents interact with external APIs (Instagram, Mailgun, Stripe) using tenant-scoped credentials retrieved securely.

  ### Mobile UX Flow (375px)
  - **Feed View:** Users see a chronological, prioritized feed of what agents have done.
  - **Agent Action Card:** Displays a translucent glass card summarising the automated action taken (e.g. "Auto-replied to Instagram DM").
  - **Approval Flow:** For critical actions (like refunds), an approval notification is presented.

  ## Implementation Prompt
  Implement the backend job queue mechanism to support asynchronous execution of tasks triggered by external events. Ensure seamless mobile-first rendering of the Agent Feed where business owners can review automated actions. Adhere to row-level multi-tenancy.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
