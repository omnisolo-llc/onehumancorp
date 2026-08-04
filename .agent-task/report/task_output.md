issue_title: "Build Native Rust Omnichannel Chat System"
issue_description: |
  # Problem Statement

  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" simply aggregate messages without context.

  # Research Report

  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic auto-replies.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone".
  - **The external chat service:** OHC previously relied on The external chat service for an omnichannel inbox, but The external chat service as an external dependency is 100% RETIRED. OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine built natively in Rust.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph, and proactively drafts a complete, accurate response.

  # Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway - Rust)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher - Rust]
      K --> A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)

  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context. Bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points

  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.

  ### Key Design Decisions

  - **Native Rust Implementation:** Replace external The external chat service dependency with a custom Rust omnichannel chat system inside `onehumancorp/mono`.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response _before_ the user opens the app.
  - **Tenant Isolation:** Enforce Row Level Security (RLS) via `tenant_id`.

  # Implementation Prompt

  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it.

  **CUJ & Acceptance Criteria:**
  1. Implement a custom native Rust Omnichannel Gateway in `src/server/integrations/chat/` to replace external The external chat service dependencies.
  2. The system MUST implement core chat models (Conversation, Message, Inbox) similar to The external chat service, mapped to the Rust PostgreSQL database with strict row-level security (`tenant_id`).
  3. The system MUST provide an API endpoint to receive external webhooks (e.g., simulating WhatsApp or Instagram) and process them.
  4. The system MUST trigger an AI Agent to draft a response upon receiving a new message.
  5. The draft must be surfaced to the UI, allowing the owner to "Approve" and send the drafted response.
  6. Provide full unit test coverage and Playwright E2E tests validating the end-to-end flow from receiving a webhook to sending the approved reply.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
