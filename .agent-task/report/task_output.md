issue_title: "Architecture: Native Rust Omnichannel Inbox & Chatwoot Replacement"
issue_description: |
  # Native Rust Omnichannel Inbox Architecture

  ## Problem Statement
  Currently, the platform lacks a native multi-tenant omnichannel inbox, and the architectural directive explicitly mandates the 100% retirement of Chatwoot as an external service. A core requirement for owner/operator personas like Maya (Baker handling Instagram DMs), Carlos (Handyman handling SMS), and Nora (Agency Principal handling email) is a unified communication layer. Without a natively integrated, high-performance messaging architecture in Rust, the OHC platform cannot securely guarantee Zero-Trust multi-tenancy, SPIFFE-based agent coordination, or offline-capable mobile workflows. The critical gap is the complete absence of a unified native inbox that can ingest, route, and support AI-agent auto-drafting for multi-channel communications natively within OHC.

  ## Research Report
  - **Market Context**: Modern SMB platforms (e.g., Shopify Inbox, Meta Business Suite, Wix Inbox) centralize communications so that business owners do not have to switch between apps to serve their customers.
  - **Chatwoot Source Code Audit Context**: Based on an audit of the OSS Chatwoot source code (`app/models/*`), a complete omnichannel system requires at minimum:
    - `conversations` (tracking states like `status`, `last_activity_at`, `assignee_id`) & `messages` (tracking `content`, `message_type`, `sender_type`, `content_attributes`).
    - `inboxes` representing specific communication channels (e.g., Email, Instagram DM, Web Widget, WhatsApp).
    - `contacts` tracking unified customer identities across multiple channels.
    - `Channel Adapters` responsible for mapping upstream provider payloads into standardized platform events.
    - Real-time event broadcasting (WebSocket) for active dashboard and mobile clients.
  - **OHC Specifics & Scaling**: Unlike standalone Chatwoot, OHC requires this system to be intrinsically tied to the AI Job Queue (PostgreSQL `SKIP LOCKED`) and Redis Distributed Locks. Furthermore, every row must be strictly isolated via `tenant_id` to guarantee multi-tenancy security.

  ## Design Doc
  ### Data Model & Architecture (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string channel_type
          jsonb settings
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          text content
          string sender_type
          timestamp created_at
      }
      TENANT ||--o{ INBOX : manages
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : holds
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### AI Agent Integration Points
  - **Work Triage / Incoming Message Flow**: When a new `Message` is created by a customer via a Channel Adapter, an AI Job is queued automatically. The Customer Relationship Assistant reads the message, fetches tenant-scoped context (from `CONTACT` and `CONVERSATION`), and writes a drafted reply (with `sender_type="ai_agent_draft"`).
  - **Human-in-the-Loop Approval**: The drafted reply appears in the mobile UI. The owner (e.g. Maya) reviews it. If approved, the webhook dispatches the message back to the external channel.

  ### Mobile UX Flow (375px First)
  - **Unified Feed Screen**: A translucent-styled, UniFi-esque list of active conversations. Unread messages or pending AI drafts have distinct UI status tokens (e.g. a blue dot for unread, a purple sparkle icon for AI-drafted reply).
  - **Conversation Thread**: Native keyboard integration on iOS/Android. Messages bubble layout. The AI draft appears inline as a distinct visual card with "Approve" and "Edit" buttons at the bottom. Touch targets are strictly > 44x44px.

  ### Implementation Prompt
  **Objective**: Implement the core Rust entity layer and PostgreSQL schema for the Native Omnichannel Inbox, completely replacing external Chatwoot dependencies.

  **Persona Outcome**: Maya can see all Instagram DMs and Email inquiries in one unified feed and approve AI-drafted replies from her 375px mobile screen without leaving the OHC platform.

  **Acceptance Criteria**:
  1. Define the SQL schema and migration for `inboxes`, `contacts`, `conversations`, and `messages`, ensuring Row-Level Security on `tenant_id` is strictly enabled.
  2. Implement the basic CRUD Rust backend module inside `onehumancorp/mono` (with 100% unit test coverage).
  3. Create an API endpoint for fetching a unified feed of conversations.
  4. Ensure all new schemas and models successfully apply and endpoints enforce tenant isolation properly to be consumed by the Flutter frontend.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
