issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  ## Research Report: Custom Rust Omnichannel Chat System

  ### Problem Statement
  Currently, OHC is replacing legacy external dependencies like Chatwoot with a fully native omnichannel chat system. The problem is that there is no deep native support for a multi-tenant unified inbox capable of bridging web, WhatsApp, Instagram, and email seamlessly while being highly performant and adhering strictly to Zero-Trust row-level security (RLS) constraints. The external service model fails the latency, offline, and integrated AI automation requirements of small-business operators who need all interactions unified in their assistant.

  ### Findings & Competitive Analysis
  Analyzing Chatwoot’s Ruby on Rails architecture revealed several core models that form the backbone of a successful omnichannel system:
  1. `Account`, `Inbox`, `Conversation`, `Message`, and `Contact`.
  2. Diverse Channel integrations: `WebWidget`, `WhatsApp`, `API`, `FacebookPage`, `Instagram`, etc.
  3. Action components: `AgentBot`, `AutomationRule`, `Macro`.

  Our native Rust implementation needs to mirror these capabilities but optimize for high-concurrency (Rust Async + Tokio) and multi-tenancy at the data layer (Postgres RLS).

  ### Design Doc
  **Architecture Overview**
  - **Data Model:**
    - `conversations` (tenant_id, inbox_id, contact_id, status)
    - `messages` (tenant_id, conversation_id, sender_type, sender_id, content)
    - `inboxes` (tenant_id, channel_type, configuration)
    - `contacts` (tenant_id, name, phone, email)
  - **Service Layer (Rust):**
    - High-throughput asynchronous message broker using a Postgres `SKIP LOCKED` job queue (or Redpanda/Kafka if scale dictates).
    - WebSocket handling for Web Widget real-time updates.
    - Webhook endpoints for WhatsApp Meta Cloud API.
  - **AI Integration Points:**
    - Every incoming message passes through the **Work Triage AI Agent** before routing to the human inbox, drafting potential responses or automatically updating the CRM/booking system.

  **Architecture Diagram:**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          uuid tenant_id
          string channel_type
          jsonb configuration
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          uuid sender_id
          text content
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string phone
          string email
      }
  ```

  **Mobile UX Flow (375px First)**
  - Unified Inbox view showing aggregated messages with status indicators (New, Draft, Resolved).
  - Tapping a message opens a full-screen conversation thread.
  - AI drafts appear as translucent glass overlay cards with "Approve" or "Edit" buttons.
  - No horizontal scrolling; fully functional on native mobile keyboards.

  ### Implementation Prompt
  **Goal:** Build the core database schema, Rust API endpoints, and a mobile-first Flutter UI for the new Unified Inbox that replaces Chatwoot.
  **CUJ:** As Maya (Baker), I want to open my OHC app, see a new Instagram DM in my Unified Inbox, see an AI-drafted reply about vegan cakes, and tap "Approve & Send" so the customer gets a quick response.
  **Acceptance Criteria:**
  - Create Postgres tables with RLS for `inboxes`, `conversations`, and `messages`.
  - Implement Rust gRPC/REST APIs for fetching and sending messages.
  - Build the Flutter UI for the Unified Inbox leveraging the OHC Premium Token library (Translucent Glass).
  - Ensure 100% unit test coverage and E2E Playwright tests simulating the CUJ.
  - ZERO external Chatwoot dependencies.

  ### Scope & Priority
  **Estimated Scope:** Large

  ### Next Steps
  Dispatch this task to the implementation swarm to begin building the core database and service layers.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
