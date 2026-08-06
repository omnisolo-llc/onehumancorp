issue_title: "Implement Native Rust Omnichannel Chat Engine"
issue_description: |
  ## Issue Brief: Native Rust Omnichannel Chat System

  **Problem Statement:**
  OneHumanCorp (OHC) has retired the external Chatwoot dependency to build a native, high-performance omnichannel inbox system in Rust. The current system lacks the core data structures and message routing required to handle multiple social channels (WhatsApp, Instagram DMs, Email, SMS) natively within a multi-tenant SaaS environment. Owners like Maya (the baker) and Carlos (the handyman) need a unified inbox that tracks customer conversations across various channels seamlessly on their 375px mobile screens, without jumping between different apps or external third-party services.

  **Research Report:**
  Based on an audit of the `chatwoot/chatwoot` source code:
  - Chatwoot's data model heavily relies on polymorphic associations for senders (users vs. contacts) and channels.
  - Core entities identified: `Inbox`, `Message`, `Conversation`, `Contact`, `ChannelAdapter` (e.g. `Channel::Whatsapp`, `Channel::Email`).
  - Chatwoot was built in Ruby on Rails, which lacks the memory safety and concurrency performance of our Rust stack.
  - By porting these concepts to a native Rust implementation, we eliminate a significant infrastructure footprint and improve Zero-Trust isolation via our SPIFFE/SPIRE architecture and Postgres row-level security.
  - Shopify Sidekick and Wix both use native inbox experiences to keep owners in the primary UI.

  **Design Doc:**

  *Architecture Overview:*
  The new Native Omnichannel Chat System will be located in `src/server/integrations/omnichannel/`.

  ```mermaid
  erDiagram
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX ||--|| CHANNEL_ADAPTER : uses
  ```

  *Data Model & Invariants:*
  - `Inbox`: Configuration for a specific channel integration (e.g., a specific WhatsApp number or Instagram account). Enforces `tenant_id` for multi-tenancy.
  - `Message`: Represents an individual message within a conversation. Must include `content`, `content_type`, `sender_type` (operator, contact, or agent), and `tenant_id`.
  - `ChannelAdapter`: A trait/interface for different integrations to implement sending and receiving messages.
  - Multi-tenant isolation is strict: all queries must filter by `tenant_id`.

  *Mobile UX Flow (375px):*
  - The inbox list uses a standard UniFi-style card layout.
  - Each conversation is tap-target optimized (44x44px minimum touch targets).
  - Messages render with macOS-style Translucent Glass bubbles.
  - Offline-first: Sent messages show a "pending" state instantly and retry in the background via the Rust AI job queue.

  *AI Agent Integration:*
  - "Customer Success - The Ambassador" AI agent will subscribe to the `omnichannel` message bus.
  - When a new message arrives, the system attempts to auto-reply using the `tenant`-scoped memory. If the AI cannot resolve the request (e.g., custom cake request for Maya), it flags the conversation for human review.

  **Implementation Prompt:**
  As an Implementer agent, your task is to build out the Rust backend data models and database migrations for the Native Omnichannel Chat Engine.
  1. Create SeaORM entities for `Inbox`, `Message`, and `Conversation` in `src/server/integrations/omnichannel/`.
  2. Ensure every table has a `tenant_id` and Row-Level Security (RLS) is applied where applicable.
  3. Implement the `ChannelAdapter` trait to standardise webhook ingestion.
  4. Write unit tests ensuring 100% test coverage for the new structs and traits.
  5. The implementation must not include specific API routes yet, focus on the core data domain and tenant isolation first.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []