issue_title: "[Native Chat] Architect Rust-native Omnichannel System"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is retiring its dependency on external chat systems in favor of a fully native, Rust-backed omnichannel chat system. The goal is to provide non-technical owners with a unified inbox that tracks customer conversations across channels (DMs, SMS, Web Widget, Email) securely within OHC's multi-tenant architecture, eliminating external dependencies and improving performance. Maya the baker needs to answer Instagram DMs seamlessly within the OHC app; Carlos the handyman needs to read SMS inquiries in the same place.

  ## Research Report
  - We analyzed the open-source omnichannel repositories to understand their data models (Account, Inbox, Contact, Conversation, Message, Attachment).
  - Robust polymorphic channel architecture (`Channel::Api`, `Channel::Email`, `Channel::TwilioSms`, `Channel::WebWidget`, etc.).
  - Conversations belong to Inboxes, which in turn map to Channels.
  - Messages track sender type (Contact, User, Bot), status, and attachments.
  - OHC needs to replicate these core domain entities natively in Rust within `src/server/ohc/domain/` to achieve 100% feature parity.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      USER ||--o{ MESSAGE : sends
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  ### Data Model (Rust Structs - Proposed)
  - `Inbox`: Maps to a specific tenant and channel.
  - `Channel`: Enum or trait representing SMS, Email, Web, IG, etc.
  - `Conversation`: Links a `Contact`, an `Inbox`, and an `Assignee`.
  - `Message`: Contains text/rich content, sender type, and timestamp.
  - `Contact`: Represents the external customer.

  ### Mobile UX Flow (375px first)
  1. The Owner opens the OHC app and sees a unified "Inbox" tab.
  2. The Inbox list displays active conversations with a channel icon (e.g., Instagram, SMS, Web).
  3. Tapping a conversation opens a familiar chat interface (bubbles for incoming/outgoing).
  4. The owner types a reply and hits send. The backend routes the message back to the correct channel via the Rust native channel adapters.

  ### AI Agent Integration Points
  - The **Customer & Relationship Assistant** (AI) can draft replies automatically based on previous context.
  - The **Work Triage** agent can summarize long conversation threads and propose the next action (e.g., "Create Quote").

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to implement the core Rust data models and database schemas for the new Native Omnichannel Chat system in `src/server/ohc/domain/`.
  1. Define the domain structs (`Inbox`, `Conversation`, `Message`, `Contact`, `Channel`) with strict multi-tenant isolation (`tenant_id` on all entities).
  2. Ensure the models support the core features (polymorphic channels, participant tracking, message statuses).
  3. Write comprehensive unit tests for the domain models to verify tenant isolation and relationship integrity.
  4. Integrate these models with the existing OHC database layer.
  *Remember to evaluate the implementation against the personas: The models must support Maya's IG DMs and Carlos's SMS inquiries.*

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
