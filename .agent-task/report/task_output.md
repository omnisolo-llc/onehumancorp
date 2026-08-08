issue_title: "[Architecture] Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OneHumanCorp currently relies on a retired third-party service (Chatwoot) for omnichannel customer support, which violates our core architecture mandate. Relying on an external provider creates latency, data privacy concerns (multi-tenancy boundaries), and fragments the owner experience. Our owner personas (e.g., Maya the Baker, Carlos the Handyman) need a unified inbox where Instagram DMs, SMS, WhatsApp, and Web Chat messages appear in a single, lightning-fast native interface, seamlessly orchestrated by AI agents (e.g., auto-drafting replies). We must build a high-performance, native Rust chat system internally to achieve 100% feature parity with Chatwoot while maintaining strict Row-Level Security (RLS) multi-tenancy.

  ## Research Report
  - **Codebase Audit:** OHC's backend is Rust/Go-based, using SeaORM/SQLx. We need native Rust data models to handle unified inboxes, channel adapters, contacts, conversations, and real-time messaging via WebSockets.
  - **Chatwoot Source Code Benchmarking:** Audited `chatwoot/chatwoot` source. Key entities identified: `Inbox`, `Channel::*` (WebWidget, SMS, WhatsApp, Instagram, FacebookPage, Email), `Conversation`, `Message`, `Contact`, `AgentBot`, and `AutomationRule`.
  - **Competitor Systems (Zendesk, Shopify Inbox):** Emphasize minimal latency, offline tolerance for mobile, and AI-assisted drafting natively embedded in the operator UI.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--|{ MESSAGE : contains
      CONVERSATION }|--|| CONTACT : involves
      TENANT ||--o{ CONTACT : manages
      TENANT ||--o{ AGENT_BOT : configures
      AGENT_BOT ||--o{ MESSAGE : drafts
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View:** A single list of conversations, sorted by recent activity. Avatars show the channel icon (e.g., IG, SMS) overlaid.
  2. **Conversation Thread:** Clean, iMessage-like UI. System messages (e.g., "AI Drafted Reply") clearly distinguished.
  3. **Reply Box:** Input area with one-tap "Approve AI Draft" button, native keyboard support, and attachment icon.

  ### AI Agent Integration Points
  - **Work Triage:** AI analyzes incoming messages to tag intent (e.g., "Lead", "Support", "Complaint") and assigns priority.
  - **Customer Assistant:** Automatically generates draft replies (`Message` entity with `is_draft: true` and `agent_id`) upon new customer message insertion.

  ### Key Design Decisions
  - **Multi-Tenancy:** Strict enforced `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, `contacts`) with PostgreSQL RLS.
  - **Real-time:** Use `tokio-tungstenite` and Redis pub/sub for low-latency WebSocket message delivery to connected clients.
  - **Channel Extensibility:** Implement a Rust trait `ChannelAdapter` to standardize webhook processing across different providers (Meta, Twilio, etc.).

  ## Implementation Prompt
  **To the Implementer:**
  Build the native Rust data models (SeaORM/SQLx) and core gRPC/REST APIs for the OHC Unified Omnichannel Inbox, replacing the deprecated Chatwoot integration.
  1. Implement the database schema migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring strict `tenant_id` isolation.
  2. Create the core CRUD service layer for these entities.
  3. Implement the WebSocket infrastructure for real-time message broadcasting to the frontend.
  4. Ensure the system can natively accept webhooks from at least one initial channel (e.g., Web Widget or SMS) and route them to the correct Tenant/Inbox/Conversation.
  5. Provide a pristine, UniFi-style mobile-first Flutter + PWA frontend component for the Unified Inbox view.
  **Acceptance Criteria:** A user can send a message via a simulated channel webhook, see it appear in the native Unified Inbox UI in real-time, and reply, with all data securely scoped to their tenant.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
