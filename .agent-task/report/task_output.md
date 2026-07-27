issue_title: "Platform Core: Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ### Problem Statement
  OneHumanCorp (OHC) is retiring its reliance on Chatwoot as a third-party omnichannel customer support system. Our small-business owner personas (Maya the baker, Carlos the handyman, etc.) require seamless, unified inboxes to handle customer inquiries from various channels (Instagram DMs, email, website widgets, WhatsApp, etc.). Operating Chatwoot as an external dependency creates unacceptable multi-tenant complexity, data privacy concerns, and architectural friction when integrating deeply with our AI Assistant departments (e.g., automated reply drafting, immediate operational syncing, and context sharing). We need a native Rust omnichannel chat system deeply embedded in `onehumancorp/mono` that guarantees Zero-Trust multi-tenant isolation, real-time sync, and native AI integration from day one.

  ### Research Report
  - **Chatwoot Source Audit:** We evaluated the core structural components of Chatwoot (`https://github.com/chatwoot/chatwoot`), including its data models (`Conversation`, `Message`, `Inbox`, `Contact`, `ChannelAdapter`), WebSocket real-time event distribution via ActionCable, and its webhook-based integrations.
  - **Competitor Systems:** We benchmarked this against Stripe's multi-tenant isolation patterns, Shopify Inbox, and Wix Inbox.
  - **Key Finding:** A native system implemented in Rust provides an order-of-magnitude reduction in latency for AI job queue triggers. By moving the chat data models into our own Bazel/PostgreSQL/Rust stack, we can enforce row-level security per tenant using our existing authentication and identity mechanisms (SPIFFE/SPIRE). This provides our AI agents direct database/queue access for replying without relying on HTTP webhook overhead.

  ### Design Doc
  **Architecture Overview**
  - **Data Models:** `Tenant`, `Inbox`, `ChannelAdapter` (e.g., Web, Instagram, Email), `Contact`, `Conversation`, `Message`.
  - **Storage:** PostgreSQL with `ENABLE ROW LEVEL SECURITY` and `tenant_id` on every table.
  - **Real-time:** Rust WebSocket server (using Tokio/Tungstenite or similar) replacing ActionCable.
  - **AI Agent Integration:** PostgreSQL `SKIP LOCKED` job queue for message processing, triggering the Customer Relationship Assistant to draft replies.

  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : holds
      CONVERSATION }o--|| CONTACT : involves
      MESSAGE }o--|| AI_DRAFT_JOB : triggers

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string provider_type
          json credentials
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
          text content
          string sender_type
      }
  ```

  **Mobile UX Flow (375px First)**
  - **Unified Inbox Screen:** Clean, Ubiquiti UniFi-style list of active conversations. Each list item shows a preview snippet, contact name, channel icon (e.g., Instagram), and a status indicator (e.g., "AI Draft Ready").
  - **Conversation Screen:** Native feel with edge-to-edge chat bubbles. At the bottom, standard input field or a one-tap "Approve AI Reply" button.
  - **Translucent UI:** macOS-style Translucent Glass materials on the bottom navigation bar and top action headers.

  **AI Agent Integration Points**
  - **Work Triage:** Incoming messages instantly hit the AI Job Queue. The Triage Agent tags urgency and updates the unified inbox status.
  - **Customer Relationship Assistant:** Analyzes message history and business context, then prepares a drafted reply invisibly in the background. The UI surfaces "Draft ready for review" to the owner.

  ### Implementation Prompt
  Implement a native Rust multi-tenant omnichannel chat engine inside `onehumancorp/mono`.

  **Requirements:**
  1. Define the core data schemas (`Inbox`, `ChannelAdapter`, `Conversation`, `Message`, `Contact`) in PostgreSQL, ensuring `tenant_id` is present on all tables with Row Level Security (RLS) enabled.
  2. Implement the backend Rust microservice (using our internal gRPC/REST framework) to handle CRUD operations for these models.
  3. Implement a WebSocket gateway for real-time message delivery to connected clients.
  4. Create the Flutter PWA/Mobile unified inbox UI with 375px responsive constraints, employing the OHC Premium Token library with translucent glass design.
  5. Ensure AI reply drafts are integrated via the PostgreSQL `SKIP LOCKED` job queue pattern.

  *CUJ:* An owner (e.g., Maya) opens her app, sees a new message in her unified inbox from a customer on the web widget, taps the conversation, sees a helpful AI-drafted reply, and hits "Approve & Send".

  *Acceptance Criteria:*
  - No external Chatwoot dependencies.
  - E2E Playwright test verifies a message sent to the inbox appears in the UI and a reply can be successfully sent back.
  - Mobile layout verified visually at 375px.

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
