issue_title: "Native Rust Omnichannel Chat System Architecture (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently relies on external systems like Chatwoot for omnichannel customer support. Chatwoot as an external service is 100% RETIRED to guarantee strict row-level multi-tenant isolation, Zero Trust security (SPIFFE/SPIRE), and seamless AI agent coordination within OHC's boundaries. A native Rust omnichannel chat system must be built inside `onehumancorp/mono` to replace it. This capability is critical for owner/operators like Maya and Carlos, who need an integrated view of Instagram DMs, SMS, and WhatsApp messages, intertwined with booking and payment capabilities, all handled by AI agents behind the scenes.

  ## Research Report
  - **Chatwoot Source Code Audit**: Chatwoot’s core architecture uses an Inbox model where multiple Channels (Email, Widget, Facebook, Twitter, WhatsApp, SMS, API) route messages to Conversations. Conversations belong to Contacts and are handled by Users (Agents). It leverages WebSockets (ActionCable) for real-time messaging, webhooks for external integrations, and a sophisticated macro/canned response system.
  - **Competitor Systems**: Shopify Inbox and Wix Chat integrate tightly with their commerce engines, allowing operators to send product links, quotes, and payment requests directly within the chat. By building our own system in Rust, we can replicate this native commerce integration while vastly outperforming Ruby on Rails in concurrent connection handling for real-time WebSockets.
  - **Identified Gaps in OHC**: Lack of native Rust-based omnichannel data models (Inboxes, Conversations, Messages, Channels) and real-time WebSocket orchestration in the OHC mono-repo.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o{ CHANNEL : configured_with
      CHANNEL {
          string type "WhatsApp, SMS, Widget, IG"
          json credentials
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid contact_id
          uuid inbox_id
          string status "open, resolved, pending"
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          string content
          string message_type "incoming, outgoing, system"
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage Dashboard**: The operator sees a unified feed of active conversations prioritizing unread messages and AI-flagged urgent inquiries (e.g., "Maya, a customer is asking about a vegan cake for tomorrow").
  2. **Conversation View**: Tapping an item opens a chat view natively built for mobile. Messages from all channels look consistent. AI-drafted replies are shown as a translucent "Draft" card at the bottom.
  3. **Commerce Integration**: An action button allows the operator to instantly inject a product catalog card, payment link, or booking calendar slot directly into the chat stream.

  ### AI Agent Integration
  - **Customer & Relationship Assistant**: Listens to the Rust WebSocket/Event bus for new incoming `MESSAGE` entities. Automatically retrieves customer history and drafts a reply, saving it with `message_type: draft`.
  - **Operations Assistant**: Intercepts intents (e.g., booking requests) and highlights calendar availability within the AI summary.
  - **Handoff Protocol**: When the AI cannot resolve the request confidently, it tags the conversation as `status: requires_human` and sends a silent push notification to the owner.

  ### Key Design Decisions
  - **Native Rust WebSocket Server**: High-throughput asynchronous WebSocket handlers using Tokio/Axum to stream real-time updates to the Flutter PWA/Mobile clients.
  - **Multi-Tenant Isolation**: Every table (`inboxes`, `conversations`, `messages`, `channels`) must include `tenant_id` enforced by PostgreSQL Row-Level Security (RLS).
  - **Zero Trust Security**: Internal microservice communication for webhooks (e.g., receiving WhatsApp payload) will be secured via SPIFFE/SPIRE identity enforcement.

  ## Implementation Prompt
  **Implementer Agent Task:**
  Design and implement the native Rust data models, PostgreSQL migrations, and core gRPC/REST APIs for the Omnichannel Chat System.
  1. Implement `Inbox`, `Conversation`, `Message`, and `Channel` entities in Rust with strict RLS on `tenant_id`.
  2. Expose secure endpoints to create conversations, send messages, and fetch conversation history.
  3. Set up the WebSocket signaling foundation (using Axum/Tokio) for real-time `message.created` events.
  4. Ensure zero mock data in tests and comprehensive unit testing.

  **Acceptance Criteria:**
  - Database migrations for the core chat schema are created.
  - Rust API endpoints for CRUD operations on conversations and messages pass all unit tests.
  - Tenant isolation is verifiably enforced.
  - No external Chatwoot dependency is used.

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
