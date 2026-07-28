issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: Implementing Custom Rust Omnichannel Chat System to Replace Chatwoot

  ## Problem Statement
  OneHumanCorp (OHC) is retiring Chatwoot as an external third-party service for its omnichannel customer support and chat engine. Relying on an external service creates friction, latency, and operational overhead that contradicts OHC's vision of an owner-centered assistant where tools and agents run seamlessly behind the scenes. We need a native Rust implementation integrated directly into `onehumancorp/mono` that achieves feature parity with Chatwoot, providing a fast, secure, and multi-tenant chat system that feels native to the OHC platform and meets the strict mobile-first design requirements.

  ## Research Report
  ### The Gap
  Currently, OHC relies on Chatwoot for handling omnichannel communications (web widget, email, social media DMs). This dependency introduces:
  - **Latency and Synchronization Issues**: Cross-service data syncing creates delays and potential data inconsistencies.
  - **Security Concerns**: Managing API keys and webhooks between services adds security risks.
  - **Operational Complexity**: Maintaining and scaling a separate Chatwoot instance is resource-intensive.
  - **Suboptimal User Experience**: The external integration can lead to a fragmented experience for the owner/operator, who expects a unified "command center" feel.

  ### Competitive Analysis
  - **Chatwoot**: Provides robust omnichannel support (Web, Email, SMS, WhatsApp, Twitter, Facebook), multi-agent routing, canned responses, and reporting. However, it's a monolithic Ruby on Rails application, which doesn't align with OHC's high-performance Rust backend.
  - **Shopify Inbox**: A strong native chat solution for merchants, offering unified messaging, automatic replies, and order context. It serves as a good benchmark for user experience but is proprietary to Shopify.
  - **Intercom**: Offers a comprehensive suite of customer communication tools, but it's expensive and not designed for the specific needs of small business owners/operators.

  ### Goal
  Build a native Rust chat system within OHC that replicates essential Chatwoot functionalities:
  - **Unified Inbox**: Aggregating messages from multiple channels.
  - **Multi-channel Support**: Web widget, email, and social media integrations.
  - **Real-time Messaging**: WebSockets for instant communication.
  - **Agent Routing**: Assigning conversations to appropriate agents (human or AI).
  - **Canned Responses/Macros**: Pre-defined replies for common queries.
  - **Data Isolation**: Strict multi-tenant row-level security.

  ## Design Doc
  ### High-Level Architecture
  The new native chat system will be built as a set of Rust microservices/crates within the OHC monorepo, communicating via gRPC internally and exposing REST/WebSocket APIs externally.

  ```mermaid
  graph TD
      A[Client (Web/Mobile)] -->|REST/WebSocket| B(API Gateway)
      B --> C{Chat Service (Rust)}
      C --> D[Database (PostgreSQL)]
      C --> E[Cache/PubSub (Redis)]
      C --> F[Agent Routing Engine]
      C --> G[Channel Adapters]
      G --> H(Web Widget)
      G --> I(Email Provider)
      G --> J(Social Media APIs)
  ```

  ### Data Model & Invariants
  - **Tenant Isolation**: Every table must have a `tenant_id` and enforce Row-Level Security (RLS).
  - **Entities**:
    - `Inbox`: Represents a specific channel (e.g., Support Email, Web Widget).
    - `Conversation`: A thread of messages between a customer and agents/AI.
    - `Message`: Individual text/media sent within a conversation.
    - `Contact`: The external user/customer.
    - `ChannelAdapter`: Configuration for specific channels (e.g., API keys, webhooks).

  ### AI Agent Integration
  - **Work Triage**: AI agents will analyze incoming messages, categorize them, and either auto-reply (based on confidence) or route them to a human agent.
  - **Customer Context**: The AI will have access to the customer's history and preferences, drafting contextual replies.

  ### Mobile UX Flow (375px)
  1. **Unified Inbox View**: A list of active conversations, prioritized by urgency and status.
  2. **Conversation View**: A standard chat interface with real-time updates.
  3. **Action Menu**: Quick access to macros, customer details, and order history without leaving the chat view.

  ## Implementation Prompt
  **Goal**: Implement the core backend infrastructure and API endpoints for the native Rust chat system, focusing on the Unified Inbox, Conversation management, and WebSocket support for real-time messaging.

  **Critical User Journey (CUJ)**:
  - An owner (e.g., Maya the baker) opens the OHC app.
  - She sees a unified inbox with messages from her web widget and Instagram DMs.
  - She opens a conversation, sees the AI-drafted reply, and clicks "Send."
  - The message is delivered in real-time via WebSockets.

  **Acceptance Criteria**:
  - Implement PostgreSQL schemas for `Inbox`, `Conversation`, and `Message` with strict `tenant_id` RLS.
  - Create REST API endpoints for fetching inboxes, conversations, and messages.
  - Implement WebSocket endpoints for real-time message broadcasting (using Redis PubSub for scaling).
  - Ensure 100% unit test coverage for the new Rust codebase.
  - Write Playwright E2E tests covering the complete flow from receiving a message to sending a reply.
  - Provide a clear, mobile-first (375px) UI implementation for the Unified Inbox and Conversation views, utilizing OHC Premium Tokens.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
