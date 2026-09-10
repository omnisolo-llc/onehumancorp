issue_title: "Architecture: Native Rust Omnichannel Customer Support Engine"
issue_description: |
  # Problem Statement
  Small business owners and operators (Maya the baker, Carlos the handyman) communicate with customers across fragmented channels—Instagram DMs, WhatsApp, SMS, and Email. Managing these manually leads to missed messages, slow response times, and lost sales. Current platform "unified inboxes" merely aggregate messages without deep context or require manual responses. OHC needs a centralized, high-performance, native Rust omnichannel chat engine to unify these interactions into a single, multi-tenant inbox, powered by agentic automation.

  # Research Report
  - **Shopify Inbox & Wix Inbox**: They aggregate basic chat but lack deep AI auto-drafting and native channel adapters for all platforms out of the box without complex plugins.
  - **Zendesk/Intercom**: Enterprise-grade and far too complex/expensive for a single-person operator on a 375px mobile screen.
  - **Proposed Solution**: A high-performance Rust WebSocket/SSE real-time messaging engine, utilizing popular open-source crates (like `tokio`, `axum`, `tungstenite`, and `sqlx`). This native engine will maintain multi-tenant conversation routing, unified customer profiles, and pluggable adapters for various channels.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: Insta/WA/SMS/Email] -->|Webhooks/APIs| B[Rust Channel Adapters]
      B --> C[Omnichannel Ingress Controller]
      C --> D{Multi-Tenant Router & Auth}
      D --> E[(PostgreSQL: RLS Enforced)]
      D --> F[Rust WebSocket/SSE Real-Time Engine]
      F --> G[OHC Mobile App 375px & Desktop]
      H[AI Ambassador Agent] -->|Context & Drafts| D
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Inbox Feed**: Tappable list of conversations ordered by SLA and priority.
  - **Conversation View**: Clean, iMessage-like UI with typing indicators, read receipts, and agent-drafted replies ready for 1-tap approval.
  - **Context Panel**: Collapsible panel showing customer purchase history and tags.

  ### AI Agent Integration Points
  - The real-time engine pipes incoming messages to the Event Mesh.
  - The **Customer Success Agent (The Ambassador)** reads the message, fetches tenant-scoped customer history, and posts an auto-drafted reply to the conversation object, awaiting owner approval.

  ### Key Design Decisions
  - **Native Rust Execution**: High performance and low memory footprint for edge compatibility.
  - **Multi-Tenant Isolation**: Enforced via `tenant_id` at the database level with PostgreSQL RLS.
  - **Open Source Crates**: Use `axum` for HTTP/routing, `sqlx` for async DB operations, and `tokio-tungstenite` for WebSockets.

  # Implementation Prompt
  **User-Facing Outcome**: As an owner, I open OHC on my phone to see a single inbox combining WhatsApp, Instagram, and SMS. AI has already drafted contextual replies for new inquiries.
  **CUJ & Acceptance Criteria**:
  1. Implement the Rust WebSocket/SSE server using `axum`.
  2. Implement the unified data models (Conversation, Message, ChannelAdapter) with strict `tenant_id` multi-tenancy.
  3. Create standard REST/Webhook ingress routes for external channels.
  4. Integrate real-time typing indicators and read receipts broadcasting.
  5. Provide Playwright E2E tests for the frontend connecting to this WebSocket and exchanging messages.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
