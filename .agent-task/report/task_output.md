issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OHC currently lacks a native, high-performance omnichannel chat system. The goal is to fully retire Chatwoot as an external dependency and build a matching native Rust microservices and UI architecture to achieve 100% feature parity. SMB owners (like Maya the baker and Carlos the handyman) need a unified inbox that brings together WhatsApp, SMS, Instagram DMs, and Emails. Relying on an external service creates data silos, breaks the tenant isolation guarantees (SPIFFE/SPIRE), and increases latency for our AI Agents.

  ## Research Report
  - **Chatwoot Source Code Audit**: Investigated the core data models of Chatwoot (`Inbox`, `Conversation`, `Message`, `Contact`, `ContactInbox`). Chatwoot uses polymorphic associations and external source IDs extensively to map different channels (e.g. `facebook_page`, `twitter_profile`, `twilio_sms`, `whatsapp`, `email`) into a unified `Conversation` and `Message` model.
  - **Data Models Discovered**:
    - `Inbox`: Configuration for a specific channel (e.g., an Instagram account).
    - `Contact`: The unified customer profile.
    - `ContactInbox`: Links a Contact to a specific Inbox and stores the external `source_id` (e.g., Instagram user ID).
    - `Conversation`: The thread linking a `ContactInbox` and an `Inbox`.
    - `Message`: The individual chat message, containing `external_source_ids` and `message_type` (incoming/outgoing).
  - **Competitor Systems Audit**: Systems like Shopify Inbox and Wix Inbox provide unified views, but OHC's differentiation requires deep AI integration where agents proactively draft responses based on omnichannel history.
  - **Identified Gap**: OHC needs these core capabilities implemented natively in Rust with strict multi-tenant isolation, backed by PostgreSQL, to enable the Customer Success Agent (The Ambassador) to function seamlessly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o{ CONTACT_INBOX : links
      CONTACT ||--o{ CONTACT_INBOX : links
      CONTACT_INBOX ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : contains

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string channel_type
          jsonb channel_config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone_number
          string email
      }
      CONTACT_INBOX {
          uuid id
          uuid contact_id
          uuid inbox_id
          string source_id
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_inbox_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          text content
          string message_type
          string external_source_id
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Layout**: Standard 375px viewport. Bottom navigation with "Inbox" tab.
  - **Inbox List View**: A unified feed of conversations. Each list item shows the contact name, a preview of the last message, a timestamp, and an icon indicating the channel (WhatsApp, IG, Email).
  - **Conversation View**: Tapping a conversation opens a chat interface. Glassmorphism header showing the contact name and channel. Scrollable message history.
  - **AI Agent Integration**: A floating "Draft Reply" card appears above the input field if the AI Ambassador has prepared a response. One-tap "Approve & Send".

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent)**: Subscribes to the event mesh for `MessageCreated` events where `message_type == 'incoming'`. Queries the `CONTACT` and previous `MESSAGE` history across all connected inboxes to generate a highly contextual drafted reply. The drafted reply is saved as an `ActionRequired` item for the owner to approve.

  ### Key Design Decisions
  - **Tenant Isolation**: Every entity MUST enforce tenant isolation via Row-Level Security (RLS) policies.
  - **Channel Extensibility**: The `channel_type` dictates which Rust adapter handles incoming webhooks and outgoing dispatch.
  - **Identity Resolution**: The linking tables and logic are crucial for unifying identities when a message arrives from an external source.

  ## Implementation Prompt
  **User-Facing Outcome**: Maya the baker connects her Instagram and WhatsApp to OHC. She receives a unified feed of messages. When a customer messages on WhatsApp, the system recognizes them from a previous Instagram order and drafts a contextual reply. Maya taps "Approve" on her phone, and the message is sent natively via the OHC backend, completely eliminating Chatwoot.
  **CUJ & Acceptance Criteria**:
  1. Implement the required database migrations to support the models outlined in the architecture diagram.
  2. Implement the native Rust domain and service layers for these entities, ensuring absolute tenant isolation (RLS).
  3. Implement the APIs required to support the Mobile UX flow (Inbox list, Conversation view, Approve Draft).
  4. Implement an Omnichannel Webhook Gateway that can ingest external messages, resolve the customer identity, and trigger the Ambassador agent for drafting.
  5. Provide Playwright E2E tests: A test that simulates an incoming webhook, verifies the data persistence, and ensures the message and drafted reply appear in the 375px UI correctly.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
