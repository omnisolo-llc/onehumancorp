issue_title: "Implement Agent Feed Core API and WebSockets for the Central Nervous System"
issue_description: |
  # Agent Feed Implementation: The Central Nervous System

  ## Problem Statement
  Business owners like Maya and Priya currently lack a centralized view of all agent activities. The Agent Feed is the "Central Nervous System" of OHC, replacing traditional dashboards by proactively pushing critical updates, drafted communications, and suggested actions directly to their mobile devices for review and approval. Without this feed, users cannot easily monitor or interact with the autonomous agents managing their business, breaking the "Invisible AI Automation" promise.

  ## Research Report
  Our competitive analysis shows that traditional platforms require users to manually seek out information across fragmented tools. The OHC Agent Feed fundamentally changes this by unifying event streams (e.g., social DMs, inventory changes) and presenting actionable cards (Approve, Edit, Discard) in a single mobile-optimized feed.

  ## Design Doc

  ### Architecture
  - **Database**: PostgreSQL with `agent_feed_items` table and row-level security for tenant isolation.
  - **Event Bus**: Redis Pub/Sub for real-time WebSocket updates.
  - **API**: Axum REST endpoints for fetching the feed history and a WebSocket endpoint for real-time updates.
  - **Cache**: HybridCache for fast read access to the feed.

  ### AI Agent Integration
  - Agents (e.g., The Ambassador, The Promoter) publish proposed actions to the `agent_feed_items` table and broadcast via Redis to trigger real-time UI updates.

  ### Mobile UX Flow
  - The feed must be the primary landing screen, optimized for 375px viewports.
  - Items are displayed as actionable cards with clear intent and context.

  ## Implementation Prompt
  - Create the API routes in `src/server/api/agent_feed.rs` to fetch feed items from `AgentFeedRepository`.
  - Implement a WebSocket handler that subscribes to the Redis `agent_feed:{tenant_id}` channel and streams events to the connected client.
  - Ensure the feed aggregates data from `agent_feed_items` and existing `agent_approvals`.
  - Add comprehensive E2E tests validating the feed rendering and WebSocket real-time updates.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
