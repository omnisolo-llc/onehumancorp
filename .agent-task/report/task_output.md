issue_title: "[Architecture] Native Rust Omnichannel Inbox to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC requires a unified, omnichannel inbox to centralize customer communications (WhatsApp, Web Chat, Instagram, SMS, etc.) so that the owner/operator can view and respond to messages in one place. Currently, OHC lacks a native Rust omnichannel architecture and we previously relied on Chatwoot as an external service. Chatwoot has been 100% RETIRED as an external dependency. We must build our own high-performance, multi-tenant conversational architecture in Rust within our `onehumancorp/mono` repo to guarantee absolute data ownership, zero-trust isolation, and tight AI triage integration. This empowers personas like Maya (Instagram DMs for custom cakes) and Carlos (SMS/WhatsApp service inquiries) to manage all interactions via an assistant-led flow.

  ## Research Report
  Based on an exhaustive audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`), the core architecture requires several main domain entities:
  - **Account/Tenant**: The root entity for multi-tenancy.
  - **Inbox**: A channel-specific configuration (e.g., a specific WhatsApp number or web widget).
  - **Channel Adapters**: Specific models per channel (e.g., `Channel::WhatsApp`, `Channel::WebWidget`).
  - **Conversation**: An ongoing dialogue between a Contact and the Inbox.
  - **Message**: Individual message payloads, supporting text, attachments, and structured interactive templates.
  - **Contact**: The customer/lead interacting via the channel.

  Competitors like Shopify Inbox, Zendesk, and Front use similar unified data models, but OHC's key differentiation is the **AI Assistant-First** approach. Our Rust implementation must tightly couple the `Message` and `Conversation` lifecycle with our background AI Job Queue, allowing the Agent Triage system to intercept messages, draft replies, update CRM state, and propose next actions to the owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--|| CHANNEL_ADAPTER : configured_by

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          jsonb config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string identifier
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
          jsonb metadata
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage Dashboard**: The owner (e.g., Maya) opens the app. A UniFi-style translucent glass card shows "3 New Messages (2 Instagram, 1 WhatsApp)".
  2. **Unified Conversation View**: Tapping the card opens the Unified Inbox. Messages from all channels appear in a single, continuous, Apple-style chat UI.
  3. **AI Drafts & Next Actions**: Below a customer's WhatsApp message, the AI Assistant provides a pre-drafted reply and a one-tap "Create Quote" action button based on the conversation context.
  4. **Reply/Approve**: The owner can edit the AI draft via a native mobile keyboard or simply tap "Send". The backend routes the message through the appropriate Rust Channel Adapter.

  ### AI Agent Integration Points
  - **Message Ingestion Hook**: When a `Message` is created via a webhook (e.g., WhatsApp), a PostgreSQL `SKIP LOCKED` job is queued for the AI Triage worker.
  - **Draft Generation**: The AI worker reads the `Conversation` history, generates a drafted reply or proposed action, and inserts an internal `Message` (type: AI_DRAFT) pending owner approval.
  - **Customer Memory**: The AI extracts preferences (e.g., "vegan cakes") and updates the `Contact`'s JSONB custom attributes directly.

  ## Implementation Prompt
  **Goal:** Implement the native Rust core domain models and API endpoints for the Omnichannel Inbox system, replacing the legacy Chatwoot dependency.

  **CUJ (Critical User Journey):**
  1. An owner sets up a new Web Widget Inbox via the OHC settings page.
  2. A customer visits the owner's public storefront and sends a message via the Web Widget.
  3. The owner sees the new conversation in their Work Triage feed, complete with an AI-drafted reply, and clicks "Send" to respond to the customer.

  **Acceptance Criteria:**
  - Define the Rust data models/entities (Tenant, Inbox, ChannelAdapter, Contact, Conversation, Message) with strict row-level security (RLS) via `tenant_id`.
  - Implement the internal gRPC / external REST API endpoints to list inboxes, create conversations, and send/receive messages.
  - Create the `Channel::WebWidget` adapter that handles real-time WebSocket messaging.
  - Ensure the message creation path integrates with the AI Job Queue for background triage.
  - Provide 100% unit test coverage for the Rust implementations.
  - Write Playwright E2E tests simulating a customer sending a message and the owner approving the AI draft.
  - NO external Chatwoot dependencies. All code must reside natively in `src/server/integrations/chat/`.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
