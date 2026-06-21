issue_title: "Implement the \"Agent Feed\" Backend Architecture and Proactive Notifications System"
issue_description: |
  **Problem Statement**
  Business owners (like Maya the baker or Carlos the handyman) currently have to seek out information in OHC: checking an inbox, viewing a calendar, or reading a dashboard. This is a traditional software paradigm, not a work-assistant paradigm. The promise of OHC is: "Open OHC and immediately know what needs attention today." Without a centralized, AI-driven Agent Feed that proactively pushes drafted communications and suggested actions to the user, OHC is just another admin portal.

  **Research Report**
  Competitor research shows that most SMB software (Shopify, Wix) relies on dashboards and notification badges. Tencent Workbuddy and advanced CRM tools push actionable feeds, but they lack native AI integration. The "Agent Feed" is OHC's key differentiator: a central nervous system that brings "Invisible AI Automation" to life. Based on `docs/business/market_research/agent_feed_deep_dive.md`, the Feed must aggregate events (DMs, bookings, payments), classify intents via LLMs, gather context, draft responses or proposed actions, and present them as simple "Action Cards" (Approve, Edit, Discard).

  **Design Doc**
  - **Architecture:**
    - A new `AgentFeedService` in the backend (using Rust).
    - It listens to the event bus (or webhook handlers) for incoming events.
    - It interacts with the LLM API (Gemini/MiniMax) to classify the event, build context, and draft a response/action.
    - It stores the resulting `AgentFeedItem` in the `agent_feed_items` database table.
    - The API exposes endpoints for the mobile client to fetch the feed and act on items (Approve, Edit, Discard).

  - **Architecture Diagram:**
    ```mermaid
    sequenceDiagram
        participant Client as Mobile App
        participant API as OHC API Gateway
        participant FeedService as AgentFeedService
        participant LLM as AI Provider
        participant DB as PostgreSQL

        Note over FeedService: External Event Occurs (e.g. DM)
        FeedService->>LLM: Classify intent & gather context
        LLM-->>FeedService: Return Drafted Action / Context
        FeedService->>DB: Persist AgentFeedItem
        Client->>API: Fetch Agent Feed
        API->>DB: Query tenant feed items
        DB-->>API: Return feed items
        API-->>Client: Render Action Cards
    ```

  - **Mobile UX Flow:** The home screen of the OHC app is the Agent Feed. It displays a list of Action Cards. Each card shows the context (e.g., "Maya asked about vegan cakes") and the proposed action (e.g., "Send drafted response").
  - **AI Agent Integration:** The service orchestrates the prompt engineering. It provides the event data, relevant tenant context, and asks the LLM to output a structured proposed action.

  **Implementation Prompt**
  Implement the backend logic for the Agent Feed.
  1.  Ensure the `agent_feed_items` table is properly managed (add migrations if needed).
  2.  Create the `AgentFeedService` which can take an incoming event (e.g., a simulated DM), call the LLM provider to classify and draft a response, and persist the `AgentFeedItem`.
  3.  Implement REST/gRPC endpoints to list feed items for a tenant and to execute an action on an item (Approve/Discard).
  4.  Ensure robust error handling and multi-tenant isolation.
  5.  Write unit tests for the service and endpoints.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
