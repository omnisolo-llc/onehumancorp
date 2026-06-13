issue_title: "Implement Tencent Workbuddy-style Agentic Conversational Workspace UI"
issue_description: |
  # OHC Research: Agentic Conversational Workspace

  ## Problem Statement
  Small business owners and operators (like Maya the baker and Fatima the food cart owner) are overwhelmed by navigating complex SaaS menus, dashboards, and settings. They don't want to "use software"; they want to *get work done*. Currently, traditional competitors force users to act as system administrators rather than business operators. OHC needs to bridge this gap by adopting a conversational, agent-first workspace akin to Tencent Workbuddy or WeChat Work, but enhanced with proactive AI.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the landscape of both traditional platforms and emerging AI-native tools by evaluating over 50 websites, community discussions (Reddit r/smallbusiness), and app store reviews. Traditional tools like Shopify and Square require extensive navigation, while tools like Slack or DingTalk are chat-centric but lack native commerce operations.

  ### Track 2: Deep-Dive Competitor Audit (Tencent Workbuddy / WeCom)
  **Tencent WeCom / Workbuddy** excels because it unifies communication and operations. The interface is primarily a chat/feed, but "mini-programs" and bots within the chat can execute complex business logic (approvals, payments, CRM updates) without the user leaving the conversation.
  - **Success Factors:** Extreme mobile-friendliness (375px native), zero learning curve (everyone knows how to chat), and contextual actions (buttons directly in the chat feed).
  - **User Sentiment:** Users love that they can run their entire day from one feed (e.g., "I just open WeCom and click approve, takes 5 seconds" - App Store review), but complain when bots are too rigid or require exact command syntax.

  ### Track 3: OHC Gap Matrix
  | Feature | Legacy SaaS (Shopify/Square) | WeCom / DingTalk | OHC Target (Agentic) |
  |---------|------------------------------|------------------|----------------------|
  | Interface | Dashboards & Menus | Chat & Mini-apps | Unified Agent Feed |
  | Proactivity | Low (Dashboards) | Medium (Alerts) | High (Drafts actions) |
  | Commerce Ops | Deep | Shallow | Deep + Conversational |

  ### Track 4: Agentic Solutions
  OHC must replace the traditional "Dashboard" home screen with an **Agent Work Feed**. Instead of a static dashboard of charts, the owner opens OHC to a conversational feed where the AI Assistant (Work Triage) presents grouped actionable items.

  ```mermaid
  graph TD
      A[Owner Opens App] --> B[Assistant Feed]
      B --> C{Agent Drafts Action}
      C --> D[Customer Inquiry Card]
      C --> E[Quote Approval Card]
      C --> F[Daily Summary Card]
      D --> G[Tap 'Draft Reply']
      E --> H[Tap 'Approve']
      F --> I[Tap 'View Details']
      G --> J[Action Executed via AI]
      H --> J
      I --> J
  ```

  ## Design Doc
  - **UI Concept:** The main screen is the "Assistant Feed". It looks more like a modern messaging app (like iMessage or WhatsApp) mixed with interactive widgets.
  - **Entity Flow:**
    - `AgentMessage`: Contains text, but also `ActionableCards`.
    - `ActionableCard`: E.g., "Draft Reply to Maya", "Approve Quote for Carlos".
  - **Mobile UX (375px):** Bottom navigation bar (Feed, Customers, Ops, Money). The Feed dominates. Tap a card to expand it inline or open a bottom sheet.

  ```mermaid
  sequenceDiagram
      participant User
      participant Agent_UI
      participant Work_Triage_AI
      participant Commerce_Engine

      User->>Agent_UI: Opens App
      Agent_UI->>Work_Triage_AI: Fetch Pending Actions
      Work_Triage_AI->>Agent_UI: Return Actionable Cards
      User->>Agent_UI: Taps 'Approve Quote'
      Agent_UI->>Commerce_Engine: Execute Approval
      Commerce_Engine-->>Agent_UI: Success
      Agent_UI-->>User: Card Updates to 'Approved'
  ```

  ## Implementation Prompt
  Implement the foundational "Agent Work Feed" UI for the mobile web/PWA (targeting 375px width).
  - Create a chat-like feed interface that renders distinct `ActionMessage` components.
  - Implement at least 3 types of interactive cards within the feed:
    1. **Customer Inquiry Card:** Shows a snippet of a message and a "Draft Reply" button.
    2. **Approval Card:** E.g., "Approve $500 Quote for John Doe" with Accept/Edit buttons.
    3. **Daily Summary Card:** A brief text summary of yesterday's sales with a "View Details" button.
  - Ensure the UI adheres to the OHC Premium Token library (translucent materials, clean hierarchy).
  - The feature should be fully usable and verified in the browser.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources
  1. [Shopify Official Blog: What is Shopify](https://www.shopify.com/blog/what-is-shopify)
  2. [Square Town Square: Business Insights](https://squareup.com/us/en/townsquare)
  3. [Tencent WeCom Official Site](https://work.weixin.qq.com/)
  4. [DingTalk Official Site](https://www.dingtalk.com/en)
  5. [Lark Suite Official Site](https://www.larksuite.com/)
  6. [Reddit r/smallbusiness: Tired of Dashboard Fatigue Managing My Store](https://www.reddit.com/r/smallbusiness/comments/18z7x9p/tired_of_dashboard_fatigue_managing_my_store/)
  7. [Reddit r/Entrepreneur: What CRM do you actually use from your phone?](https://www.reddit.com/r/Entrepreneur/comments/14m9t2q/what_crm_do_you_actually_use_from_your_phone/)
  8. [Trustpilot: Shopify Reviews](https://trustpilot.com/review/www.shopify.com)
  9. [Trustpilot: Square Reviews](https://trustpilot.com/review/squareup.com)
  10. [WeChat Official Site](https://www.wechat.com/)
  11. [HubSpot Breeze: AI Agents for CRM](https://hubspot.com/breeze)
  12. [Wix Studio AI Capabilities](https://wix.com/studio/ai)
  13. [Squarespace Blueprint AI Tool](https://squarespace.com/blueprint)
  14. [WooCommerce AI Features](https://woocommerce.com/ai)
  15. [BigCommerce Predictive Analytics](https://bigcommerce.com/articles/ecommerce/predictive-analytics)
  16. [GoDaddy Airo: Automated Branding](https://godaddy.com/airo)
  17. [Weebly E-commerce Builder](https://weebly.com)
  18. [PrestaShop E-commerce Modules](https://prestashop.com)
  19. [Durable AI Website Builder](https://durable.co)
  20. [10Web AI WordPress Manager](https://10web.io)
  21. [Mixo AI Landing Page Builder](https://mixo.io)
  22. [Framer AI Design Output](https://framer.com/ai)
  23. [Lindy.ai Executive Assistant](https://lindy.ai)
  24. [Relevance AI Workforce](https://relevanceai.com)
  25. [Skyvern Browser Automation](https://skyvern.com)
  26. [Reddit r/SaaS: AI Agents Are the New Dashboards](https://www.reddit.com/r/SaaS/comments/17q3b2z/ai_agents_are_the_new_dashboards/)
  27. [Reddit r/smallbusiness: How much time do you spend on admin work?](https://www.reddit.com/r/smallbusiness/comments/12c5x9m/how_much_time_do_you_spend_on_admin_work/)
  28. [Linktree Creator Links](https://linktr.ee/)
  29. [Stan Store Monetization Tool](https://stan.store/)
  30. [Beacons AI Link in Bio](https://beacons.ai/)
  31. [TechCrunch: The Future of Software is Agentic](https://techcrunch.com/2023/10/12/the-future-of-software-is-agentic/)
  32. [a16z: Emerging Architectures for LLM Applications](https://a16z.com/2023/06/20/emerging-architectures-for-llm-applications/)
  33. [Y Combinator: B2B SaaS Directory](https://www.ycombinator.com/companies/industry/b2b-saas)
  34. [Hacker News Discussion on Agentic Workflows](https://news.ycombinator.com/item?id=38102931)
  35. [Hacker News Discussion on Small Business Software](https://news.ycombinator.com/item?id=37894215)
  36. [Apple App Store: Shopify POS](https://apps.apple.com/us/app/shopify-point-of-sale-pos/id663008892)
  37. [Apple App Store: Square POS](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  38. [Apple App Store: WeCom](https://apps.apple.com/us/app/wecom/id1189871149)
  39. [Apple App Store: DingTalk](https://apps.apple.com/us/app/dingtalk/id930368978)
  40. [Google Play: Shopify Mobile App](https://play.google.com/store/apps/details?id=com.shopify.mobile)
  41. [Google Play: Square Point of Sale](https://play.google.com/store/apps/details?id=com.squareup)
  42. [Google Play: Tencent WeCom App](https://play.google.com/store/apps/details?id=com.tencent.wework)
  43. [Google Play: DingTalk Android App](https://play.google.com/store/apps/details?id=com.alibaba.android.rimet)
  44. [G2: Top E-commerce Platforms](https://www.g2.com/categories/e-commerce-platforms)
  45. [G2: Top CRM Software](https://www.g2.com/categories/crm)
  46. [Capterra: Retail Management Systems](https://www.capterra.com/retail-management-systems/)
  47. [Software Advice: Retail Software Solutions](https://www.softwareadvice.com/retail/)
  48. [Stripe Terminal Documentation](https://stripe.com/docs/terminal)
  49. [Stripe Billing Documentation](https://stripe.com/docs/billing)
  50. [Google Cloud Vertex AI Platform](https://cloud.google.com/vertex-ai)
  51. [OpenAI Enterprise Offerings](https://openai.com/enterprise)
  52. [Anthropic Claude Product Page](https://www.anthropic.com/product)
  53. [Playwright End-to-End Testing Docs](https://playwright.dev/docs/intro)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
