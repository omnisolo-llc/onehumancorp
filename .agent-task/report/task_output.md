issue_title: "Architecture: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  OHC is retiring its external Chatwoot dependency to build a native, high-performance, multi-tenant Rust omnichannel chat system. Small business owners (like Maya the Baker and Carlos the Handyman) need a unified inbox that captures Instagram DMs, WhatsApp messages, and website chats without needing third-party tools. The current lack of a native engine slows down onboarding and complicates tenant isolation, data latency, and AI triage routing.

  ## Research Report
  Benchmarking Chatwoot's Ruby-on-Rails source code (`https://github.com/chatwoot/chatwoot`) reveals several core data models crucial for an omnichannel system:
  - `Inbox`: Configuration point for a channel, binding it to a business and managing CSAT, out-of-office rules, and auto-assignment.
  - `Channel`: Specific provider implementations (e.g., `Channel::Whatsapp`, `Channel::WebWidget`). Holds credentials, webhook configurations, and provider-specific states.
  - `Conversation`: The central thread tying messages to an inbox and contact, managing status (open, snoozed, resolved), assignees, and SLA policies.
  - `Message`: The granular unit of communication, containing content type, sender/receiver info, and provider-specific IDs.
  - `Contact`: Represents the customer/visitor across all channels.

  Industry best practices for systems handling high-volume webhooks and WebSockets (like Shopify or Stripe) favor statically typed, async-first languages (like Rust) for minimal footprint and maximum concurrency. OHC's implementation will use Rust, utilizing Tokio for async tasks, Axum for webhook/WebSocket ingress, and Postgres with Row Level Security (RLS) via `tenant_id` for strict data isolation.

  ## Design Doc
  ### Data Model & Invariants
  We need native Rust representations of Chatwoot's core entities.
  - **Inboxes:** Configured per tenant, linking a channel to a team/agent.
  - **Channels:** Interfaces for Meta (WhatsApp/Instagram) and WebWidget.
  - **Conversations & Messages:** Tracking state and threads securely.

  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : "owns"
      Tenant ||--o{ Contact : "owns"
      Inbox ||--o{ Conversation : "has"
      Contact ||--o{ Conversation : "participates in"
      Conversation ||--o{ Message : "contains"
      Inbox ||--|| Channel_Whatsapp : "uses"
      Inbox ||--|| Channel_WebWidget : "uses"

      Tenant {
          uuid tenant_id PK
          string name
      }
      Inbox {
          int id PK
          uuid tenant_id FK
          string name
          boolean enable_auto_assignment
      }
      Contact {
          int id PK
          uuid tenant_id FK
          string name
          string email
          string phone_number
      }
      Conversation {
          int id PK
          uuid tenant_id FK
          int inbox_id FK
          int contact_id FK
          string status
      }
      Message {
          int id PK
          uuid tenant_id FK
          int conversation_id FK
          string content
          int message_type
      }
      Channel_Whatsapp {
          int id PK
          uuid tenant_id FK
          string phone_number
          string provider
      }
      Channel_WebWidget {
          int id PK
          uuid tenant_id FK
          string website_url
          string website_token
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed:** The owner opens the app (e.g., Maya on her iPhone) and sees a unified feed of unresolved conversations from WhatsApp, IG, and WebWidget.
  2. **Conversation View:** Tapping a thread opens a chat interface with a sticky bottom reply bar.
  3. **Context Sheet:** A swipeable bottom sheet reveals customer details (Contact model), past orders, and agent draft proposals.
  4. **Performance Target:** WebSocket connections must establish in < 100ms. Local persistence must allow offline reads of cached threads.

  ### AI Agent Integration
  - **Operations Agent:** Receives Webhook events on `Message` creation via the AI Job Queue (Postgres `SKIP LOCKED`).
  - **Triage Protocol:** The agent reads the conversation context, drafts a suggested reply based on memory/knowledge base, and presents it to the owner in the UI. For predefined intents (e.g., "do you do vegan cakes?"), it can auto-reply if the owner has configured the Inbox to allow it.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the native Rust data layer and core service handlers for the new Omnichannel Chat System in `src/server/integrations/chat`, fully deprecating Chatwoot logic.

  **Acceptance Criteria:**
  1. Define Rust structs and `sqlx` database schemas/migrations for `Inbox`, `ChannelWhatsapp`, `ChannelWebWidget`, `Conversation`, `Message`, and `Contact`. Ensure all tables have a `tenant_id` with Postgres RLS enabled.
  2. Implement an Axum webhook ingress route for WhatsApp that verifies Meta's signature, parses the incoming JSON, creates/updates the `Contact`, `Conversation`, and `Message` entities, and triggers a local event.
  3. Implement an Axum WebSocket route for the WebWidget channel that handles incoming client connections, authenticates via `website_token`, and streams new `Message` entities bi-directionally.
  4. Provide 100% unit test coverage for the domain logic and parsing.
  5. Provide at least one Playwright E2E test verifying a message flows from the WebWidget mock through the API to the database.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []