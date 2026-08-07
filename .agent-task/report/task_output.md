issue_title: "Native Rust Omnichannel Chat System for OHC (Replacing Chatwoot)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a unified, native omnichannel inbox system to power the Customer Relationship Assistant capability. The product vision and engineering standards explicitly mandate the 100% retirement of Chatwoot as an external dependency and demand a native Rust implementation of an omnichannel messaging system within `onehumancorp/mono`. Non-technical owner/operator personas (e.g., Maya the Baker managing Instagram DMs, Carlos the Handyman answering SMS) need a centralized, mobile-first unified inbox where AI agents can draft replies seamlessly. If an owner has to context-switch between Instagram, WhatsApp, email, and SMS, the "Assistant-First" promise is broken.

  ## Research Report
  ### Competitive Analysis
  - **Chatwoot**: Open-source customer engagement suite. Uses Ruby on Rails and PostgreSQL. Key strengths are its data model for Channels (Facebook, Twitter, WhatsApp, SMS, Line, API, Web Widget), Inboxes, Conversations, Messages, and Contacts. It supports macros, canned responses, automation rules, and webhooks. We must replicate this feature parity natively in Rust.
  - **Shopify Inbox**: Centralized chat for Shopify merchants. Focuses heavily on surfacing product context and converting chats to sales.
  - **Stripe / Wix**: Both offer CRM and lightweight inbox features, but lack deep omnichannel integrations (WhatsApp/IG) out of the box without complex app store plugins.

  ### Chatwoot Source Audit Findings
  Inspecting `/tmp/chatwoot/app/models/`, the core domain models we need to adapt for OHC's Rust backend include:
  - `Account` (Maps to OHC `Tenant`)
  - `User` / `AccountUser` (Maps to OHC `User` / `Member`)
  - `Contact` / `ContactInbox` (Customer representation per channel)
  - `Inbox` (A specific channel instance, e.g., "Maya's Instagram")
  - `Conversation` (A thread between a Contact and the Inbox)
  - `Message` (Individual message payload, supports attachments, text, template, form)
  - `Channel::*` (Specific adapters like `WebWidget`, `Email`, `Whatsapp`, `Instagram`, `Sms`)

  ## Design Doc
  ### High-Level Architecture
  The native Rust omnichannel system will be composed of a highly scalable, multi-tenant conversational engine inside `src/server/`.

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : manages
      TENANT ||--o{ CONTACT : has
      CONTACT ||--o{ CONTACT_INBOX : interacts_via
      INBOX ||--o{ CONTACT_INBOX : connects
      INBOX ||--o{ CONVERSATION : receives
      CONTACT_INBOX ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX }|--|| CHANNEL_ADAPTER : configured_with
  ```

  ### Components
  1.  **Core Domain Models & PostgreSQL Schema (Rust + SQL)**:
      - Implement schemas with strict Row Level Security (RLS) bound to `tenant_id`.
      - Use `pgvector` optionally for semantic search over past conversations.
  2.  **Channel Adapters (Rust)**:
      - Trait-based adapter system for handling incoming webhooks from external providers (Meta/IG, Twilio/SMS, Email, Web Widget).
  3.  **Real-time Event Bus (Rust/Valkey)**:
      - Replace Chatwoot's ActionCable/Redis with Axum WebSockets + Valkey Pub/Sub to push new messages instantly to the Tauri/web clients.
  4.  **AI Customer Relationship Assistant Integration**:
      - The `Customer Assistant Agent` will subscribe to the "New Message" event stream via the AI Job Queue (PostgreSQL `SKIP LOCKED`).
      - When a new message arrives, the agent analyzes the context (using tenant memory and past orders) and generates a draft reply. The draft is persisted with `status: 'draft'` waiting for owner approval.

  ### Mobile-First UX Flow (375px)
  1.  **Triage Feed (Home)**: The owner sees unread conversations grouped by urgency.
  2.  **Conversation View**:
      - Standard chat UI (bubbles).
      - **CRITICAL**: The bottom input area shows the AI-drafted reply automatically.
      - Buttons: [Send Draft] [Edit] [Discard].
      - Swipe right on a message to view the Contact's profile and order history natively.

  ## Implementation Prompt
  Implement the core API and data models for the native Rust Omnichannel Chat system.
  1. Define the SQL migrations for `inboxes`, `contacts`, `contact_inboxes`, `conversations`, and `messages`, ensuring Row Level Security (`tenant_id`) is strictly enforced.
  2. Implement the Rust Axum API routes in `src/server/services/chat/` for listing conversations and sending messages.
  3. Create a trait `ChannelAdapter` and implement a basic `WebWidget` dummy adapter.
  4. Integrate the API with the frontend using Tauri/React, ensuring the Conversation screen is fully responsive on 375px viewports. It must show a hardcoded AI draft reply to prove the UI concept.
  5. **MANDATORY**: Adhere to the "Zero Mock Data" rule for the final product state; data must come from the DB. Write comprehensive unit tests for the API and Playwright E2E tests simulating a user receiving a message and approving an AI draft.

  ## Priority
  P0

  ## Estimated Scope
  Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
