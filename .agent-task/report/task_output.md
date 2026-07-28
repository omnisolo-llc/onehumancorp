issue_title: "Native Rust Omnichannel Chat & Support Engine"
issue_description: |
  # Native Rust Omnichannel Chat & Support Engine

  ## Problem Statement
  The legacy external chat integration as a third-party service is being 100% retired. OHC needs its own native, high-performance omnichannel chat system built in Rust. Owners like Maya (baker), Carlos (handyman), and Priya (boutique owner) require a unified inbox to manage customer requests from Instagram DMs, WhatsApp, SMS, and website chat without juggling multiple apps. Without a native solution, multi-tenant isolation, real-time sync, and agent-assisted responses remain fragmented and brittle.

  ## Research Report
  Based on an audit of the legacy external repository source code (`app/models/`):
  - **Data Entities:** The legacy system uses `conversation`, `message`, `inbox`, `contact`, `channel`, `webhook`, `agent_bot` to manage omnichannel support.
  - **Mechanics:** It employs WebSocket for real-time messaging, channels for integrations (WhatsApp, email, web widget), and macros/canned responses for agent efficiency.
  - **Competitors:** Zendesk, Shopify Inbox, and Wix Inbox offer similar capabilities, but OHC's key differentiation is the integration of AI Work Assistants directly into the unified inbox, allowing proactive agent responses.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ ChannelAdapter : configures
      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : initiates
      Conversation ||--o{ Message : has
      Message }o--|| User : authored_by_agent
      Message }o--|| Contact : authored_by_customer
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox List:** A translucent, clean Unifi-style list of active conversations, tagged by source (Instagram, Web).
  2. **Conversation View:** A chat interface where an AI draft is presented natively.
  3. **Action:** The owner reviews the AI draft and taps "Send" or edits it using the mobile-native keyboard.

  ### AI Agent Integration
  - **Operations Agent:** Monitors incoming messages and creates task drafts if a service request is detected.
  - **Customer Assistant Agent:** Drafts replies based on historical context, utilizing RAG on past conversions.

  ## Implementation Prompt
  Implement the Core Chat Data Models & WebSocket Gateway in Rust.
  - Create the `Conversation`, `Message`, `Inbox`, and `Contact` entities.
  - Implement strict row-level security (`tenant_id` check) for multi-tenancy.
  - Build a real-time WebSocket messaging layer for the web/mobile client.
  - Do NOT prescribe the exact database schema here; design it such that it integrates smoothly with OHC's existing Postgres and Spike/Redis infrastructure.
  - Ensure the API is fully exposed over gRPC and JSON/REST.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
