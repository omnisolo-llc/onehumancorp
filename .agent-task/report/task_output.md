issue_title: "[Architecture] Proactive Agent Feed System & Action Cards Engine"
issue_description: |
  # Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by daily tasks required to run their businesses. They often miss important updates, fail to follow up with customers, or forget to reorder inventory because they are busy with their craft. Existing platforms provide dashboards but require the user to actively check them ("pull" model). This is a pain point for non-technical users who need a "push" model where the system proactively works for them.

  # Research Report
  Based on the competitive landscape (Shopify, Wix, Squarespace), there is a gap in autonomous agentic proactive feeds. Shopify requires third-party plugins. Wix/Squarespace lack proactive intelligence. OHC must transition from a reactive tool to a proactive partner by implementing an Agent Feed—a central nervous system that pushes critical updates, drafted communications, and suggested actions directly to the user's mobile device.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  erDiagram
      EVENT_BUS ||--o{ AGENT_FEED_ENGINE : "Subscribes"
      AGENT_FEED_ENGINE ||--o{ LLM_RESOLVER : "Classifies intent & drafts"
      AGENT_FEED_ENGINE ||--o{ REDIS_LOCKS : "Coordinates agents"
      AGENT_FEED_ENGINE }|--|| AGENT_ACTIONS_TABLE : "Persists actions"

      AGENT_ACTIONS_TABLE {
          string id PK
          string tenant_id
          string agent_id
          string action_type
          json payload
          string status
      }

      AGENT_ACTIONS_TABLE ||--o{ MOBILE_UI : "Pushes Action Cards"
  ```

  ## UI Wireframes & UX
  - **Mobile First:** 375px viewport target.
  - **Feed View:** A scrollable list of "Action Cards".
  - **Action Card:** Translucent glassmorphic cards (`backdrop-filter: blur(20px)`) displaying a proactive suggestion (e.g., "Drafted an email for 3 abandoned carts" or "Vegan cake availability inquiry"). Contains prominent 1-tap "Approve" or "Dismiss" buttons.

  ## Data Model & Invariants
  - Multi-tenant isolation using `tenant_id` on the `agent_actions` table.
  - Redis Redlock (`ohc:lock:{tenant_id}:agent_action:{resource_id}`) to prevent duplicate actions/drafts from concurrent AI agent processes.
  - PostreSQL `SKIP LOCKED` pattern for asynchronous action queue processing.

  # Implementation Prompt
  Implement the backend infrastructure for the Agent Feed.
  1. Define the exact schema for the `agent_actions` table, ensuring strict multi-tenant isolation.
  2. Implement an `AgentFeedService` (Rust) that can receive events, coordinate with the LLM provider to generate an action draft (e.g., a customer DM reply), and persist it as a pending action card in the database.
  3. Use Redis Redlock to coordinate agents so multiple agents don't draft conflicting responses to the same event.
  4. Create a REST endpoint `/api/v1/agent-feed` that the mobile client can poll/fetch to display the feed of pending action cards.
  5. Include full unit tests with 100% coverage.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
