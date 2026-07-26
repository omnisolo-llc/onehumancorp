issue_title: "Native Rust Omnichannel Chat: Data Model & Architecture Design"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) is replacing Chatwoot with a high-performance, native Rust omnichannel chat system. The external Chatwoot dependency has been fully removed, but OHC currently lacks the core data models, multi-tenant isolation, and backend architecture to handle omnichannel conversations (Web widget, WhatsApp, Instagram, Email, SMS, etc.) natively. We need to implement a Chatwoot-compatible but OHC-native architecture to serve personas like Maya (Instagram DMs for cakes) and Carlos (WhatsApp leads for handyman services).

  ## Research Report
  - **Source Audited:** Chatwoot's core Ruby on Rails models (`Account`, `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*`).
  - **Key Learnings:** Chatwoot relies on polymorphic associations (e.g., `channel_id` + `channel_type` in `Inbox`, `sender_type` in `Message`) and heavy JSONB blobs for `additional_attributes`. It uses standard RDBMS indexing but lacks strictly enforced row-level multi-tenancy at the ORM layer by default, relying on application logic.
  - **OHC Differentiation:** OHC will enforce strict multi-tenancy via PostgreSQL Row Level Security (RLS) on `tenant_id` for every table. OHC will use strongly typed Rust structs and enums rather than loose polymorphic strings. OHC's data model will integrate seamlessly with existing `omni_inbox_messages` and `integration_credentials`.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Tenant ||--o{ Contact : owns
      Tenant ||--o{ Conversation : owns
      Tenant ||--o{ Message : owns

      Inbox ||--o{ Conversation : contains
      Contact ||--o{ Conversation : participates
      Conversation ||--o{ Message : contains

      Inbox {
          uuid id PK
          string tenant_id FK
          string name
          enum channel_type
          jsonb channel_config
          boolean enable_auto_assignment
      }

      Contact {
          uuid id PK
          string tenant_id FK
          string name
          string email
          string phone_number
          string identifier
          jsonb custom_attributes
      }

      Conversation {
          uuid id PK
          string tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          enum status
          uuid assignee_id FK
          datetime last_activity_at
      }

      Message {
          uuid id PK
          string tenant_id FK
          uuid conversation_id FK
          uuid sender_id
          enum sender_type
          enum message_type
          text content
          enum status
      }
  ```

  ### Data Model & Invariants
  All tables MUST include `tenant_id` and have `ENABLE ROW LEVEL SECURITY`.
  - **Inbox:** Represents a channel (WhatsApp, Web, Email). Contains specific configuration.
  - **Contact:** A unified representation of a customer across channels.
  - **Conversation:** Links a Contact and an Inbox. Tracks state (open, resolved, snoozed).
  - **Message:** The actual payload. Types: `incoming`, `outgoing`, `template`, `activity`.

  ### Mobile UX Flow (375px)
  1. **Triage Feed:** Owner sees a unified list of active `Conversation`s sorted by `last_activity_at`. Fully functional on a 375px-wide phone without horizontal scroll. Touch targets are at least 44x44px.
  2. **Thread View:** Tapping a conversation shows a timeline of `Message`s. Input field sticks to the bottom, uses native mobile keyboards.
  3. **Context Pane:** Swiping left (or a top action button) reveals `Contact` details and recent orders.

  ### AI Agent Integration Points
  - **Work Triage:** AI monitors new `Message`s, categorizes intent (inquiry, complaint, order), and updates `Conversation` priority.
  - **Customer Assistant:** Drafts replies based on tenant context, saving them as `Message`s with status `draft`. Uses Gemini Pro primary with fallback.

  ## Implementation Prompt
  **Task for Implementer Agent:**
  Implement the foundational backend data schemas and domain models to support native omnichannel messaging for OHC.

  **User-Facing Outcome:**
  As a business owner (e.g., Maya), I want all customer messages from Instagram, WhatsApp, and the web widget to funnel into a single inbox so I can view and respond to them in one unified feed.

  **Critical User Journey (CUJ):**
  1. The owner navigates to the "Conversations" or "Inbox" screen.
  2. The system fetches and displays a list of unified conversations across multiple channels (WhatsApp, Web, Instagram) belonging only to their tenant.
  3. The owner clicks on a conversation and sees the message history and the associated contact details.

  **Acceptance Criteria:**
  - Migrations establish the unified chat schema with strict multi-tenant isolation (RLS on all tables).
  - Backend models and repository access layer are functional and securely scoped to the tenant.
  - Unit tests comprehensively verify tenant isolation and CRUD operations.
  - No specific API endpoints, DB libraries, or function signatures are prescribed here; design them appropriately for the OHC backend.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
