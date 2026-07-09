issue_title: "[Research] The Ambassador Agent: Omnichannel Inbox to Agent Feed Architecture"
issue_description: |
  # OHC Agent Solutions: The Ambassador Agent Issue Brief

  ## 1. Problem Statement
  Solopreneurs like Maya (the Home Baker) and Carlos (the Handyman) often lose potential sales because they cannot monitor social media direct messages (Instagram, WhatsApp, Messenger) while running their daily operations. Existing omnichannel inbox solutions are passive—they require the business owner to manually open an app, read the message, look up their schedule or inventory, and type a response. Automation tools (like ManyChat) require complex visual builders that are too technical for our target audience.

  ## 2. Research Report
  - **Market Context**: Customers expect responses within minutes. Missing a DM often means losing a sale. Shopify offers an Inbox app, but it is primarily a manual tool. Wix has a unified inbox, but it lacks proactive, business-aware AI drafting.
  - **The OHC Opportunity**: OHC can differentiate by acting as an *active* participant. Instead of just displaying the DM, OHC's "Ambassador Agent" intercepts the message, checks real-time business context (inventory from PostgreSQL, availability from the Booking calendar), and drafts a highly accurate reply.
  - **The Mobile-First Gap**: Non-technical owners operate from their phones (375px screens). A complex unified inbox is overwhelming. The solution is the "Agent Feed"—a single stream of actionable cards.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant IG as Instagram Graph API
      participant Webhook as OHC Webhook Ingestion (Pg SKIP LOCKED)
      participant LLM as Agent Layer (Intent + RAG)
      participant Feed as Agent Feed Service
      participant Mobile as OHC Mobile UI (375px)

      Customer->>IG: "Do you have vegan cakes?"
      IG->>Webhook: Webhook Event (Message)
      Webhook->>LLM: Process Event
      LLM->>LLM: 1. Classify Intent (Availability)
      LLM->>LLM: 2. RAG Context (Check Inventory DB)
      LLM->>LLM: 3. Draft Response
      LLM->>Feed: Create AgentFeedItem (Status: PENDING_APPROVAL)
      Feed-->>Mobile: Push Notification / WS Update
      Mobile->>Mobile: Display 375px Action Card
      Mobile->>Feed: Owner taps "Approve & Send"
      Feed->>IG: Send Response via Graph API
      IG->>Customer: "Yes, we have 2 vegan cakes left!"
  ```

  ### Data Model & Integration Points
  - **Webhook Queue**: A new or existing table (e.g., `omnichannel_events`) using the PostgreSQL `SKIP LOCKED` pattern to ensure reliable, deduplicated processing of incoming DMs.
  - **AgentFeedItem Schema**: Leverage the existing `AgentFeedItem` in `agent_feed_repo.rs`. The `context_payload` will store the original DM and customer info. The `proposed_action` will store the drafted reply and the integration target (e.g., `instagram`).
  - **RAG Integration**: The LLM processor must query the `inventory` and `booking` domains to gather context before drafting the reply.

  ### Mobile UX Flow (375px)
  1. **Notification**: Maya receives a push notification: "New Instagram DM. Draft reply ready."
  2. **Agent Feed**: She opens the app. The top card in the Unified Agent Feed shows the customer's message ("Do you have vegan cakes?") and the AI-drafted reply ("Yes, we have vegan cakes available! Would you like to reserve one?").
  3. **Action**: The card has two large (44x44px min) buttons: "Approve & Send" and "Edit Draft".
  4. **Execution**: Tapping "Approve & Send" immediately dispatches the message via the Instagram Graph API.

  ## 4. Implementation Prompt
  **Feature Name**: The Ambassador Agent - Omnichannel DM Integration

  **Target Persona**: Maya the Home Baker

  **User-Facing Outcome**: When a customer sends a DM on Instagram, Maya receives a drafted response in her OHC Agent Feed that she can approve with one tap, without opening Instagram.

  **Critical User Journey (CUJ)**:
  1. System receives a simulated Instagram DM payload via webhook.
  2. The system queues the event, extracts intent, and drafts a reply based on mock inventory.
  3. A new `AgentFeedItem` appears in the UI dashboard feed.
  4. The owner taps "Send Draft" (Approve) on the UI card.
  5. The system records the approval and simulates sending the response back to Instagram.

  **Acceptance Criteria**:
  - Implement a robust webhook ingestion endpoint that securely receives external payloads.
  - Integrate this endpoint with the `AgentFeedService` to generate actionable drafts.
  - Ensure the Mobile UI correctly renders the DM action card and handles the approval action.
  - Include full unit and E2E Playwright tests covering the flow from webhook ingestion to UI approval.

  **Note to Implementer**: Do not prescribe specific database schemas for the webhook queue here; design it for robust concurrency (e.g., `SKIP LOCKED`). Ensure all UI changes strictly follow the 375px mobile-first design tokens.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []