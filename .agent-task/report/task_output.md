issue_title: "Native Rust Omnichannel Chat: Core Data Models & Inbox Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System - Architecture & Design

  ## Problem Statement
  OneHumanCorp (OHC) is replacing its legacy external chat dependency (Chatwoot) with a native, high-performance Rust implementation. The core platform needs a unified omnichannel chat architecture to power work intake from multiple channels (WhatsApp, Web Widget, Email, Meta) while maintaining strict tenant isolation and mobile-first readiness. The goal is to design the core data entities, multi-tenant isolation strategy, and backend boundaries that will replace Chatwoot's core architecture while deeply integrating with OHC's AI work triage agent.

  ## Research Report
  - **Codebase & Docs Audit**: OHC currently lacks robust native data models for managing unified communication in Rust. `src/server/integrations/chat/README.md` clearly dictates building a native Rust omnichannel chat system to handle WhatsApp and Web Widget messages without external dependencies.
  - **Chatwoot Source Audit**: We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot/tree/develop/app/models`), explicitly observing how it handles unified inbox architecture using abstract channels (WhatsApp, Twitter, WebWidget, etc), Conversations, Inboxes, and Contacts.
  - **Competitive Analysis**: High-performing CRMs and customer support products like Meta WhatsApp Cloud API and Shopify Inbox aggregate multiple channels into a single unified `Inbox`, representing a `Conversation` spanning across distinct channel identities, allowing a single view for the operator.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL : contains
      INBOX ||--o{ CONVERSATION : manages
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL ||--o{ MESSAGE : routes

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string provider_type
          jsonb provider_config
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone_number
          string email
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
          uuid channel_id
          string content
          string sender_type
      }
  ```

  ### Core Entities (Rust & PostgreSQL)
  1. **Inbox**: The top-level grouping for conversations for a specific tenant.
  2. **Channel**: Represents the integration type (e.g., WhatsApp, Web Widget, Instagram).
  3. **Contact**: The customer interacting with the business.
  4. **Conversation**: The active thread between the business (via an Inbox) and the Contact.
  5. **Message**: Individual payloads (text, images, templates) sent within a Conversation, tagged with the sending Channel.

  ### Multi-Tenancy & Zero Trust
  Every table must include a `tenant_id` column. PostgreSQL Row-Level Security (RLS) policies must be strictly applied using `tenant_id` to ensure absolute tenant isolation. The Rust layer must inject `tenant_id` from the authenticated SPIFFE/SPIRE context.

  ### AI Agent Integration
  - **Work Triage Agent**: Will subscribe to new messages on the `Conversation` model to generate automatic replies, summarize context, or escalate urgency to the human operator.

  ### Mobile UX Flow (375px first)
  - **Unified Inbox View**: A simple list view of active `Conversations` sorted by recent activity.
  - **Conversation View**: Real-time chat interface showing messages, agent notes, and the contact profile side-by-side (collapsible on mobile).
  - **Empty State**: Clear prompt indicating "No active conversations. Connect a channel to get started."

  ## Implementation Prompt
  Implement the core database migrations and Rust SeaORM/SQLx entity models for the Native Omnichannel Chat system described in the design above.
  - Create strict PostgreSQL migrations ensuring RLS policies using `tenant_id` for all chat-related tables (`inboxes`, `channels`, `contacts`, `conversations`, `messages`).
  - Create the corresponding Rust struct models representing these entities in `src/server/domain/omnichannel.rs` or `src/server/integrations/chat/models.rs`.
  - Write comprehensive unit tests for the data models and ensure all Bazel tests pass.
  - Verify tenant isolation works as expected using isolated unit tests.
  - Do NOT implement the frontend UI or external webhook listeners in this task; focus entirely on the core data schema and Rust models.
  - Acceptance Criteria: `bazel test //...` passes 100%, new entities are fully covered by unit tests, and the database schema implements proper RLS.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
