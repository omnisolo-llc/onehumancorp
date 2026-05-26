issue_title: "Implement the Unified Omnichannel AI Ambassador Inbox"
issue_description: |
  # [architecture] Unified Omnichannel AI Ambassador Inbox

  ## Title
  Implement the Unified Omnichannel AI Ambassador Inbox

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) suffer from "Customer Communication Chaos." They lose track of leads and inquiries scattered across Instagram DMs, WhatsApp, SMS, and email. Solopreneurs lose up to 30% of sales simply due to slow response times or forgotten messages. They need a single, centralized inbox where an AI "Silent Ambassador" watches the communication stream, proactively drafts context-aware replies (e.g., answering "do you do vegan cakes?"), and presents them for a quick 1-tap approval from their phone's lock screen.

  ## Research Report
  *   **The Market Reality:** A core reason SMBs abandon DIY tools (Shopify, Wix) is because those platforms give them a static storefront but don't help them capture the high volume of conversational commerce that happens via social media. Our internal gap analysis (`ohc_small_business_platform_gap_analysis.md`) identified that missing an omnichannel social inbox is a critical deficiency.
  *   **Competitor Baseline:** Shopify requires expensive third-party apps (like Gorgias) for omnichannel support. Wix has a basic inbox but it relies entirely on the human to respond.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Autonomy":** Instead of a static "chatbot," OHC deploys the **Ambassador Agent**—an invisible, always-on AI representative that hooks into the merchant's unified inbox. It understands the business context (menu, calendar, pricing), engages customers naturally across any channel, and escalates to the human only when necessary (e.g., a highly custom complex order).
  *   **Key Dependencies Discovered in Research:**
      *   `[social_media]_meta_graph_api.md` & `[social_media]_instagram_meta_api.md`: Native webhooks for IG DMs.
      *   `[social_media]_whatsapp.md`: Integration for WhatsApp Business API.
      *   `[sms]_twilio.md`: Handling two-way SMS.
      *   `[email]_resend.md` / `sendgrid`: Inbound/outbound email parsing.

  ## Design Doc

  ### Business Journey Mapping
  The "Unified Inbox" sits at the intersection of **Acquisition** (turning a DM into a lead) and **Retention** (handling support requests).
  1.  **Acquisition:** A customer texts Maya on WhatsApp: "Hi, do you have any gluten-free options available for pickup tomorrow?"
  2.  **Ambassador Triage:** The `Omnichannel Gateway` normalizes the WhatsApp payload. The Ambassador Agent reads the message, queries the `Inventory Ledger` (which confirms gluten-free cookies are in stock), and drafts a reply.
  3.  **1-Tap Approval:** Maya's phone receives a push notification with the drafted reply. From the lock screen, she taps "Approve & Send".
  4.  **Conversion:** The Ambassador sends the reply along with a native OHC deep-link checkout URL for a $10 deposit.

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ UNIFIED_THREAD : "owns"
      CUSTOMER ||--o{ UNIFIED_THREAD : "participates in"
      UNIFIED_THREAD ||--o{ MESSAGE : "contains"

      MESSAGE {
          uuid id PK
          uuid thread_id FK
          enum channel "IG_DM, WHATSAPP, SMS, EMAIL"
          string external_message_id "ID from Meta/Twilio"
          enum direction "INBOUND, OUTBOUND"
          enum sender_type "CUSTOMER, HUMAN_MERCHANT, AI_AMBASSADOR"
          text body
          timestamp created_at
      }

      UNIFIED_THREAD {
          uuid id PK
          uuid merchant_id FK
          uuid customer_id FK
          string subject "Optional"
          boolean requires_human_attention
          timestamp last_activity_at
      }

      INBOX_ROUTER ||--o{ AMBASSADOR_AGENT : "Triggers"
      AMBASSADOR_AGENT ||--o{ AGENT_DEPARTMENTS : "Consults (CS, Ops, Finance)"
      AMBASSADOR_AGENT }|--|| UNIFIED_THREAD : "Appends to"
  ```

  ### Mobile UX Flow (375px Viewport)
  1.  **The Hub:** The primary app screen is not a dashboard of charts; it is the Hub Feed.
      *   Unread messages appear as prominent, swipeable cards.
      *   Each card clearly indicates the source icon (Instagram, WhatsApp, SMS).
      *   If the Ambassador Agent has drafted a reply, the card background has a subtle highlight (e.g., translucent glass effect) and the draft text is shown immediately below the customer's message.
  2.  **Interaction:** The user can swipe right to instantly "Approve & Send" the AI draft, or tap the card to open the full thread to manually type a response.
  3.  **The Thread View:** A standard chat interface, but unified.
      *   Sent messages (both human and AI) are shown clearly but distinguished by a small signature (e.g., "✨ Ambassador").
      *   A prominent "+" button opens the action menu (Send Invoice, Request Deposit, Share Calendar Link).

  ### Zero Trust & Security
  *   **Multi-tenant Isolation:** The `MESSAGE` and `UNIFIED_THREAD` tables MUST enforce strict Row-Level Security (RLS) ensuring `merchant_id` isolation.
  *   **SPIFFE/SPIRE:** The Omnichannel Gateway (which receives public webhooks from Meta/Twilio) must securely authenticate and pass identity context via SPIFFE to the internal Inbox Service and Agent Mesh, ensuring external payloads cannot spoof internal AI commands.

  ## Implementation Prompt
  Your goal is to build the underlying architecture and UI for the "Unified Omnichannel AI Ambassador Inbox" so a user like Maya can manage Instagram, SMS, and Email conversations from one mobile-optimized view.

  **Core User Journeys (CUJ) to Implement:**
  1.  **Ingestion:** Create a unified gateway service that can receive mock webhooks representing incoming Instagram DMs, WhatsApp messages, and SMS, normalizing them into a standard `Message` schema.
  2.  **AI Auto-Drafting:** Implement the listener where the Ambassador AI (mock the LLM call for now) automatically generates a draft response for any incoming message based on the merchant's business profile.
  3.  **Mobile-First UI:** Build the React/React Native (or equivalent mobile web) "Hub" feed demonstrating the translucent glass card design where a merchant can see the incoming message, the AI draft, and a 1-tap "Approve & Send" action.
  4.  **Action Extension:** Inside a chat thread, allow the merchant to click a button to append an "OHC Checkout Link" to their reply.

  *Do not prescribe specific database schemas or API endpoint routing; focus on achieving the end-to-end flow and mobile UI parity described above.*

  ## Priority
  P0 (Critical to the core platform value proposition)

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []