issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance, native Rust omnichannel chat system to support non-technical business owners like Maya, Carlos, Priya, Leo, and Fatima. The goal is to provide a seamless messaging experience directly within the OHC platform, unifying DMs, SMS, and web chat into a single, cohesive interface without relying on external third-party services. This allows our users to easily interact with their customers, schedule appointments, send quotes, and more, all from a mobile-first, intuitively designed application.

  ## Research Report
  - **Market Context**: Platforms like Shopify, Wix, and Squarespace have integrated chat and inbox capabilities that heavily drive merchant adoption. Standalone tools like Chatwoot provide robust omnichannel features (webhooks, channels, SLA, macros) but add external dependencies and integration overhead.
  - **Codebase Context**: OHC requires a tight integration of messaging with core business operations (bookings, payments, AI agent capabilities).
  - **Chatwoot Source Code Audit**: We have audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture:
    - **Models**: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel` (WebWidget, API, Email, SMS).
    - **Real-time**: ActionCable (WebSockets) for pushing updates to clients.
    - **Routing/Agents**: Round-robin assignment, team management, SLA policies.
    - **Automation**: Macros, Canned Responses, Webhooks.
  - **OHC Implementation Target**: We must build a matching native Rust microservice architecture within `onehumancorp/mono` that replicates these core capabilities, tailored for our specific user personas and integrated with our existing PostgreSQL/Redis backend and Flutter frontend.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      Client[Mobile/Web Client] --> API_Gateway[API Gateway]
      Client --> WebSocket[Real-time WebSocket]
      API_Gateway --> Chat_Service[Rust Chat Service]
      WebSocket --> Chat_Service
      Chat_Service --> DB[(PostgreSQL: Conversations, Messages)]
      Chat_Service --> Redis[(Redis: Pub/Sub, Cache)]
      Chat_Service --> Agent_Harness[AI Agent Harness]
      Chat_Service --> Channels[Channel Adapters: SMS, Web, IG]
  ```

  ### Core Data Models (PostgreSQL with RLS)
  - **Tenant (`tenant_id`)**: Base isolation boundary.
  - **Inbox**: Logical grouping of channels (e.g., "Customer Support", "Sales").
  - **Channel**: Integration type (Web, SMS, Instagram).
  - **Contact**: External customer identity.
  - **Conversation**: Thread of messages between Contact and Agent (Human or AI).
  - **Message**: Individual message payload, attachments, metadata.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View**: Clean, scrollable list of active conversations, clearly indicating unread status and source channel.
  - **Conversation Thread**: Familiar chat interface (like iMessage/WhatsApp). Large touch targets (44x44px). Sticky input bar with attachment options (photos for Maya's cakes, PDFs for Nora's proposals).
  - **AI Assistant Integration**: "Draft Reply" button powered by the AI Agent Harness, contextually aware of the conversation history.

  ### Key Design Decisions
  - **Native Rust**: Ensures high performance, low latency, and memory safety for real-time messaging.
  - **Multi-Tenant Isolation**: Row-Level Security (RLS) in PostgreSQL ensures strict separation between OHC users (Maya vs. Carlos).
  - **WebSocket Real-time**: Essential for live chat experiences; fallback to HTTP polling if needed.

  ## Implementation Prompt
  Implement the core native Rust Omnichannel Chat microservice matching the Chatwoot feature set.
  - **CUJ**: As a business owner (e.g., Maya), I want to view all incoming messages (Web, IG, SMS) in a single unified inbox, reply to customers directly, and leverage my AI assistant to draft responses.
  - **Acceptance Criteria**:
    - Build Rust models and database migrations for Inboxes, Conversations, Contacts, and Messages with RLS.
    - Implement REST API endpoints for fetching inboxes, conversations, and sending messages.
    - Implement a WebSocket server in Rust for real-time message delivery to connected clients.
    - Integrate the chat service with the existing AI Agent Harness to allow AI-drafted replies.
    - Ensure 100% unit test coverage and at least one E2E Playwright test simulating a customer message flow.
    - Provide a robust error handling and observability strategy (OpenTelemetry tracing).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
