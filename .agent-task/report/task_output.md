issue_title: "Implement Native Rust Omnichannel Inbox & Unified Chat Architecture"
issue_description: |
  **Title**: Implement Native Rust Omnichannel Inbox & Unified Chat Architecture

  **Problem Statement**:
  Owners like Maya (baker) and Carlos (handyman) communicate with customers across multiple platforms—Instagram DMs, WhatsApp, SMS, and website chat. Jumping between apps causes missed leads and dropped context. They need one unified inbox inside OHC where every message arrives, AI agents can automatically draft responses, and action can be taken immediately, without setting up complex third-party services like Chatwoot.

  **Research Report**:
  - **Chatwoot Analysis**: We audited Chatwoot's source code (`https://github.com/chatwoot/chatwoot`), focusing on its omnichannel data models (`conversations`, `messages`, `inboxes`, `contacts`, `channel_adapters`) and real-time event architecture (WebSockets, webhooks). While robust, integrating it as an external dependency breaks OHC's zero-trust SPIFFE/SPIRE isolation, complicates tenant row-level security (RLS), and introduces latency when coordinating with OHC's internal AI triage agents.
  - **Competitor Approaches**:
    - **Shopify Inbox**: Unifies Apple Business Chat, Instagram, and web chat directly into the admin app, providing tight integration with the storefront.
    - **Wix Inbox**: Tightly coupled with their CRM and automation workflows.
  - **Recommendation**: Complete the retirement of external Chatwoot by building a native Rust-based omnichannel messaging system inside `onehumancorp/mono`. This system will feature real-time WebSocket support for web widgets, Meta webhooks for WhatsApp/Instagram, and strict PostgreSQL RLS for multi-tenant data isolation.

  **Design Doc**:

  *Architecture Diagram*:
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      CHANNEL ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string channel_type
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          string content
          string sender_type
      }
  ```

  *Mobile UX Flow (375px first)*:
  - **Unified Inbox Feed (Home)**: A bottom navigation tab for "Messages". Displays a scrollable list of recent conversations. Each card shows the user's avatar, name, preview text, and an indicator of the source channel (WhatsApp, Web Widget, IG).
  - **Conversation Thread**: Tapping a conversation opens the chat view.
    - Sticky header with Customer Name, Channel Icon, and Back Button.
    - Scrollable message thread with proper Chat bubble spacing.
    - Sticky bottom input area: Native keyboard integration, "AI Draft" button, Text input field, and a Send button.

  *AI Agent Integration Points*:
  - **Work Triage & Drafts**: When a customer asks a question (e.g., "Do you have vegan cakes?"), the OHC Work Triage agent parses the intent and pre-fills a draft response in the input box, tagged as an "AI Suggestion". The owner can seamlessly tap "Send" or manually edit the response.

  *Key Design Decisions*:
  - **Native Rust**: Ensures maximum performance and tight coupling with existing OHC infrastructure (Bazel, SPIFFE/SPIRE).
  - **PostgreSQL RLS**: Mandates `tenant_id` on all tables (`inboxes`, `conversations`, `messages`, etc.) to enforce strict multi-tenant isolation at the database level.

  **Implementation Prompt**:
  To the Implementer Agent:
  Build the foundational Native Rust Omnichannel Inbox architecture.
  - **CUJ**: A customer sends a message via WhatsApp, which is routed through a Meta webhook to our Rust backend. The backend creates a `Contact`, `Conversation`, and `Message` under the owner's `tenant_id`. The owner opens the OHC mobile app, navigates to the Inbox tab, sees the new message, and sends a reply which is routed back out to the WhatsApp API.
  - **Acceptance Criteria**:
    - PostgreSQL schemas are created for Inboxes, Channels, Conversations, Contacts, and Messages with proper RLS `tenant_id` policies.
    - Rust API endpoints for webhook ingestion and sending outbound messages are implemented.
    - UI Inbox feed view and Conversation thread view are built and responsive for a 375px mobile viewport, utilizing Translucent Glass styling.
    - WebSocket event broadcasting is established to notify connected mobile clients of new messages.
    - 100% Unit test and Playwright E2E test coverage for the messaging CUJ. ZERO mock data in the UI; all data must flow through the real application stack.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
