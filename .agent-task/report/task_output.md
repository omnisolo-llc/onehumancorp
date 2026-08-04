issue_title: "[Native Chat] Omnichannel Conversation Data Model & Multi-Tenant Boundaries"
issue_description: |
  # Problem Statement
  OneHumanCorp is transitioning away from the legacy chat platform to a fully native Rust-based omnichannel chat architecture to better serve owners like Carlos the handyman and Maya the baker. To enable a unified inbox where owners can manage messages from WhatsApp, Instagram DMs, Email, and the web widget seamlessly, we must first establish the foundational data models. Currently, there are multiple competing persistence models (`inbox_messages`, `omni_inbox_messages`, etc.). We need a single, strongly-typed, tenant-isolated data model in our Rust backend that maps directly to the required schema, replicating the legacy chat platform's successful core entities (`Account`/`Tenant`, `Contact`, `Conversation`, `Message`, `Inbox`) while removing its complexity and legacy baggage.

  # Research Report
  **Findings & Competitive Analysis:**
  - **the legacy chat platform Source Code Audit:** the legacy chat platform uses separate models for `Contact`, `Conversation`, `Message`, and `Inbox`, tightly coupled to an `Account` ID for multi-tenancy.
    - `Contact`: Represents the customer, capturing identity (email, phone, external identifiers).
    - `Conversation`: The thread of communication between a `Contact` and the business, tied to a specific `Inbox` (channel).
    - `Message`: The individual message payloads, tracking status, sender type (agent/contact/bot), and message type.
    - `Inbox`: Represents a specific channel configuration (e.g., WhatsApp number X, Instagram page Y).
  - **OHC Architecture Shift:** We are adopting a Zero-Trust, row-level multi-tenant approach. Every entity must enforce `tenant_id` isolation inherently.
  - **Data Layer:** The Rust backend will use `sqlx` (or similar ORM/Query builder defined in the repo) against PostgreSQL for the cloud and SQLite for the desktop (local-first).

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONTACT : manages
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : contains

      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string channel_type "email, whatsapp, instagram, widget"
          string name
          jsonb credentials
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
          string avatar_url
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          uuid sender_id "Nullable, links to Contact or User"
          string sender_type "contact, agent, bot, system"
          string message_type "incoming, outgoing, template"
          string status "sent, delivered, read, failed"
          text content
          jsonb attachments
      }
  ```

  ### Mobile UX Flow (375px First)
  - This is a backend data modeling task; however, it powers the mobile Unified Inbox view.
  - The model ensures that the Unified Inbox can query: `SELECT * FROM conversations WHERE tenant_id = ? ORDER BY last_activity_at DESC`.
  - It guarantees that when Maya opens a conversation, the messages load securely within her tenant boundary without cross-talk.

  ### AI Agent Integration Points
  - **The Ambassador Agent (Customer Success):** Will query `CONVERSATION` and `MESSAGE` tables within the `tenant_id` scope to build conversation context and RAG memory.
  - **Operations Agent:** Will monitor new `MESSAGE` entities of type `incoming` to trigger background task extraction (e.g., creating a booking request).

  ### Key Design Decisions
  - **Strict Multi-Tenancy:** Every table MUST include a `tenant_id` column. We will leverage PostgreSQL Row Level Security (RLS) policies based on the session's tenant claim.
  - **UUIDs for Primary Keys:** Moving away from sequential IDs (like the legacy chat platform's integers) to UUIDs to prevent enumeration attacks and support offline-first sync (PowerSync).
  - **Unified Message Table:** All message types (text, rich media, templates) are stored in one table using JSONB for unstructured metadata/attachments, simplifying the query path.

  # Implementation Prompt
  **User-Facing Outcome:** Business owners will experience a robust, glitch-free inbox where messages never bleed across accounts, and loading history is lightning fast, enabling seamless AI drafting.

  **CUJ & Acceptance Criteria:**
  1. Define the Rust structs (`Tenant`, `Inbox`, `Contact`, `Conversation`, `Message`) in the `omnichannel` domain module within the `server_ohc` crate.
  2. Implement database migrations (for both PostgreSQL and SQLite profiles) to create these tables.
  3. EVERY table must include a `tenant_id` column and foreign key constraints must be scoped by `tenant_id` where applicable.
  4. Write comprehensive Unit Tests verifying that repository functions (e.g., `create_message`, `get_conversation`) correctly enforce the `tenant_id` boundary (i.e., querying with the wrong tenant ID returns no results/error).
  5. Provide Playwright E2E tests: A test user logs in, sends a message via a simulated webhook, and the message appears in the UI, proving the full data path works end-to-end.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
