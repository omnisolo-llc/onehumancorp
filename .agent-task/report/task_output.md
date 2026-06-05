issue_title: "[Architecture] Unified Omnichannel Inbox & Autonomous Auto-Reply Engine"
issue_description: |
  # Unified Omnichannel Inbox & Autonomous Auto-Reply Engine

  ## Problem Statement
  SMB owners like Maya (the baker) and Carlos (the handyman) receive customer inquiries across multiple scattered channels: Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Keeping track of these messages is overwhelming and context-switching leads to lost sales. Maya misses custom cake orders because she is busy baking and cannot reply to Instagram DMs immediately. Carlos gets scattered requests and struggles to consolidate them into cohesive jobs. They need a single, unified inbox that not only aggregates messages but autonomously drafts and sends contextual replies (e.g., answering "do you do vegan cakes?" or "how much for a plumbing fix?") while they sleep or work, seamlessly moving prospects toward booking or checkout.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Shopify Inbox aggregates chat and some social channels, and Sidekick offers some AI assistance to the merchant, but true autonomous auto-reply to DMs requires piecing together third-party apps like ManyChat, which are too complex for non-technical users.
  - **Wix / Squarespace:** Offer basic inbox features but very limited AI reply capabilities, mostly reliant on static FAQ decision trees and simple bots rather than context-aware LLMs.
  - **GoDaddy:** Provides a basic unified conversations inbox, but lacks deep AI integration for autonomous sales and support.

  **OHC Opportunity:**
  By embedding an omnichannel inbox natively with the "Customer Success" and "Sales & Acquisition" AI agents, OHC can provide zero-click automations out of the box. The agents maintain long-term memory of the business (e.g., Maya's vegan cake policy, Carlos's pricing) and autonomously handle tier-1 support and lead qualification. They convert DMs directly into Stripe checkout sessions or calendar bookings without the owner's intervention, giving OHC a massive differentiator in the market.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      subgraph External_Channels
          IG[Instagram DM]
          WA[WhatsApp]
          SMS[SMS]
          Web[Web Chat]
      end

      IG -->|Webhook| Gateway[Channel Ingestion Gateway]
      WA -->|Webhook| Gateway
      SMS -->|Webhook| Gateway
      Web -->|WebSocket| Gateway

      Gateway --> MsgBus[Event Bus / Queue]
      MsgBus --> InboxStore[(Unified Inbox DB)]
      MsgBus --> AgentRouter[AI Agent Router]

      AgentRouter -->|Customer Intent| CSAgent[Customer Success Agent]
      AgentRouter -->|Sales Intent| SalesAgent[Sales & Acquisition Agent]

      CSAgent -->|Query Context| Memory[(pgvector Memory)]
      SalesAgent -->|Generate Checkout| Stripe[Stripe API]

      CSAgent --> AutoReply[Auto-Reply Engine]
      SalesAgent --> AutoReply

      AutoReply --> Gateway
      Gateway --> External_Channels

      InboxStore --> MobileUI[Flutter Mobile App - 375px]
  ```

  ### UI Wireframes (375px mobile-first)
  - **Screen 1: Unified Feed.** A vertical list of ongoing conversations. Each row features a customer avatar, a channel icon badge (IG, WA, Web), a snippet of the latest message, and unread indicators. Auto-replied messages have a subtle, premium translucent "AI handled" indicator.
  - **Screen 2: Conversation View.** A clean chat interface showing customer messages and AI replies. A sticky bottom bar allows the owner to seamlessly take over the chat manually. AI drafts (when confidence is low) are shown in a distinct "glassmorphism" bubble with an "Approve & Send" button.

  ### Mobile UX Flow
  1. Owner opens the "Inbox" tab from the main navigation.
  2. Taps a conversation marked with the "AI Handled" badge.
  3. Reviews the AI's interaction, which successfully answered a query and sent a custom checkout link.
  4. If a message requires owner input, it bubbles to the top of the feed with a red "Needs Attention" dot. The owner can tap to review the AI's suggested draft, edit it using the native keyboard, and send.

  ### AI Agent Integration Points
  - **Customer Success Agent:** Handles FAQs, store policies, and order status updates.
  - **Sales & Acquisition Agent:** Generates quotes, schedules bookings, and provides Stripe payment links.
  - **Memory Layer:** Uses pgvector embeddings of past interactions, the product catalog, and store policies to provide hyper-contextual answers.

  ### Key Design Decisions
  - **Unified Data Model:** Normalize all incoming messages into a standard `Conversation` and `Message` schema regardless of the source channel, simplifying rendering on the mobile UI.
  - **Confidence-Based Auto-Reply:** The LLM generates a confidence score for its reply. High confidence = auto-send. Low confidence = create draft and flag for owner review.
  - **Seamless Human Takeover:** Once the human owner types a message, the Auto-Reply Engine pauses for that specific conversation until re-enabled, preventing the AI from talking over the owner.

  ## Implementation Prompt
  **User-Facing Outcome:** A unified inbox in the OHC mobile app where business owners can view and manage messages from all connected channels. The AI automatically replies to routine inquiries using the business's context, escalating only complex issues to the owner.

  **Critical User Journey (CUJ):**
  1. Maya connects her Instagram account to her OHC platform.
  2. A customer sends an Instagram DM: "Do you have vegan options for next Tuesday?"
  3. The OHC Auto-Reply Engine intercepts the webhook, queries Maya's business memory, and replies autonomously: "Yes! We offer vegan chocolate and vanilla cakes. You can order and pay your deposit here: [Checkout Link]".
  4. Maya wakes up, opens her OHC app on her iPhone (375px), checks the unified inbox, and sees the resolved conversation marked "AI Handled" and a new deposit in her account.

  **Acceptance Criteria:**
  - Ingestion pipeline can receive and normalize messages from at least two sources (e.g., Web Chat and simulated external webhooks).
  - AI Agent evaluates incoming messages, queries the business catalog/memory, and generates context-aware replies.
  - Mobile-first UI (strictly 375px compatible) displays the unified conversation thread, clearly distinguishing human vs. AI messages.
  - "Takeover" functionality allows the human owner to pause AI replies for a specific thread, switching state in the database.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
