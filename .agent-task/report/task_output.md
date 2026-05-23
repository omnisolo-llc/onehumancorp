issue_title: "[Architecture] Omni-Channel AI Conversational Commerce"
issue_description: |
  # [Architecture] Omni-Channel AI Conversational Commerce

  ## Problem Statement
  Small business owners like Maya (a custom cake baker) and Carlos (a handyman) lose potential business because they cannot instantly respond to customer inquiries while they are actively working, sleeping, or away from their phones. Customers today expect immediate responses via channels like Instagram DMs, WhatsApp, and Facebook Messenger. Maya needs an AI agent that can negotiate custom cake orders, answer questions like "Do you have vegan options?", and take deposits directly within an Instagram DM. Carlos needs an AI that can automatically reply to a WhatsApp message, ask for photos of a broken sink, draft a quote, and send a booking link. They both need a unified inbox on their phone that consolidates all these channels and allows them to seamlessly take over from the AI when needed, without any complex setup.

  ## Research Report
  **Competitive Analysis:**
  - **ManyChat / Chatfuel:** Powerful chatbot builders for social media, but they require complex visual flow builders, logic branching, and coding concepts that overwhelm non-technical users. They also lack native integration with inventory and ledgers out-of-the-box.
  - **Shopify Inbox:** Consolidates messages and provides some automated responses, but it lacks the advanced generative AI negotiation and booking capabilities required for service-based or highly customized physical products.
  - **Wix Inbox:** Similar to Shopify Inbox; good for basic chat but relies on static rules rather than dynamic agentic behaviors.

  **Gaps Identified:**
  OHC lacks a unified, multi-channel conversational commerce engine where AI agents autonomously handle full-funnel customer interactions (discovery, quoting, and payment) directly within third-party messaging platforms (WhatsApp, Instagram). This gap limits the ability of OHC merchants to scale their customer acquisition without hiring dedicated sales staff.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Third-Party Channels
          IG[Instagram DMs]
          WA[WhatsApp]
          Messenger[FB Messenger]
      end

      IG --> Webhook[Webhook Gateway]
      WA --> Webhook
      Messenger --> Webhook

      Webhook --> EventBus[Event Bus / PubSub]

      EventBus --> InboxService[Unified Inbox Service]
      EventBus --> AIAgentRouter[AI Agent Router]

      InboxService --> MobileApp[OHC Mobile App 375px]

      AIAgentRouter --> MarketingAgent[Marketing/Sales Agent]
      AIAgentRouter --> OpsAgent[Ops Agent - Inventory/Calendar]

      MarketingAgent --> LLM[LLM Gateway]
      LLM --> RAG[RAG - Business Context & FAQs]

      MarketingAgent -- "Draft Quote / Payment Link" --> InboxService
      MarketingAgent -- "Reply to Customer" --> Webhook

      subgraph Mobile Device
          MobileApp --> LocalCache[(Local Inbox Cache)]
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox View:** Maya opens the OHC app and taps the "Inbox" tab. The screen displays a clean, glassmorphism list of conversations across Instagram, WhatsApp, and SMS, seamlessly unified.
  2. **AI Autopilot Toggle:** Each conversation thread has a prominent "AI Autopilot" toggle at the top. When green, the AI is handling the negotiation. If Maya wants to step in, she simply taps it off or starts typing, which gracefully hands off the context to her.
  3. **In-Thread Actions:** Within a chat thread, the AI's messages are visually distinct (e.g., subtle sparkling border). Below the chat input, quick action pills allow Maya to instantly drop in a "Quote Card" or "Payment Link" without leaving the screen.
  4. **Agent Setup:** In the "AI Settings", Maya sees a simple text area: "What should your AI know?" She types plain English instructions ("I charge $50 for deposits. Vegan cakes are $10 extra. I need 3 days notice."). No flowcharts or logic trees.

  ### AI Agent Integration Points
  - **Sales/Marketing Agent:** Reads incoming messages, accesses the business's RAG context (pricing, FAQs), and generates contextual, brand-aligned responses. It detects intent to purchase and generates OHC payment links.
  - **Operations Agent:** Connected to the Sales Agent to check real-time inventory (for physical goods) or calendar availability (for services) before confirming an order.
  - **Customer Success Agent:** Follows up 2 days after the sale via the same channel to ask for a review or ensure satisfaction.

  ### Key Design Decisions
  - **Zero Configuration:** AI behavior is driven purely by natural language system prompts and RAG (Retrieval-Augmented Generation) based on the merchant's catalog and settings, eliminating the need for node-based flow builders.
  - **Graceful Handoff:** The system must flawlessly handle transitions between AI and human. If the AI detects negative sentiment or a highly complex request, it escalates to the merchant via push notification and pauses itself.
  - **Zero-Trust & Isolation:** Webhooks from third parties are strictly validated and scoped to the specific merchant's tenant ID before processing. AI contexts are strictly isolated to prevent one business's agent from accessing another's data.

  ## Implementation Prompt
  Implement the Omni-Channel AI Conversational Commerce engine and Unified Inbox.
  - **User-Facing Outcome:** Merchants have a single inbox on their phone that consolidates Instagram, WhatsApp, and SMS. An AI agent autonomously replies to customers, negotiates, and secures payments, with a simple toggle for human takeover.
  - **CUJ (Critical User Journey):**
    1. Customer messages the merchant's Instagram asking for a quote.
    2. The AI Sales Agent receives the webhook, checks availability/pricing via RAG, and replies with a drafted quote and payment link.
    3. The merchant receives a push notification, views the unified inbox, and can seamlessly take over the conversation or let the AI finish the sale.
  - **Acceptance Criteria:**
    - Build the unified inbox UI adhering to the 375px baseline and macOS-style glassmorphism design tokens.
    - Implement webhook ingesters for at least one third-party channel (e.g., simulated Instagram).
    - Implement the AI Sales agent with natural language context injection (no visual flow builders).
    - Provide a clear, real-time handoff mechanism between AI and human.
    - Guarantee strict multi-tenant isolation for all incoming messages and AI context retrieval.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
