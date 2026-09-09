issue_title: "Native Omnichannel Customer Inbox Architecture"
issue_description: |
  # Title: Native Omnichannel Customer Inbox Architecture

  ## Problem Statement
  For business owners like Maya (baker) and Carlos (handyman), keeping up with customer messages across Instagram DMs, WhatsApp, SMS, and website chat is overwhelming. They currently have to switch between multiple apps, manually link messages to orders, and remember conversation context. There is a need for a unified, real-time native inbox within OneHumanCorp that consolidates all customer communications into one seamless stream, enabling AI agents to draft replies, track SLA, and suggest actions without the owner needing to manage multiple tools.

  ## Research Report
  - **Market Context**: Products like Front, Zendesk, Intercom, and HubSpot provide omnichannel inboxes, but they are often too complex or enterprise-focused for a small business operator.
  - **Competitor Benchmarking**: Shopify Inbox and Wix Chat offer lightweight native experiences but limit channels mostly to their own ecosystems. High-growth tools (e.g., ManyChat, respond.io) focus heavily on WhatsApp and Instagram but lack deep operational coupling (orders, appointments, billing).
  - **Findings**: Operators require an inbox that not only aggregates messages but actively ties them to business outcomes (e.g., turning an Instagram DM into a quote or appointment booking instantly). Real-time capabilities (typing indicators, read receipts, WebSocket/SSE) are expected as baseline functionality to maintain a modern messaging experience.
  - **Technical Gap**: The current OHC system has disparate integration points (e.g., `whatsapp_cloud`, `meta`) but lacks a unified real-time messaging substrate.

  ## Design Doc
  - **Architecture Overview**:
    - **Unified Message Bus**: A multi-tenant Native Rust WebSocket / Server-Sent Events (SSE) gateway for real-time client delivery.
    - **Channel Adapters**: Native adapters for Instagram DM, WhatsApp, SMS, and Web Chat that normalize incoming payloads into a standard OHC `Conversation` and `Message` model.
    - **AI Routing & Drafting**: Background processors attached to the unified inbox that automatically assign labels, match against customer profiles, and draft AI responses.

  - **Mermaid Architecture Diagram**:
    ```mermaid
    graph TD
      subgraph External Channels
        IG[Instagram DM] --> Adapters
        WA[WhatsApp] --> Adapters
        SMS[SMS/Twilio] --> Adapters
        Web[Web Widget] --> Adapters
      end

      subgraph OHC Backend
        Adapters[Channel Adapters] --> Router[Unified Routing Engine]
        Router --> Core[Inbox Controller]
        Core --> DB[(PostgreSQL)]
        Core --> AI[AI Draft & Context Service]
      end

      subgraph Real-Time Transport
        Core -.-> WS[WebSocket / SSE Gateway]
      end

      subgraph Clients
        WS --> Mobile[Mobile App 375px]
        WS --> Desktop[Desktop PWA]
      end
    ```

  - **Mobile UX Flow (375px First)**:
    1. **Unified Feed**: Owner opens the app and sees a single unified inbox list. Each thread displays the channel icon (e.g., WhatsApp, IG) and an unread badge.
    2. **Conversation View**: Tapping a thread opens a chat interface optimized for 375px, showing message bubbles, typing indicators, and real-time incoming messages.
    3. **Context Pane**: A swipeable drawer or floating action button brings up the customer profile (past orders, notes, active quotes) natively linked to the conversation.
    4. **AI Assistance**: An inline "Draft Reply" button instantly generates a suggested message based on conversation context and business state, allowing the owner to approve or edit before sending.

  - **Key Design Decisions**:
    - **Native Rust**: High-performance Rust WebSockets/SSE to minimize resource footprint and maximize concurrency, especially in standalone environments.
    - **Standardized Protobufs**: All internal communication between adapters and the routing engine uses a unified Protobuf schema for message consistency.
    - **Multi-Tenant Isolation**: Row-level security strictly enforces that conversations are isolated per tenant ID on all backend layers.

  ## Implementation Prompt
  - **Objective**: Implement the unified real-time omnichannel inbox infrastructure in Rust, integrating with the Next.js/Flutter frontend.
  - **User Journey**: As an owner, I receive a WhatsApp message and an Instagram DM. They both appear in my unified OHC inbox in real-time. I can read, get an AI draft, and reply directly from the OHC mobile view without switching apps.
  - **Acceptance Criteria**:
    - Real-time WebSocket/SSE gateway is operational.
    - Normalization layer successfully routes WhatsApp and Instagram DMs into unified conversation threads.
    - Mobile UI properly displays real-time updates and AI draft suggestions.
    - Multi-tenant isolation verified via backend tests.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
