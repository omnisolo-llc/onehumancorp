issue_title: "[Architectural Design] Custom Rust Omnichannel Chat Engine"
issue_description: |
  # Research Report: Architectural Design for Native Rust Omnichannel Chat System

  ## 1. Problem Statement
  OneHumanCorp currently has a major capability gap in omnichannel customer communication. While the initial architecture considered using Chatwoot as a third-party dependency, the new directive mandates the complete retirement of Chatwoot as an external service. Instead, OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine natively implemented in Rust, built entirely within `onehumancorp/mono`.

  Non-technical owner/operators like Maya (Baker) or Carlos (Handyman) need a unified inbox where they can seamlessly communicate with customers across multiple channels (Instagram DMs, WhatsApp, SMS, Web Chat) without juggling different apps or understanding underlying APIs. This unified inbox must be integrated directly into OHC, supporting agent-assisted responses and automation natively.

  ## 2. Research Report
  - **Chatwoot Source Code Audit:** I have cloned and audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`). Key architectural components identified for replication in Rust:
    - **Models:** `Account`, `User`, `Inbox`, `Channel`, `Contact`, `Conversation`, `Message`, `AgentBot`, `AutomationRule`, `CannedResponse`, `Macro`.
    - **Channels:** Modular adapters for Web Widget, Email, API, WhatsApp, SMS, Line, Telegram, Facebook/Instagram.
    - **Real-time:** WebSockets (ActionCable in Chatwoot) for real-time messaging, presence, and typing indicators.
    - **Routing:** Assignment policies, working hours, and inbox members.
    - **AI/Bots:** Agent bots that can intercept and respond to conversations.
  - **Competitive Analysis:** Platforms like Shopify Inbox, Zendesk, and Intercom provide unified messaging, but often lack deep, native AI-agent integration *as an active participant* that can take actions (like drafting a quote or booking a calendar slot) rather than just suggesting text.
  - **OHC Gap:** OHC currently lacks the Rust data models, channel adapters, and WebSocket infrastructure required to support this unified, agent-enabled inbox.

  ## 3. Design Doc (Architecture)

  ### 3.1 Data Model & Invariants
  - **Multi-tenant Isolation:** Every entity must belong to an `account_id` (mapping to OHC's `tenant_id`) with PostgreSQL Row Level Security (RLS) enforced.
  - **Core Entities:**
    - `Account`: The business/tenant.
    - `User`: Team members/agents.
    - `Contact`: The customer across channels.
    - `Inbox`: A grouping mechanism for conversations.
    - `ChannelAdapter`: A polymorphic relationship linking an Inbox to a specific channel configuration (e.g., WhatsApp credentials, Web Widget settings).
    - `Conversation`: A thread between Contacts, Users, and AgentBots within an Inbox.
    - `Message`: Individual messages within a Conversation (Text, Attachments, Template).

  ### 3.2 Architecture Diagram
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : has
      ACCOUNT ||--o{ USER : employs
      ACCOUNT ||--o{ CONTACT : manages
      INBOX ||--|| CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      USER ||--o{ CONVERSATION : assigned_to
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### 3.3 AI Department Coordination
  - **Customer Service Agent:** Intercepts incoming messages via a Redis background queue. Evaluates the message using the LLM and tenant context. Can draft a response, execute an action (e.g., `create_booking`), or escalate to a human (User) by modifying the Conversation's assignment/status.
  - **Marketing/Sales Agents:** Can inject outbound messages or follow-ups into existing conversations based on campaigns or abandoned carts.

  ### 3.4 Mobile-First UX Flow
  - **Unified Inbox View (375px):** A clean vertical list of active conversations. Unread indicators and agent-drafted suggestions are prominently displayed using OHC Premium Tokens (Glassmorphism, clean typography).
  - **Conversation View (375px):** Standard chat interface. Input area uses native mobile keyboard. A prominent "Agent Suggestion" floating action button or inline card appears if the AI has drafted a reply.
  - **Performance Targets:** <100ms latency for message delivery via WebSockets (Rust Tokio/Tungstenite). Offline capability for reading recent messages and queueing outbound messages using a PWA Service Worker or mobile local database.

  ## 4. Implementation Prompt
  **Objective:** Implement the core domain data models and PostgreSQL migrations for the native Rust Omnichannel Chat Engine.

  **User-Facing Outcome:** The foundational database schema is established, allowing subsequent implementation of the unified inbox API and UI. This ensures that when a customer messages a business (e.g., Maya's Bakery), the data is structured correctly to support real-time delivery and AI agent intervention.

  **Acceptance Criteria:**
  1. Create SQLx migrations defining the schema for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2. Implement strict multi-tenant isolation (RLS) using `tenant_id` on all new tables.
  3. Create Rust domain models (`struct` definitions) and repository traits for these entities in `src/server/domain/`.
  4. Ensure all models include necessary fields for basic chat functionality (sender, receiver, content, status, timestamps).
  5. Achieve 100% unit test coverage for the new models and repository traits.

  **Note:** Do not implement the API endpoints, WebSocket handlers, or frontend UI in this task. Focus strictly on the core data persistence layer.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
