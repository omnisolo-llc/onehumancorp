issue_title: "Implement Native Rust Omnichannel Chat System (Replace Chatwoot)"
> Superseded architecture: Chatwoot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.

issue_description: |
  # Problem Statement
  Small business owners like Carlos (handyman) and Maya (baker) receive customer inquiries across unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages and slow response times. Traditional unified inboxes (Shopify Inbox, Wix Inbox) aggregate messages but lack context and require manual typing. OHC currently relies on Chatwoot (a 3rd party Ruby on Rails service), which limits our ability to seamlessly integrate our AI Agent Workflows ("The Ambassador") and enforce strict multi-tenant Data Modeling. We need to retire Chatwoot and build a native, high-performance omnichannel chat system in Rust within `onehumancorp/mono`.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** We audited Chatwoot's source code (Ruby on Rails). It heavily relies on complex schema structures for `conversations`, `messages`, `inboxes`, and `contacts` with many external dependencies.
  - **Shopify/Wix Inbox:** They lack proactive AI drafting based on a unified customer graph.
  - **OHC Opportunity:** By building natively in Rust, OHC can use an Event Mesh to instantly trigger the `Customer Success Agent (The Ambassador)`. When a message arrives, the system resolves the customer's identity, and the Agent proactively drafts a highly contextual reply (based on past purchases and interactions). The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM / WhatsApp / Email] -->|Webhook| B(Rust Omnichannel Gateway)
      B --> C{Customer Identity Resolution Engine}
      C -->|Lookup| D[(Unified Customer Graph DB - Tenant Isolated)]
      C --> E[Rust Event Mesh]
      E --> F[The Ambassador Agent]
      F -->|Query Context| D
      F -->|Draft Reply| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Rust Omnichannel Dispatcher]
      I --> A
  ```

  ### Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply.
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG against the tenant's product catalog and the customer's specific history to draft highly personalized replies.

  ### Key Design Decisions
  - **Native Rust Implementation:** Eliminates the dependency on external Chatwoot services. Enables tight integration with the rest of the OHC ecosystem.
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response before the user opens the app.
  - **Tenant Isolation:** Every entity (`Inbox`, `Contact`, `Conversation`, `Message`) must have a `tenant_id` and enforce strict row-level security or programmatic multi-tenancy.

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **CUJ & Acceptance Criteria:**
  1. Implement core Rust models and DB schema for `Inbox`, `Contact`, `Conversation`, and `Message`, ensuring strict multi-tenant isolation.
  2. Build a webhook ingestion gateway to receive simulated external messages and map them to conversations.
  3. Integrate the event trigger to queue "The Ambassador" agent to draft a reply when a new message is received.
  4. Ensure the drafted reply surfaces in the Action Required Queue for the tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
