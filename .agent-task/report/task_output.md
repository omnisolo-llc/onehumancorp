issue_title: "Implement Custom Rust Omnichannel Chat System based on Chatwoot Architecture"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is retiring the external Chatwoot dependency to build a native Rust omnichannel customer support engine. Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. We need to implement the foundational models for this native chat system, reverse-engineering the core of Chatwoot into our Rust/PostgreSQL/Kubernetes stack.

  # Research Report
  **Findings & Competitive Analysis:**
  - Our agent system relies on a unified customer context. Chatwoot provides excellent models for this, which we've audited:
      - `Conversation`, `Message`, `Inbox`, `Channel`, `Contact`, `ContactInbox`.
  - A unified inbox without a unified graph creates a reactive, labor-intensive process.
  - The implementation must support multi-tenant isolation natively using `tenant_id` at the database level.
  - The models should map neatly to our existing `ohc` microservice.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Data Model (Rust)
  The core Rust models should reside in `src/server/ohc/domain/chat/`.
  - **Inbox**: Represents a channel endpoint (e.g. "WhatsApp Support", "Instagram Main").
  - **Contact**: Represents a unified customer across channels.
  - **ContactInbox**: Associates a contact with an inbox to resolve identity.
  - **Conversation**: A thread of messages between a contact and an inbox.
  - **Message**: A single message within a conversation.
  - **ChannelAdapter**: A trait or enum for defining how to dispatch messages (e.g., `ChannelSms`, `ChannelInstagram`).

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Action Flow:** Clicking the card opens a translucent glass pane showing the conversation context, recent purchase history, and an AI-drafted reply.
  - **Approve/Edit:** The user can tap "Send Draft" or edit the text.

  ### AI Agent Integration
  - These models will be queried by "The Ambassador" AI agent to provide historical context for drafting replies.

  ### Key Design Decisions
  - **Row-Level Security (RLS)**: Every entity MUST have a `tenant_id` column to leverage PostgreSQL RLS for multi-tenant isolation.
  - **Zero Trust & Security**: Follow standard SPIFFE/SPIRE patterns if interacting across services.

  # Implementation Prompt
  Implement the core domain models and repository traits for the native Rust omnichannel chat system in `src/server/ohc/domain/chat/`. Use the provided architectural guidelines. Define the structs (`Inbox`, `Contact`, `Conversation`, `Message`) with strong types, ensuring `tenant_id` is present on all entities for multi-tenancy. Create a basic in-memory or PostgreSQL repository trait for these models. Do NOT implement the API layer or frontend UI yet; focus strictly on the core domain layer and ensure unit tests pass.

  # Priority
  P0

  # Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
