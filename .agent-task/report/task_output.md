issue_title: "Implement Agent Feed Event Ingestion and Push"
issue_description: |
  # Research Report: Agent Feed Infrastructure

  ## Problem Statement
  Owners using OHC currently must seek out information (e.g., check for new messages, check inventory). To fulfill the "Invisible AI Automation" vision, OHC needs a proactive "Agent Feed" that pushes critical updates, drafted communications, and suggested actions directly to the owner. The core missing piece is a robust event ingestion pipeline and push mechanism.

  ## Research Findings
  Based on `docs/business/market_research/agent_feed_deep_dive.md`, the Agent Feed requires three core components:
  1.  **Event Ingestion Pipeline**: Central message bus (Redis Pub/Sub or Kafka) for incoming webhooks, state changes, and scheduled jobs.
  2.  **Intent & Context Resolution (LLM Layer)**: To classify intent and generate drafts.
  3.  **Notification & Approval UX**: Pushing "Action Cards" to a mobile-first interface.

  Competitor analysis shows that proactive surfacing of actions (like Shopify Sidekick or Microsoft Copilot) significantly increases user engagement and reduces cognitive load compared to traditional dashboards.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Webhooks/Internal Events] -->|Publish| B(Event Bus - Redis Pub/Sub);
      B --> C{Agent Workers};
      C -->|Process & Query State| D[(Postgres DB)];
      C -->|Generate Draft| E[LLM Provider];
      C -->|Create Feed Item| F[(Feed Store)];
      F --> G[Client Apps (Mobile/Web)];
  ```

  ### Core Components
  1.  **Event Bus**: Utilize Redis Pub/Sub for lightweight, reliable event distribution across workers.
  2.  **Agent Workers**: Go-based background workers that subscribe to the event bus, query necessary context, and call the LLM to generate action items.
  3.  **Feed Store**: A new Postgres table to store feed items (Action Cards) for each tenant.
  4.  **Client Sync**: Mechanism to push new feed items to connected clients (e.g., SSE or WebSockets).

  ### Mobile UX Flow
  -   The main screen is the Agent Feed.
  -   Feed items appear as distinct cards (e.g., "Draft Reply: Maya's Cakes").
  -   Each card has clear actions: "Approve", "Edit", "Discard".
  -   Action cards take priority over raw data tables.

  ## Implementation Prompt

  **User Goal**: As an owner (like Maya), I want to open my OHC app and immediately see actionable items (like drafted replies to Instagram DMs) pushed to my feed, so I can approve them with one tap and get back to work.

  **Acceptance Criteria**:
  1.  Implement a basic Redis Pub/Sub event ingestion pipeline in the Go backend.
  2.  Create a background worker that listens for a "test" event, simulates an LLM draft generation, and stores it as a feed item in the database.
  3.  Expose an API endpoint to retrieve feed items for a given tenant.
  4.  Ensure the solution supports multi-tenancy.

  ## Scope & Priority
  -   **Priority**: P0
  -   **Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
