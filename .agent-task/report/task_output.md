issue_title: "Native Rust Omnichannel Chat System: Architecting The Ambassador"
issue_description: |
  # Native Rust Omnichannel Chat System: Architecting The Ambassador

  ## Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur. Furthermore, OHC's mandate dictates that Chatwoot as an external service must be 100% retired in favor of a high-performance, multi-tenant native Rust omnichannel chat system inside `onehumancorp/mono`.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox / Wix Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. No proactive AI drafting with full graph context.
  - **Zendesk / Intercom:** Enterprise-grade, complex, and expensive for SMBs.
  - **Chatwoot Source Audit:** We have audited Chatwoot's source code (`https://github.com/chatwoot/chatwoot`). Its architecture uses Rails controllers, ActionCable for WebSockets, Sidekiq for background jobs, and PostgreSQL for data modeling (Accounts, Inboxes, Contacts, Conversations, Messages).
  - **OHC Native Rust Vision:** We must replicate Chatwoot's core omnichannel architecture in Rust using Axum/Tonic, PostgreSQL, and Valkey/Redis. Crucially, we integrate this directly with "The Ambassador" (Customer Success Agent). Instead of merely routing messages to a human agent, the system automatically builds a unified customer identity graph, queries past purchases via RAG, and drafts a contextual response.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Omnichannel Gateway - Axum)
      B --> C{Identity Resolution Engine}
      C -->|Read/Write| D[(PostgreSQL: Unified Customer Graph & Inbox DB)]
      C --> E[Message Event Bus - Redis Pub/Sub]
      E --> F[The Ambassador Agent Service]
      E --> G[WebSocket Server for Live UI]
      F -->|RAG Query| D
      F -->|Draft Reply| H[Action Required Queue - Redis/K8s Job]
      H --> I[Mobile App Feed 375px]
      I -->|1-Tap Approve| J[Omnichannel Dispatcher]
      J --> A
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** The user's primary view is a feed of actionable cards. The top card shows: "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view.
      - **Top Half:** Customer context summary (e.g., "Sarah bought a vegan cake 2 months ago").
      - **Bottom Half:** The AI-drafted reply (e.g., "Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** Prominent primary button "Approve & Send" and a secondary "Edit".
  - **Visual Design:** Utilizes OHC Premium Tokens (macOS Translucent Glass materials, large touch targets ≥ 44x44px, clean typography).

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Triggered by the Message Event Bus. It performs a RAG query against the tenant's product catalog and the customer's specific interaction/purchase history to draft highly personalized replies.
  - **The Manager (Operations Agent):** If the incoming message implies an order change or booking request, The Manager agent is concurrently invoked to verify inventory or calendar availability before The Ambassador drafts the final reply.

  ### Key Design Decisions (Native Rust Focus)
  - **Retirement of Chatwoot:** 100% replacement of Chatwoot dependencies with a native Rust backend (Axum for webhooks/API, Tokio for async processing).
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial for linking a social handle to an email address if they've purchased before, creating a single `Contact` entity per tenant.

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the native Rust Omnichannel Gateway.
  2. The Identity Resolution Engine correctly matches the incoming identifier to an existing customer record in the database.
  3. The Ambassador Agent is triggered, successfully querying the customer's past orders.
  4. The Agent generates a draft reply and places it in the Action Required Queue.
  5. **Verification:** A Playwright E2E test must log in as the owner, verify the drafted message card appears on the mobile-sized (375px) feed, tap "Approve", and verify the system dispatches the message back to the mocked external channel.
  6. **100% Unit Test Coverage:** The new Rust services and database models must have 100% test coverage.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
