issue_title: "Native Rust Omnichannel Chat System & Chatwoot Retirement"
issue_description: |
  **Title**: Native Rust Omnichannel Chat System & Chatwoot Retirement

  **Problem Statement**:
  As a business owner like Maya the baker or Carlos the handyman, communicating with customers is my most critical task. Currently, the system relies on an external third-party chat service (Chatwoot). This leads to disconnected experiences, potential data silos, and a confusing setup process where I have to manage configurations outside of my main workspace. I need a single, unified "Inbox" inside my assistant app that just works—combining Instagram DMs, web chat, and emails into one place without any technical setup or third-party accounts.

  **Research Report**:
  *   **Chatwoot Audit**: A deep dive into the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals a highly structured omnichannel data model. Key entities include `Inbox`, `Channel::*` (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Api`), `Conversation`, `Message`, and `Contact`. It heavily utilizes WebSockets for real-time messaging, Webhooks for integrations, and complex routing logic.
  *   **Competitor Analysis**:
      *   **Shopify Inbox**: Integrates seamlessly into the Shopify admin. It aggregates chats from the online store, Instagram, and Facebook Messenger. It emphasizes simplicity and direct integration with product catalogs.
      *   **Wix Inbox**: Built into the Wix dashboard. Supports live chat, forms, and social integrations. Very focused on small business workflows and CRM integration.
      *   **WeCom / DingTalk**: Highly integrated messaging embedded directly into enterprise operations.
  *   **Findings for OHC**: Relying on an external Chatwoot service breaks the "OneHumanCorp" promise of a unified, assistant-first experience. We must implement a native Rust backend that replicates the core omnichannel capabilities (inboxes, conversations, messages, contacts) with a multi-tenant, Row-Level Security (RLS) approach in our PostgreSQL database.

  **Design Doc**:

  **Architecture Diagram**:
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT ||--o{ CONTACT : manages

      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean is_active
      }
      CHANNEL {
          uuid id PK
          uuid inbox_id FK
          string channel_type "web, email, instagram"
          jsonb config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string content
          string message_type "incoming, outgoing, template"
      }
  ```

  **Mobile UX Flow (375px first)**:
  1.  **Work Triage Home**: The owner opens the app. A clean, translucent glass-styled card shows "3 New Inquiries".
  2.  **Unified Inbox**: Tapping the card opens the Inbox view. A single list of conversations, regardless of source (Instagram, Web Chat, SMS). Each list item shows the contact avatar, source icon, snippet, and time.
  3.  **Conversation View**: Tapping a conversation opens the chat thread. The AI assistant's drafted reply (if applicable) is visible at the bottom as a suggested action, alongside standard native keyboard input.
  4.  **Customer Context Pane**: Swiping left or tapping a header button reveals the customer's CRM context (past orders, notes, tags) without leaving the chat context.

  **AI Agent Integration Points**:
  *   **Work Triage Agent**: Monitors new `Message` events via PostgreSQL triggers or Redis pub/sub. Automatically groups and prioritizes conversations in the owner's feed.
  *   **Customer & Relationship Assistant**: Hooks into incoming messages to automatically draft replies based on past context and knowledge base, saving them as pending drafts on the `Conversation` for owner approval.
  *   **Operations Assistant**: Detects intent in messages (e.g., "I want to book a cake") and surfaces quick-action widgets (e.g., "Create Quote") directly in the chat UI.

  **Key Design Decisions**:
  *   **Native Rust**: Replacing external Chatwoot with internal Rust microservices ensures we own the entire data lifecycle and can guarantee multi-tenant Zero-Trust isolation.
  *   **Multi-Tenancy**: Every table must include `tenant_id` and have Row Level Security (RLS) enabled.
  *   **Event-Driven**: The system must emit events (e.g., `message.created`) to a queue (like Redis or PostgreSQL SKIP LOCKED) so AI agents can react asynchronously without blocking the main chat API.

  **Implementation Prompt**:
  *Objective*: Implement the foundational native Rust data models and API for the OHC Omnichannel Chat system, completely replacing external Chatwoot dependencies.
  *CUJ*: As an owner, I want to see a unified list of messages from customers across different channels (e.g., web chat, simulated email) inside my OHC app, so I can respond to them from one place.
  *Acceptance Criteria*:
  1. Define the PostgreSQL database schema (with RLS) for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2. Implement the corresponding Rust models and basic CRUD API endpoints in `src/server/api`.
  3. Ensure all APIs enforce multi-tenant isolation via SPIFFE/SPIRE identity or the current authentication context.
  4. Build a basic Playwright E2E test demonstrating a message arriving and being visible in the owner's inbox API response. No UI changes are strictly required for this foundational data layer, but the API must support the 375px mobile UX flow described.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
