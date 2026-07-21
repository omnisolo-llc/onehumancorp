issue_title: "Implement the 'Operations Manager' Agent Protocol"
issue_description: |
  # Research Report: Operations Manager Agent Protocol

  ## Problem Statement
  Small business owners often suffer from "App Tax" fatigue and setup paralysis. Current AI tools are mostly advisory, functioning as glorified manuals. To truly relieve the burden on owners like Maya (baker) and Carlos (handyman), OHC needs AI agents that don't just advise, but autonomously execute CRUD operations (e.g., modifying inventory, sending emails, processing orders) securely and effectively. This capability is currently missing or underdeveloped.

  ## Research Report
  Our competitive analysis shows that platforms like Shopify, while powerful, rely heavily on a complex ecosystem of third-party apps and offer AI that is primarily advisory (e.g., Shopify Sidekick). OHC's unique value proposition is the "Invisible AI Automation." To achieve this, we need a robust underlying protocol that allows our departmental AI workers (Operations, Customer Service, Sales, Finance, etc.) to coordinate and execute actions autonomously, shifting the paradigm from manual configuration to autonomous execution.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Event Ingestion Pipeline] --> B(Intent & Context Resolution LLM)
      B --> C{Operations Manager Protocol}
      C --> D[Departmental Agent: Inventory]
      C --> E[Departmental Agent: Sales]
      C --> F[Departmental Agent: Communications]
      D --> G[(Database CRUD via secure API)]
      E --> G
      F --> G
  ```

  ### Mobile UX Flow (375px first)
  The protocol itself is headless, but it surfaces actions through the Agent Feed:
  1. An event occurs (e.g., low inventory on a popular item).
  2. The Operations Manager Agent detects this and drafts an action (e.g., "Draft PO for supplier X").
  3. The owner sees an Action Card in their 375px mobile feed: "Inventory Low: Restock Item Y?" with simple 'Approve', 'Edit', or 'Discard' buttons.
  4. Upon approval, the agent executes the secure CRUD operation to create the PO.

  ### AI Agent Integration Points
  - The protocol must interface with the `Agent Feed` to push actionable notifications.
  - It must utilize the existing distributed lock (Redis Redlock) and job queue mechanisms to ensure safe, concurrent execution across tenants.
  - It needs a secure API layer to perform CRUD operations on behalf of the user, strictly honoring row-level security (RLS) by tenant ID.

  ### Key Design Decisions
  - **Headless Protocol first**: We focus on the backend protocol and security boundaries first, enabling the UI later.
  - **Zero Trust**: All agent actions must be authenticated and authorized exactly as if the human owner initiated them, utilizing SPIFFE/SPIRE where applicable or strong internal service tokens.
  - **Observable Handoffs**: When an agent needs human approval or another agent's input, the state transition must be recorded in the distributed state machine.

  ## Implementation Prompt
  Implement the core `Operations Manager Protocol` service layer in Rust. This service should provide a secure API for departmental AI agents to execute standard CRUD operations (Create, Read, Update, Delete) against core business entities (e.g., Inventory, Orders, Tasks). The implementation must enforce strict multi-tenant isolation (tenant_id checks) and log all actions for observability. Acceptance criteria: A new Rust service module that agents can call to perform database operations, with 100% unit test coverage and proven tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
