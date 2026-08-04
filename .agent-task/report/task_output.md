issue_title: "Research: AI Work Assistant Market & Competitor Analysis"
issue_description: |
  # Research Report: AI Work Assistant Market & Competitor Analysis

  ## 1. Problem Statement
  Small business owners and operators (e.g. Maya the baker, Carlos the handyman) face significant pain points: scattered work across multiple tools, complex setups without AI help, manual quoting, missed leads when busy, and lack of true mobile-first management. They need a system that acts like a unified assistant—an assistant that can track leads, sync inventory, handle omnichannel messaging, automate quotes, and manage daily operations without a steep learning curve.

  ## 2. Research Report
  ### Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. WeCom
  6. DingTalk
  7. Feishu/Lark
  8. Wix
  9. Thryv
  10. Housecall Pro

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Harvey
  3. Lindy.ai
  4. MultiOn
  5. Synthia
  6. Sierra
  7. Chatwoot
  8. Intercom Fin
  9. Artisan AI
  10. 11x.ai

  ### Track 2 & 3: Deep-Dive Audit & Gap Analysis (Shopify & Sidekick)
  **Capabilities:** Shopify allows store creation, inventory management, and basic POS. Sidekick helps with queries and basic task execution.
  **Success Factors:** Huge ecosystem, established trust, easy payment processing.
  **User Sentiment:** Users love the ecosystem but heavily criticize the complexity ("too many apps needed", "expensive subscriptions", "overwhelming setup for a simple bakery").

  **OHC Gap:** Shopify lacks native omnichannel AI messaging that feels like a singular assistant handling both *demand intake* and *operations*. OHC needs an integrated, native Rust-based chat system (replacing the retired Chatwoot dependency) that unifies WhatsApp, IG, Email, and SMS into a single feed with AI triage.

  ### Track 4: Agentic Solution Design
  OHC should implement a unified "Work Triage" interface where agents pre-draft responses to IG DMs or emails based on the owner's past behavior and current inventory/schedule.
  The owner simply reviews the drafted quote or message and taps "Approve."
  This relies on the native Rust omnichannel chat system to reliably ingest messages and trigger AI agent routines via PostgreSQL SKIP LOCKED queues.

  ## Persona-Specific Pain Points
  | Persona | Pain Point | How OHC Solves It |
  |---------|------------|-------------------|
  | Maya (Baker) | Overwhelmed by IG DMs and manual quoting | AI drafts quotes based on IG DMs, owner 1-tap approves |
  | Carlos (Handyman) | Misses leads while driving/working | Automated SMS follow-ups, capturing intent before he calls back |
  | Priya (Boutique) | Disjointed online and in-store inventory | Unified inventory sync with proactive reorder alerts |
  | Leo (Tutor) | Chaos in booking lessons across platforms | One-link scheduling integrated into chat replies automatically |
  | Fatima (Food Cart) | Fast-paced prep, slow mobile data | Offline-first PWA for order lists, bulk notification to customers |

  ## Feature Gap Heatmap
  ```mermaid
  graph TD
      A[Features] --> B[Shopify]
      A --> C[Square]
      A --> D[Chatwoot]
      A --> E[OHC Vision]

      B -->|High| F(Storefront)
      B -->|Low| G(Native Omnichannel Chat)
      B -->|Med| H(AI Assistant)

      C -->|High| I(POS)
      C -->|Low| J(AI Assistant)
      C -->|Low| K(Omnichannel)

      D -->|High| L(Omnichannel)
      D -->|Low| M(Storefront / POS)
      D -->|Low| N(Operations AI)

      E -->|High| O(Unified Assistant)
      E -->|High| P(Omnichannel Chat)
      E -->|High| Q(Agentic Workflows)
  ```

  ## Competitive Landscape
  ```mermaid
  quadrantChart
      title Market Positioning: Capability vs AI Integration
      x-axis Low Capabilities --> High Capabilities
      y-axis Bolt-on AI --> AI-Native Assistant
      quadrant-1 Specialized AI
      quadrant-2 Broad AI Leaders
      quadrant-3 Legacy Tools
      quadrant-4 Broad Feature Suites
      Shopify: [0.8, 0.4]
      Square: [0.7, 0.2]
      Chatwoot: [0.4, 0.3]
      Lindy.ai: [0.3, 0.8]
      OHC (Target): [0.9, 0.9]
  ```

  ## Comparative Table
  | Feature | OHC (Vision) | Shopify | Square | Chatwoot |
  |---------|--------------|---------|--------|----------|
  | Target Audience | Owners/Operators | E-commerce | Retail/Services | Customer Support |
  | Primary Interface | AI Assistant Feed | Dashboard | POS/Dashboard | Inbox |
  | Native Omnichannel | Yes (Rust-based) | Via Apps | Limited | Yes |
  | AI Autonomy | High (Drafts/Executes) | Medium (Sidekick) | Low | Low |

  ## 3. Design Doc
  **Architecture:**
  - `tenant_id` isolated PostgreSQL tables for `messages`, `conversations`, `customers`.
  - Rust microservice for webhook ingestion (WhatsApp, IG, Email) replacing Chatwoot.
  - Redis Redlock for preventing duplicate AI agent responses to the same message.
  - WebP compression for incoming attachments.

  **UI Flow (375px Mobile First):**
  - **Screen 1 (Command Center):** Unified inbox. A card says "3 new cake inquiries. 2 drafted replies ready."
  - **Screen 2 (Approval):** Shows the incoming DM and the AI's drafted reply (e.g., "Hi! Yes, I can do a vegan cake for the 15th. It will be $50. [Payment Link]").
  - **Actions:** 'Send', 'Edit', or 'Dismiss'.

  ## 4. Implementation Prompt
  Implement the Core Omnichannel Triage Interface for the owner.
  - **User Journey:** The owner opens the app (375px wide) and sees a list of pending customer inquiries. They tap an inquiry to view the context (past orders, current schedule) and the AI-generated draft response. They can approve or edit the draft.
  - **Acceptance Criteria:**
    - UI must render correctly at 375px width.
    - Zero mock data; use the real API layer (or documented seed data).
    - Implement Playwright E2E tests verifying the flow from viewing the inbox to approving a draft.

  ## References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://shopify.com/sidekick
  3. https://reddit.com/r/smallbusiness
  4. https://reddit.com/r/ecommerce
  5. https://trustpilot.com/review/www.shopify.com
  6. https://squareup.com/us/en
  7. https://hubspot.com/
  8. https://notion.so/
  9. https://work.weixin.qq.com/ (WeCom)
  10. https://dingtalk.com/
  11. https://larksuite.com/ (Feishu)
  12. https://wix.com/
  13. https://thryv.com/
  14. https://housecallpro.com/
  15. https://harvey.ai/
  16. https://lindy.ai/
  17. https://multion.ai/
  18. https://synthia.com/
  19. https://sierra.ai/
  20. https://intercom.com/fin
  21. https://artisan.co/
  22. https://11x.ai/
  23. https://reddit.com/r/Entrepreneur
  24. https://reddit.com/r/macapps
  25. https://trustpilot.com/review/www.squareup.com
  26. https://trustpilot.com/review/www.wix.com
  27. https://capterra.com/p/134444/Shopify/
  28. https://capterra.com/p/146039/Square-Point-of-Sale/
  29. https://g2.com/products/hubspot-sales-hub/reviews
  30. https://g2.com/products/notion/reviews
  31. https://news.ycombinator.com/item?id=36688755 (Hacker News discussion on Sidekick)
  32. https://news.ycombinator.com/item?id=38392942 (HN on autonomous agents)
  33. https://techcrunch.com/2023/07/26/shopify-sidekick/
  34. https://theverge.com/2023/7/12/23792553/shopify-sidekick-ai-assistant-ecommerce
  35. https://bloomberg.com/news/articles/2023-07-26/shopify-launches-ai-assistant-to-help-merchants-run-their-stores
  36. https://forbes.com/advisor/business/software/shopify-competitors/
  37. https://nerdwallet.com/article/small-business/shopify-alternatives
  38. https://merchantmaverick.com/square-alternatives/
  39. https://zapier.com/blog/best-ecommerce-platforms/
  40. https://pcmag.com/picks/the-best-ecommerce-platforms
  41. https://websitebuilderexpert.com/ecommerce-website-builders/shopify-alternatives/
  42. https://youtube.com/watch?v=kYJvU8eW0mY (Review of Sidekick)
  43. https://youtube.com/watch?v=9_Y_V-W_y_Q (Shopify vs Square)
  44. https://twitter.com/tobi/status/1679124424785461248 (Tobi Lutke on Sidekick)
  45. https://about.instagram.com/blog/announcements/instagram-messaging-api-updates
  46. https://developers.facebook.com/docs/whatsapp/cloud-api/
  47. https://stripe.com/docs/api (Stripe Payments for OHC)
  48. https://playwright.dev/docs/intro (Playwright testing for OHC)
  49. https://bazel.build/ (Bazel build system)
  50. https://redis.io/docs/manual/patterns/distributed-locks/ (Redis Redlock pattern)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
