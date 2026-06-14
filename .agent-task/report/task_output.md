issue_title: "Implement Agentic Omnichannel Inbox & Omnichannel Customer Memory"
issue_description: |
  ## 1. Title
  Implement Agentic Omnichannel Inbox & Omnichannel Customer Memory

  ## 2. Problem Statement
  Small business owners (e.g., Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## 3. Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## 4. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
  ```

  ### Data Model (PostgreSQL - Multi-Tenant)
  - `CustomerIdentity`: Links disparate identifiers (phone number, email, IG handle) to a single `customer_id` under a `tenant_id` (RLS enforced).
  - `OmnichannelMessage`: Stores the raw message, channel source, timestamp, and linked `customer_id`.
  - `AgentActionDraft`: Stores the LLM-generated proposed response, context used, and state (pending, approved, discarded).

  ### Mobile UX Flow (375px)
  1. **Notification/Feed:** Maya receives a push notification and sees an Action Card at the top of her OHC feed: "Draft Reply: Instagram DM from @cakefan."
  2. **Context View:** Expanding the card shows the customer's message, a summary of their past orders (e.g., "Ordered a vegan cake last month"), and the AI's drafted response.
  3. **Action:** Maya can tap "Approve & Send", "Edit" (opens native keyboard to tweak the message), or "Discard".
  4. **Post-Action:** The card transitions to a success state and disappears from the priority feed, executing the send via the Omnichannel Dispatcher.

  ### AI Agent Integration
  - **The Ambassador (Customer Success Agent):** Subscribes to the event mesh for new incoming messages. It retrieves the unified customer history and current business context (e.g., inventory availability for requested items) to craft a highly personalized and accurate draft reply.

  ## 5. Implementation Prompt
  **Feature Name:** Agentic Omnichannel Inbox
  **Target Persona:** Maya the Baker (needs to quickly reply to IG DMs while baking)
  **Task:** Implement the unified inbox capability that not only aggregates messages from different channels but proactively drafts responses using the customer's historical context.
  **Acceptance Criteria:**
  - Create the necessary database tables (ensure `tenant_id` RLS).
  - Implement the `Customer Identity Resolution Engine` to link different channel IDs to a single customer.
  - Integrate "The Ambassador" agent to automatically generate an `AgentActionDraft` upon receiving a new message.
  - Build the Mobile UI (375px optimized) displaying the "Action Required" card with the drafted response, context summary, and Approve/Edit/Discard actions.
  - Ensure end-to-end functionality is covered by Playwright tests simulating an incoming webhook, agent draft creation, and user approval in the UI.

  ## 6. Priority & Scope
  **Priority:** P0 (Critical for differentiation)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []