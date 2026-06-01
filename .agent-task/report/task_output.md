issue_title: "Implement Autonomous Department Agent Event Sourcing"
issue_description: |
  # Architecture Discovery: Autonomous Department Event Mesh

  ## Problem Statement
  Currently, OneHumanCorp (OHC) is missing a robust event sourcing and messaging architecture to facilitate seamless communication between "Autonomous Departments" (Sales, Marketing, Operations, etc.). The existing `QueueManager` in `src/server/queue.rs` handles job dispatch, but there is no formalized event mesh that allows one department (e.g., Marketing) to autonomously trigger subsequent background actions in another (e.g., Finance) securely within a tenant boundary.

  ## Research Report
  Our competitive analysis shows that platforms like Shopify use reactive mechanisms (Shopify Sidekick) and legacy webhook chains for asynchronous tasks. By natively embedding an event sourcing mechanism (e.g., using Redis Pub/Sub for high-throughput or PG notify as fallback), OHC can enable truly invisible automation. For instance, when the `Generative Promoter` (Marketing) completes a social media draft, it should emit a `MarketingContentDrafted` event that the `Silent Ambassador` (Customer Success) can cache.

  ## Design Doc
  - **Architecture**: Introduce a Tenant-Isolated Event Mesh layer built on top of gRPC (internal service mesh) and Redis Pub/Sub.
  - **Data Schema additions**: Define a `DepartmentEvent` schema tracking source department, target department, `tenant_id`, and payload.
  - **Security**: Embed `tenant_id` at the lowest mesh level to ensure Zero-Trust multi-tenant isolation.

  ## Implementation Prompt
  Implement a scalable Event Mesh for Autonomous Departments. The mesh should accept events via a new gRPC service (`DepartmentEventBus`), securely validate the `tenant_id`, and distribute the events to listening agents (e.g., Operations, Finance) via a Redis-backed channel or PostgreSQL SKIP LOCKED table if standalone. E2E tests must verify that an event emitted by Marketing is successfully consumed by Customer Success.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
