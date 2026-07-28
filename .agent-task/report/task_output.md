issue_title: "[Architectural Feature] Native Rust Omnichannel Chat System"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  As mandated by the engineering standards, OneHumanCorp (OHC) is retiring external reliance on third-party services like Chatwoot for omnichannel customer support and messaging. We need to implement a native, high-performance, and multi-tenant chat engine in Rust within the `onehumancorp/mono` repository to support our owner/operator personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun) in managing communications seamlessly across multiple channels (Web Widget, Email, WhatsApp, Instagram, Facebook, SMS, etc.).

  Currently, our users rely on disparate communication tools, leading to fragmented customer relationships, missed leads, and a lack of unified context for AI agents. An in-house omnichannel system will ensure strict row-level security (RLS) multi-tenancy, deep integration with our AI assistants (Customer, Operations, Sales), and alignment with our mobile-first, translucent glass design principles without exposing the complexity to the owners.

  ## Research Report
  - **Chatwoot Source Code Audit**: A clone and analysis of the `chatwoot/chatwoot` repository (v3) revealed the core domain models required for an omnichannel inbox:
    - **Inbox**: The central configuration point for a channel, linking to `Channel` records (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, etc.).
    - **Conversation**: Represents a thread between a `Contact` and an `Agent`/Bot.
    - **Message**: Individual messages within a conversation, supporting text, attachments, and structured templates.
    - **Contact**: Represents the end-user (customer).
    - **Channel Adapters**: Handlers for specific communication platforms.
  - **Competitor Systems Analysis**: We looked at Shopify Inbox and Stripe's communication tools. Shopify Inbox provides a deeply integrated, minimal-setup chat experience that directly ties into product catalogs and order history, which aligns perfectly with OHC's goals for our business personas.
  - **OHC Technical Alignment**:
    - The backend requires new Rust crates/modules under `src/server/integrations/chat` and potentially core domain modules in `src/server/common/models`.
    - We will leverage PostgreSQL with RLS (tenant_id) for storage, Redis for real-time pub/sub/presence, and Tonic/gRPC for internal API communication.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|{ CHANNEL : uses
      CHANNEL ||--|{ WEB_WIDGET : is_type
      CHANNEL ||--|{ WHATSAPP : is_type
      CHANNEL ||--|{ EMAIL : is_type

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          uuid channel_id FK
          string channel_type
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string sender_type "contact, agent, bot"
          uuid sender_id
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Inbox Screen**:
    - A clean, translucent list view displaying active conversations grouped by urgency/status.
    - Touch targets ≥ 44x44px.
    - Avatars indicate the channel source (e.g., WhatsApp icon, Web Widget icon).
  - **Conversation Detail Screen**:
    - Scrollable message history with clear visual distinction between customer messages (left aligned, neutral tone) and owner/agent responses (right aligned, primary OHC tint).
    - An AI-assisted input bar at the bottom: "Draft reply...", "Send quote...", "Request payment...".
    - Sticky header with customer name and quick actions (call, view profile).
  - **Settings (Advanced)**:
    - Hidden behind a standard "Settings" gear, allowing configuration of new channels (e.g., connecting a WhatsApp Business account) with simplified, guided flows.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant**: Automatically drafts suggested replies based on conversation context and business knowledge base (RAG).
  - **Work Triage**: Analyzes incoming messages, categorizes intent (e.g., lead, support, complaint), and flags urgent conversations in the owner's primary feed.
  - **Sales & Revenue Assistant**: Detects purchase intent and can automatically inject product cards, quotes, or payment links into the chat interface for the owner to review and send.

  ### Key Design Decisions
  - **Strict Multi-Tenancy**: All database tables (`inboxes`, `conversations`, `messages`, `contacts`) MUST include a `tenant_id` column and enforce RLS at the PostgreSQL level.
  - **Event-Driven Pub/Sub**: Real-time updates (new messages, typing indicators) will be handled via Redis Pub/Sub, bridging to WebSockets/SSE on the Flutter client.
  - **Abstracted Channels**: The core messaging logic is agnostic to the source channel. Specific adapters (WhatsApp, Email) handle the mapping of external webhook payloads into the standard OHC `Message` format.

  ## Implementation Prompt
  **Role**: Implementer Agent
  **Task**: Build the foundational data models, gRPC/REST APIs, and core backend service logic for the Native Rust Omnichannel Chat System in OHC.

  **Critical User Journey (CUJ)**:
  As an owner (e.g., Maya), I want to receive a message from a customer via my website's chat widget and reply to it from my OHC mobile app, so that I can handle inquiries without switching tools.

  **Acceptance Criteria**:
  1. Define the Protocol Buffer (`.proto`) schemas for `Inbox`, `Conversation`, `Message`, and `Contact`, including necessary CRUD operations and list endpoints.
  2. Implement the PostgreSQL database migrations for these entities, ensuring `tenant_id` is present on all tables and RLS is configured.
  3. Create the Rust gRPC service implementation for the Chat service, handling standard CRUD and message creation.
  4. Implement a rudimentary Web Widget channel adapter that can receive incoming messages (e.g., via a REST webhook endpoint) and route them to the correct `Inbox` and `Conversation`.
  5. Ensure comprehensive Unit Tests for all new Rust code.
  6. Ensure the `main` branch tests continue to pass 100% (`bazel test //...`).

  **Notes**:
  - Do not implement the full frontend UI yet; focus on the robust backend foundation.
  - Reference the Chatwoot models but adapt them to OHC's architecture (Rust, Tonic, SQLx, strict multi-tenancy).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
