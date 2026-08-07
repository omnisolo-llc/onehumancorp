issue_title: "Design and Implement Native Rust Omnichannel Chat System"
issue_description: |
  # Problem Statement
  OneHumanCorp previously relied on Chatwoot as an external third-party service for omnichannel customer support. This dependency introduced complexity, latency, and security concerns. We need to implement a native, high-performance, multi-tenant omnichannel chat engine in Rust inside `ohc-mono` to replace Chatwoot completely, providing a seamless experience for owners like Maya and Carlos.

  # Research Report
  Based on an audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`):
  - **Data Models:** Chatwoot uses core entities: `Account` (Tenant), `Inbox`, `Conversation`, `Message`, `Contact`, `Channel` (e.g., Whatsapp, Email, Twilio).
  - **Architecture:** It heavily utilizes WebSockets for real-time messaging and background workers for processing integrations/webhooks.
  - **Competitors:** Shopify Inbox and Wix Inbox aggregate messages but lack contextual agentic replies. A native OHC system allows deep integration with our "Teammate" AI (The Ambassador) to draft proactive contextual responses based on tenant's unified customer graph.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: Instagram, WhatsApp, Email] --> B[Omnichannel Gateway / Router]
      B --> C{Channel Adapters}
      C --> D[Message Ingestion Service]
      D --> E[(Unified Customer Graph & Postgres Database)]
      D --> F[Event Mesh / Redis PubSub]
      F --> G[The Ambassador Agent]
      G -->|Query Context| E
      G -->|Draft Reply| H[Action Required Queue]
      F --> I[WebSocket Server - Real-time UI Updates]
      I --> J[Mobile App Feed 375px]
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed:** Top card displays unified messages, e.g., "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping opens the chat. It shows the customer's purchase history and past interactions.
  - **Action:** Primary button to "Send Draft" (pre-written by AI) or "Edit" using the native mobile keyboard.
  - **Visual Design:** Clean macOS Translucent Glass styling, distinct bubbles for different channels, clear read/receipt indicators.

  ### AI Agent Integration Points
  - **The Ambassador:** Triggered by incoming messages. Uses RAG against the product catalog and customer history to generate automated responses or drafts for approval.
  - **The Manager:** Handles booking or inventory queries automatically if specified in tenant settings.

  ### Key Design Decisions
  - **Data Modeling:** Native Rust `sea-orm` models for `Inbox`, `Conversation`, `Message`, `Contact` linked to a specific `Tenant`.
  - **Real-time Engine:** Use `tokio-tungstenite` or Axum WebSockets connected to Redis PubSub for real-time delivery to the frontend PWA/Flutter app.
  - **Security:** Strict row-level security using `tenant_id` for multi-tenancy. SPIFFE/SPIRE for internal service authentication.

  # Implementation Prompt
  **User-Facing Outcome:** Business owners can view all customer communications (Instagram, WhatsApp, SMS) in one native interface without managing a separate tool. The Ambassador agent proactively drafts replies based on past orders, saving the owner time.
  **CUJ & Acceptance Criteria:**
  1. Implement Rust `sea-orm` entities for `Inbox`, `Conversation`, `Message`, and `Contact`.
  2. Create Axum REST API endpoints to manage inboxes and fetch conversations.
  3. Implement a WebSocket endpoint for real-time message broadcasting to clients.
  4. Write Playwright E2E tests simulating an incoming webhook, storing the message, and verifying it appears in the 375px mobile UI feed.
  5. Ensure 100% unit test coverage for new Rust code.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, backend]
assignees: []
