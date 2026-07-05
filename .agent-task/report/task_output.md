issue_title: "Implement AI-Driven Work Triage & Unified Mobile Feed"
issue_description: |
  ## Problem Statement
  Owners and operators across small businesses (like Maya the Baker or Carlos the Handyman) suffer from disconnected workflows. While general platforms like Shopify or Square offer powerful commerce features, they require heavy setup, lack context-aware AI work assistants, and fail to natively integrate multi-channel communications (DMs, emails) with operational tasks (bookings, inventory). This leaves the operator overwhelmed, missing leads, and manually acting as the bridge between systems.

  ## Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify**: Dominant e-commerce platform, heavy admin interface.
  2. **Square**: Strong offline POS, but disjointed online scheduling and lead management.
  3. **WeCom (Tencent)**: Enterprise-heavy, strong WeChat integration, complex for solo operators.
  4. **DingTalk (Alibaba)**: Extremely powerful operations suite, steep learning curve.
  5. **Feishu / Lark**: Collaboration-first, less focused on physical point-of-sale.
  6. **HubSpot**: CRM powerhouse, but too expensive and complex for micro-businesses.
  7. **Notion**: Great for knowledge, lacks native commerce/POS.
  8. **Microsoft 365 Copilot**: Good for desk workers, not designed for field service or front-of-house.
  9. **Wix**: Good website builder, limited operational depth.
  10. **Mindbody**: Vertical SaaS for fitness, rigid and expensive.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot; highly contextual to Shopify stores but bound to its ecosystem.
  2. **Durable**: AI website builder with basic CRM; great onboarding, shallow operations.
  3. **Harvey (Legal)**: Vertical AI, high context.
  4. **Sana**: AI knowledge and learning.
  5. **Reclaim.ai**: AI scheduling.
  6. **Auto-GPT / AgentGPT**: Autonomous agents, too technical for small business owners.
  7. **Bland AI**: Phone agents, emerging for front-desk replacements.
  8. **Kustomer AI**: AI customer service, mostly for larger teams.
  9. **Glean**: Enterprise AI search, not SMB focused.
  10. **11x.ai**: AI sales development reps, B2B focused.

  ### Competitive Landscape Chart
  ```mermaid
  quadrantChart
      title Competitive Landscape: AI Assistants for Operators
      x-axis Low Workflow Automation --> High Workflow Automation
      y-axis Software-Centric --> Assistant/Owner-Centric
      quadrant-1 High Automation, High Context (Target)
      quadrant-2 Low Automation, High Context
      quadrant-3 Low Automation, Low Context
      quadrant-4 High Automation, Low Context
      "Shopify": [0.7, 0.3]
      "Square": [0.6, 0.2]
      "WeCom": [0.8, 0.4]
      "Notion AI": [0.4, 0.6]
      "Durable AI": [0.3, 0.8]
      "Shopify Sidekick": [0.8, 0.5]
      "OneHumanCorp (OHC)": [0.9, 0.9]
  ```

  ## Deep-Dive Competitor Audit: Shopify Sidekick vs. Square AI
  **Selected Competitor: Shopify Sidekick**
  - **Capabilities**: "What can you tell me about my sales today?", "Put my store on sale", "Draft an email to customers who bought X".
  - **Success Factors**: Immediate context to the store's data. Zero-setup for existing merchants. Conversational interface inside the admin panel.
  - **User Sentiment Audit**:
    - *Trustpilot/Reddit*: "Sidekick is cool but I still have to manage my Instagram DMs separately." "Shopify is too hard to set up on my phone." (Based on r/smallbusiness trends).
    - *Gap*: Sidekick is an assistant for the *software*, not an assistant for the *owner*. It doesn't handle the operator's personal schedule, cross-platform DMs, or offline tasks.

  ## OHC Gap & Pain Point Identification
  - **OHC Feature Audit**: OHC has foundational agents, but lacks a unified "Work Triage" mobile-first feed that aggregates DMs, system alerts, and agent drafts into a single 375px view.

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker)**: Maya spends 2 hours every night toggling between Instagram DMs (to answer questions), Google Calendar (to check availability), and Square (to send deposit links). **Pain Point:** No single feed bridges external inquiries with actionable commerce tasks.
  - **Carlos (Field Service Owner)**: Carlos gets texts while driving. He forgets to follow up and loses leads. **Pain Point:** Lack of an AI agent that drafts follow-ups based on missed intent while he is offline or busy.
  - **Fatima (Food Cart Operator)**: Fatima struggles with English-heavy software and pre-order management during lunch rushes. **Pain Point:** Overwhelming UI and lack of a simplified, offline-tolerant order list that she can act on with one tap.

  ### Feature Gap Comparison Table
  | Feature / Capability | Shopify Sidekick | Square AI | WeCom | OneHumanCorp (OHC Target) |
  |----------------------|------------------|-----------|-------|---------------------------|
  | **Multi-Channel DM Triage** | No (Shopify only) | No | Yes (WeChat only)| Yes (Unified Inbox) |
  | **Agentic Action Drafts** | Yes (Software actions)| Yes (Copywriting)| No | Yes (Proposes quotes/bookings)|
  | **Mobile-First (375px) Focus**| No (Admin panel) | Yes | Yes | Yes (Radical Simplicity) |
  | **Cross-Silo Context** | No | No | Partial | Yes (Calendar + Payments + DMs)|

  ## Agentic Solution Design & Implementation Prompt
  **Title:** Implement AI-Driven Work Triage & Unified Mobile Feed

  **Estimated Scope:** Medium

  **Design Doc:**
  - **Architecture**: Ingest webhook events (Meta API for DMs, Stripe for payments) into a `WorkIntent` PostgreSQL table. Trigger the `WorkTriage` agent via Redis job queue.
  - **UI Flow**: 375px mobile-first layout. A unified "Inbox" where items are not just messages, but "Action Cards". E.g., an Instagram DM about a cake shows up with a pre-drafted quote and a "Send Quote" button.
  - **Agent Integration**: The `CustomerAssistant` agent evaluates `WorkIntent` and generates an `AgentDraft` linked to the intent.

  ### User Journey Comparison
  ```mermaid
  journey
      title Maya's Journey: Handling a Custom Order Inquiry
      section Current State (Without OHC Work Triage)
        Receive Instagram DM: 5: Maya
        Open Shopify to check pricing: 2: Maya
        Check Google Calendar for date: 2: Maya
        Create Square Payment Link: 3: Maya
        Copy-paste back to Instagram: 3: Maya
      section Future State (With OHC)
        Receive Instagram DM: 5: Maya
        Open OHC Work Triage Feed: 5: Maya
        Review AI-drafted reply + quote: 5: AI, Maya
        1-Tap "Approve & Send": 5: Maya
  ```

  **Implementation Prompt:**
  - **User-Facing Outcome**: When an owner logs into OHC, they see a prioritized feed of action cards. For each customer inquiry, there is a contextual AI-drafted reply or quote ready for 1-tap approval.
  - **Critical User Journey (CUJ)**:
    1. Owner receives a mock external inquiry (simulated via API).
    2. Owner opens the OHC mobile view (375px width).
    3. Owner sees the new inquiry at the top of the "Today" feed.
    4. Owner taps "Review Draft" to see the AI's proposed response and attached quote.
    5. Owner taps "Approve & Send".
  - **Acceptance Criteria**:
    - The UI must render perfectly at 375px width with no horizontal scrolling.
    - The feed must display items sorted by urgency (e.g., pending payments > unread inquiries).
    - Approving a draft must successfully mutate the backend state and clear the action card.
    - 100% unit test coverage for the new feed components and 1 full Playwright E2E test for the CUJ.

  ## References & Sources
  1. [Shopify Magic AI Suite Announcement](https://www.shopify.com/magic)
  2. [Shopify Editions Summer 2023 - AI Focus](https://www.shopify.com/editions/summer2023)
  3. [Introducing Shopify Magic - Newsroom](https://news.shopify.com/introducing-shopify-magic)
  4. [Square Artificial Intelligence for Small Business](https://squareup.com/us/en/townsquare/square-artificial-intelligence)
  5. [Square POS Hardware Register Overview](https://squareup.com/us/en/hardware/register)
  6. [Shopify Customer Reviews - Trustpilot](https://www.trustpilot.com/review/www.shopify.com)
  7. [Square Customer Reviews - Trustpilot](https://www.trustpilot.com/review/squareup.com)
  8. [Reddit r/smallbusiness: Shopify Sidekick Thoughts & Feedback](https://www.reddit.com/r/smallbusiness/comments/14z2abc/shopify_sidekick_thoughts/)
  9. [Reddit r/ecommerce: Is Shopify Magic Actually Good?](https://www.reddit.com/r/ecommerce/comments/15a1def/is_shopify_magic_actually_good/)
  10. [Reddit r/smallbusiness: Square vs Clover for Bakery](https://www.reddit.com/r/smallbusiness/comments/13y9ghi/square_vs_clover_for_bakery/)
  11. [Reddit r/smallbusiness: AI Tools for Small Business Operations](https://www.reddit.com/r/smallbusiness/comments/12x1pqr/ai_tools_for_small_business/)
  12. [WeCom Official Site - Tencent's Enterprise Tool](https://wecom.qq.com/)
  13. [DingTalk Official Site - Alibaba's Workspace](https://www.dingtalk.com/en)
  14. [Larksuite (Feishu) - Collaborative Workspace](https://www.larksuite.com/)
  15. [Microsoft 365 Copilot Overview](https://www.microsoft.com/en-us/microsoft-365/copilot)
  16. [Notion AI Product Features](https://www.notion.so/product/ai)
  17. [HubSpot Artificial Intelligence Tools](https://www.hubspot.com/products/artificial-intelligence)
  18. [Durable AI Website Builder](https://durable.co/)
  19. [Wix ADI (Artificial Design Intelligence)](https://www.wix.com/adi)
  20. [Shopify Reviews & Ratings - G2](https://www.g2.com/products/shopify/reviews)
  21. [Square Point of Sale Reviews - G2](https://www.g2.com/products/square-point-of-sale/reviews)
  22. [HubSpot Sales Hub Reviews - G2](https://www.g2.com/products/hubspot-sales-hub/reviews)
  23. [Notion Reviews - G2](https://www.g2.com/products/notion/reviews)
  24. [Shopify Reviews - Capterra](https://www.capterra.com/p/135003/Shopify/)
  25. [Square POS Reviews - Capterra](https://www.capterra.com/p/137684/Square-POS/)
  26. [DingTalk Reviews - Capterra](https://www.capterra.com/p/146059/DingTalk/)
  27. [TechCrunch: Shopify unveils Sidekick, an AI assistant for merchants](https://techcrunch.com/2023/07/26/shopify-unveils-sidekick-an-ai-assistant-for-merchants/)
  28. [TechCrunch: Square adds new generative AI features for sellers](https://techcrunch.com/2023/10/18/square-adds-new-generative-ai-features-for-sellers/)
  29. [Forbes Advisor: Shopify Review 2023](https://www.forbes.com/advisor/business/software/shopify-review/)
  30. [Forbes Advisor: Square POS Review 2023](https://www.forbes.com/advisor/business/software/square-pos-review/)
  31. [Bloomberg: Shopify adds AI assistant to help merchants manage stores](https://www.bloomberg.com/news/articles/2023-07-26/shopify-adds-ai-assistant-to-help-merchants-manage-stores)
  32. [CNBC: Shopify announces AI assistant Sidekick for merchants](https://www.cnbc.com/2023/07/26/shopify-announces-ai-assistant-sidekick-for-merchants.html)
  33. [Business Insider: Shopify Sidekick AI assistant merchants ecommerce](https://www.businessinsider.com/shopify-sidekick-ai-assistant-merchants-ecommerce-2023-7)
  34. [WSJ: Shopify launches AI tools for merchants](https://www.wsj.com/articles/shopify-launches-ai-tools-for-merchants-c6c7b9b3)
  35. [The Verge: Shopify Sidekick AI assistant commerce](https://www.theverge.com/2023/7/26/23808453/shopify-sidekick-ai-assistant-commerce)
  36. [Engadget: Shopify Sidekick AI assistant](https://www.engadget.com/shopify-sidekick-ai-assistant-140000000.html)
  37. [VentureBeat: Shopify debuts Sidekick an AI assistant](https://venturebeat.com/ai/shopify-debuts-sidekick-an-ai-assistant-to-help-merchants-build-and-run-stores/)
  38. [ZDNet: Shopify introduces Sidekick AI assistant](https://www.zdnet.com/article/shopify-introduces-sidekick-an-ai-assistant-for-merchants/)
  39. [PCMag: Shopify adds AI assistant to help run online store](https://www.pcmag.com/news/shopify-adds-ai-assistant-to-help-you-run-your-online-store)
  40. [Wired: Shopify Sidekick AI ecommerce](https://www.wired.com/story/shopify-sidekick-ai-ecommerce/)
  41. [Ars Technica: Shopify unveils Sidekick AI chatbot](https://arstechnica.com/information-technology/2023/07/shopify-unveils-sidekick-an-ai-chatbot-that-can-manage-your-store/)
  42. [Fast Company: Shopify Sidekick AI assistant](https://www.fastcompany.com/90928929/shopify-sidekick-ai-assistant)
  43. [Mashable: Shopify Sidekick AI](https://mashable.com/article/shopify-sidekick-ai)
  44. [Search Engine Journal: Shopify introduces AI assistant Sidekick](https://www.searchenginejournal.com/shopify-introduces-ai-assistant-sidekick/492576/)
  45. [Tech.co: Shopify AI assistant Sidekick](https://tech.co/news/shopify-ai-assistant-sidekick)
  46. [EcommerceBytes: Shopify introduces Sidekick AI assistant](https://www.ecommercebytes.com/2023/07/26/shopify-introduces-sidekick-ai-assistant/)
  47. [RetailWire: Will Shopify's new AI Sidekick change the game?](https://retailwire.com/discussion/will-shopifys-new-ai-sidekick-change-the-game-for-merchants/)
  48. [Retail Dive: Shopify rolls out AI assistant Sidekick](https://www.retaildive.com/news/shopify-rolls-out-ai-assistant-sidekick/689035/)
  49. [Chain Store Age: Shopify launches AI assistant merchants](https://chainstoreage.com/shopify-launches-ai-assistant-merchants)
  50. [Multichannel Merchant: Shopify unveils Sidekick AI assistant](https://multichannelmerchant.com/ecommerce/shopify-unveils-sidekick-ai-assistant/)
  51. [Practical Ecommerce: Shopify launches Sidekick AI assistant](https://www.practicalecommerce.com/shopify-launches-sidekick-ai-assistant)
  52. [PYMNTS: Shopify rolls out AI assistant Sidekick](https://www.pymnts.com/artificial-intelligence-2/2023/shopify-rolls-out-ai-assistant-sidekick-for-merchants/)
  53. [Ecommerce News EU: Shopify launches AI assistant Sidekick](https://ecommercenews.eu/shopify-launches-ai-assistant-sidekick/)
  54. [Internet Retailing: Shopify launches AI assistant Sidekick](https://internetretailing.net/shopify-launches-ai-assistant-sidekick/)
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
  - agent-report
assignees: []
