issue_title: "Implement Custom Rust Omnichannel Chat System"
issue_description: |
  # Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp (OHC) currently relies on external chat integrations or incomplete models. As a platform serving owners and operators (Maya the baker, Carlos the handyman), we need a centralized inbox that unifies DMs, SMS, WhatsApp, and emails. The platform must transition away from external Chatwoot dependencies and implement a high-performance, multi-tenant omnichannel chat engine natively in Rust.

  ## Research Report
  - Competitor Analysis: Shopify Inbox, Wix Inbox, Chatwoot. Chatwoot provides a robust open-source reference for data models and channel integrations.
  - Current state: OHC lacks a unified Rust-native chat inbox that maps different messaging platforms to a single threaded view for the owner.
  - Chatwoot Benchmarking: Analyzed `app/models` in Chatwoot. Key entities include: `Inbox`, `Conversation`, `Message`, `Contact`, `Channel::*` (API, Facebook, WhatsApp, Email, SMS, WebWidget).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### Core Entities & Invariants
  - **Tenant Isolation:** Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST include `tenant_id` and utilize PostgreSQL Row-Level Security (RLS).
  - **Inbox:** Represents a unified view or a specific channel connection (e.g., "Maya's IG DMs").
  - **ChannelAdapter:** Rust traits/structs to handle specific platform webhooks and API calls (Instagram, SMS, Email, Web Widget).
  - **Conversation:** A threaded interaction with a contact.
  - **Message:** Individual messages within a conversation.
  - **Contact:** The customer/client communicating with the business.

  ### Mobile UX Flow (375px first)
  1. **Unified Inbox List:** A clean list of active conversations, sorted by recent activity and urgency. Unread dots and channel icons (IG, SMS) indicate the source.
  2. **Conversation View:** Standard chat interface. Messages from the contact on the left, owner replies (and AI drafts) on the right.
  3. **Action Bar:** Quick actions above the keyboard to "Generate Quote", "Send Payment Link", or "Book Appointment".
  4. **AI Integration:** "Draft Reply" button that uses context from previous messages and business rules to suggest a response.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Listens to new inbound messages via a message bus or trigger. If configured, it drafts a suggested reply and saves it as a draft or sends it directly based on owner preferences.
  - **Work Triage:** Flags urgent conversations (e.g., "Need to change my order for tomorrow") and bubbles them to the top of the owner's daily feed.

  ## Implementation Prompt
  Implement the core database schema (PostgreSQL) and native Rust gRPC/REST services for the Omnichannel Chat System.
  1. Create database migrations for `inboxes`, `conversations`, `messages`, `contacts`, and channel configuration tables, enforcing tenant isolation with RLS.
  2. Implement the Rust service layer with CRUD operations for these entities.
  3. Define the gRPC protobuf definitions for the chat service.
  4. Ensure all operations are protected by authentication and tenant validation (SPIFFE/SPIRE context).
  5. Provide unit tests with 100% coverage and integrate with the existing Bazel build system. Do NOT implement specific external API integrations (like Twilio or Meta) in this first pass; focus on the core data structures and internal APIs.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
