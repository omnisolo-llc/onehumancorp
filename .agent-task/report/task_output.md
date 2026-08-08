issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) currently relies on Chatwoot as an external service and dependency for its omnichannel customer support and chat functionality. The mandate requires the **complete retirement** of Chatwoot as an external dependency. OHC must implement its own high-performance, multi-tenant omnichannel customer support and chat engine natively in Rust inside the `onehumancorp/mono` repository to achieve full data isolation, better integration with the OHC platform, and zero external dependencies for core chat functionality.

  ## Research Report
  - **Source Code Benchmarking:** We performed an exhaustive audit of the `chatwoot` source code (`https://github.com/chatwoot/chatwoot`).
  - **Identified Core Features:**
    - Omnichannel Inbox Architecture (Web widget, Email, Social integrations).
    - Multi-tenant data models with rigid RLS.
    - WebSocket real-time messaging layer with ActionCable equivalents.
    - Automation: Macros, SLA policies, Agent routing, Canned responses.
  - **Competitor Systems:** Shopify Inbox and Stripe support tools handle messages natively within their admin boundaries, eliminating third-party friction and allowing direct context injection (e.g., cart data into the chat thread).
  - **Missing Capability in OHC:** A native, multi-tenant chat layer tightly coupled with OHC's `Customer Context` (e.g., `CustomerMemoryGraph`), allowing OHC's AI agents to transparently read, draft, and respond without syncing to an external vendor.

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD;
      Client[Mobile / Web Client] -->|WebSocket / REST| OHC_Gateway[API Gateway]
      OHC_Gateway --> Rust_Chat_Service[Rust Chat Microservice]
      Rust_Chat_Service -->|Pub/Sub| Redis[Redis / ActionCable Equivalent]
      Rust_Chat_Service -->|CRUD| PostgreSQL[PostgreSQL Database]

      subgraph PostgreSQL
        Conversations
        Messages
        Inboxes
        ChannelAdapters
      end

      Rust_Chat_Service --> OHC_Agent_Layer[AI Agents (Drafting/Routing)]
    ```
  - **Database Schema (Multi-Tenant):**
    - Tables: `omni_inboxes`, `omni_conversations`, `omni_messages`, `omni_contacts`, `omni_channel_adapters`.
    - Strict `tenant_id` on every table with PostgreSQL Row-Level Security (RLS) enabled.
  - **Mobile UX Flow (375px Target):**
    1. Owner opens OHC app to the "Inbox" tab.
    2. Conversations are listed with prominent notification dots.
    3. Tapping a thread opens a Chat interface.
    4. The UI seamlessly displays real-time updates and embedded "Draft Replies" from the AI assistant.
    5. Action buttons (e.g., "Approve Draft", "Send Checkout Link") are accessible without horizontal scrolling.
  - **AI Agent Integration Points:**
    - AI reads the `omni_messages` stream as context.
    - AI writes `draft_reply` payloads to `omni_messages` or creates separate Draft records linked to the Conversation.
  - **Key Design Decisions:**
    - Build in Rust for safety and concurrency (WebSocket performance).
    - Use Redis Pub/Sub for cross-node real-time message broadcasting.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend core of the Native Rust Omnichannel Chat System based on the detailed data entities (Inboxes, Conversations, Messages, Contacts, Channel Adapters).
  - Ensure every table has strict `tenant_id` Row-Level Security (RLS).
  - Build the Rust WebSocket controllers required for real-time messaging.
  - Expose REST/gRPC APIs for the Flutter/Next.js frontends to interact with.
  - Verify that the chat system integrates natively with OHC's `tenant` architecture without relying on any external Chatwoot APIs.
  - Write comprehensive E2E tests validating the end-to-end journey of receiving a message and broadcasting it to connected WebSocket clients for that tenant.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
