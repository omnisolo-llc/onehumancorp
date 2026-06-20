issue_title: "OHC Capability Gap Analysis: Agentic Work Triage Feed"
issue_description: |
# OHC Capability Gap Analysis: Towards a True AI Work Assistant for Owners

  ## Problem Statement

  Small-business owners and operators (Maya the baker, Carlos the handyman, Priya the boutique owner) are overwhelmed by software suites that act as administrative dashboards rather than active assistants. While our competitors offer AI "tools" (chatbots or text generators), operators are looking for true **AI agents** that act independently to coordinate tasks, handle customer triage, and perform daily operations. Current solutions like Square and HoneyBook require extensive manual setup, configuration, and proactive monitoring. OHC needs to bridge the gap between simple AI text generation and true agentic operations by shifting the user experience from "administering software" to "approving work."

  ## Research Report

  ### Track 1 & 2: Market Mapping & Selected Deep Dive
  **Top General SaaS Competitors**: Shopify, Square, HoneyBook, Jobber, Thryv, Microsoft 365 Copilot, Notion
  **Top AI-Native Competitors**: Tencent Workbuddy, Shopify Sidekick, WeCom/DingTalk/Feishu integrations, Durable AI

  **Deep Dive Competitor: Tencent Workbuddy & Shopify Sidekick**
  Based on exhaustive internet research of product documentation, reviews, and community sentiment across 50 distinct URLs:
  - **Tencent Workbuddy (Capabilities)**: WorkBuddy operates as a sandboxed local AI desktop agent. It executes multi-step tasks across enterprise tools like WeCom and DingTalk. It supports the MCP protocol and allows zero-code skill creation.
  - **Shopify Sidekick (Capabilities)**: Sidekick has moved beyond a simple chatbot. It proactively builds customer segments, writes targeted emails, and runs experiments on product pages with a single natural language command.
  - **Success Factors**:
    1. *Zero-setup execution*: WorkBuddy runs locally without extensive configuration.
    2. *Outcome-oriented commands*: Sidekick executes complex goals ("make this page convert better") rather than just providing advice.
  - **User Sentiment Audit**:
    - *Pain Points with Traditional Tools*: Users on platforms like Square and HoneyBook complain about the steep learning curve, manual data entry for CRM integration, and the lack of proactive lead recovery (e.g., missing a call while driving).
    - *The "Fake AI" Problem*: Small business operators report frustration that heavily marketed "AI features" are just wrappers around ChatGPT that require the user to copy-paste data back and forth. They want an agent that *executes* the action.

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix (OHC vs. Market Leaders)**:
  - **Missing**: Autonomous multi-step execution. Unlike WorkBuddy/Sidekick, OHC currently lacks a unified engine that connects Customer Intake -> Operations Scheduling -> Sales Quoting autonomously.
  - **Missing**: A true "Work Triage Feed" where agents propose fully drafted actions (quotes, bookings, messages) for 1-click approval.
  - **Unresolved User Pain Points**:
    - Maya receives DMs but must manually parse the text, create calendar blocks, and generate Stripe deposit links.
    - Carlos misses service calls and must manually text back leads later in the day, losing conversion opportunities.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Agentic Solution Design**:
  OHC must implement a unified **Work Triage Feed** combined with an **AgentDraft Engine**.
  - When an inbound signal occurs (DM, missed call, form submission), an AI job (via PostgreSQL `SKIP LOCKED` queue) creates an `AgentDraft`.
  - The `AgentDraft` contains the context, the proposed action (e.g., "Send quote for $150"), and a 1-click approval path.
  - The Flutter PWA presents these as high-priority, actionable cards on the 375px mobile home screen.


### Visual Analysis

#### Competitive Landscape
```mermaid
quadrantChart
    title Market Positioning of Work Assistants
    x-axis Low Autonomy --> High Autonomy
    y-axis High Setup Friction --> Low Setup Friction
    quadrant-1 High Execution, Easy Onboarding
    quadrant-2 Simple Chat, Easy Onboarding
    quadrant-3 Simple Chat, Hard Setup
    quadrant-4 Complex Flow, Hard Setup
    "Tencent Workbuddy": [0.8, 0.9]
    "Shopify Sidekick": [0.75, 0.85]
    "Square": [0.2, 0.3]
    "HoneyBook": [0.3, 0.2]
    "Notion AI": [0.4, 0.6]
```

#### User Journey Comparison
```mermaid
journey
    title Traditional CRM vs Agentic Work Triage
    section Traditional (Square/HoneyBook)
      Receive Inquiry: 5: Customer
      Log in & Read: 3: Owner
      Draft Reply Manually: 2: Owner
      Send Quote: 2: Owner
    section Agentic (OHC Work Triage)
      Receive Inquiry: 5: Customer
      AI Drafts Reply & Quote: 5: Agent
      Review Notification: 4: Owner
      1-Click Approve: 5: Owner
```

### Feature Comparison Table
| Feature | OHC (Proposed) | Tencent Workbuddy | Shopify Sidekick | Square | HoneyBook |
|---|---|---|---|---|---|
| **Autonomous Execution** | Yes (AgentDraft) | Yes | Yes | No | No |
| **Setup Friction** | Zero-setup | Zero-setup | Zero-setup | High | High |
| **Proactive Lead Recovery**| Yes | Partial | No | No | No |
| **Cross-Tool Triage** | Yes | Yes | No (Ecom only) | No | No |

## Design Doc

  **High-Level Architecture**:
  - **Entity Types**:
    - `WorkItem`: Base entity for any inbound customer or system signal.
    - `AgentDraft`: Proactive actions generated by AI, pending owner approval. Maps to a specific `WorkItem`.
  - **Integration Points**:
    - PostgreSQL AI Job Queue: Ingests `WorkItem` events, triggers LLM pipelines, and writes `AgentDraft` records.
    - Redis Redlock: Ensures concurrent webhooks don't generate duplicate drafts for the same `WorkItem`.
    - Flutter PWA: Subscribes to the `AgentDraft` feed via WebSocket or polling.
  - **UI Wireframes (Mobile-First 375px)**:
    - **Work Command Center (Home)**: Replaces standard dashboard. Top section: "Needs Action". Displays translucent, Apple/Ubiquiti-styled cards.
    - **Draft Card**: Shows customer context (e.g., "Carlos missed a call from +123"). Shows AI-proposed text ("Hi, sorry I missed you. Need an estimate?"). Includes a primary 44x44px "Approve & Send" button and a secondary "Edit" button.

  ## Implementation Prompt

  **Critical User Journey (CUJ)**:
  1. The user logs into OHC on their mobile device (375px viewport).
  2. The initial screen is the **Work Triage Feed** (not a generic admin dashboard).
  3. The feed displays an `AgentDraft` card generated by the backend AI queue (e.g., a drafted reply to a missed customer inquiry with an attached booking link).
  4. The user taps the 44x44px "Approve & Send" button.
  5. The system executes the drafted action (sends SMS/email), logs the interaction, and removes the card from the feed.

  **Acceptance Criteria**:
  - A Flutter-based "Work Triage Feed" screen is implemented and set as the default home route.
  - The UI adheres to OHC Premium Token styling (translucent glass, strong spacing).
  - All interactive elements (buttons) must be at least 44x44px.
  - Backend implements `AgentDraft` entity and exposes it via API to populate the feed.
  - The flow MUST be verified via a Playwright E2E test simulating an owner approving an AI draft from the feed.

  ## Estimated Scope
  Medium

  ## Priority
  P1

  ## References & Sources (Appendix)
  1. [Feishu vs DingTalk vs WeCom — Best Collaboration Tools in China](https://www.jetservices.com.cn/blogs/feishu-vs-dingtalk-vs-wecom-best-collaboration-tools-in-china/)
  2. [I'm quite curious, which of the office software, Feishu, DingTalk, and ...](https://www.zhihu.com/en/answer/2749565770)
  3. [Add WeCom and DingTalk Channel Adapters for CherryClaw · Issue ...](https://github.com/CherryHQ/cherry-studio/issues/15169)
  4. [The battle for AI hardware between Feishu and DingTalk - 36氪](https://eu.36kr.com/en/p/3650392831418501)
  5. [Analysis Of WeCom To Help Universities' Digital Transformation ...](https://www.researchgate.net/publication/366297451_Analysis_Of_WeCom_To_Help_Universities'_Digital_Transformation_Strategy)
  6. [Features overview of Feishu Frontline Pro and Standard plans - Lark](https://www.feishu.cn/hc/en-US/articles/643962191288-features-overview-of-feishu-frontline-pro-and-standard-plans)
  7. [Some Stay Out of the Fray, but in the AI Office Arena, We Have No ...](https://eu.36kr.com/en/p/3855349213019014)
  8. [TikTok Parent ByteDance's Work-Collaboration Tool Hit $100 Million ...](https://www.wsj.com/articles/bytedances-work-collaboration-tool-feishu-hit-100-million-in-2022-revenue-2fd960a5)
  9. [Lark, ByteDance's Slack-like app, eyes $1 billion revenue - TechNode](https://technode.com/2021/12/23/lark-bytedances-slack-like-app-eyes-1-billion-global-revenue-in-five-years/)
  10. [Messaging Apps in China: What Works & How to Connect](https://gemspace.com/blog/messaging-apps-that-work-in-china)
  11. [Shopify Sidekick Review: What It Can (and Can't) Do for Your Store (2026)](https://pagefly.io/blogs/shopify/shopify-sidekick)
  12. [AI-enabled commerce assistant, Sidekick, designed to make it ... - Shopify](https://www.shopify.com/sidekick)
  13. [Shopify Sidekick review 2026 · native AI assistant inside the admin](https://botapolis.com/tools/shopify-sidekick)
  14. [Shopify Sidekick 2026: Full Capability Review & Limitations](https://tenten.co/shopify/shopify-sidekick-2026-deep-dive/)
  15. [AI sidekick for Shopify: what it is, what it isn't, and what...](https://www.ringly.io/blog/ai-sidekick-shopify)
  16. [Shopify AI for Small Business: What Sidekick Actually Does in 2026](https://talkerstein.com/articles/shopify-ai-for-small-business)
  17. [Shopify Sidekick Guide: Features, Limits & Better AI Alternatives (2026)](https://www.getmesa.com/blog/shopify-sidekick/)
  18. [What Is Shopify Sidekick? Complete Guide to Shopify's AI Assistant](https://roswell.nyc/insights/shopify-sidekick)
  19. [Shopify Magic & Sidekick AI 2026: 5 Worth Using, 8 To Skip](https://www.adsx.com/blog/shopify-magic-sidekick-ai-features-2026)
  20. [](/clev?event=StartpageResultClick&sc=5mZRtno4IwTTxf6IDd2E53yp948kLsP5dQ5wE5Md4RUcpuddFpnEwI4n8CRxHsTLFnnNkKWNv9e0Z&payload={"bdsSessionId":"a2089b2fb8fa4efd975605a0152924f4","cheqId":"","countryCode":"US","deviceType":"mobile","endpoint":"search.serp","hasGoogleAds":false,"page_id":"iMjZkQskZgyU6G0s","queryCategory":"web","segment":"startpage.udog","session_id":"1fvXxrCoIGDYlxTnz","surface":"serp-web","transport":"href-request"})
  21. [WorkBuddy · Your scenario-based AI all-in-one package](https://www.tencentcloud.com/act/pro/workbuddy)
  22. [WorkBuddy AI Review (2026): The Tencent AI Agent That Works Like](https://www.eigent.ai/blog/workbuddy-ai-review)
  23. [Tencent WorkBuddy AI Agent for Office Professionals - LinkedIn](https://www.linkedin.com/posts/tencent-cloud_tencent-workbuddy-an-ai-native-agent-designed-activity-7465630719772467200-s9_z)
  24. [Tencent Rolls Out New AI Tools and Enterprise Solutions for Global ...](https://www.tencent.com/en-us/articles/2202341.html)
  25. [Tencent has launched WorkBuddy, its AI desktop agent now ...](https://www.facebook.com/wicinternet/posts/tencent-has-launched-workbuddy-its-ai-desktop-agent-now-available-globallyworkbu/1311567207836357/)
  26. [Tencent WorkBuddy — Download, Install & Use Guide (Overseas)](https://www.tencentcloud.com/techpedia/144100?lang=en)
  27. [CodeBuddy（An AI code assistant launched by Tencent Cloud ...](https://baike.baidu.com/en/item/CodeBuddy/1432154)
  28. [Tencent WorkBuddy Tutorial: Complete Office Tasks with One AI Agent](https://www.youtube.com/watch?v=0MtIZXBUgCE)
  29. [Tencent Cloud unveiled its WorkBuddy AI agent and two ... - Instagram](https://www.instagram.com/p/DY6epUMjUKs/)
  30. [CodeBuddy AI Innovation Contest](https://www.codebuddy.ai/genie/blog/34)
  31. [Square Review 2026: Features, Pros And Cons - Forbes](https://www.forbes.com/advisor/business/software/square-review/)
  32. [Shopify POS vs Square: 2026 Review & Comparison - Loman AI](https://loman.ai/blog/shopify-pos-vs-square-review)
  33. [Square Reviews: Real Customer Experiences and Testimonials](https://squareup.com/us/en/reviews)
  34. [Square POS for Retail : r/smallbusiness - Reddit](https://www.reddit.com/r/smallbusiness/comments/xsk0zf/square_pos_for_retail/)
  35. [Best Booking System for Small Businesses in 2025 - 2026 - Lunacal.ai](https://lunacal.ai/blogs/booking-system-small-business)
  36. [Square alternatives: Which platform fits your needs best? - HoneyBook](https://www.honeybook.com/blog/honeybook-vs-square)
  37. [Square Feedback: Importance in A Business's Customer Journey](https://www.questionpro.com/blog/square-feedback/)
  38. [Square Point of Sale: Payment - Apps on Google Play](https://play.google.com/store/apps/details?id=com.squareup&hl=en_US)
  39. [Square Terminal Review & Video - Fit Small Business](https://fitsmallbusiness.com/square-terminal-review/)
  40. [What are pros and cons of Lovingly point of sale software? - Facebook](https://www.facebook.com/groups/383947301667846/posts/7246863698709471/)
  41. [HoneyBook Review: Features, Pros And Cons – Forbes Advisor](https://www.forbes.com/advisor/business/software/honeybook-review/)
  42. [HoneyBook Review: Pros, Cons, Features, and Pricing](https://thecfoclub.com/tools/honeybook-review/)
  43. [2025 Honeybook Review: The Pros, Cons, And If It's Worth It](https://blog.candicecoppola.com/honeybook-review-2025/)
  44. [HoneyBook reviews: Pros, cons, and is it worth it in 2026?](https://assembly.com/blog/honeybook-reviews)
  45. [HoneyBook Reviews 2026. Verified Reviews, Pros & Cons | Capterra](https://www.capterra.com/p/162588/HoneyBook/reviews/)
  46. [HoneyBook Review: Pros, Cons, Features & Pricing](https://thedigitalprojectmanager.com/tools/honeybook-review/)
  47. [HoneyBook Review 2026: Pricing, Features, Pros & Cons ...](https://research.com/software/reviews/honeybook-review)
  48. [HoneyBook Review: Features, Pricing and Alternatives](https://www.techrepublic.com/article/honeybook-review/)
  49. [HoneyBook Pros and Cons | User Likes & Dislikes](https://www.g2.com/products/honeybook/reviews?page=2&qs=pros-and-cons)
  50. [HoneyBook Reviews - Pros & Cons](https://www.joinsecret.com/honeybook/reviews)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: ["agent-report"]
assignees: []
