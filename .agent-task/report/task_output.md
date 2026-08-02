issue_title: "Native Rust Omnichannel Chat Integration - Chatwoot Migration"
issue_description: |
  # Native Rust Omnichannel Chat Integration - Chatwoot Migration

  ## Problem Statement
  OHC requires a fully integrated omnichannel customer support and chat engine. Currently, the system does not have this capability, and the directive requires replacing any external reliance on Chatwoot with a native, high-performance Rust implementation built directly into OHC. Small business owners need to communicate with customers across multiple channels (Web Chat, WhatsApp, Email, Instagram) from a single interface without configuring or paying for external third-party services.

  ## Research Report
  - **Tool Evaluated**: Chatwoot (Open Source Version - `https://github.com/chatwoot/chatwoot`)
  - **Target User**: Small business owners (like Maya, Carlos, Priya) who need a unified inbox for all customer communications.
  - **Key Capabilities Discovered**:
    - Inbox and Channel management (Web, API, Social Media).
    - Conversation threading and messaging.
    - Agent assignment and routing.
    - Webhooks and WebSocket events for real-time updates.
    - Canned responses and macros.
  - **OHC Strategy**: We will not integrate with Chatwoot as a third-party service. Instead, we will build a native Rust microservice/crate inside the OHC monorepo that replicates these core capabilities, tailored specifically for our multi-tenant SaaS architecture. This ensures tight integration with our existing PostgreSQL (Row-Level Security), Redis (Redlock), and authentication systems.

  ## Design Doc
  - **Core Component**: A new Rust service `ohc-chat-engine` responsible for managing conversations, messages, and channels.
  - **Data Model**:
    - `Channels`: Represents the source of messages (e.g., Web Widget, WhatsApp).
    - `Conversations`: Groups messages between a customer and the business.
    - `Messages`: Individual chat messages.
  - **Real-time Updates**: Implement WebSockets for real-time message delivery to the OHC Flutter frontend.
  - **Frontend Integration**: The OHC Flutter app will include an "Inbox" view that connects to this new Rust service, displaying a unified feed of messages.
  - **Tenant Isolation**: Ensure all database queries and WebSocket channels are scoped by `tenant_id` to maintain strict multi-tenant isolation.

  ## Implementation Prompt
  Create a native Rust chat engine that replicates the core functionality of Chatwoot.
  1. Define the database schema (PostgreSQL) for channels, conversations, and messages, ensuring row-level security by `tenant_id`.
  2. Implement a Rust gRPC/REST API for creating and retrieving conversations and messages.
  3. Implement a WebSocket server in Rust for real-time message broadcasting to connected clients.
  4. Develop the corresponding Flutter UI components for a unified inbox view, allowing owners to view and reply to messages.
  5. Ensure the new chat system is seamlessly integrated into the OHC Assistant-First Shell.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
