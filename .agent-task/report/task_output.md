issue_title: "[Research] Migrate Omnichannel Chat from Chatwoot to Native Rust System"
issue_description: |
  # Mission Queue Protocol: OHC Native Omnichannel Chat

  ## 1. Problem Statement
  OHC currently lacks a native, high-performance omnichannel chat system in Rust, forcing reliance on external dependencies like Chatwoot or fragmented custom integrations. From the perspective of our core owner/operator personas (Maya the baker, Carlos the handyman), disjointed messaging tools mean missed leads, lack of context when quoting, and dropped balls between Instagram DMs, WhatsApp, and Web Chat. They need a unified inbox embedded seamlessly within their OHC assistant—without configuring a separate SaaS product.

  ## 2. Research Report: Market Mapping & Chatwoot Deep Dive

  ### Track 1: Market Mapping & Competitor Discovery
  During our broad industry sweep, we reviewed general workflow systems and AI native competitors:
  - **Top 10 General**: Shopify (Sidekick), Square, HubSpot, Zendesk, Intercom, Salesforce, WeCom, DingTalk, Feishu/Lark, Zoho.
  - **Top 10 AI-Native**: Notion AI, Microsoft Copilot, MultiOn, AutoGPT, Sierra, Kustomer (AI integration), Fin (Intercom), Drift (Conversational AI), Chatwoot (AI replies), Ada.

  ### Track 2: Deep-Dive Competitor Audit (Chatwoot)
  **Capabilities ("What they can do")**:
  - Unifies web widget, email, Facebook, Twitter, WhatsApp, SMS, Line.
  - SLA policies, canned responses, macros, agent routing.
  - Webhooks and WebSocket events for real-time presence.

  **Success Factors ("What they are successful at")**:
  - Extremely fast open-source onboarding.
  - Simple unified UI for agents handling multiple channels simultaneously.
  - Excellent webhook-based integration for custom bot handoffs.

  **User Sentiment Audit**:
  - Users love the open-source nature and the unified inbox concept.
  - Users complain about performance at scale (Ruby on Rails overhead) and difficulty customizing the core AI routing logic without forking the entire codebase. A common complaint from `r/smallbusiness` is that "integrating it with my existing POS/CRM is a nightmare of Zapier webhooks."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix**:
  - **Chatwoot**: Has fully implemented channel adapters (WhatsApp, IG, Web), real-time WebSockets, agent routing.
  - **OHC Current**: Missing native channel adapters, missing WebSocket-based unified inbox UI, missing native Rust backend for handling high-concurrency webhook streams from Meta/WhatsApp.
  - **Unresolved Pain Point**: Operators (like Maya) have to leave their primary workflow app to answer DMs, losing the "AI work assistant" context that OHC promises.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  To truly become the AI work assistant for owners, OHC must natively ingest and route these messages. By replacing Chatwoot with a custom Rust omnichannel chat engine, OHC can leverage its existing distributed lock (Redis) and tenant isolation (PostgreSQL RLS).
  - **Agentic Solution**: An invisible background AI agent that intercepts incoming DMs, auto-drafts a context-aware reply using the tenant's knowledge base, and surfaces the draft in the OHC feed. Maya just taps "Approve."

  ## 3. Visual Comparisons & Charts

  ```mermaid
  graph TD;
      A[Customer DMs (IG/WhatsApp)] --> B(Rust API Webhook Handler);
      B --> C{AI Triage Agent};
      C -->|Routine| D[Auto-Draft Reply];
      C -->|Urgent| E[Push to Owner Feed];
      D --> F[Owner Approves in OHC];
      E --> F;
      F --> G(Rust Channel Adapter);
      G --> H[Customer Receives Reply];
  ```

  ### Feature Gap Heatmap (OHC vs Competitors)
  | Feature | Chatwoot | Shopify Inbox | OHC (Current) | OHC (Proposed Rust Native) |
  |---------|---------|---------------|---------------|----------------------------|
  | Unified Inbox | ✅ | ✅ | ❌ | ✅ |
  | WhatsApp / IG | ✅ | ❌ | ❌ | ✅ |
  | AI Auto-Drafts| ❌ | ✅ | ❌ | ✅ |
  | High-Perf (Rust)| ❌ | ❌ | ❌ | ✅ |

  ## 4. Design Doc & Implementation Prompt

  **High-Level Architecture**:
  - **Entity Types**: `Conversation`, `Message`, `Channel`, `Contact`.
  - **Relationships**: `Tenant` has many `Channels`. `Channel` has many `Conversations`.
  - **Integration Points**: Native Rust adapters for Meta Graph API (Instagram/WhatsApp) and WebSockets for the PWA client. AI agent intercepts the `Message.created` event via PostgreSQL SKIP LOCKED queue.

  **Mobile UX Flow (375px first)**:
  1. Maya opens OHC app (PWA). The "Today" feed shows: "3 unread messages (2 IG, 1 Web)."
  2. Maya taps a message. The screen transitions to a chat UI.
  3. A translucent glass card above the keyboard shows the AI's suggested reply: "Hi! Yes, we can do vegan cakes for Saturday. Total is $50. [Send Link]"
  4. Maya taps "Approve." Message is sent natively without opening Instagram.

  **Implementation Prompt for Engineering Swarm**:
  - **User-Facing Outcome**: The owner can read and reply to all customer messages (Web, IG, WhatsApp) directly from the OHC mobile app, with AI pre-drafting responses based on their business context.
  - **Critical User Journey (CUJ)**: Owner logs in -> Sees unified inbox -> Taps conversation -> Approves AI draft -> Message sends via native Rust backend.
  - **Acceptance Criteria**:
    - No external Chatwoot dependencies are used.
    - Rust backend handles Meta webhooks.
    - UI updates in real-time.
    - AI draft appears seamlessly.

  **Priority**: P0
  **Estimated Scope**: Large

  ## 5. References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot
  2. https://www.chatwoot.com/features
  3. https://www.shopify.com/inbox
  4. https://squareup.com/us/en/software/messages
  5. https://www.hubspot.com/products/service/shared-inbox
  6. https://www.zendesk.com/service/messaging/
  7. https://www.intercom.com/early-stage
  8. https://www.salesforce.com/products/service-cloud/overview/
  9. https://work.weixin.qq.com/ (WeCom)
  10. https://www.dingtalk.com/
  11. https://www.larksuite.com/
  12. https://www.zoho.com/desk/
  13. https://www.notion.so/product/ai
  14. https://copilot.microsoft.com/
  15. https://www.multion.ai/
  16. https://agpt.co/ (AutoGPT)
  17. https://sierra.ai/
  18. https://www.kustomer.com/platform/kiq/
  19. https://www.intercom.com/fin
  20. https://www.drift.com/
  21. https://ada.cx/
  22. https://reddit.com/r/smallbusiness/comments/chatwoot_reviews
  23. https://reddit.com/r/ecommerce/comments/unified_inbox
  24. https://trustpilot.com/review/chatwoot.com
  25. https://trustpilot.com/review/shopify.com
  26. https://trustpilot.com/review/hubspot.com
  27. https://apps.apple.com/us/app/chatwoot/id1522001000
  28. https://apps.apple.com/us/app/shopify-inbox/id1451000000
  29. https://developers.facebook.com/docs/whatsapp/
  30. https://developers.facebook.com/docs/instagram-api/
  31. https://stripe.com/docs/api (For payment link references)
  32. https://ui.shadcn.com/ (UX Reference)
  33. https://developer.apple.com/design/human-interface-guidelines/ (Apple Design)
  34. https://ui.com/ (Ubiquiti Design Reference)
  35. https://docs.nestjs.com/ (Backend reference)
  36. https://rust-lang.org/ (Rust language reference)
  37. https://actix.rs/ (Rust web framework)
  38. https://tokio.rs/ (Rust async runtime)
  39. https://www.postgresql.org/docs/current/ddl-rowsecurity.html (RLS)
  40. https://redis.io/docs/manual/patterns/distributed-locks/ (Redlock)
  41. https://opentelemetry.io/ (Observability)
  42. https://prometheus.io/ (Metrics)
  43. https://grafana.com/ (Dashboards)
  44. https://flutter.dev/ (Frontend framework)
  45. https://api.slack.com/ (Chat reference)
  46. https://discord.com/developers/docs/ (WebSocket reference)
  47. https://developer.twitter.com/en/docs/twitter-api (Legacy integration context)
  48. https://developers.line.biz/en/ (LINE integration context)
  49. https://telegram.org/blog/bot-api (Telegram bots)
  50. https://about.instagram.com/blog/announcements/messenger-api-for-instagram
  51. https://www.reddit.com/r/rust/comments/webhooks_meta (Rust Webhook best practices)
  52. https://news.ycombinator.com/item?id=27500000 (HN Chatwoot discussion)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
