issue_title: "Unified Agentic Background Job Mesh"
issue_description: |
  # Architecture Brief: Unified Agentic Background Job Mesh

  ## Problem Statement
  Small business owners rely heavily on OneHumanCorp's (OHC) AI agents (Operations, Finance, Ambassador) to run their businesses invisibly in the background. Currently, as more complex tasks like multi-location inventory reconciliation, cross-channel message synthesis, and automated localized invoicing occur, there is no unified, high-performance, and resilient background job mesh specifically designed for long-running, stateful agentic workflows. Without this, agents might fail silently during heavy load, duplicate actions, or lose context during retries, leading to double-booked services for Leo or missing offline orders for Fatima. The system needs an enterprise-grade job mesh that allows agents to reliably yield, sleep, and resume without blocking resources, guaranteeing "zero drop" operational stability.

  ## Research Report
  - **Current Bottlenecks:** Agent background tasks are tightly coupled with synchronous API endpoints or basic task queues that lack state-machine resumption capabilities.
  - **Competitor Analysis:** High-scale platforms like Shopify use Resque/Sidekiq variants, and Temporal or Temporal-like systems are becoming the gold standard for stateful, long-running workflows where steps can fail and resume gracefully.
  - **Discovery:** OHC requires a Distributed Agentic Job Mesh based on a durable state machine pattern (like Temporal or AWS Step Functions, but abstracted for OHC's stack). It must support long-polling, sleep-until-event, and precise compensation logic for multi-step agent actions.

  ## Design Doc

  ### Architecture and Entity-Relationship Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ AGENT_WORKFLOW : "owns"
      AGENT_WORKFLOW ||--|{ WORKFLOW_STEP : "consists of"
      WORKFLOW_STEP ||--o{ JOB_QUEUE : "enqueued in"

      AGENT_WORKFLOW {
          uuid id
          uuid tenant_id
          string status "running, sleeping, completed, failed"
          json context_state
      }

      WORKFLOW_STEP {
          uuid id
          string action_type
          int retry_count
          timestamp scheduled_at
      }

      JOB_QUEUE {
          string priority
          string worker_pool
      }

      WORKER_NODE ||--o{ JOB_QUEUE : "consumes from"
      WORKER_NODE ||--o{ AGENT_WORKFLOW : "updates state"
  ```

  ### Key Architectural Invariants
  1. **Durable State and Resumption:** Every agent workflow must be persisted at every step. If a worker node crashes, the workflow must securely resume from the exact last successful step without duplicating side-effects (e.g., charging a customer twice).
  2. **Multi-Tenant Isolation at the Queue Level:** Job priorities and execution boundaries must strictly enforce `tenant_id` isolation. A spike in Maya's bakery orders must not delay the processing of Carlos's service quotes.
  3. **Yield and Wait Pattern:** Agents must be able to securely yield execution (e.g., "Wait 24 hours for customer response") without consuming active compute resources, re-awakening instantly when the event occurs or the timeout expires.

  ### UI Wireframes & Screen Flow (375px First)
  - **Merchant Dashboard View:** This is an invisible infrastructure upgrade; however, the impact is surfaced in the "Activity Feed." When Maya views an order, she sees a clean, chronological timeline of agent actions (e.g., "Operations Agent verified inventory," "Finance Agent sent invoice," "Ambassador Agent waiting for payment").
  - **Error State (Grandmother Test):** If a workflow permanently fails, instead of an error code, Maya sees a simple actionable card: "We couldn't automatically send the invoice to John due to a network issue. Tap here to send it now."

  ### Mobile UX Flow
  - The mobile UX remains frictionless. The mesh guarantees that when a merchant performs an action (e.g., "Auto-fulfill all orders"), the UI returns instantly, and the background mesh handles the load gracefully, updating the UI via real-time sockets or push notifications as milestones are reached.

  ### AI Agent Integration Points
  - **The Orchestrator Agent:** Dynamically spawns new workflows based on business events and monitors the health of the mesh, adjusting worker node scaling.
  - **Cross-Department Collaboration:** The Finance Agent can safely await a signal from the Operations Agent (e.g., "Inventory Picked") within the mesh before executing the "Capture Payment" step.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Unified Agentic Background Job Mesh. Design a durable state machine engine (or integrate a suitable robust queuing backend like Temporal, Oban, or a custom NATS JetStream implementation) that supports multi-step, stateful agent workflows. Ensure the system provides `yield/resume` capabilities and strict multi-tenant isolation. Integrate this mesh with the existing agent architecture so that long-running tasks (like daily business briefings or multi-channel message synthesis) are reliably executed with guaranteed retry and compensation mechanisms. Do not prescribe specific database tables, but ensure the state persistence is ACID compliant and horizontally scalable.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
