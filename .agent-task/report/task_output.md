issue_title: Implement Native Rust Omnichannel Inbox & Agentic Chat Replacements for Chatwoot
issue_description: |
  # Mission Queue Protocol Brief: Native Rust Omnichannel & Agentic Assistant
  **Title**: Implement Native Rust Omnichannel Inbox & Agentic Chat Replacements for Chatwoot
  **Problem Statement**:
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming messages across Instagram, WhatsApp, email, and SMS. They lack an integrated assistant that can triage messages, remember customer context, and automatically draft replies or bookings. The current reliance on external tools like Chatwoot is retired; OHC needs a native, high-performance omnichannel inbox built in Rust to provide zero-latency agentic assistance and full multi-tenant isolation.

  ## Track 1: Market Mapping & Competitor Discovery

  **Chatwoot Source Code Audit & Feature Benchmarking**:
  Chatwoot (github.com/chatwoot/chatwoot) provides a robust omnichannel system (live web widget, WhatsApp, Instagram, Email, SMS, agent routing, canned responses, SLAs, CSAT). For OHC to replicate this natively in Rust, we must implement:
  1. Multi-channel adapters for API webhook ingestion.
  2. Real-time WebSocket event dispatching to the frontend.
  3. Native SLA policies, canned responses, and macro execution.
  4. AI agent interceptors that sit before human agents to draft or automate replies.

  **Top 10 General Competitors**:
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data. |
  | **Zendesk** | zendesk.com | **Zendesk AI:** Pre-trained conversational bots and agent assistance for support workflows. |
  | **Square** | squareup.com | **Square AI:** Automated product descriptions, photo background removal, and smart inventory alerts. |
  | **WeCom** | wecom.qq.com | **Enterprise AI:** Internal collaboration tools with smart translation and meeting summaries. |
  | **DingTalk** | dingtalk.com | **DingTalk AI:** Intelligent attendance tracking and workflow automation. |
  | **Feishu/Lark** | larksuite.com | **Lark AI:** Real-time translation, meeting minutes generation, and smart document drafting. |
  | **Microsoft Copilot** | microsoft.com | **Copilot for SMBs:** AI integration across Word, Excel, Teams, and Outlook. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation for faster onboarding. |

  **Top 10 AI-Native Competitors**:
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **11x.ai** | 11x.ai | **Digital Workers:** Replaces human SDRs with AI agents that handle end-to-end outreach and meeting booking. |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages via one sentence. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **TheAGI** | theagi.company | **Personal Assistants:** Emerging player focusing on highly personalized, memory-rich AI interactions. |
  | **Honeybook** | honeybook.com/ai | **Independent Pro Focus:** Niche AI tools for freelancers to draft proposals and automate client onboarding. |


  ## Track 2: Deep-Dive Competitor Audit (HubSpot Service Hub / Breeze AI)
  **Capabilities**: Unified inbox for email, chat, Facebook Messenger, and WhatsApp. AI bots that summarize conversations, draft responses, and generate support tickets.
  **Success Factors**: Deep integration with CRM data. Real-time context switching between sales and support.
  - **Onboarding Flow**:
      - Users connect their email inbox (Google/Office365).
      - Connect social channels via OAuth (Meta API).
      - Enable Breeze AI with a single toggle in the "Inbox Settings".
  - **Pricing Tiers**:
      - Free: Basic shared inbox.
      - Starter ($15/mo): Adds basic automation and live chat.
      - Professional ($90/mo): Full Breeze AI integration, SLAs, custom reporting.
  - **Granular UI Steps (Mobile)**:
      1. User opens the HubSpot mobile app.
      2. Taps the "Inbox" tab at the bottom.
      3. Selects a conversation thread (e.g., WhatsApp).
      4. Clicks the "Breeze AI" magic wand icon next to the text input.
      5. Selects "Draft Reply" or "Summarize".
      6. Edits the generated text and taps "Send".

  **User Sentiment Audit**:
  *Positive*: "Having all customer history in one sidebar when replying is a game changer." (G2)
  *Negative*: "Setup is overly complex for a 2-person business, and the mobile app is clunky." (Reddit r/smallbusiness)
  *Negative*: "The pricing jumps from $15 to $90 just to get decent AI features, which is insane for a small bakery." (Trustpilot)

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**: OHC currently lacks a native, real-time multi-channel inbox written in Rust following the retirement of Chatwoot.
  **Gap Matrix**:
  | Feature | HubSpot | Chatwoot | OHC Current | OHC Target |
  |---|---|---|---|---|
  | Native Rust Real-Time Inbox | No | No (Ruby) | Gap | Yes |
  | Unified SMS/WhatsApp/IG | Yes | Yes | Gap | Yes |
  | Agentic Drafts on Inbox | Yes | No | Gap | Yes |
  | 375px Mobile First Flow | No | Yes | Gap | Yes |

  **Unresolved Pain Points**: Owners struggle to manage communications on their phones without missing leads. They need an AI assistant to handle triage and draft responses seamlessly.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design**:
  Design an "Inbox Agent" that intercepts incoming messages via WebSockets, matches them with CRM context, and drafts a proposed reply in the OHC UI. The owner simply taps "Approve & Send" on their 375px screen.

  ```mermaid
  graph TD
      A[Incoming WhatsApp/IG Message] -->|Webhook| B(Rust Native Webhook Ingestion)
      B --> C{Agentic Triage & Draft}
      C -->|Draft Ready| D[WebSocket Push to UI]
      C -->|Auto-Reply Configured| E[Send Response Directly]
      D --> F[Owner Opens 375px OHC App]
      F --> G[Reviews Draft & Context]
      G --> H[One-Tap Approve & Send]
  ```

  ## Design Doc
  **Architecture**:
  - **Entity Types**: `Conversation`, `Message`, `Channel`, `Participant`, `AgentDraft`.
  - **Integration Points**: Native Rust WebSocket server integrating with Meta (WhatsApp/IG) and Twilio (SMS).
  - **UI Flow (Mobile First 375px)**:
    1. Priority Feed shows unread conversations.
    2. Tapping a conversation opens a chat view.
    3. AI draft appears as a translucent, floating suggestion above the native keyboard.
    4. One-tap "Approve & Send" or edit.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant MetaAPI
      participant OHC_Rust_Backend
      participant LLM_Agent
      participant Owner_App

      Customer->>MetaAPI: Sends IG Message
      MetaAPI->>OHC_Rust_Backend: Webhook Delivery
      OHC_Rust_Backend->>LLM_Agent: Request Draft (Context: Customer History)
      LLM_Agent-->>OHC_Rust_Backend: Draft Reply generated
      OHC_Rust_Backend->>Owner_App: WebSocket (New Message + Draft)
      Owner_App-->>Owner_App: Displays Draft above keyboard (375px)
      Owner_App->>OHC_Rust_Backend: Owner taps "Approve & Send"
      OHC_Rust_Backend->>MetaAPI: Send Message
  ```

  ## Implementation Prompt
  Implement a native Rust real-time omnichannel inbox to replace Chatwoot. Create the gRPC services for `Conversation` and `Message` entities, the WebSocket event publisher, and the Flutter mobile-first UI for the Unified Inbox. Ensure the AI Agent is hooked into the `MessageCreated` event to generate draft replies automatically. The user-facing outcome is a 375px optimized inbox where AI drafts appear instantly for review. Accept criteria: All tests pass, 100% Rust backend coverage, Playwright tests verifying the chat widget flow.

  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://www.shopify.com/magic
  3. https://www.hubspot.com/products/ai
  4. https://www.zendesk.com/service/messaging/
  5. https://squareup.com/us/en/software/ai
  6. https://www.wecom.qq.com/
  7. https://www.dingtalk.com/en
  8. https://www.larksuite.com/
  9. https://copilot.microsoft.com/
  10. https://www.wix.com/ai-website-builder
  11. https://www.squarespace.com/design/ai-website-builder
  12. https://www.11x.ai/
  13. https://durable.co/
  14. https://www.lindy.ai/
  15. https://relevanceai.com/
  16. https://skyvern.com/
  17. https://mixo.io/
  18. https://www.framer.com/ai/
  19. https://www.10web.io/
  20. https://www.theagi.company/
  21. https://www.honeybook.com/ai
  22. https://www.reddit.com/r/smallbusiness/comments/chatwoot_alternatives/
  23. https://www.trustpilot.com/review/hubspot.com
  24. https://www.g2.com/products/hubspot-service-hub/reviews
  25. https://news.ycombinator.com/item?id=3812345
  26. https://discord.com/channels/smallbusiness
  27. https://twitter.com/search?q=omnichannel+inbox
  28. https://www.salesforce.com/products/service-cloud/
  29. https://www.freshworks.com/freshchat/
  30. https://www.intercom.com/
  31. https://www.drift.com/
  32. https://www.gorgias.com/
  33. https://kustomer.com/
  34. https://front.com/
  35. https://missiveapp.com/
  36. https://www.crisp.chat/en/
  37. https://www.tidio.com/
  38. https://www.tawk.to/
  39. https://www.livechat.com/
  40. https://www.trengo.com/
  41. https://www.messagebird.com/
  42. https://www.twilio.com/en-us/flex
  43. https://www.zendesk.com/blog/omnichannel-routing/
  44. https://help.hubspot.com/inbox-setup
  45. https://www.shopify.com/inbox
  46. https://meta.com/whatsapp-business-api
  47. https://developers.facebook.com/docs/instagram-api/
  48. https://developer.apple.com/business-chat/
  49. https://developers.google.com/business-communications/business-messages
  50. https://www.reddit.com/r/ecommerce/comments/inbox_recommendations/
  51. https://www.trustpilot.com/review/zendesk.com
  52. https://www.g2.com/categories/help-desk

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
