issue_title: "Native Rust Omnichannel Chat: Data Model & Protocol Architecture"
issue_description: |
  # Native Rust Omnichannel Chat System Architecture

  ## Problem Statement
  OneHumanCorp previously relied on Chatwoot as an external dependency for omnichannel customer support and chat functionality. The mandate now is to completely retire Chatwoot and replace it with a high-performance, native Rust omnichannel chat system that lives inside `onehumancorp/mono`. This new system must achieve feature parity with Chatwoot, including inboxes, conversations, messages, channels (WhatsApp, Facebook, Twitter, Email, API, etc.), routing, and agent collaboration. The current `src/proto/inbox.proto` is too simplistic and needs a comprehensive data model and gRPC API definition reflecting the rich entities and relationships found in Chatwoot's source code, adapted for a multi-tenant, Rust-based backend.
  This limits our target personas, like Carlos (handyman needing SMS/WhatsApp integration) and Maya (baker using Instagram DMs), from having a unified, high-scale inbox managed by AI and their owner interface.

  ## Research Report
  - **Chatwoot Source Code Audit:**
    - Chatwoot's core entities: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel` (polymorphic).
    - `Inbox` connects to a specific channel (e.g., `Channel::Email`, `Channel::Whatsapp`, `Channel::FacebookPage`, etc.).
    - `Conversation` links a `Contact`, an `Inbox`, and `Messages`. It has a `status` (open, resolved, pending, snoozed), `assignee`, `team`, and `unread_count`.
    - `Message` has `content`, `attachments`, `message_type` (incoming, outgoing, template), `content_type` (text, html, etc.), `sender` (Contact or User/Bot), and a rich set of metadata/attributes.
    - `Contact` represents the end customer across different channels, merging identifiers like email, phone number, and social IDs.
  - **OHC Architecture Requirements:**
    - Multi-tenant by default (`tenant_id` on every resource).
    - Rust-based gRPC API (`src/proto/inbox.proto` expansion).
    - AI Agent integration: The routing and AI drafting capabilities require clear interfaces in the data model.
    - High scale and offline-first support for mobile via gRPC-Web / PWA.

  ## Design Doc
  ### Data Model & System Architecture
  We will expand `src/proto/inbox.proto` (or create dedicated protos like `chat.proto` / `channel.proto`) to include representations for:
  - **Inbox Entity:** Managing channel connections and routing rules.
  - **Contact Entity:** Managing customer data and custom attributes.
  - **Conversation Entity:** Tracking status, assignees, priorities, and unread counts.
  - **Message Entity:** Handling various content types (text, html, attachments), message types (incoming, outgoing), and sender contexts.

  *Note: Specific gRPC field definitions and exact schemas are left to the implementer to ensure best fit with Rust and our multi-tenant patterns.*

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : holds
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
      INBOX ||--|| CHANNEL : connects_via
  ```

  ### UI Wireframes & Mobile UX Flow
  - **Work Triage (375px viewport):**
    - The main inbox screen features a unified feed of messages, tasks, and alerts using the clean UniFi modular card layout.
    - Tap on a conversation card opens the detailed chat view.
  - **Chat Details (375px viewport):**
    - The conversation screen shows message bubbles.
    - A floating action button (FAB) or sticky bottom bar provides quick actions like "Reply", "Assign to Agent", or "Draft Proposal".
    - AI drafted replies appear as a distinct translucent glass card above the text input, with "Approve" or "Edit" buttons.
  - **Customer Profile:** Swipe left on a conversation to reveal a side sheet (or dedicated screen on mobile) with the Contact's contextual data (past orders, notes, attributes).

  ### AI Integration Points
  - **Work Triage:** AI monitors new `Conversation`s, updates status, sets priority, and suggests routing based on NLP analysis of the first message.
  - **Drafting:** AI reads `Conversation` history and generates a `Message` with `is_private = true` (or a dedicated draft status) containing the suggested reply for the human to approve.
  - **Auto-Reply:** Bot assignee can immediately create an `OUTGOING` message and optionally hand off to a human by setting `assignee_id` to null and status to open.

  ## Implementation Prompt
  Implement the gRPC data models and service definitions in `src/proto/inbox.proto` (and related files if needed) for the new native Rust omnichannel chat system.
  1. Define the necessary protobuf messages incorporating the conceptual entities (Inbox, Contact, Conversation, Message, Attachment) inspired by the audited Chatwoot schema but tailored for gRPC and OHC's multi-tenant architecture.
  2. Implement the gRPC service endpoints for standard CRUD operations and real-time streaming (e.g., creating messages, listing conversations, streaming updates).
  3. Ensure all messages and RPCs strictly include `tenant_id` for security/isolation.
  4. Build a comprehensive unit test suite in Rust that instantiates these models, tests validation logic (e.g., proper state transitions for Conversation status), and verifies the API behavior.

  **Estimated Scope:** Medium

  **Acceptance Criteria:**
  - `bazel test //...` passes 100%.
  - Protobuf definitions cover the core Chatwoot entities required for a full replacement.
  - Rust models are generated and available in the backend module.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chatwoot-replacement]
assignees: []
