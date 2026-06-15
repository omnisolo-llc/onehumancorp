issue_title: "Build the Omni-Channel AI Inbox Architecture for The Ambassador Agent"
issue_description: |
  # Omni-Channel AI Inbox Architecture for The Ambassador Agent

  ## Target Persona: Maya (Home Baker)

  ## Problem Statement
  Small business owners (e.g., Maya the Home Baker) receive customer inquiries across multiple platforms—Instagram DMs, WhatsApp, SMS, Email, and Facebook Messenger. Keeping track of these messages, responding promptly, and ensuring consistency across channels is overwhelming. The current disconnected communication silos prevent AI agents (like the Customer Success and Operations agents) from providing unified, context-aware support. If a customer asks "Do you do vegan cakes?" on Instagram, the AI needs to reply seamlessly without the business owner having to manually switch contexts. Traditional platforms (Shopify Inbox, Wix Inbox) simply aggregate messages without context, requiring manual responses that lack full omnichannel customer memory. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  ## Research Report
  Our research across the e-commerce platform landscape reveals two distinct categories:
  - **Shopify Inbox:** Good for web chat and basic Instagram, but relies heavily on manual responses or rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Channels
          IG[Instagram DM]
          WA[WhatsApp]
          FB[FB Messenger]
          SMS[SMS / Twilio]
          Email[Email / Resend]
      end

      subgraph OHC Integration Layer
          MetaWebhook[Meta Webhook Handler]
          TwilioWebhook[Twilio Webhook Handler]
          EmailWebhook[Email Webhook Handler]
      end

      subgraph Core
          MessageBus[NATS / Redis PubSub]
          UnifiedInboxDB[(Postgres Unified Inbox)]
          AgentQueue[AI Job Queue]
      end

      subgraph AI Agents
          CSAgent[Customer Success Agent - The Ambassador]
          OpsAgent[Operations Agent]
      end

      IG --> MetaWebhook
      WA --> MetaWebhook
      FB --> MetaWebhook
      SMS --> TwilioWebhook
      Email --> EmailWebhook

      MetaWebhook --> MessageBus
      TwilioWebhook --> MessageBus
      EmailWebhook --> MessageBus

      MessageBus --> UnifiedInboxDB
      MessageBus --> AgentQueue

      AgentQueue --> CSAgent
      AgentQueue --> OpsAgent

      CSAgent -.-> |Drafts/Sends Reply| UnifiedInboxDB
      OpsAgent -.-> |Takes Action e.g. Books| UnifiedInboxDB
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Screen (375px):** An "Inbox" tab with a notification badge on the primary feed.
  - **Inbox View:** A consolidated list of conversations. Each item shows the customer's name, the latest message snippet, and an icon indicating the channel (Instagram, WhatsApp, etc.).
  - **Conversation View:** A chat interface (Translucent Glassmorphism style).
      - Messages from the customer appear on the left.
      - AI-drafted replies appear at the bottom with a prominent "Approve & Send" button or an "Edit" button.
      - If the AI auto-replied (based on confidence threshold), it shows "Sent by AI" below the message bubble.
  - **Context Panel (Drawer/Swipe):** Swiping left reveals customer context: previous orders, upcoming bookings, and notes, powered by the unified identity graph.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message intent is transactional (e.g., "I need to change my booking to 3 PM", "Where is my order?"), this agent handles the state change and drafts the confirmation reply.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points (e.g., "Sarah's last order was #1234").

  ## Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier (e.g., social handle) to an existing customer record in the database.
  3. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places it in the `ActionRequiredQueue` for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed (375px), taps "Approve," and the system dispatches the message back to the mocked external channel.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
