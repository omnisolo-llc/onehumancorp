issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema"
issue_description: |
  **Problem Statement**
  Currently, OneHumanCorp does not have a native, fully multi-tenant, Rust-based Omnichannel Chat engine. As part of the OHC mandate, we must retire any reliance on external 3rd-party services like Chatwoot, and instead bring these capabilities internally. This allows owners like Maya (baker) and Carlos (handyman) to unify SMS, WhatsApp, and Web Chat messages directly into a singular native workflow without maintaining multiple platforms.

  **Research Report**
  I have completed an audit of the `chatwoot/chatwoot` source code. Their architecture relies heavily on:
  - `Contact`: The unified customer identity.
  - `Inbox`: The reception point that links to a channel adapter.
  - `Conversation`: A threaded context between a `Contact` and an `Inbox`.
  - `Message`: The individual message payloads.
  - Channels (`Channel::Api`, `Channel::Email`, `Channel::Whatsapp`, etc.).

  We need to replicate this core relational schema using SeaORM in Rust within `src/server/ohc`, strictly enforcing our `tenant_id` based multi-tenancy requirements.

  **Design Doc**
  - **Architecture:** We will introduce a new module under `src/server/ohc/src/models/` (or similar location depending on OHC conventions) for:
    - `Contact` (tenant_id, name, email, phone, custom_attributes)
    - `Inbox` (tenant_id, name, channel_type, settings)
    - `Conversation` (tenant_id, inbox_id, contact_id, status)
    - `Message` (tenant_id, conversation_id, sender_type, content)
  - **Mobile UX Flow:** Mobile users will see a new "Inbox" view. Unread messages surface in the Work Triage section. The mobile view requires strict 375px responsive design.
  - **AI Agent Integration:** The Customer & Relationship Assistant agent will hook into `Conversation` creations and new `Message` events to automatically draft replies.

  **Implementation Prompt**
  1. Create SeaORM migration scripts to define the database tables for `contacts`, `inboxes`, `conversations`, and `messages`. Every table MUST include a `tenant_id` column for RLS (Row Level Security).
  2. Generate the SeaORM entity structs for these tables.
  3. Ensure that the entities are properly registered in the central Database setup.
  4. Write unit tests ensuring basic CRUD and verifying that `tenant_id` invariants hold.

  **Priority:** P0
  **Estimated Scope:** Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
