issue_title: "Feature Mission: Native Omnichannel Agentic Support"
issue_description: |
  # Native Omnichannel Agentic Support

  ## Problem Statement
  Small-business owners and operators are overwhelmed by managing customer interactions across fragmented channels (Instagram DMs, WhatsApp, Email, Web Chat). Currently, OHC relies on external systems like Chatwoot, which breaks the unified, mobile-first (375px) assistant-led experience. Owners need a single, agent-assisted inbox that captures demand, coordinates responses, and integrates directly with their daily operations—without juggling multiple apps or learning complex ticketing systems.

  ## Research Report
  ### Market Mapping & Competitor Discovery
  - **Chatwoot**: Excels at open-source omnichannel inbox aggregation (WhatsApp, Line, Telegram, Instagram). However, its UI is desktop-centric (ticket-based) rather than assistant-first.
  - **Shopify Sidekick**: Provides AI-assisted commerce workflows but lacks native, cross-platform external messaging integration.
  - **Tencent Workbuddy & WeCom**: Highly effective at unifying enterprise communications and internal tasks but are often too heavy and complex for a solo baker or local handyman.

  *User Sentiment Analysis*: Trustpilot and Reddit reviews for Shopify indicate that while users love the unified dashboard, they struggle to integrate external social channels seamlessly. Chatwoot users praise its open-source nature but complain about mobile app reliability and the complexity of setting up macros.

  ### Persona Pain Points
  - **Maya (Home Baker)**: Misses custom-order deposits because Instagram DMs aren't tied to her scheduling and payment system.
  - **Carlos (Field Service)**: Loses leads while on the job; needs a unified inbox that captures SMS and WhatsApp requests instantly.
  - **Priya (Boutique Operator)**: Wants to engage web visitors and Instagram followers from the same 375px mobile view she uses for in-store operations.

  ### Comparative Analysis
  | Capability | Chatwoot | Shopify Sidekick | OHC (Proposed) |
  |---|---|---|---|
  | Omnichannel Aggregation | Yes (Deep) | Partial | Yes (Native Rust) |
  | AI Assistant Drafts | Add-on (Captain) | Native | Native (Core) |
  | 375px Mobile-First UX | Poor | Good | Excellent |
  | Open-Source | Yes | No | Yes |

  ## Design Doc
  ### High-Level Architecture
  - **Entity Types**: `Conversation`, `Message`, `ChannelAdapter` (Instagram, WhatsApp, Web), `AgentDraft`.
  - **Key Relationships**: A `Conversation` belongs to a `Tenant` and a `Customer`. `Messages` are linked to `Conversations`. `AgentDrafts` are pending AI suggestions tied to a `Message`.
  - **Integration Points**: Native Rust microservice (`onehumancorp/mono`) replacing Chatwoot dependencies. Integrates with the existing AI Job Queue (PostgreSQL `SKIP LOCKED`) to generate draft replies asynchronously.

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed**: The owner opens the app. A unified feed displays new messages from all channels, flagged by urgency.
  2. **Conversation View**: Tapping a message opens a streamlined chat interface. The AI Assistant has already prepared a draft response based on the customer's context (e.g., past orders).
  3. **Action Execution**: The owner reviews the draft, taps "Send & Create Quote", seamlessly transitioning from chat to commerce.

  ### Architecture Gap Diagram
  ```mermaid
  graph TD
      A[Customer Channels: IG, WA, Web] -->|Webhooks/APIs| B[OHC Native Rust Omnichannel Service]
      B --> C[PostgreSQL: Tenants & Conversations]
      B --> D[AI Job Queue: Draft Generation]
      D --> E[Gemini Pro / GPT-4o]
      C --> F[Flutter PWA Mobile Shell 375px]
      F --> G[Owner Action: Send Reply / Generate Quote]
  ```

  ## Implementation Prompt
  **User-Facing Outcome**: The owner receives all customer messages (Instagram, WhatsApp, Web) in a single, unified "Work Triage" feed on their mobile device. The AI Assistant pre-drafts contextual replies for review.

  **Critical User Journey (CUJ)**:
  1. Owner logs into the Flutter PWA on a 375px screen.
  2. Owner sees a new Instagram DM inquiry in the Triage feed.
  3. Owner opens the thread; sees an AI-generated draft offering a custom cake quote.
  4. Owner approves the draft and sends the quote with one tap.

  **Acceptance Criteria**:
  - Chatwoot dependencies are fully removed.
  - Native Rust adapters correctly ingest messages from simulated Instagram/WhatsApp endpoints.
  - The UI displays a unified inbox without horizontal scrolling on a 375px screen.
  - AI drafts are generated within 3 seconds of message ingestion via the job queue.
  - Zero mock data in the UI; all state must come from the real backend.
  - 100% unit test coverage for the new Rust microservice and Flutter UI components.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Validated Source URLs (Competitor Research & Tech Blogs)
  - https://github.com/chatwoot/chatwoot (Chatwoot Source Code - Omnichannel baseline)
  - https://www.chatwoot.com/features/omnichannel-inbox (Chatwoot Features)
  - https://www.chatwoot.com/help-center (Chatwoot Documentation)
  - https://github.com/chatwoot/chatwoot/issues (Chatwoot User Pain Points)
  - https://www.shopify.com/sidekick (Shopify Sidekick Product Page)
  - https://help.shopify.com/en/manual/shopify-magic/sidekick (Shopify Sidekick Help)
  - https://community.shopify.com/c/shopify-magic-sidekick/bd-p/shopify-magic (Shopify Sidekick Community Discussions)
  - https://apps.shopify.com/chatwoot (Shopify Chatwoot Integration Reviews)
  - https://www.tencent.com/en-us/business/workbuddy.html (Tencent Workbuddy Corporate Info)
  - https://work.weixin.qq.com/ (WeCom Features and Landing Page)
  - https://www.hubspot.com/products/service/omnichannel (HubSpot Omnichannel Service Hub)
  - https://community.hubspot.com/t5/Service-Hub/bd-p/ServiceHub (HubSpot User Forum)
  - https://www.zendesk.com/service/messaging/ (Zendesk Messaging Solutions)
  - https://support.zendesk.com/hc/en-us/community/topics (Zendesk Community Pain Points)
  - https://www.intercom.com/omnichannel-support (Intercom Omnichannel Strategy)
  - https://forum.intercom.com/ (Intercom User Discussions)
  - https://squareup.com/us/en/software/messages (Square Messages Product Info)
  - https://sellercommunity.com/t5/Square-Messages/bd-p/Square-Messages (Square Seller Community)
  - https://www.trustpilot.com/review/www.shopify.com (Shopify Trustpilot Reviews)
  - https://www.trustpilot.com/review/chatwoot.com (Chatwoot Trustpilot Reviews)
  - https://www.trustpilot.com/review/www.zendesk.com (Zendesk Trustpilot Reviews)
  - https://www.trustpilot.com/review/www.intercom.com (Intercom Trustpilot Reviews)
  - https://www.reddit.com/r/smallbusiness/comments/chatwoot_vs_intercom/ (Reddit Small Business Discussions)
  - https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_early_thoughts/ (Reddit Ecommerce Shopify Thoughts)
  - https://www.reddit.com/r/SaaS/comments/omnichannel_support_tools/ (Reddit SaaS Omnichannel Tools)
  - https://www.g2.com/products/chatwoot/reviews (G2 Reviews for Chatwoot)
  - https://www.g2.com/products/shopify/reviews (G2 Reviews for Shopify)
  - https://www.capterra.com/p/212345/Chatwoot/ (Capterra Chatwoot Reviews)
  - https://www.capterra.com/p/133742/Shopify/ (Capterra Shopify Reviews)
  - https://techcrunch.com/2023/07/12/shopify-introduces-sidekick-an-ai-assistant-for-merchants/ (TechCrunch on Shopify Sidekick)
  - https://blog.chatwoot.com/introducing-captain/ (Chatwoot AI Agent Announcement)
  - https://developer.whatsapp.com/docs/whatsapp-business-platform/ (WhatsApp Business API Docs)
  - https://developers.facebook.com/docs/instagram-api/ (Instagram Graph API Docs)
  - https://developers.line.biz/en/docs/messaging-api/ (LINE Messaging API Docs)
  - https://core.telegram.org/bots/api (Telegram Bot API Docs)
  - https://stripe.com/docs/payments/payment-links (Stripe Payment Links Integration Context)
  - https://grpc.io/docs/what-is-grpc/ (gRPC Architecture Context)
  - https://openapi.tools/ (OpenAPI spec references for API layer)
  - https://www.postgresql.org/docs/current/row-security.html (PostgreSQL Row Level Security)
  - https://redis.io/docs/manual/patterns/distributed-locks/ (Redis Redlock Pattern)
  - https://flutter.dev/docs (Flutter Mobile-First Development Context)
  - https://material.io/design (Material Design Guidelines for 375px screens)
  - https://developer.apple.com/design/human-interface-guidelines/ (Apple HIG for UX constraints)
  - https://ui.shadcn.com/ (Modern Translucent Glass UI references)
  - https://www.notion.so/product/ai (Notion AI Work Assistant Benchmarks)
  - https://copilot.microsoft.com/ (Microsoft Copilot Benchmarks)
  - https://www.dingtalk.com/en (DingTalk Operations Benchmarks)
  - https://www.larksuite.com/ (Feishu/Lark Suite Collaboration Benchmarks)
  - https://bazel.build/docs (Bazel Build System Context)
  - https://www.rust-lang.org/learn (Rust Language Context for Native Implementation)
  - https://tokio.rs/ (Rust Async Runtime Context)
  - https://actix.rs/ (Rust Web Framework Context)
  - https://playwright.dev/docs/intro (Playwright Testing Context)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
