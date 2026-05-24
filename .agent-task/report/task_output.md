issue_title: "Autonomous Multilingual Conversational Checkout Engine"
issue_description: |
  ## 1. Title
  **Autonomous Multilingual Conversational Checkout Engine**

  ## 2. Problem Statement
  For OneHumanCorp (OHC)'s core personas—especially **Maya (baker)** and **Fatima (food cart, limited English)**—customers often reach out via Instagram DMs or WhatsApp in various languages asking for prices, availability, and how to pay. Translating, answering, quoting, and managing these orders manually while working is highly stressful and results in missed sales. Existing platforms (Shopify, Wix) treat social media DMs as a separate inbox, requiring a human to manually send payment links. OHC needs a completely invisible AI agent that can negotiate, take orders, and collect payments directly inside any chat application, in any language, autonomously.

  ## 3. Research Report
  ### Competitive Landscape
  *   **Shopify:** Offers Inbox, but relies heavily on manual intervention to drop product links.
  *   **Wix:** Basic automations, but conversational AI checkout is non-existent.
  *   **Stripe:** Has payment links, but no conversational AI agent to handle the negotiation/sales process.

  ### Market Data
  *   Over 60% of Gen Z and Millennial buyers prefer messaging a business on Instagram or WhatsApp over navigating a website.
  *   Language barriers cost non-native speaking micro-merchants (like Fatima) up to 40% of potential localized business.

  ### Opportunity
  By deploying an autonomous conversational agent that understands the merchant's catalog and inventory, we can completely automate the DMs-to-Cash funnel. The agent negotiates the order in the customer's language, processes the payment directly in the chat via a secure Web3/Apple Pay/Google Pay intent, and updates the OHC central ledger instantly.

  ## 4. Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Social API (Meta/WhatsApp)
      participant OHC Edge Gateway
      participant Conversational AI Agent (Sales Dept)
      participant Inventory & Capacity Ledger
      participant Secure Payment Engine (SPIFFE/SPIRE)
      participant OHC Mobile Dashboard

      Customer->>Social API: "Can I order 2 vegan cakes for tomorrow?" (in Spanish)
      Social API->>OHC Edge Gateway: Webhook Event
      OHC Edge Gateway->>Conversational AI Agent: Route DM
      Conversational AI Agent->>Inventory & Capacity Ledger: Check availability
      Conversational AI Agent->>Social API: Replies (in Spanish) with price and checkout link
      Customer->>Secure Payment Engine: Completes Payment via link
      Secure Payment Engine->>Inventory & Capacity Ledger: Deduct inventory
      Secure Payment Engine->>OHC Mobile Dashboard: Push Notification "New Order Paid!"
  ```

  ### UI Wireframes (375px Mobile-First) & Mobile UX Flow
  **Screen 1: Conversational Settings (Dashboard)**
  *   Clean, macOS-style Translucent Glass dashboard card.
  *   Toggle: `[ AI Assistant: Active ]`
  *   Slider: `[ Negotiation Flexibility: Strict Pricing <--> Allow 10% Discounts ]`
  *   No complex API keys or webhook setups. Grandmother Test passed.

  **Screen 2: Unified Inbox View**
  *   Thread shows AI interacting with customer.
  *   Clear visual badge indicating "AI Handled" vs "Human Needed".
  *   Bottom bar: `[ Take Over Conversation ]`

  **Screen 3: Order Notification**
  *   Push Notification: "Maya, 2 vegan cakes ordered and paid by Maria! Delivery tomorrow."

  ### AI Agent Integration Points
  *   **Sales Department (Conversational AI):** Handles natural language understanding, multi-lingual translation, context management, and intent extraction.
  *   **Operations Department (Inventory & Ledger):** Verifies if the request can be fulfilled and deducts from capacity.
  *   **Finance Department:** Generates dynamic, secure checkout sessions mapped to the specific DM interaction.

  ### Key Design Decisions and Why
  *   **Zero-Trust Isolation:** Payment links generated in the chat must be cryptographically signed (SPIFFE/SPIRE) to prevent tampering and ensure they map to the correct tenant ID.
  *   **No Code Configuration:** The merchant does not train the AI. The AI automatically ingests the existing catalog and availability mesh.
  *   **Omnichannel Agnostic:** The engine sits behind the OHC Edge Gateway, meaning it works identically whether the webhook comes from Instagram, WhatsApp, or SMS.

  ## 5. Implementation Prompt
  **To the Implementer:**
  Your task is to build the "Autonomous Multilingual Conversational Checkout Engine".
  The Core User Journey (CUJ) is as follows: A customer messages the business's connected Instagram or WhatsApp account. The AI Sales Agent seamlessly responds in the customer's language, confirms inventory, and provides a secure, one-click checkout link. The merchant (using the OHC mobile app) simply receives a "Paid Order" notification without ever opening the chat.

  **Acceptance Criteria:**
  *   **Multi-tenant Isolation:** Chat webhooks and payment intents must be strictly isolated per tenant using our established architecture.
  *   **Mobile-First Merchant UI:** The setup and monitoring UI on the 375px viewport must be effortless (a simple toggle to activate the AI).
  *   **Seamless Handoff:** If the AI is confused or the customer requests a human, it must route to a "Human Needed" queue in the Unified Inbox.
  *   **No API Tinkering:** Hide all Meta/WhatsApp Graph API complexity from the merchant.
  *   **Language Agnosticism:** The AI must auto-detect the customer's language and reply in kind, while logging the summary in the merchant's native language.

  ## 6. Priority
  P0 (Critical)

  ## 7. Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
