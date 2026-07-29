issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  OneHumanCorp (OHC) is retiring Chatwoot as an external dependency. We need a native Rust omnichannel customer support and chat engine to replace it, integrated directly into our monorepo. This allows us to have full control over performance, multi-tenant isolation, and data ownership, ensuring a seamless experience for non-technical business owners like Maya (baker) and Carlos (handyman) who need to handle customer inquiries directly from their unified OHC workspace.

  **Research Report**
  I have audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to understand its architecture. Key findings:
  - Chatwoot relies on Ruby on Rails models like `Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel`, etc.
  - It uses WebSockets for real-time messaging and background jobs for routing, notifications, and webhooks.
  - It supports multiple channel types (Email, API, Web Widget, etc.) through adapters.
  - OHC needs to replicate this functionality in Rust, ensuring strict multi-tenant isolation using our `tenant_id` pattern and Row-Level Security (RLS) in PostgreSQL.
  - This custom chat system will act as a core component of the "Work Triage" and "Customer & Relationship Assistant" capabilities.

  **Design Doc**
  - **Architecture Diagram (Mental Model / ER Outline):**
    - `Tenant` (1:N) `Inbox`
    - `Inbox` (1:N) `Conversation`
    - `Conversation` (1:N) `Message`
    - `Contact` (1:N) `Conversation`
    - `ChannelAdapter` interfaces for different message sources.
    - Real-time updates via WebSockets and Redis Pub/Sub.
  - **Mobile UX Flow (375px):**
    - The chat interface must be built into the OHC Flutter app, optimized for a 375px width.
    - Unified inbox view showing all customer conversations across channels.
    - Clear distinction between human messages and AI agent drafts.
    - Quick actions for creating quotes, booking appointments, or accepting payments directly from the chat context.
  - **AI Agent Integration:**
    - AI agents will listen to incoming messages, analyze intent, and draft suggested replies (or auto-reply based on owner settings).
    - AI will also extract context (e.g., requested dates, item preferences) to populate task or booking cards.
  - **Key Design Decisions:**
    - Use Rust for the backend chat microservice/crate for high performance and concurrency.
    - Enforce tenant isolation at the database layer (PostgreSQL RLS).
    - Use gRPC for internal service communication and REST/WebSockets for external clients.

  **Implementation Prompt**
  Implement a native Rust omnichannel chat system for OHC to replace the external Chatwoot dependency. The system must support core entities like Inboxes, Conversations, Messages, and Contacts, with strict multi-tenant isolation. Build the necessary API endpoints for the Flutter client to fetch and send messages. Ensure the architecture supports real-time updates and seamless integration with AI assistant agents for automated replies and context extraction. The UI must be fully functional and premium-looking on a 375px mobile screen. All new code must have 100% test coverage and pass existing E2E tests.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
