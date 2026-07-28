issue_title: "Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  # Issue Brief: Native Rust Omnichannel Inbox

  ## Problem Statement
  Owners like Carlos (the handyman) and Priya (the boutique operator) interact with customers across multiple platforms: SMS, WhatsApp, Instagram DMs, and website chat. Keeping track of these conversations across different apps is chaotic, leads to missed revenue, and makes it impossible for the AI assistant to help draft responses or coordinate tasks. We need a single unified inbox that brings all these conversations into one place, without relying on third-party dependencies like Chatwoot, which is being retired. The system must feel like a natural extension of the owner's daily work on their phone.

  ## Research Report
  - **Codebase & Feature Benchmarking**: Chatwoot’s source code (`https://github.com/chatwoot/chatwoot`) reveals a complex architecture for handling omnichannel messaging, including distinct `Channel` adapters (e.g., `Channel::Whatsapp`, `Channel::WebWidget`), a unified `Conversation` state machine, and real-time WebSocket updates. OHC must replicate this unified model natively in Rust to achieve feature parity while adhering to our strict multi-tenant Row Level Security (RLS) PostgreSQL architecture.
  - **Competitive Analysis**:
    - **Shopify Inbox**: Extremely focused on e-commerce. Great at showing cart contents alongside the chat, but lacks deep service/appointment integration.
    - **Wix Inbox**: Good unified approach, but often feels heavy and disconnected from the core operational flow.
    - **GoDaddy Conversations**: Basic omnichannel, but lacks AI automation and smart handoffs.
  - **Finding**: OHC's unique advantage will be tightly integrating this inbox with our AI agents. When a customer messages on Instagram, the AI should not just see the message, but immediately understand the customer's history, active orders, and draft a response for the owner to approve with one tap.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Client App - Flutter/Tauri] -->|WebSocket/REST| API(API Gateway - Axum)
      API --> Engine{Conversation Engine}
      Engine --> DB[(PostgreSQL - RLS Multi-Tenant)]
      Engine --> PubSub[Real-Time Publisher - Redis/Valkey]
      PubSub --> API
      Engine --> AgentModule[Agent Handoff Module]
      AgentModule --> Agents[OHC AI Agents]
      Webhooks[Webhooks/External APIs] -->|Ingress| Adapters[Channel Adapters]
      Adapters --> Engine
      Engine -->|Egress| Adapters
  ```

  ### Key Design Decisions
  - **Native Rust Implementation**: Build the engine natively within `onehumancorp/mono` to eliminate external dependencies and ensure tight integration with our existing data models and AI agents.
  - **Strict Multi-Tenancy**: Every entity (conversations, messages, contacts) must strictly adhere to our RLS policies using `tenant_id` to guarantee zero-trust data isolation.
  - **Event-Driven AI Integration**: Instead of polling, AI agents will subscribe to conversation events to provide real-time suggestions and automated task generation.

  ### Mobile UX Flow (375px First)
  - **Unified List View**: A high-density, scannable list of active threads. Each row shows the customer's avatar, channel icon (e.g., WhatsApp), a snippet of the latest message, and an unread indicator.
  - **Thread View**: Employs the macOS-style Translucent Glass materials. The chat bubble layout clearly distinguishes between customer messages, owner replies, and AI agent drafts.
  - **Smart Actions**: A fixed bottom bar (above the keyboard) provides one-tap actions: "Approve AI Draft", "Create Quote", or "Schedule Visit".

  ### AI Agent Integration Points
  - **Triage & Routing**: The AI assistant analyzes incoming messages to prioritize them and route them to the appropriate human or sub-agent.
  - **Draft Generation**: The Customer Relationship Assistant automatically drafts contextual replies based on the conversation history and business knowledge.

  ## Implementation Prompt
  **User-Facing Outcome:** The owner can open the OHC app on their phone and see a unified list of conversations from various channels. They can tap into a conversation, see the full history, and reply directly, with the message seamlessly routed back to the original channel.

  **Critical User Journey (CUJ):**
  1. Owner logs in and navigates to the Inbox.
  2. Owner sees a new message from a contact via a specific channel (e.g., Web Widget).
  3. Owner taps the conversation and reads the history.
  4. Owner types a reply and hits send.
  5. The reply appears in the thread and is successfully transmitted to the customer.

  **Acceptance Criteria:**
  - Implement the core database tables and Rust service layer for managing Contacts, Conversations, and Messages.
  - Ensure all database interactions strictly enforce multi-tenant RLS.
  - Expose API endpoints for listing conversations and sending/receiving messages.
  - Integrate a real-time mechanism (e.g., WebSocket) to push new messages to the client instantly.
  - Write comprehensive automated tests, including Playwright E2E tests, covering the entire CUJ from the perspective of a non-technical owner using realistic data.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
