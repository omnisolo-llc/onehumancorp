issue_title: "Implement The Ambassador Agent (Omnichannel Draft/Approval Workflow)"
issue_description: |
  # The Ambassador Agent (Omnichannel Customer Success)

  ## Problem Statement
  Small business owners like Maya the Baker receive customer inquiries across multiple unlinked channels (Instagram DMs, WhatsApp, SMS, email). Managing these manually leads to missed messages, slow response times, and lost sales. Traditional "unified inboxes" simply aggregate messages without context. They require manual typing, often without customer history or product catalog context. The owner needs an AI agent that proactively drafts contextual responses across all channels, presenting an "Action Required: Approve Reply" card in their mobile feed.

  ## Research Report
  - **Market Landscape**: Existing tools (Shopify Inbox, Wix Inbox) are glorified aggregators.
  - **Pain Points**: SMBs spend hours replying to repetitive inquiries (availability, pricing, order status).
  - **Solution Strategy**: Implement "The Ambassador" agent. It should intercept incoming messages via the Omnichannel Gateway, resolve the customer identity, use RAG against the catalog/inventory, and generate a draft response pushed to the `agent_feed` for the owner to 1-tap approve.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Omnichannel Gateway] --> B[Identity Resolution]
      B --> C[The Ambassador Agent]
      C -->|Query| D[Inventory & Customer DB]
      C -->|Draft Reply| E[Agent Feed (Action Required)]
      E -->|User Approves| F[Action Router]
      F --> G[Dispatch Reply via Gateway]
  ```

  ### Mobile UX Flow (375px First)
  - The `agent_feed` must surface a `MobileAgentFeedItem` representing the drafted reply.
  - The card must display the original message context and the AI-generated draft.
  - Primary CTA: "Approve". Secondary CTA: "Edit".
  - Must conform to OHC Premium Glassmorphism standards on mobile.

  ### AI Agent Integration Points
  - **Trigger**: `tenant.omnichannel.message.received` event triggers `message_triage` job.
  - **Agent Action**: The Ambassador agent reads the `omni_inbox_messages`, formulates a draft, and creates an `agent_feed` item with `feature_type = "ambassador_reply"`.
  - **Approval**: When the user approves the feed item via the mobile UI, `action_router.rs` invokes `inbox.rs:handle_inbox_action`, which updates the message status and optionally dispatches (e.g., via Twilio for SMS/WhatsApp).

  ## Implementation Prompt
  - **Goal**: Complete the end-to-end "Ambassador" workflow. A simulated incoming message must result in an `agent_feed` item being created containing a drafted reply. Approving that feed item must trigger the `handle_inbox_action` to "send" the reply.
  - **CUJ**: An online customer sends an SMS inquiring about a product. The Ambassador agent drafts a reply. The owner sees the draft in their mobile feed, clicks "Approve", and the system marks it as replied and dispatches it.
  - **Acceptance Criteria**:
    1. The Ambassador agent logic properly handles incoming omnichannel messages and generates an `agent_feed` draft.
    2. The approval flow correctly routes to `handle_inbox_action` and updates the database state.
    3. Include automated Playwright E2E tests verifying the 375px mobile UI approval flow.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
