issue_title: "Implement Invisible Auto-Replenishment for Digital Goods (AI Operations Assistant)"
issue_description: |
  # AI Subscription Replenishment Engine for OHC

  ## Problem Statement
  Business owners (like Maya the Baker or Priya the Boutique Owner) currently have to manually reorder digital goods or monitor inventory levels. There is no automated, AI-driven way to ensure that digital goods or standard inventory are auto-replenished or reordered when they run low, causing stockouts and lost revenue.

  ## Research Report
  Our competitive analysis indicates that Shopify and Wix require third-party apps for robust subscription and auto-replenishment flows. Link-in-bio tools entirely lack this. OHC can differentiate by embedding an AI Operations Assistant that monitors stock levels and autonomously drafts reorder or replenishment tasks for the owner to approve via a push notification (Agent Feed).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ INVENTORY_ITEM : "owns"
    INVENTORY_ITEM ||--o{ REPLENISHMENT_RULE : "has"
    REPLENISHMENT_RULE ||--o{ AGENT_ACTION : "triggers"
    AGENT_ACTION ||--o{ NOTIFICATION : "generates"

    TENANT {
      uuid tenant_id PK
      string name
    }

    INVENTORY_ITEM {
      uuid item_id PK
      uuid tenant_id FK
      string sku
      int current_stock
      int threshold
    }

    REPLENISHMENT_RULE {
      uuid rule_id PK
      uuid item_id FK
      uuid tenant_id FK
      string action_type
    }

    AGENT_ACTION {
      uuid action_id PK
      uuid rule_id FK
      uuid tenant_id FK
      string status
      timestamp created_at
    }
  ```

  ```mermaid
  sequenceDiagram
    participant InventoryService
    participant EventBus
    participant OpsAgent
    participant AgentFeed
    participant MobileApp

    InventoryService->>EventBus: Publish InventoryLowEvent
    EventBus->>OpsAgent: Consume InventoryLowEvent
    OpsAgent->>OpsAgent: Retrieve context (tenant, item, supplier)
    OpsAgent->>OpsAgent: Draft replenishment ActionCard
    OpsAgent->>AgentFeed: Create ActionCard
    AgentFeed->>MobileApp: Push Notification
    MobileApp->>AgentFeed: Owner approves ActionCard
    AgentFeed->>OpsAgent: Approve Action
    OpsAgent->>InventoryService: Execute Replenishment
  ```

  ### Data Model & Invariants
  - Multi-tenant isolation MUST be strictly enforced at the database level. Every table (`INVENTORY_ITEM`, `REPLENISHMENT_RULE`, `AGENT_ACTION`, etc.) must include `tenant_id` and utilize PostgreSQL's Row Level Security (RLS) feature ensuring users can only access their data.

  ### Zero Trust & Security
  - All communication between internal microservices (e.g., `InventoryService`, `OpsAgent`, `AgentFeed`) must be authenticated and authorized using mutual TLS (mTLS) backed by SPIFFE/SPIRE.
  - Identity and authorization decisions are continuously evaluated per request without implicit trust between internal services.

  ### Mobile UX Flow
  - 375px optimized layout: When the owner opens the notification, the application presents a single-focused Action Card using translucent materials (OHC Premium Token).
  - The card clearly states the problem (e.g., "Item X is below threshold (5 remaining).") and provides a one-tap primary "Approve Reorder" button alongside a secondary "Dismiss" option. The interactions must be snappy with immediate optimistic UI updates reflecting the user's choice.

  ## Implementation Prompt
  Implement the backend event pipeline (in Go or Rust as appropriate for the target service) that listens for inventory updates and triggers an AI draft for stock replenishment. Update the database schema to support replenishment rules, ensuring RLS for tenant isolation. Ensure this flows into the existing Agent Feed system. Verify the flow end-to-end where a low inventory state creates an Action Card that the owner can approve or dismiss on the mobile interface.

  ## Priority: P1
  ## Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
