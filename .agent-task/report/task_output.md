issue_title: "Architecture Design: Native Rust Omnichannel Chat System (the legacy chat platform Replacement)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native, high-performance, multi-tenant omnichannel customer support and chat engine, having fully retired external dependencies like the legacy chat platform. Our non-technical owner/operators (e.g., Maya the baker, Carlos the handyman) need a unified inbox that aggregates Instagram DMs, WhatsApp, web chat, and email into a single, seamless, and mobile-friendly command center. They need this unified inbox to coordinate seamlessly with their AI Customer and Operations Assistants to automatically draft replies, manage context, and handle routine inquiries without needing technical configuration.

  ## Research Report
  - **the legacy chat platform Audit**: Based on an architectural review of the legacy chat platform's source code, its core strengths lie in its structured data models (`Inbox`, `Conversation`, `Message`, `Contact`, `Channel`), its WebSocket-based real-time messaging, and its clear channel adapter pattern for unifying diverse message sources.
  - **Market Dynamics**: SMBs struggle with fractured communications across multiple apps. A unified inbox is critical for converting leads and retaining customers. Our competitors (e.g., Front, Zendesk) provide these, but they are designed for support teams, not solo owner/operators. OHC's differentiation is AI-first triage and response drafting directly integrated into the core operations engine.

  ## Design Doc
  ### High-Level Architectural Design
  The system will be implemented natively in Rust within the OHC mono-repo, leveraging asynchronous processing and strict multi-tenant isolation.

  #### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_by
      CHANNEL_ADAPTER ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string provider_type
          json config
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone
      }
  ```

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant ExternalProvider as Web/WhatsApp
      participant RustAPI as OHC Rust Ingress
      participant AgentQueue as AI Job Queue
      participant AI_CS_Agent as Customer Assistant
      participant WebSocket as OHC WS Hub
      actor Owner as OHC Mobile App

      Customer->>ExternalProvider: Sends Message
      ExternalProvider->>RustAPI: Webhook Delivery
      RustAPI->>RustAPI: Multi-tenant Auth & Route to Inbox
      RustAPI->>RustAPI: Save Message (Status: Unread)
      RustAPI->>WebSocket: Broadcast New Message Event
      WebSocket->>Owner: Push Notification / UI Update
      RustAPI->>AgentQueue: Enqueue Draft Reply Job
      AgentQueue->>AI_CS_Agent: Process Context
      AI_CS_Agent->>RustAPI: Save Draft Message
      RustAPI->>WebSocket: Broadcast Draft Event
      WebSocket->>Owner: UI displays Draft
      Owner->>RustAPI: Approves & Sends Draft
      RustAPI->>ExternalProvider: Dispatch Message
  ```

  #### Mobile UX Flow (375px First)
  - **Unified Inbox View**: Clean, Apple-style list view of active conversations. Unread messages and AI-drafted replies are highlighted with translucent status tokens.
  - **Conversation View**: Full-height chat interface using native mobile keyboards. AI draft is shown inline with a distinct, muted background, offering one-tap "Send" or "Edit" buttons.
  - **Offline Resilience**: Read paths are cached via local mobile storage. Sent messages are optimistically rendered and queued for background sync if the network is flaky.

  #### AI Agent Integration Points
  - **Work Triage Department**: Hooks into new message events to categorize urgency and update the owner's daily feed.
  - **Customer & Relationship Assistant**: Listens to the `AgentQueue`, reads tenant-scoped conversation memory, and generates draft replies (e.g., answering "do you do vegan cakes?").

  #### Security & Zero Trust
  - **Multi-Tenant Isolation**: Row-level tenant isolation enforced via `tenant_id` on all tables (`ENABLE ROW LEVEL SECURITY` in Postgres). Distributed Redis locks used to coordinate agent processing per conversation.

  ## Implementation Prompt
  **Goal:** Implement the foundational Rust API and data models for the Native Omnichannel Inbox, completely replacing the legacy chat platform dependencies.

  **CUJ:** As an owner (Maya), I receive an Instagram DM. It appears in my OHC Unified Inbox. The AI Customer Assistant drafts a reply, which I can view and send with one tap on my mobile device.

  **Acceptance Criteria:**
  - Create Rust data entities: `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`.
  - Implement PostgreSQL migrations with strict RLS (Row Level Security) by `tenant_id`.
  - Expose REST/gRPC endpoints for webhook ingestion and mobile client synchronization.
  - Implement WebSocket broadcasting for real-time updates.
  - Integrate with the AI Job Queue for asynchronous draft generation.
  - Provide 100% unit test coverage for the new Rust modules.
  - E2E Playwright tests must verify the message ingestion and real-time broadcasting flow.
  - All visual components must strictly adhere to the OHC Premium Token library (translucent materials, clean spacing).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust]
assignees: []
