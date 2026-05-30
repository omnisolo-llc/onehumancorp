issue_title: "[Architecture] High-Performance Agentic Background Job Queue"
issue_description: |
  # Problem Statement
  Small business owners like Priya (Boutique owner) or Leo (Music Tutor) run massive holiday campaigns or bulk inventory updates directly from their phones. When they tap "Launch Campaign" or "Sync Inventory," they expect it to happen instantly so they can close the app and get back to work. Currently, if an operation involves complex AI tasks—like drafting 50 custom emails or parsing dozens of product variants—it can freeze the app, show endless loading spinners, or even fail silently if their phone goes offline or the app is closed. This causes confusion and lost sales. They need an invisible system that guarantees their requested actions will be completed safely in the background, even if they turn off their phone.

  # Research Report
  **Competitive Analysis:**
  - **Shopify:** Utilizes a robust background processing system to handle email dispatches and inventory updates, ensuring the merchant dashboard remains highly responsive.
  - **Wix:** Processes background tasks but often surfaces loading indicators or delays to the user during complex operations like site duplication or mass inventory updates.
  - **Stripe:** Exceptional at background processing for webhooks and payment state transitions, ensuring high availability and exact-once processing semantics.

  **Discovery:**
  For OHC to succeed as an "invisible" OS, the background queue must handle potentially millions of AI-driven micro-tasks (e.g., parsing an Instagram DM, generating a quote, updating the ledger) reliably. It must support prioritization, retries, and rate-limiting to prevent LLM API exhaustion, all while keeping the mobile frontend instantly responsive.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Client (375px)
          App[OHC Mobile App] --> API[OHC API Gateway];
      end

      subgraph OHC Backend
          API --> ActionController[Action Controller];
          ActionController --> Queue[Distributed Job Queue (Redis/NATS)];
          ActionController --> DB[(Postgres Main DB)];
      end

      subgraph Worker Nodes
          Queue --> Worker1[Agent Worker Pool 1];
          Queue --> Worker2[Agent Worker Pool N];
          Worker1 --> OpsAgent[Operations Agent];
          Worker2 --> MarketingAgent[Marketing Agent];
      end

      subgraph External Systems
          OpsAgent --> LLM[LLM Provider API];
          MarketingAgent --> Email[Email Service];
      end
  ```

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      TENANT ||--o{ JOB : "enqueues"
      JOB {
          uuid id
          string type "e.g., ig_dm_reply, inventory_sync"
          json payload
          string status "pending, processing, completed, failed"
          int priority "0 (high) to 10 (low)"
          int retry_count
          timestamp created_at
          timestamp process_after
      }
      JOB ||--o{ JOB_LOG : "has"
      JOB_LOG {
          uuid id
          string message
          timestamp logged_at
      }
  ```
  **Invariants:**
  - **Multi-Tenant Isolation:** Every job must strictly encapsulate its `tenant_id`. Workers must assume the tenant context before execution to guarantee zero data leakage.
  - **Idempotency:** Job execution must be idempotent to handle at-least-once delivery semantics safely.

  ### Mobile UX Flow (375px First)
  1.  **Trigger:** Leo creates a new "Holiday Discount" campaign on his phone. He taps "Launch Campaign".
  2.  **Optimistic UI:** The button immediately transitions to a checkmark stating "Campaign Launched". No spinning loader blocks his screen.
  3.  **Background Processing:** The app enqueues a background job to generate personalized emails for his 50 active students using the Marketing Agent.
  4.  **Completion Notification:** Once the queue finishes processing (seconds or minutes later), Leo receives a subtle push notification: "Your Holiday Campaign emails have been sent!"

  ### AI Agent Integration Points
  - **The Orchestrator Agent:** Dynamically routes complex, multi-step tasks into a DAG (Directed Acyclic Graph) of smaller jobs to be executed concurrently by different departments.
  - **The Customer Success Agent:** Ingests high-priority jobs (e.g., a customer DM asking for a quote) and processes them ahead of lower-priority background syncs.

  ### Key Design Decisions
  - **Asynchronous by Default:** Any action that doesn't strictly require a synchronous response to the client must be offloaded to the queue.
  - **Priority Tiers:** The queue must support strict priority levels (e.g., P0 for real-time chat replies, P3 for daily reporting rollups).
  - **Graceful Degradation:** If LLM APIs are rate-limited or degraded, jobs must safely pause and exponentially back off without failing.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement a distributed, high-performance background job queue. Define the core `Job` and `Worker` interfaces. Ensure the queue supports priority levels, retries with exponential backoff, and strict multi-tenant isolation based on `tenant_id`. Create a robust API for enqueueing tasks from the Action Controller. The system must guarantee at-least-once delivery and encourage idempotent worker design. Do not surface queue internals or errors directly to the mobile UI; rely on optimistic updates and push notifications for status changes.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
estimated_scope: Large
