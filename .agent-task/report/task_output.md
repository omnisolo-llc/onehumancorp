issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Chat Engine

  ## Problem Statement
  OHC currently relies on third-party integrations for omnichannel customer support, which breaks our core promise of a unified, native assistant. Maya (baker) and Carlos (handyman) don't want to log into another portal or pay another subscription to see their Instagram, WhatsApp, and website messages. They need a single, fast, zero-configuration inbox right inside the OHC app that their AI assistant can access directly to draft replies and manage context.

  ## Research Report
  - **Market Context**: Shopify Inbox, Wix Inbox, and GoDaddy Conversations all provide native, unified messaging. Chatwoot provides an open-source model for omnichannel (Channels, Inboxes, Conversations, Contacts, Messages) but relies on Ruby on Rails, Sidekiq, and external Redis, which is too heavy for our standalone/mobile-first deployments.
  - **Codebase Findings**: OHC uses Rust for the backend (`onehumancorp/mono`). We need a native Rust implementation of the core omnichannel messaging engine as Chatwoot is being 100% retired.
  - **Competitor Benchmarking (Chatwoot)**:
    - **Models**: `Account`, `Inbox`, `Channel::*`, `Conversation`, `Message`, `Contact`.
    - **Real-time**: WebSockets for typing indicators and message delivery.
    - **Architecture**: Multi-tenant by default.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CONVERSATION : contains
      INBOX ||--o| CHANNEL_ADAPTER : uses
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          string name
          uuid tenant_id
      }
      CHANNEL_ADAPTER {
          string type
          json config
      }
      CONVERSATION {
          uuid id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          text content
          string sender_type
      }
  ```

  ### UI Wireframes & 375px Flow
  1. **Home Triage**: Bottom nav "Inbox" tab with unread badge.
  2. **Inbox List (375px)**: Unified list of conversations. Each row shows contact avatar, last message, time, and channel icon (Instagram, WhatsApp, Web).
  3. **Conversation View (375px)**: Chat bubbles. Bottom sticky input field. "AI Draft Reply" floating action button.
  4. **Contact Context**: Swiping left on the chat reveals contact details, past orders, and tags.

  ### AI Agent Integration
  - **Customer Assistant Agent**: Subscribes to new message events. Automatically drafts replies based on tenant context (e.g., checking delivery dates) and saves them as `pending` messages.
  - **Memory/Context**: AI reads the last 10 messages from the `CONVERSATION` model to maintain context without needing separate external API calls to a third-party chat service.

  ### Key Design Decisions
  - **Native Rust**: High performance, zero extra infrastructure overhead.
  - **Unified Inbox Model**: All channel adapters (WhatsApp, IG, Web) normalize into a single `Message` interface for the frontend.
  - **Multi-Tenant Row-Level Security**: Enforced via PostgreSQL `tenant_id` on all tables to prevent cross-bleed.

  ## Implementation Prompt
  **User-Facing Outcome**: Build the core database schema, Rust API endpoints, and a basic 375px-friendly mobile UI (in Tauri/React) for a unified inbox. Maya should be able to open "Inbox", see a test message, and reply to it, all natively within OHC.
  **CUJ**:
  1. Login as business owner.
  2. Navigate to "Inbox" tab.
  3. See a list of active conversations.
  4. Open one, type a reply, and hit send (persisted locally).
  **Acceptance Criteria**:
  - Rust models and DB migrations for Inbox, Conversation, Message, and Contact.
  - API endpoints for fetching and sending messages.
  - UI components reflecting the macOS translucent glass style, perfectly responsive at 375px.
  - 100% unit test coverage for new Rust code and 1 E2E Playwright test verifying the inbox flow.

  ## Priority
  P0 (critical)

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []