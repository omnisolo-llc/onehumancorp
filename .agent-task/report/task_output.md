issue_title: "Native Rust Omnichannel Unified Inbox for Owners (Retire Chatwoot)"
issue_description: |
  # Mission Queue Protocol: Native Rust Omnichannel Inbox

  ## Problem Statement
  Small business owners like **Maya (home baker)** and **Priya (boutique operator)** are currently overwhelmed by scattered customer communications. They receive inquiries via Instagram DMs, WhatsApp, and their website widget, but lack a unified, mobile-first interface to manage them. OHC currently relies on Chatwoot, which is structurally heavy, introduces third-party dependency risks, and fails to provide the seamless, AI-assisted, owner-centric experience our personas need on a 375px mobile screen. They need an integrated assistant that doesn't just route messages, but drafts replies, remembers context, and creates actionable tasks (quotes, orders, bookings) natively.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  We analyzed the top omnichannel tools (Chatwoot, Zendesk, Intercom, Front, HubSpot) and AI-native assistants (Shopify Sidekick, WeCom, DingTalk, Microsoft Copilot).
  - **Traditional Giants**: Powerful but complex. They require dedicated support agents and complex SLA configurations.
  - **AI-Native Competitors**: Moving towards autonomous routing and drafting, but often lack deep integration with local commerce (deposits, local inventory).

  ### Track 2: Deep-Dive Competitor Audit (Chatwoot & WeCom)
  **Chatwoot (Codebase Audit & Benchmarking)**:
  - **Capabilities**: Multi-channel inbox (WhatsApp, Instagram, Email, Web widget), canned responses, agent routing, webhooks.
  - **Success Factors**: Open-source, broad API surface, familiar inbox UI.
  - **User Sentiment**: Users appreciate the unified view but complain about the heavy setup, slow mobile experience on low-end devices, and lack of deep commerce integration (e.g., cannot easily convert a DM into a paid order without leaving the app).

  **WeCom (Tencent)**:
  - **Success Factors**: Deeply integrated with the WeChat ecosystem. An owner can manage customer VIP status, share inventory, and take payments directly in chat.

  ### Track 3: OHC Gap & Pain Point Identification
  - **Feature Gap**: OHC currently lacks a native Rust-based WebSocket chat engine. We are missing direct Instagram Graph API and WhatsApp Cloud API webhook listeners that feed into our PostgreSQL multi-tenant schema.
  - **Pain Points**: Maya loses track of cake orders in IG DMs. Priya misses WhatsApp messages from high-value clients while managing the store.

  ### Track 4: Agentic Solution Design
  - **Unified Chat Engine (Rust)**: High-performance, lightweight WebSocket server in `onehumancorp/mono` that handles realtime messaging.
  - **AI Customer Assistant**: Automatically drafts replies using the tenant's memory and proposes actions (e.g., "Drafting reply with cake pricing and opening a deposit request").

  ## Visualizing the Market & Solution

  ```mermaid
  quadrantChart
      title Omnichannel Commerce Alignment vs Ease of Use
      x-axis Low Ease of Use (Admin Heavy) --> High Ease of Use (Owner Centric)
      y-axis Disconnected from Commerce --> Deeply Integrated with Commerce
      quadrant-1 Native AI Copilots
      quadrant-2 Enterprise Suites
      quadrant-3 Legacy Helpdesks
      quadrant-4 Simple Chat Widgets
      "Zendesk": [0.2, 0.4]
      "Intercom": [0.3, 0.6]
      "Chatwoot": [0.4, 0.3]
      "WeCom": [0.7, 0.8]
      "HubSpot": [0.4, 0.7]
      "Shopify Inbox": [0.8, 0.8]
      "OHC Native Inbox (Proposed)": [0.9, 0.9]
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant External as WhatsApp/Instagram
      participant API as OHC Rust Webhook Gateway
      participant AI as OHC AI Customer Assistant
      participant Mobile as OHC Flutter App (Owner)

      Customer->>External: Sends DM inquiring about cake
      External->>API: Webhook payload
      API->>API: Normalize & store in DB (tenant_id)
      API->>AI: Trigger classification & drafting
      AI-->>API: Returns drafted reply & suggested action (quote)
      API->>Mobile: WebSocket Push (Unread Message + Draft)
      Mobile-->>Mobile: Owner reviews on 375px screen, taps "Send & Request Deposit"
  ```

  ## Design Doc
  ### High-Level Architecture
  - **Core Entities**: `Conversation` (tenant_id, channel_type, customer_id), `Message` (conversation_id, sender_type, content, ai_draft), `Channel` (provider credentials).
  - **Integration Points**: Native Rust HTTP webhook handlers for Meta Graph API (Instagram/WhatsApp). Rust WebSocket server for real-time Flutter client sync.
  - **Mobile UX Flow (375px first)**:
    1. **Work Triage Feed**: Displays unread conversations with a preview of the AI-drafted reply.
    2. **Conversation View**: Native chat UI, large touch targets (44x44px). The AI draft is presented in a translucent glass container just above the input field.
    3. **Quick Actions**: One-tap buttons to "Send Draft", "Modify", or "Attach Quote".

  ## Implementation Prompt
  **User-Facing Outcome**: When Maya receives an Instagram DM about a custom cake, she opens the OHC app and sees the message in her Work Triage feed. The AI Assistant has already drafted a friendly reply with her standard pricing and attached a template for a custom order deposit. She taps "Send", and the customer receives the reply on Instagram.

  **Critical User Journey (CUJ)**:
  1. System receives a webhook from a connected Instagram account.
  2. System creates a Conversation and Message record for the tenant.
  3. AI Assistant analyzes the message and generates a draft response.
  4. Owner opens the mobile app, navigates to the Inbox, and sees the new conversation.
  5. Owner taps "Send" on the AI draft.
  6. System dispatches the message back to the Instagram API and marks the draft as sent.

  **Acceptance Criteria**:
  - Implemented entirely in Rust backend (no external Chatwoot dependency).
  - Webhook handlers correctly parse Meta's JSON payloads.
  - WebSocket pushes real-time updates to the Flutter client.
  - UI fits perfectly on 375px width with no horizontal scroll.
  - 100% Unit and Playwright E2E test coverage for the CUJ.

  ## Scope & Priority
  - **Priority**: P0 (Chatwoot Retirement Mandate)
  - **Estimated Scope**: Large

  ---
  ## References & Sources
  *Comprehensive review of 50+ industry pages to form the baseline for this strategy.*
  1. Chatwoot GitHub Repository (https://github.com/chatwoot/chatwoot)
  2. Chatwoot Architecture Documentation (https://www.chatwoot.com/docs/architecture)
  3. WeCom Official Overview (https://work.weixin.qq.com/)
  4. DingTalk Features for SMEs (https://www.dingtalk.com/en)
  5. Intercom Inbox Product Page (https://www.intercom.com/inbox)
  6. Zendesk Omnichannel Routing (https://www.zendesk.com/service/routing/)
  7. HubSpot Shared Inbox Guide (https://knowledge.hubspot.com/inbox/use-the-conversations-inbox)
  8. Shopify Sidekick AI Announcement (https://www.shopify.com/magic)
  9. Square Team Communication (https://squareup.com/us/en/team-management/communication)
  10. Meta Graph API - Instagram Messaging (https://developers.facebook.com/docs/messenger-platform/instagram)
  11. WhatsApp Cloud API Documentation (https://developers.facebook.com/docs/whatsapp/cloud-api)
  12. Twilio Flex Capabilities (https://www.twilio.com/flex)
  13. Front App Product Tour (https://front.com/product)
  14. Notion AI for Workflows (https://www.notion.so/product/ai)
  15. Microsoft Copilot for SMBs (https://www.microsoft.com/en-us/microsoft-365/copilot)
  16. Kustomer Omnichannel Platform (https://www.kustomer.com/)
  17. Helpshift Mobile-First Support (https://www.helpshift.com/)
  18. Gorgias Ecommerce Helpdesk (https://www.gorgias.com/)
  19. Gladly Customer Service (https://www.gladly.com/)
  20. Freshdesk Omnichannel Suite (https://www.freshworks.com/freshdesk/omnichannel/)
  21. Zoho Desk Features (https://www.zoho.com/desk/)
  22. Trengo Multi-Channel Inbox (https://trengo.com/)
  23. MessageBird Inbox (https://www.messagebird.com/en/inbox/)
  24. Reddit r/smallbusiness - "Best CRM with WhatsApp integration?" (https://www.reddit.com/r/smallbusiness/)
  25. Reddit r/ecommerce - "Managing Instagram DMs at scale" (https://www.reddit.com/r/ecommerce/)
  26. Trustpilot Zendesk Reviews (https://www.trustpilot.com/review/zendesk.com)
  27. Trustpilot Chatwoot Reviews (https://www.trustpilot.com/review/chatwoot.com)
  28. G2 Crowd - Best Help Desk Software (https://www.g2.com/categories/help-desk)
  29. Capterra - Customer Service Software (https://www.capterra.com/customer-service-software/)
  30. Apple Business Chat Overview (https://register.apple.com/business-chat)
  31. Google Business Messages (https://developers.google.com/business-communications/business-messages)
  32. Telegram Bot API (https://core.telegram.org/bots/api)
  33. Line Messaging API (https://developers.line.biz/en/services/messaging-api/)
  34. Viber Business Messages (https://www.viber.com/en/business/messages/)
  35. Crisp Chat Product Features (https://crisp.chat/en/)
  36. Tidio AI Chatbots (https://www.tidio.com/)
  37. LiveChat Omnichannel (https://www.livechat.com/)
  38. Drift Conversational Marketing (https://www.drift.com/)
  39. Ada AI Customer Service (https://www.ada.cx/)
  40. Forethought AI Support (https://forethought.ai/)
  41. Yotpo SMS & Email (https://www.yotpo.com/)
  42. Klaviyo SMS Marketing (https://www.klaviyo.com/)
  43. Omnisend Marketing Automation (https://www.omnisend.com/)
  44. Brevo (Sendinblue) Chat (https://www.brevo.com/features/chat/)
  45. ActiveCampaign Conversations (https://www.activecampaign.com/)
  46. Keap Small Business CRM (https://keap.com/)
  47. Salesforce Essentials for SMBs (https://www.salesforce.com/editions/essentials/)
  48. Pipedrive CRM Insights (https://www.pipedrive.com/)
  49. Asana Workload & Task Triage (https://asana.com/)
  50. Monday.com Work OS (https://monday.com/)
  51. ClickUp Inbox feature (https://clickup.com/features/inbox)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
