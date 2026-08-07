issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Context

  OHC is retiring the external Chatwoot dependency to build a custom, native Rust omnichannel chat system within `onehumancorp/mono`. We need a foundational data model and API that mirrors Chatwoot's functionality but is integrated deeply with our multi-tenant OHC architecture and optimized for high performance.

  # Problem Statement

  - Currently, OHC lacks a native chat system and rely on an external service (Chatwoot), which introduces latency, synchronization issues, and breaks our isolated, secure multi-tenant architecture.
  - Owners like Maya (baker) and Carlos (handyman) need seamless real-time messaging integrated into their daily work triage feed without switching apps or dealing with external tools.
  - The new system must support various channels (web widget, WhatsApp, Instagram, email) in a unified inbox, maintaining the same rich feature set (macros, canned responses, assignments, SLAs) but executing inside our Rust backend.

  # Research Report

  - **Source Code Audit (Chatwoot):** I have cloned and audited the `chatwoot/chatwoot` repository. The core data models revolve around:
    - `Account` (maps to OHC `Tenant`)
    - `Inbox` (the core container for channels)
    - `Channel::*` (adapters for Email, WebWidget, Api, etc.)
    - `Conversation` (the thread, linking Contact and Inbox)
    - `Message` (individual entries in a conversation)
    - `Contact` (the customer interacting with the business)
  - **Data Isolation:** Chatwoot uses row-level tenant mapping via `account_id`. We will use our standard PostgreSQL RLS (Row-Level Security) with `tenant_id` for strict isolation.
  - **Real-time:** Chatwoot relies on ActionCable (Ruby). OHC's Rust backend can leverage `tokio-tungstenite` and `axum` WebSockets, combined with Redis/NATS for scalable pub/sub across instances.
  - **Competitors:** Shopify Inbox and Stripe's communication tools focus heavily on commerce context (e.g., tying conversations directly to orders/invoices). OHC's chat must inherently link `Conversation` to `Work Intake`, `Bookings`, and `Payments`.

  # Design Doc

  ## Architecture

  - **Core Entities (Rust / SeaORM):**
    - `Inbox`: Configuration for a messaging channel (e.g., "Main Support", "Instagram DMs").
    - `ChannelAdapter`: Stores credentials and settings for specific platforms (Web Widget, Meta, Twilio).
    - `Conversation`: A thread belonging to an Inbox, a Contact, and a Tenant. Tracks status (open, resolved, snoozed) and assignee.
    - `Message`: Contains content (text, attachments), sender type (Contact, User, Agent/Bot), and metadata (read receipts).
    - `Contact`: Represents the customer/lead.
  - **Real-Time Engine:**
    - Rust `axum` WebSocket routes for client connections.
    - NATS or Redis pub/sub to broadcast new messages across the distributed K8s cluster.
    - Event schema matches OHC's internal event bus for AI Agent observation.
  - **Multi-Tenancy:**
    - All tables MUST include `tenant_id` and utilize PostgreSQL RLS policies.
  - **AI Agent Integration:**
    - The `Operations Assistant` and `Customer & Relationship Assistant` will subscribe to the `conversation.message.created` topic on the event bus to draft replies, identify intent, and extract work tasks automatically.

  ## Entity-Relationship Diagram

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : configures
      Tenant ||--o{ Conversation : owns
      Tenant ||--o{ Contact : tracks
      Inbox ||--|| ChannelAdapter : uses
      Inbox ||--o{ Conversation : routes
      Contact ||--o{ Conversation : engages_in
      Conversation ||--o{ Message : contains

      Tenant {
          uuid id PK
          string name
      }
      Inbox {
          uuid id PK
          uuid tenant_id FK
          string name
          uuid channel_id FK
      }
      ChannelAdapter {
          uuid id PK
          string channel_type
          json credentials
      }
      Contact {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      Conversation {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
      }
      Message {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string content
          string sender_type
      }
  ```

  ## Mobile UX Flow (375px First)

  - **Inbox List:** Clean list view of recent conversations. Unread indicators prominent. Avatars for contacts. Swipe actions for "Resolve" or "Assign".
  - **Conversation View:** Standard chat interface. Sticky bottom input bar with attachment icon and quick-reply/macro button.
  - **Context Panel:** Sliding drawer or top sheet to view Contact details, recent orders, and AI-generated summaries.

  # Implementation Prompt

  - Implement the SeaORM entity definitions and database migrations for the new Omnichannel Chat core tables: `inbox`, `conversation`, and `message`. Ensure `tenant_id` is present and RLS is enabled for all tables.
  - Create the CRUD API services (using `axum` or `tonic` gRPC) to create an inbox, start a conversation, and send a message.
  - Implement a basic WebSocket handler in Rust to broadcast new message events to connected clients.
  - Include integration tests verifying that messages cannot cross `tenant_id` boundaries.
  - **Acceptance Criteria:** A user can create an Inbox, a Contact can start a Conversation, messages can be sent/received via REST/gRPC, and real-time updates are emitted over WebSocket. Multi-tenant isolation is enforced.

  # Estimated Scope

  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
