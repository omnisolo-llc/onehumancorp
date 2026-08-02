issue_title: "Native Rust Omnichannel Chat: Chatwoot Replacement Architecture"
issue_description: |
  # Native Rust Omnichannel Chat Architecture

  ## Problem Statement
  OHC relies on Chatwoot as an external third-party service for omnichannel customer support (Instagram, WhatsApp, Email, Web Widget). As per our engineering mandate, Chatwoot must be retired and replaced with a native Rust implementation embedded within the OHC mono-repo, providing an owner-focused unified inbox experience out-of-the-box. This reduces external dependencies, enforces strict tenant isolation (`tenant_id` RLS), and deeply integrates chat with OHC's AI work agents (Operations, Sales, Customer Service). The personas (Maya, Carlos, Priya) rely heavily on DMs and require zero configuration to start using these channels.

  ## Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core entities that map to our system are:
  - **Account -> Tenant**: Represents the OHC tenant (the owner's business).
  - **Inbox**: A container for a specific channel integration (e.g., "Maya's Instagram", "Support Email").
  - **Channel**: Specific integrations (Instagram, Web Widget, Email, WhatsApp).
  - **Conversation**: A thread between a Contact and the Inbox.
  - **Message**: Individual payloads (Text, Attachments, interactive elements).
  - **Contact**: The customer or lead on the other end.

  **Competitive & Architectural Analysis**:
  - **Chatwoot's Approach**: Uses Ruby on Rails with PostgreSQL and Redis for pub/sub WebSocket delivery. Channels are modeled as polymorphic associations.
  - **OHC Native Rust Approach**: We will leverage our existing Axum/gRPC stack and PostgreSQL RLS. Channels will be mapped to specific Rust structs/traits (`ChannelAdapter`), handling webhooks and pushing normalized events to a central unified event bus. Redis will be used for distributed locking (Redlock) and WebSocket pub/sub across our distributed backend.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
    subgraph External
      IG[Instagram Graph API]
      WA[WhatsApp API]
      Web[Web Widget]
      Mail[Email Webhook]
    end

    subgraph "OHC Native Rust Backend"
      WH[Webhook Gateway]
      CA_IG[Instagram Adapter]
      CA_WA[WhatsApp Adapter]
      CA_Web[Web Adapter]
      CA_Mail[Email Adapter]

      Core[Omnichannel Core Service]
      Bus[Redis Pub/Sub & Job Queue]

      Agents[AI Assistants]
      WS[WebSocket Manager]
    end

    subgraph "Data Layer (PostgreSQL with RLS)"
      DB[(Tenant DB: Inboxes, Contacts, Conversations, Messages)]
    end

    subgraph "Frontend"
      App[OHC Flutter/PWA Shell]
    end

    IG --> WH
    WA --> WH
    Mail --> WH
    Web <--> WS

    WH --> CA_IG
    WH --> CA_WA
    WH --> CA_Mail

    CA_IG --> Core
    CA_WA --> Core
    CA_Web --> Core
    CA_Mail --> Core

    Core --> DB
    Core --> Bus

    Bus --> WS
    Bus --> Agents
    Agents --> Core

    WS <--> App
  ```

  ### Mobile UX Flow (375px)
  1. **Work Feed (Home)**: The owner sees unread DMs grouped by urgency on the main dashboard.
  2. **Unified Inbox View**: Tapping a message opens a unified chat view. The UI clearly indicates the channel source (e.g., Instagram icon) but provides a consistent native compose experience.
  3. **AI Drafts**: The AI pre-drafts responses based on knowledge/context. A floating "Send Draft" button appears above the keyboard.
  4. **Action Integration**: From the chat, a bottom sheet allows quick creation of Quotes, Tasks, or Bookings linked to the Conversation and Contact.

  ### AI Agent Integration Points
  - **Triage Agent**: Listens to `ConversationCreated` events to assess urgency and categorize intent.
  - **Customer Assistant**: Listens to `MessageCreated` events to generate contextual drafts (stored as pending messages or suggestions in the UI).
  - **Operations Assistant**: Scans messages for intent to book or purchase, proposing actionable next steps directly in the conversation flow.

  ### Key Design Decisions
  - **Strict Tenant Isolation**: All entities (Inbox, Contact, Conversation, Message) will include `tenant_id` and be protected by Postgres Row Level Security.
  - **Channel Adapters**: A unified `ChannelAdapter` Rust trait will handle normalizing incoming webhooks and formatting outgoing API calls to specific providers, decoupling core logic from vendor quirks.
  - **Event-Driven Delivery**: Messages are saved to Postgres, then broadcast via Redis pub/sub to any connected WebSocket clients for real-time delivery.

  ## Implementation Prompt
  **User Facing Outcome**: As Maya (a baker), when a customer DMs my business Instagram, I want it to instantly appear in my OHC work feed so I can reply and send a deposit link without leaving the OHC mobile app. The UI should show me the customer's past order history alongside the chat.

  **Critical User Journey (CUJ)**:
  1. System receives a simulated webhook payload from a channel (e.g., an Instagram DM).
  2. The webhook is processed, creating/updating the Contact, Conversation, and Message records under the correct `tenant_id`.
  3. The real-time event is broadcasted.
  4. The frontend (Unified Inbox) updates to display the new message.
  5. The owner sends a reply via the UI, which routes back through the backend to the simulated external channel adapter.

  **Acceptance Criteria**:
  - Implement the core database schemas (Inboxes, Contacts, Conversations, Messages, Channel configurations) with `tenant_id` RLS.
  - Create the Rust services/handlers to manage conversations and messages.
  - Implement a basic Web Widget channel adapter as the first proof-of-concept.
  - Implement WebSocket endpoints for real-time sync with the frontend.
  - Build the 375px mobile-first Unified Inbox UI components (Flutter/Next).
  - Write full end-to-end Playwright tests proving a message can be received, displayed, and replied to.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
