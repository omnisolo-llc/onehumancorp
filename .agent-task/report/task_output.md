issue_title: "Research: AI Assistant for Unified Customer Inbox & Communications"
issue_description: |
  # Research Report: AI Assistant for Unified Customer Inbox & Communications

  ## Problem Statement
  Small business owners and operators (like Maya the baker or Carlos the field service owner) struggle to keep up with customer communications across multiple channels (Instagram DMs, email, website forms, SMS). Messages fall through the cracks, context is lost between platforms, and replying manually is time-consuming. They need a unified inbox where an AI assistant not only aggregates messages but proactively drafts context-aware replies and links them to business operations (like orders or bookings).

  ## Research Report
  - **Market Context**: Platforms like HubSpot Breeze and Shopify Sidekick offer AI-assisted communication, but they are often complex to set up or tied strictly to e-commerce. AI-native tools like Lindy.ai show the potential of AI assistants managing communications directly.
  - **Target Persona Gap**: Our core personas (Maya, Carlos) need a single place on their mobile phone to see all customer inquiries. The current OHC Agent Feed concept (`docs/business/market_research/agent_feed_deep_dive.md`) outlines the vision, but a dedicated "Unified Inbox" view with AI drafting capabilities is missing from the core product architecture.
  - **Competitor Insights**: WeCom (Tencent) excels at integrating customer communication directly into daily operational workflows, providing a single view of the customer. OHC needs a similar unified approach, powered by our local agent harness.

  ## Design Doc
  ### Mobile UX Flow (375px first)
  1. **Unified Inbox View**: A simple, unified list of conversations across all channels. Each item shows the customer, channel icon, and a preview.
  2. **AI Draft Badge**: Conversations where the AI has drafted a reply show a distinct "Draft Ready" badge.
  3. **Conversation Thread**: Tapping a conversation opens the thread. The AI's suggested reply is pre-filled in the compose box, ready for one-tap approval ("Send") or editing.
  4. **Context Panel (Collapsible)**: A swipeable or collapsible panel showing the customer's history, active orders, or previous interactions, retrieved via the RAG system.

  ### Architecture & Integration
  - **Data Model**: Establish `Conversation`, `Message`, and `ChannelIntegration` entities with strict multi-tenant RLS in PostgreSQL.

  ```mermaid
  erDiagram
      Tenant ||--o{ Conversation : has
      Tenant ||--o{ ChannelIntegration : has
      Conversation ||--o{ Message : contains
      ChannelIntegration ||--o{ Conversation : sources
      Conversation {
          uuid id
          uuid tenant_id
          string customer_id
          string status
          timestamp updated_at
      }
      Message {
          uuid id
          uuid conversation_id
          uuid tenant_id
          string content
          string sender_type
          string draft_status
          timestamp created_at
      }
      ChannelIntegration {
          uuid id
          uuid tenant_id
          string provider
          jsonb credentials
      }
  ```

  - **Agent Coordination**:
    - **Ingestion**: Webhooks (e.g., IG Graph API) publish to Redis Pub/Sub.
    - **Triage Agent**: Classifies intent and urgency using Go workers.
    - **Drafting Agent (LLM Node)**: Uses RAG against customer history to generate a `MessageDraft` via the Go API layer.
  - **UI Integration**: The Flutter app subscribes to updates. Action cards for urgent messages can be pushed to the main Agent Feed, while the dedicated Inbox tab provides the full view.

  ```mermaid
  sequenceDiagram
      autonumber
      participant Webhook as External Channel (IG/Email)
      participant API as Go API Server
      participant Queue as Job Queue (PG/Redis)
      participant Agent as Triage/Drafting Agent (Go)
      participant DB as PostgreSQL
      participant UI as Flutter Mobile App

      Webhook->>API: Receive incoming message
      API->>DB: Store raw message
      API->>Queue: Enqueue message for triage
      Queue-->>Agent: Dequeue message
      Agent->>DB: Fetch customer context (RAG)
      Agent->>Agent: Generate LLM draft response
      Agent->>DB: Update message with draft
      Agent->>API: Publish drafting complete event
      API->>UI: Push notification/WebSocket update
      UI-->>API: Owner taps "Approve & Send"
      API->>Webhook: Send API request to external channel
      API->>DB: Mark message as sent
  ```

  ### Key Design Decisions
  - The AI does not auto-send by default; the owner must explicitly approve (One-tap "Send" or "Edit").
  - The inbox must work offline-first on mobile, syncing when connected.

  ## Implementation Prompt
  **Goal**: Implement the foundational Go backend and Flutter mobile UI for the Unified Inbox, featuring AI-drafted responses.

  **CUJ (Critical User Journey)**:
  1. Maya opens the OHC app and goes to the "Inbox" tab.
  2. She sees a new Instagram DM from a customer asking about vegan cake availability.
  3. The conversation has an "AI Draft Ready" badge.
  4. She taps the conversation. The AI has drafted: "Yes, we have vegan cakes available! Would you like to order?" based on her inventory.
  5. She taps "Approve & Send".

  **Acceptance Criteria**:
  - Implement the `Conversation` and `Message` data models in PostgreSQL with multi-tenant isolation (RLS).
  - Create the Go gRPC/REST API endpoints for listing conversations and fetching threads.
  - Implement a mobile-first (375px) Flutter view for the Unified Inbox list and conversation thread.
  - Integrate a basic AI drafting agent in Go that listens for new messages and generates a draft response via the LLM provider.
  - Ensure all new code has 100% unit test coverage.
  - Add at least one Playwright E2E test covering the CUJ (simulating an incoming message and the owner approving the draft).
  - Use the established Translucent Glass design tokens for the Flutter UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
