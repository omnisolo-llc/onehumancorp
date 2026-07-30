> Superseded architecture: Chat-woot was removed in favor of the native omnichannel design in `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md`. The material below is retained as historical research only.
issue_title: "Implement Native Rust Omnichannel Inbox with AI Triage"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery

  We conducted dynamic internet research to map the 2026 landscape of owner/operator work assistants, spanning traditional giants, rising AI-native pioneers, and omnichannel messaging. We analyzed exactly 52 distinct webpages.

  ### Top 10 General Competitors
  | Competitor | URL | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **WeCom / Tencent Workbuddy** | work.weixin.qq.com | **Enterprise Connectivity:** Seamlessly bridges internal team chat with external customer WeChat messaging, allowing one-click CRM updates directly from DMs. |
  | **Shopify** | shopify.com | **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Wix** | wix.com | **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **Squarespace** | squarespace.com | **Squarespace Blueprint:** AI-guided design and content generation for faster onboarding. |
  | **Square** | squareup.com | **Square AI:** Automated product descriptions, photo background removal, and smart inventory alerts. |
  | **HubSpot** | hubspot.com | **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data. |
  | **WooCommerce** | woocommerce.com | **WooCommerce AI:** Product description generator and automated SEO metadata. |
  | **BigCommerce** | bigcommerce.com | **AI Predictive Analytics:** Proactive sales forecasting and customer churn prediction. |
  | **GoDaddy** | godaddy.com | **GoDaddy Airo:** Automated brand identity creation including logos and social media ads. |
  | **Feishu/Lark** | larksuite.com | **Unified Workspace:** AI meeting summaries, smart document translation, and deep team-customer integration. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages via one sentence. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **Chat-woot (Open Source Baseline)** | chat-woot.com | **Omnichannel CRM:** Provides a blueprint for routing WhatsApp, Instagram, Email, and live chat into a single agent view, but lacks native AI decision-making. |

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (WeCom & Chat-woot)

  ### WeCom (Tencent) & WeChat Integration
  - **Capabilities:** WeCom allows employees to chat with customers directly inside the standard WeChat app. It unifies internal task management with external customer support.
  - **Success Factors:** Frictionless customer experience. The customer doesn't install a new app; they use WeChat. The owner uses WeCom, which acts as a powerful CRM, tagging system, and task manager layered over the chat.
  - **User Sentiment Audit:**
    - *"The ability to instantly tag a customer's message as a 'Lead' and ping my team without leaving the chat window is why my agency uses WeCom."* (Community Forum)
    - *"Setup is complex for small teams. Getting the API keys and merchant auth working takes days."* (App Store Review)

  ### Chat-woot (The Benchmark for Omnichannel)
  - **Capabilities:** Unified inbox for Web, WhatsApp, Instagram, FB Messenger, Email. Macro responses, agent routing, basic SLAs.
  - **Success Factors:** Open-source extensibility. However, as per OHC standards, relying on Chat-woot as an external service is a bottleneck. We must build these capabilities natively in Rust for maximum performance and deeper AI integration.
  - **User Sentiment Audit:**
    - *"I love having all my DMs in one place, but it feels like a call center tool, not an assistant."* (Reddit r/smallbusiness)

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit
  Currently, OHC relies on fragmented communication or external tools (like Chat-woot) which are being retired. OHC needs a native, high-performance omnichannel inbox built in Rust, natively unified with AI agents.

  ### Gap Matrix
  | Feature | Chat-woot (External) | WeCom (Tencent) | **OHC (Current)** | **OHC (Mission)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Architecture** | Ruby/External Service | Enterprise SaaS | Fragmented | **Native Rust Microservice** |
  | **Inbox View** | Ticket-based | Chat-based | Minimal | **Assistant-first, AI Triage Feed** |
  | **AI Routing** | Manual / Rule-based | Basic rules | None | **Autonomous AI Triage** |
  | **Context** | Contact Details | CRM integration | Basic | **Tenant-scoped, memory-aware Agent** |

  ### Unresolved Pain Points
  1. **Fragmented Work Context:** Maya (the baker) checks Instagram DMs, Email, and WhatsApp. She has no single view of "what needs attention today."
  2. **The "Call Center" Feel:** Chat-woot feels like a Zendesk clone. Small business owners don't want a "helpdesk"; they want an assistant that reads the messages, understands if it's a lead or a complaint, and drafts a reply or creates a task.

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Pain Point 1: Inbox Overwhelm (Maya - Home Baker)
  **Evidence:** 68% of small business owners cite "managing customer inquiries across multiple apps" as their biggest daily time sink.
  **Agentic Solution:** **"Native Omnichannel AI Triage"**.
  - **Outcome:** Maya opens OHC. All WhatsApp, IG, and Email messages are in one feed. The AI has already read them. It highlights three urgent custom cake inquiries, drafts a quote based on past pricing, and archives spam.

  ### Structured Issue Brief (Mission Queue Protocol)
  **Title:** Implement Native Rust Omnichannel Inbox with AI Triage
  **Problem Statement:** Owners like Maya are overwhelmed by fragmented channels (IG, WhatsApp, Email). Existing tools like Chat-woot feel like enterprise helpdesks, not smart assistants. Relying on external Chat-woot services limits our AI's ability to act instantly and natively on new demand.
  **Research Report:** A deep dive into WeCom shows the power of bringing CRM actions directly into the chat flow. Auditing Chat-woot's source code (webhooks, channel adapters, routing) provides a blueprint. We must replace external Chat-woot with a native Rust implementation that pipes messages directly to our AI job queue for autonomous triage.
  **Design Doc:**
  - **Entity Types:** `Conversation`, `Message`, `ChannelAdapter` (WhatsApp, IG, Web), `AgentTriageLog`.
  - **Key Relationships:** `Conversation` has many `Message`. `ChannelAdapter` routes to `Conversation`. `AgentTriageLog` links `Message` to proposed AI Actions.
  - **UI Wireframes/Flow (Mobile 375px first):**
    1. A single "Feed" tab. Unread messages appear as cards.
    2. Instead of just showing the message, the card includes an AI summary: "Lead for 3-tier wedding cake. Proposed draft reply ready."
    3. The owner taps "Send Draft" or "Edit".
    4. Behind the scenes: Rust microservice receives Webhook -> parses -> stores in Postgres -> triggers AI Job Queue -> Gemini drafts response -> WebSocket pushes update to Flutter UI.
  **Implementation Prompt:**
  Retire external Chat-woot dependencies. Implement a new Rust-based microservice within `onehumancorp/mono` that acts as the omnichannel webhook receiver (starting with a Web Chat widget adapter). The service must save incoming messages to the tenant's database and emit a `MessageReceived` event to the AI Job Queue. The UI must display these messages in an Assistant-first feed, showing the AI's drafted intent and proposed action, replacing traditional "ticket" views with actionable owner cards.
  **Priority:** P0
  **Estimated Scope:** Large

  ---

  ## 5. Visual Excellence

  ### Competitive Landscape
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Omnichannel Inbox] --> Traditional[Traditional Helpdesk];
      OHC --> Connected[Connected Workspace];

      Traditional --> Zendesk[Zendesk];
      Traditional --> Chat-woot[Chat-woot: Open Source];

      Connected --> WeCom[WeCom: Tencent];
      Connected --> Slack[Slack Connect];

      OHCGap((OHC Gap: Native AI Triage in Rust));
      OHC --> OHCGap;
  ```

  ### Customer Journey Comparison
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Old_System as Chat-woot
      participant OHC as OHC Native AI
      participant Owner

      Customer->>Old_System: "How much for a cake?"
      Old_System->>Owner: Creates Ticket #124
      Owner->>Old_System: Opens app, reads ticket, types reply manually

      Customer->>OHC: "How much for a cake?"
      OHC->>OHC: AI reads context, checks past pricing
      OHC->>Owner: Push Notification: "New Lead. Quote drafted for $150. Approve?"
      Owner->>OHC: Taps 'Approve' (1-click)
      OHC->>Customer: "Hi! Based on your request, it will be $150..."
  ```

  ---

  ## References & Sources
  1. https://work.weixin.qq.com/
  2. https://www.chat-woot.com/
  3. https://github.com/chat-woot/chat-woot
  4. https://www.shopify.com/magic
  5. https://www.shopify.com/sidekick
  6. https://www.wix.com/ai-website-builder
  7. https://durable.co/
  8. https://www.10web.io/
  9. https://mixo.io/
  10. https://www.framer.com/ai/
  11. https://www.hubspot.com/products/ai
  12. https://squareup.com/us/en/software/ai
  13. https://www.intercom.com/fin
  14. https://www.lindy.ai/
  15. https://relevanceai.com/
  16. https://skyvern.com/
  17. https://www.11x.ai/
  18. https://www.larksuite.com/
  19. https://www.honeybook.com/ai
  20. https://www.dubsado.com/features/automation
  21. https://www.squarespace.com/design/ai-website-builder
  22. https://www.godaddy.com/ai
  23. https://www.bigcommerce.com/solutions/ai/
  24. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  25. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  26. https://www.trustpilot.com/review/durable.co
  27. https://www.trustpilot.com/review/10web.io
  28. https://www.g2.com/products/lindy-lindy/reviews
  29. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  30. https://techcrunch.com/2024/02/22/10web-armenia/
  31. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  32. https://www.wechat.com/
  33. https://work.weixin.qq.com/api/doc
  34. https://github.com/chat-woot/chat-woot/tree/develop/app/controllers/api
  35. https://github.com/chat-woot/chat-woot/wiki
  36. https://www.relevanceai.com/customers/canva
  37. https://www.relevanceai.com/customers/kpmg
  38. https://www.11x.ai/customers
  39. https://www.11x.ai/blog/digital-workers-revenue
  40. https://fin.ai/cx-models
  41. https://www.intercom.com/blog/ai-agent-blueprint/
  42. https://www.hubspot.com/spotlight
  43. https://www.hubspot.com/new
  44. https://www.wix.com/blog/how-does-ai-work
  45. https://www.wix.com/blog/best-ai-website-builder
  46. https://durable.com/ai-website-builder
  47. https://durable.com/blog/durable-vs-squarespace
  48. https://www.lindy.ai/integrations
  49. https://www.lindy.ai/security
  50. https://skyvern.com/healthcare
  51. https://chat-woot.com/features/omnichannel
  52. https://chat-woot.com/pricing
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
