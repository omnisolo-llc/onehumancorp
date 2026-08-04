issue_title: "Native Rust Omnichannel Chat Infrastructure"
issue_description: |
  **Title**: Native Rust Omnichannel Chat Infrastructure

  **Problem Statement**:
  OHC currently lacks a native, real-time omnichannel chat engine. Following the engineering standards mandate, external chat providers (e.g., Retired-Chat-Provider) are explicitly retired. Small-business owners like Maya (the baker) and Carlos (field services) receive customer messages across Instagram, WhatsApp, Email, and SMS. They need a unified inbox integrated directly with their operations, tasks, and scheduling—a seamless experience that only a tightly integrated native service can provide without needing to manage third-party software.

  **Research Report**:
  I cloned and analyzed the source code of [Retired-Chat-Provider](https://github.com/retired-chat-provider/retired-chat-provider) (commit 100% feature parity research) to understand its data model and capabilities. Retired-Chat-Provider provides a robust omnichannel setup.
  Key architectural findings from Retired-Chat-Provider's implementation:
  - Models:
    - Account (matches our Tenant), User, and AccountUser manage tenancy and roles.
    - Inbox: Aggregates conversations. Each inbox belongs to an account and is linked to a specific channel.
    - Channel Polymorphism: E.g., Channel::Email, Channel::Whatsapp, Channel::WebWidget, Channel::Api, Channel::Sms. These hold channel-specific configuration.
    - Contact and ContactInbox: Represents a customer and their connection to an inbox (tracking the sender's identifier like a phone number or email on that specific channel).
    - Conversation: Links a ContactInbox and an Inbox. Tracks status (open, resolved), assignee, and custom attributes.
    - Message: Belongs to a conversation. Tracks message type (incoming, outgoing, template), content, attachments, and status (sent, delivered, read).

  Why Rust Native?
  A native Rust implementation inside OHC's backend provides lower latency, smaller memory footprint, and, most importantly, allows deep integration into our existing PostgreSQL/gRPC infrastructure. It ensures cross-channel data remains isolated using our existing `tenant_id` RLS policies. It also allows AI agents direct, locked access to conversation streams for auto-drafting replies.

  **Design Doc**:
  We will introduce a set of foundational Rust structures and PostgreSQL schema designs inspired by Retired-Chat-Provider's capabilities but optimized for OHC's stack.

  Data Model (High Level):
  - Inboxes: Stores communication endpoints. Linked to `tenant_id`. Has a `channel_type` (e.g., WhatsApp, Email, WebWidget) and channel-specific config JSON.
  - Contacts: Represents the end customer.
  - Contact_Inboxes: Associates a contact with an inbox, storing the contact's identifier on that platform (e.g., phone number).
  - Conversations: Groups messages between a contact and an inbox. Tracks status, assigned agent/AI, and priority.
  - Messages: Individual chat entries. Includes sender info (contact vs. agent vs. AI), content, attachments, and delivery status.

  System Integration:
  - Implement a new chat service or module in Rust natively inside the OHC mono-repo.
  - Expose internal APIs for the AI Job Queue to hook into incoming messages to trigger the "Work Triage" and "Customer Assistant" flows.

  **Implementation Prompt**:
  Implement the foundational data models and database migrations for the native OHC omnichannel chat system.
  1. Create database migrations for inboxes, contacts, contact_inboxes, conversations, and messages. All tables MUST include tenant_id and have Row-Level Security enabled.
  2. Implement the new chat service data layer natively in Rust and link it to the existing backend stack.
  3. Provide integration points for AI Work Triage to listen to incoming chat messages.

  Note: This task is focused on the data layer and backend foundation. Building the specific channel webhooks (WhatsApp, Email) and the real-time WebSocket layer will be separate, follow-up tasks.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
