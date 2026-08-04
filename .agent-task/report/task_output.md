issue_title: "Native Rust Omnichannel Unified Inbox (Chatwoot Replacement)"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context and require manual typing. OHC requires an AI-first, proactive customer success assistant ("The Ambassador") combined with a high-performance, native Rust unified inbox that entirely replaces the retired external Chatwoot dependency. The owner should be prompted to approve pre-drafted context-aware replies.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** Analyzed the `chatwoot/chatwoot` source repository. Key architectural components identified for replication in native Rust include:
    - **Models:** `conversation.rb`, `message.rb`, `contact.rb`, `inbox.rb`, `channel/*` (adapters for IG, WhatsApp, Email, etc.), `webhook.rb`.
    - **Core Mechanics:** Multi-tenant row-level isolation per account, polymorphic channel associations, WebSocket-driven real-time event broadcasting, and robust webhook processing for incoming messages.
  - **Shopify / Wix Inbox:** Basic aggregation, lacking deep context or autonomous, predictive drafting based on omnichannel history.
  - **OHC Opportunity:** A native Rust implementation integrated directly into OHC's mono repo eliminates the Chatwoot external dependency, ensures strict `tenant_id` RLS (Row Level Security) isolation at the Postgres level, and connects seamlessly to the OHC Event Mesh to trigger The Ambassador AI agent instantly upon message receipt.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Rust Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[(PostgreSQL: Unified Customer Graph DB)]
      E --> G[Event Mesh / Redis]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Flutter Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Rust Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top priority card displays "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified inbox view. Top half displays rich customer context (e.g., past purchases). Bottom half displays the AI-drafted reply.
  - **Action:** Prominent primary button "Send Draft" and a secondary button "Edit". Native mobile keyboard integration.
  - **Visual Design:** OHC Premium Token library, translucent materials (macOS/Ubiquiti style), strong spacing, clear status tokens.

  ### AI Agent Integration Points
  - **The Ambassador (Customer Success Agent):** Subscribes to the Event Mesh. Uses RAG against tenant's product catalog and specific customer history to draft highly personalized replies.
  - **Operations Coordination:** If a message implies an order change, The Ambassador coordinates with Operations agents to verify inventory/calendar before drafting.

  ### Key Design Decisions
  - **Native Rust Implementation:** Complete replacement of Chatwoot with a bespoke Rust microservice/crate structure (`ohc:chat` or similar), implementing necessary WebSockets and Channel Adapters.
  - **Proactive Drafting:** The AI drafts the response before the user opens the app, moving from "read-reply" to "read-approve".
  - **Zero-Trust Multi-Tenancy:** Strict Postgres RLS enforced on all chat tables (`conversations`, `messages`, `contacts`).

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram, I open the OHC app to find a perfectly accurate response already drafted by AI. I tap one button to send it, taking 2 seconds. All chat infrastructure is now native to OHC.
  **CUJ & Acceptance Criteria:**
  1. Implement the database schema (PostgreSQL) for `conversations`, `messages`, `contacts`, and `inboxes` with `tenant_id` RLS.
  2. Build a native Rust Omnichannel Gateway service that can receive simulated webhooks (e.g., from Instagram/WhatsApp).
  3. The Identity Engine matches the incoming identifier to an existing contact.
  4. The Ambassador Agent is triggered, queries context, generates a draft, and persists it to the `ActionRequiredQueue`.
  5. The Flutter frontend (mobile 375px) displays the drafted message card; tapping "Approve" dispatches the message back out.
  6. Provide 100% unit test coverage for the Rust service and at least five Playwright E2E tests verifying the complete flow from mocked webhook receipt to user approval in the UI. ZERO mock data in the UI; all data must flow through the real stack.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat-system]
assignees: []