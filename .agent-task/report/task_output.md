issue_title: "Implement Custom Rust Omnichannel Chat System to replace Chatwoot"
issue_description: |
  **Problem Statement:**
  OneHumanCorp (OHC) currently relies on Chatwoot as an external dependency for its omnichannel customer support and chat capabilities. This introduces unnecessary complexity, limits native multi-tenant isolation within OHC, and adds a point of failure for our non-technical business owners (Maya, Carlos, Priya, Leo, Fatima) who need a simple, unified, and highly available inbox without thinking about external services.

  **Research Report:**
  Based on the source code of Chatwoot (`https://github.com/chatwoot/chatwoot`) and leading competitors (Zendesk, Intercom), a scalable omnichannel platform requires:
  1. A robust data model for Conversations, Messages, Contacts, and Inboxes.
  2. WebSocket-based real-time messaging capabilities.
  3. Pluggable channel adapters (WhatsApp, Instagram, Email, Web Widget).
  4. SLA policies, macros, and AI agent routing.

  By implementing these natively in Rust inside `onehumancorp/mono`, OHC can guarantee row-level security per tenant via PostgreSQL RLS, improve performance, reduce operational overhead, and unify the agent/customer experience seamlessly into the OHC Premium Token design system.

  **Design Doc:**
  - Architecture: Native Rust microservices for core entities (Conversations, Messages). Real-time communication via WebSocket. Background job queue via PostgreSQL `SKIP LOCKED` for processing incoming webhooks (e.g., from Meta/WhatsApp).

  **Architecture Diagram:**
  ```mermaid
  erDiagram
    TENANT ||--o{ INBOX : has
    INBOX ||--o{ CONVERSATION : contains
    CONVERSATION ||--o{ MESSAGE : contains
    CONVERSATION }o--|| CONTACT : involves
    INBOX {
      uuid id
      string name
      string channel_type
    }
    CONVERSATION {
      uuid id
      string status
      uuid contact_id
    }
    MESSAGE {
      uuid id
      string content
      string sender_type
    }
    CONTACT {
      uuid id
      string name
      string email
      string phone
    }
  ```

  **Mobile UX Flow:**
  The "Inbox" tab aggregates all channels into a single list. Swiping a conversation reveals actions (Assign to Agent, Mark Done). Tapping opens a unified chat interface with native mobile keyboard support and quick replies.

  **AI Integration:**
  AI agents intercept incoming messages to draft replies or take automated actions based on tenant-scoped memory before escalating to the owner.

  **Implementation Prompt:**
  Implement the core Rust data models and gRPC/REST APIs for the native omnichannel chat system. This includes schemas for `Inbox`, `Conversation`, `Message`, and `Contact`. Ensure full multi-tenant isolation using PostgreSQL RLS. Implement basic WebSocket real-time event broadcasting for new messages. Follow the macOS Translucent Glass styling and UniFi modular dashboard layout for any related admin UI components. Ensure all endpoints are fully unit tested and at least 5 E2E Playwright tests cover the core "Receive message and reply" flow.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
