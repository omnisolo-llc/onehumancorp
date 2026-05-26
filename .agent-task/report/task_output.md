issue_title: "[Architecture] High-Performance Agentic Background Job Queue"
issue_description: |
  # Research Report: High-Performance Agentic Background Job Queue

  ## Problem Statement
  Small business owners rely on OneHumanCorp (OHC) to automate complex tasks silently in the background. Whether it's the Operations Agent updating inventory across channels for Priya's boutique, or the Marketing Agent drafting follow-up emails for Leo's tutoring students, these tasks must execute reliably without impacting the frontend responsiveness. Currently, as the user base scales, synchronous processing or inefficient batching can lead to dropped tasks, sluggish UI performance, and delayed automations. We need a high-performance, distributed background job queue designed specifically for agentic workflows to ensure seamless, invisible automation.

  ## Research Report
  - **Competitive Analysis**:
    - **Shopify:** Utilizes a robust background processing system (Sidekiq/Resque equivalents) to handle webhooks, email dispatches, and inventory updates. This ensures the merchant dashboard remains highly responsive.
    - **Wix:** Processes background tasks but often surfaces loading indicators or delays to the user during complex operations like site duplication or mass inventory updates.
    - **Stripe:** Exceptional at background processing for webhooks and payment state transitions, ensuring high availability and exact-once processing semantics.
  - **Market Needs**: For OHC to succeed as an "invisible" OS, the background queue must handle potentially millions of AI-driven micro-tasks (e.g., parsing an Instagram DM, generating a quote, updating the ledger) with sub-second latency for critical tasks and guaranteed delivery. It must support prioritization, retries, and rate-limiting to prevent LLM API exhaustion.

  ## Design Doc
  - **Architecture Diagram**:
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
  - **Business Journey Mapping**:
    - **Acquisition & Onboarding**: When a user registers (e.g., Maya), the Marketing agent asynchronously triggers welcome tasks via the background queue.
    - **Activation & Retention**: During daily operations, the queue powers the Operations agent to sync inventory and the CS agent to automatically draft Instagram DM replies, preventing the "never-ending inbox" fatigue.
    - **Revenue & Referral**: When a new sale is made (e.g., Leo's lessons), the ledger is updated asynchronously with exact-once guarantees, ensuring financial accuracy. The queue manages these background automation processes invisibly, protecting the merchant's 375px mobile experience from any performance overhead.
  - **Data Model & Invariants**:
    ```mermaid
    erDiagram
        TENANT ||--o{ JOB : "enqueues"
        JOB {
            uuid id PK
            uuid tenant_id FK
            string queue_name "e.g., ai-high, webhooks, email"
            json payload
            string status "queued, processing, completed, failed, retrying"
            int retries
            timestamp scheduled_at
            timestamp created_at
        }
    ```
  - **Mobile-First UX Flow**: No user-facing UI is required. The UI simply returns immediately when an action is dispatched. For long-running tasks, the action feed can optimisticly display a "queued" or "processing" state without blocking the user.
  - **Performance & Offline Targets**: P99 queueing latency < 100ms. Exact-once execution for financial ledger updates. Configurable backoff and jitter for LLM API retries. Zero-trust execution with tenant-isolated worker processes.

  ## Implementation Prompt
  Implement a robust agentic background job queue based on this architectural design. Use a highly reliable message broker like NATS JetStream or Redis to handle task dispatching. The solution should expose an API for enqueuing jobs with priorities and delayed execution. The worker pool must be isolated per tenant and support graceful degradation during LLM API rate limits.
  - **Acceptance Criteria**: The system can enqueue, execute, and monitor millions of asynchronous micro-tasks reliably, without impacting the API response times of the main client. Implement comprehensive testing for exact-once semantics.
  - **Priority**: P1
  - **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
