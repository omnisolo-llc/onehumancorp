issue_title: "Omnichannel Social Commerce & Conversational Checkout Gateway"
issue_description: |
  # [Architecture] Omnichannel Social Commerce & Conversational Checkout Gateway

  ## Problem Statement
  Small business owners like Maya (a baker who receives custom cake requests via Instagram DMs) and Carlos (a handyman who gets text message inquiries) lose potential revenue when they cannot instantly respond to customers. They often miss messages while working, sleeping, or driving. Currently, converting a social media inquiry into a paid order requires manual back-and-forth messaging, sharing external links, and manually tracking payments. They need an AI-driven, omnichannel gateway that invisibly handles inquiries across Instagram, WhatsApp, and SMS, generating instant quotes and tap-to-pay checkout links directly within the chat, without the business owner lifting a finger. The experience must pass the "grandmother test"—completely intuitive, with no complex setup.

  ## Research Report
  **Competitor Systems Audit:**
  - **ManyChat / Chatfuel:** Powerful for social media automation, but require building complex visual flowcharts (decision trees) that non-technical users find overwhelming. They also lack deep integration with an underlying inventory and physical POS system.
  - **Shopify Inbox:** Good for webchat and basic Instagram integration, but heavily relies on standard e-commerce flows and lacks flexible AI negotiation for custom services (like Maya's custom cakes).
  - **Stripe Payment Links:** Excellent for generating quick links, but generating them automatically within a conversational flow requires custom developer integration.

  **Gaps Identified:**
  OHC currently lacks a unified conversational commerce capability where an AI agent can autonomously read DMs, negotiate custom orders, check live inventory (or calendar availability), and generate a secure checkout link directly in the thread. The solution must integrate seamlessly with our existing Zero-Trust architecture and shared distributed task list.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Social Channels
          IG[Instagram DMs]
          WA[WhatsApp]
          SMS[SMS / Twilio]
      end

      IG --> WebhookGateway[Omnichannel Webhook Gateway]
      WA --> WebhookGateway
      SMS --> WebhookGateway

      WebhookGateway --> EventBus[KAIROS Event Bus / Shared Task List]

      subgraph Agent Departments
          EventBus --> SalesAgent[Sales & Support Agent]
          SalesAgent --> Inventory[Inventory CRDT Store]
          SalesAgent --> Calendar[Booking Calendar]
      end

      SalesAgent --> CheckoutEngine[Instant Checkout Link Generator]
      CheckoutEngine --> PaymentGateway[Localized Payment Gateway]

      CheckoutEngine -- Returns Link --> SalesAgent
      SalesAgent -- Sends Reply + Link --> WebhookGateway

      subgraph Mobile App (375px First)
          App[OHC Mobile App] --> UnifiedInbox[Unified Inbox UI]
          EventBus --> UnifiedInbox
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **The Setup (Invisible):** Maya links her Instagram account via a simple one-tap OAuth flow. No rules, no flowcharts.
  2. **The Inquiry:** A customer DMs: "Do you have vegan cupcakes for this Saturday? Need 12."
  3. **The Agent Acts:** The KAIROS Sales Agent intercepts the message, checks Maya's inventory/calendar, and replies within 5 seconds: "Yes, we do! 12 vegan cupcakes will be $40. Would you like to place the order?"
  4. **The Checkout:** Customer says "Yes." Agent generates a secure OHC checkout link and sends it in the DM.
  5. **The Unified Inbox:** Maya opens her OHC app. The "Unified Inbox" screen, styled with macOS Translucent Glass materials and Ubiquiti UniFi modular cards, shows the conversation. A green "Paid" badge appears on the chat card once the customer checks out. Advanced details (like webhook logs or token usage) are hidden behind an "Advanced Settings" switch.

  ### AI Agent Integration Points
  - **Sales & Support Agent:** Acts as the primary conversational interface, analyzing intent, extracting order details (quantities, dates), and generating empathetic, brand-aligned responses.
  - **Operations Agent:** Updates inventory or blocks calendar time slots immediately when the checkout link is generated to prevent double-booking.
  - **Finance Agent:** Reconciles the payment once completed and updates the omnichannel ledger.

  ### Key Design Decisions & Security
  - **Unified Event Bus:** All incoming messages are normalized into a standard `ConversationEvent` and pushed to the KAIROS Shared Task List. This decouples the channel API specifics from the AI logic.
  - **Zero-Trust & Multi-Tenancy:** Each incoming webhook is immediately authenticated and scoped to the specific tenant's SPIFFE SVID. The AI agent only accesses the isolated inventory and calendar of that tenant.
  - **No-Code Conversational Rules:** We strictly avoid flowchart builders. The agent's behavior is driven by the global "Business Profile" and product catalog, passing the "grandmother test."
  - **Premium Visuals:** The Unified Inbox in the mobile app utilizes heavy glassmorphism, soft shadows, and clear typography to make managing cross-channel conversations feel luxurious and effortless.

  ## Implementation Prompt
  Implement the Omnichannel Social Commerce & Conversational Checkout Gateway.
  - **User-Facing Outcome:** Business owners can connect their social channels (Instagram, WhatsApp) with one click. Their AI agent will automatically respond to customer inquiries, check availability, and send checkout links directly in the chat. The owner can monitor everything in a premium Unified Inbox on their mobile app.
  - **CUJ (Critical User Journey):**
    1. Business owner links Instagram.
    2. Customer sends a DM asking to buy a product.
    3. AI Agent reads the DM, verifies inventory, and replies with a custom checkout link.
    4. Customer pays via the link.
    5. Owner sees the successful order in the mobile app's Unified Inbox.
  - **Acceptance Criteria:**
    - Build a scalable webhook gateway to ingest messages from at least two channels (e.g., Instagram, SMS).
    - Normalize messages and route them through the KAIROS Shared Task List.
    - Implement an AI agent prompt/workflow that can generate a checkout link and reply to the webhook.
    - Ensure strict tenant isolation using SPIFFE SVIDs for all data access.
    - Design the mobile Unified Inbox UI using glassmorphism and card-based layouts, adhering to the 375px baseline.
    - Ensure all complex configuration is hidden from the user by default.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
