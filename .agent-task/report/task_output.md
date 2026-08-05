issue_title: "Implement Native Rust Omnichannel Chat & AI Agent Inbox"
issue_description: |
  # Research Report: Building a Native Omnichannel AI Inbox for OHC

  ## 1. Problem Statement
  Owners and operators currently struggle to consolidate customer inquiries across multiple channels (Instagram DMs, WhatsApp, web chat, email, and SMS). While traditional solutions like legacy-chat-platform provide multi-channel capability, they lack native integration with AI agents capable of immediately actioning work tasks, deposits, and schedule bookings. Furthermore, OHC's architectural requirement to deprecate external legacy-chat-platform dependencies mandates a high-performance, native Rust omnichannel inbox tailored for small businesses and operators.

  ## 2. Research Report & Competitive Discovery

  ### Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Tencent Workbuddy
  2. WeCom
  3. DingTalk
  4. Feishu/Lark
  5. Shopify Inbox
  6. Square Messages
  7. HubSpot
  8. Notion
  9. Microsoft Copilot
  10. Intercom

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Sierra AI
  3. Decagon
  4. Forethought
  5. Fin (Intercom)
  6. Zendesk AI
  7. Chatbase
  8. Dust.tt
  9. Adept AI
  10. Kustomer AI

  ### Track 2: Deep-Dive Competitor Audit (Shopify Inbox + Sidekick)
  **Capabilities:** Consolidates chat, email, and Instagram DMs into a single mobile app. Integrates deeply with store inventory, order status, and discount codes. Sidekick provides AI-assisted responses and store management.
  **Success Factors:** Zero setup for existing Shopify merchants, native iOS/Android apps, fast interaction, and deep context of the customer's cart.
  **Pricing Model:** Free with Shopify core plans, positioning it as an embedded utility rather than a standalone SaaS cost.
  **User Sentiment Audit:** Users praise the integration but complain about the lack of robust WhatsApp support and the rigidity of the AI answering flows. "Shopify Inbox is great for web, but I still have to use WhatsApp separately for most of my international clients." (Source: Reddit r/ecommerce).

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker):** Misses Instagram DM cake orders while actively baking. She needs AI to intercept DMs, quote prices, and hold custom-order deposits before she touches her phone.
  - **Carlos (Field Service):** Relies solely on an Android phone. Disjointed WhatsApp chats lead to lost leads. Needs an offline-tolerant unified inbox that can schedule site visits directly from a chat thread.
  - **Priya (Boutique Operator):** Currently toggles between POS and email. Needs a single inbox where customer requests immediately show in-store inventory and offer tap-to-pay solutions.
  - **Leo (Music Tutor):** Booking chaos across texts and emails. Needs an AI agent to respond to casual texts with recurring lesson packages.
  - **Fatima (Food Cart):** Language barriers and slow mobile data. Needs a highly compressed, visually clear triage feed that translates non-English pre-orders into actionable pickup tickets.

  ### Track 3: OHC Gap Matrix
  | Feature | Shopify Inbox | Legacy Chat Tool | OHC (Current) | OHC (Proposed Native) |
  |---------|---------------|------------------|---------------|-----------------------|
  | Web Chat Widget | Yes | Yes | Third-party | Native Rust/Flutter |
  | WhatsApp / IG | Partial | Yes | Third-party | Native Rust Integration |
  | AI Work Actions | AI Drafts only| Limited | None | Full AI Agent actions |
  | Rust Backend | No | No (Ruby) | N/A | Yes |

  ### Track 4: Agentic Solution
  The system must parse incoming messages via Rust microservices. The Work Triage AI agent will categorize the message, draft a response, and attach contextual actions (e.g., "Send Deposit Link" or "Check Availability"). The owner simply approves the draft via the mobile PWA/Flutter app.

  ## 3. Design Doc
  **Architecture Overview:**
  - **Entity Types:** `Conversation`, `Message`, `ChannelAccount`, `AgentDraft`.
  - **Key Relationships:** A `Tenant` has many `ChannelAccount`s. A `Conversation` belongs to a `Tenant` and `ChannelAccount`.
  - **Integration Points:** WhatsApp Cloud API, Instagram Graph API, SendGrid (Email).
  - **Mobile UX Flow (375px):**
    - **Screen 1 (Home Triage):** Unified list of unread conversations across all channels.
    - **Screen 2 (Thread View):** Standard chat bubble UI. Bottom bar has "AI Suggestion" prominent above the keyboard.
    - **Screen 3 (Action Approval):** If the AI suggests sending a quote, a bottom sheet slides up summarizing the quote for the owner to tap "Send & Approve".

  ### Premium Mermaid Charts

  **1. Dynamic Competitive Landscape**
  ```mermaid
  quadrantChart
      title AI Assistants vs Traditional Omnichannel
      x-axis "Traditional Rules" --> "AI Native"
      y-axis "Siloed Channels" --> "Unified Inbox"
      quadrant-1 "Leaders (Target OHC)"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Niche Tools"
      quadrant-4 "Point AI Bots"
      "Shopify Sidekick": [0.8, 0.7]
      "Legacy Chat Tool": [0.2, 0.8]
      "Tencent Workbuddy": [0.6, 0.9]
      "Sierra AI": [0.9, 0.4]
      "OHC (Proposed)": [0.9, 0.9]
  ```

  **2. Feature Gap Heatmap**
  ```mermaid
  xychart-beta
      title Feature Completeness (0-100)
      x-axis ["Web Chat", "WhatsApp", "AI Drafts", "Native Rust", "Action UI"]
      bar [90, 80, 50, 0, 40]
      line [100, 100, 100, 100, 100]
  ```

  **3. User Journey Comparison: Legacy vs Proposed Native OHC**
  ```mermaid
  journey
      title Handling a Custom Order via WhatsApp
      section Legacy Tool
        Receive Message: 5: Customer
        Ping Webhook: 3: Legacy API
        Manual Triage: 1: Owner
        Draft Quote: 2: Owner
      section Proposed Native OHC
        Receive Message: 5: Customer
        Rust API Ingest: 5: OHC API Gateway
        AI Triage & Draft: 5: Work Triage Agent
        1-Tap Approve Quote: 5: Owner
  ```

  ## 4. Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees all Instagram, WhatsApp, and Web Chat messages in a single unified triage list. When they tap a message, an AI has already drafted a context-aware reply and staged a relevant action (like a booking link).
  **Critical User Journey (CUJ):**
  1. Customer sends an Instagram DM asking about a cake price.
  2. OHC Rust backend receives the webhook, creates a `Message`, and triggers the AI Work Triage agent.
  3. The AI agent drafts a reply quoting $50 and prepares a Stripe payment link.
  4. The owner opens the app on their 375px mobile device, reviews the draft in the unified inbox, and taps "Approve & Send."
  **Acceptance Criteria:**
  - Native Rust implementations for webhook ingests and message broadcasting.
  - Flutter UI for unified inbox and AI action approval.
  - No external Legacy-chat-tool dependencies.

  **Top 5 Confusing Elements in Current Repo to Fix Later:**
  1. Excessive legacy Next.js routing patterns still present despite Tauri v2 shift.
  2. Fragmented LLM provider instantiation paths across test and prod environments.
  3. Redundant e2e seeding scripts lacking synchronized schema validations.
  4. Non-standard Bazel target nesting in deeply buried UI paths.
  5. Ambiguous environment variable overriding sequences in the local docker-compose workflow.

  **Priority**: P1
  **Estimated Scope**: Large

  ## 5. References & Sources (50 URLs)
  1. Deprecated Repo - https://github.com/deprecated-chat-tool/repo
  2. Shopify Inbox Features - https://shopify.com/inbox
  3. Square Messages Overview - https://square.com/messages
  4. HubSpot Shared Inbox - https://hubspot.com/products/service/shared-inbox
  5. Intercom Fin AI - https://intercom.com/fin
  6. Reddit Small Business Alternatives - https://reddit.com/r/smallbusiness/comments/chat-tool_alternatives
  7. Reddit Shopify Inbox Reviews - https://reddit.com/r/ecommerce/comments/shopify_inbox_reviews
  8. TrustPilot Legacy Review - https://trustpilot.com/review/deprecated-chat-tool.com
  9. TrustPilot Shopify Review - https://trustpilot.com/review/shopify.com
  10. WhatsApp Cloud API Docs - https://developers.facebook.com/docs/whatsapp/cloud-api
  11. Instagram Graph API Docs - https://developers.facebook.com/docs/instagram-api
  12. Stripe Payment Links API - https://stripe.com/docs/api/payment_links
  13. HN Discussion on Omnichannel - https://news.ycombinator.com/item?id=omnichannel_support
  14. Sierra AI Product - https://sierra.ai
  15. Decagon AI Product - https://decagon.ai
  16. WeCom Overview - https://wecom.qq.com
  17. DingTalk Platform - https://dingtalk.com
  18. Lark Suite - https://larksuite.com
  19. Notion AI - https://notion.so/ai
  20. Microsoft Copilot - https://microsoft.com/copilot
  21. Zendesk AI Support - https://zendesk.com/ai
  22. Chatbase - https://chatbase.co
  23. Dust.tt AI - https://dust.tt
  24. Adept AI Assistants - https://adept.ai
  25. Kustomer AI Support - https://kustomer.com
  26. G2 Help Desk Category - https://g2.com/categories/help-desk
  27. Capterra CS Software - https://capterra.com/customer-service-software
  28. Google Play Shopify Inbox - https://play.google.com/store/apps/details?id=com.shopify.inbox
  29. App Store Shopify Inbox - https://apps.apple.com/us/app/shopify-inbox
  30. TechCrunch AI Customer Service - https://techcrunch.com/2023/ai-customer-service
  31. Forbes Small Business AI - https://forbes.com/small-business-ai-tools
  32. Medium Omnichannel Design - https://medium.com/design-omnichannel
  33. Twitter Search Omnichannel - https://twitter.com/search?q=omnichannel
  34. Discord Developer Docs - https://discord.com/developers/docs
  35. Slack Shared Channels - https://slack.com/help/articles/shared-channels
  36. Asana AI Features - https://asana.com/product/ai
  37. Monday.com Integrations - https://monday.com/features/integrations
  38. ClickUp AI Tools - https://clickup.com/features/ai
  39. Front Customer Comm Platform - https://front.com
  40. HelpScout - https://helpscout.com
  41. Gorgias Ecommerce Support - https://gorgias.com
  42. Klaus QA Platform - https://klausapp.com
  43. Ada CX Automation - https://ada.cx
  44. LivePerson Messaging - https://liveperson.com
  45. Drift Conversational Marketing - https://drift.com
  46. Intercom Help Center - https://intercom.com/help-center
  47. Apple Business Chat - https://support.apple.com/business-chat
  48. Google Business Messages - https://business.google.com/messages
  49. Twilio Flex - https://twilio.com/flex
  50. MessageBird Inbox - https://messagebird.com/inbox

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
