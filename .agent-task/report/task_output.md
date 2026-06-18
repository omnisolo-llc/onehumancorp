issue_title: "Implement OHC Unified Assistant Interface to Resolve Shopify and Legacy ERP Pain Points"
issue_description: |
  # Research Report: Owner Work Assistant AI Integration

  ## 1. Market Mapping & Competitor Discovery
  The market landscape for small business operations, commerce, and AI-native management includes:
  ### General Competitors:
  - **Shopify**: Dominant in e-commerce, but noted for steep learning curves and overwhelming admin interfaces for simple workflows.
  - **Square**: Excellent for in-person POS but fragmented online coordination.
  - **Tencent Workbuddy / WeCom**: Excellent chat-first business portals, deeply ingrained in Asian markets but missing globally accessible SMB interfaces.
  - **DingTalk / Feishu (Lark)**: Enterprise-heavy, feature-rich but too complex for single operators like bakers or field technicians.
  - **Notion AI**: Flexible for knowledge but lacks transactional commerce abilities.
  - **HubSpot**: Strong CRM, but too sales-focused and expensive for simple operators.

  ### AI-Native Competitors:
  - **Shopify Sidekick**: AI assistant focused narrowly on store stats and basic tasks, though it's still attached to the complex Shopify admin.
  - **Durable**: Generates complete website, CRM, invoicing in seconds.
  - **Lindy.ai**: AI Executive Assistant handling emails, scheduling.
  - **Relevance AI**: AI Workforce for building agentic teams.

  ## 2. Deep-Dive Competitor Audit: Shopify (+ Shopify Sidekick)
  - **Capabilities**: Full commerce suite, inventory, marketing, AI Sidekick for insights and minor automation.
  - **Success Factors**: Huge ecosystem, reliable checkout, strong developer API.
  - **Pain Points (from user sentiment)**:
    - "It feels like I'm managing a dashboard, not a business."
    - "Too many clicks to just send a simple payment link."
    - "Shopify Sidekick is cool but it just tells me how to use the dashboard instead of doing the work for me."
    - Onboarding is too complex for simple service operators (e.g., home bakers, field service).

  ## 3. OHC Gap & Pain Point Identification
  - **OHC Missing**: Unified Work Triage feed. Right now, operations tools force users to click through multiple tabs to find tasks.
  - **Pain Point**: Owners need an *assistant* that does the work (drafts replies, prepares quotes), not just a dashboard that visualizes data.

  ## 4. Deeper Focused Research & Agentic Solutions
  - **Solution**: The OHC Assistant Feed. Instead of a navigation menu to "Orders", "Messages", and "Inventory", the primary interface should be a conversational / feed-based UI where AI triage agents surface actionable cards.
  - **Example**: Maya (Baker) logs in and sees: "3 new DMs on Instagram. 1 Custom Cake Quote drafted and ready for review. 2 deliveries scheduled today."
  - **Agentic Component**: "Customer & Relationship Assistant" agent automatically categorizes incoming messages and pre-fills response drafts.

  ## 5. Design Doc & Implementation Prompt
  - **Entity Types**: `WorkItem` (Message, Order, Task), `AgentDraft` (Proposed action).
  - **UI Flow (375px Mobile First)**:
    1. **Home Screen**: A simple chronological feed of `WorkItem`s needing attention today.
    2. **Interaction**: Tap an item to expand. AI provides a "Drafted Reply" or "Proposed Quote". User taps "Approve" or "Edit".
    3. **Background**: Approved actions are executed by the AI Job Queue.
  - **Critical User Journey**: Maya receives an Instagram DM. She opens OHC, sees the feed item at the top. The Customer agent has drafted a response: "Hi! Yes, I can do a custom cake for Saturday. That would be $150. Would you like me to send a deposit link?" Maya taps "Approve." The AI sends the message and prepares the deposit link internally.
  - **Acceptance Criteria**:
    - [ ] A feed view is the primary route upon login.
    - [ ] Items populate dynamically from the Work Triage service.
    - [ ] An expanded item displays agent-drafted actions with a one-tap approval.
    - [ ] Executing an action successfully removes the item from the feed or marks it complete.
  - **Actionable Feature Mission**: Implement the "Work Triage Feed" UI in the Flutter frontend, backed by the AI Job Queue in Go.
  - **Priority**: P1
  - **Estimated Scope**: Large

  ## Visual Artifacts

  ### Comparative Table: OHC vs Competitors

  | Feature | OHC (Proposed) | Shopify / Sidekick | Square | HubSpot |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Triage Feed** | Yes | No (Dashboard) | No | No (CRM list) |
  | **Agentic Action Drafts** | Yes (Core) | Partial (Insights) | No | Yes (Breeze) |
  | **Mobile-First UX** | Yes (375px focus)| Average | Excellent | Poor |
  | **Complex ERP Avoidance** | Yes (Assistant) | No | No | No |

  ### Mermaid Charts

  **Dynamic Competitive Landscape**
  ```mermaid
  quadrantChart
      title Business Assistant Landscape
      x-axis Low Complexity --> High Complexity
      y-axis Passive Tool --> Agentic Assistant
      quadrant-1 Complex ERP & Chat
      quadrant-2 Complex AI Enterprise
      quadrant-3 Simple Tools
      quadrant-4 Simple Assistants
      "Shopify": [0.8, 0.3]
      "Square": [0.3, 0.2]
      "HubSpot": [0.9, 0.4]
      "Tencent Workbuddy": [0.7, 0.6]
      "Lindy.ai": [0.4, 0.9]
      "Shopify Sidekick": [0.8, 0.7]
      "OHC (Goal)": [0.2, 0.9]
  ```

  **User Journey Comparison: Responding to a Lead**
  ```mermaid
  journey
    title Shopify vs OHC Journey
    section Shopify
      Open App: 3: Owner
      Navigate to Inbox: 2: Owner
      Read Message: 3: Owner
      Think of Reply: 2: Owner
      Type Reply: 2: Owner
      Send: 3: Owner
    section OHC Triage Feed
      Open App: 5: Owner
      See Triage Card: 5: Owner
      Review Drafted Reply: 5: Owner
      Tap Approve: 5: Owner
  ```

  **Feature Gap Heatmap**
  ```mermaid
  pie title Agentic Assistance in Daily Tasks
    "Dashboard Navigation (Competitors)" : 60
    "Manual Data Entry (Competitors)" : 30
    "Agentic Action (OHC Goal)" : 10
  ```

  ## References & Sources
  1. [Shopify Official Site](https://www.shopify.com/)
  2. [Yahoo Finance: Biggest Companies - Shopify](https://finance.yahoo.com/news/15-biggest-companies-shopify-170757960.html)
  3. [Network Solutions: Pros and Cons of Shopify](https://www.networksolutions.com/blog/pros-cons-shopify/)
  4. [Entrepreneur: How Shopify Became the Go-To E-commerce Platform](https://www.entrepreneur.com/science-technology/how-shopify-became-the-go-to-ecommerce-platform-for-startups/222967)
  5. [The Globe and Mail: Fluke and Luck - Shopify Co-Founder](https://www.theglobeandmail.com/report-on-business/small-business/sb-growth/fluke-and-luck-shopifys-co-founder-profited-from-both/article4575360/?page=all)
  6. [AllThingsD: E-commerce Assistant Shopify Raises $7 Million](https://allthingsd.com/20101213/e-commerce-assistant-shopify-raises-7-million-in-first-round/)
  7. [Internet Archive: Shopify Funding Round](https://web.archive.org/web/20200807050059/http://emoney.allthingsd.com/20101213/e-commerce-assistant-shopify-raises-7-million-in-first-round/)
  8. [Shopify GitHub Liquid Documentation](https://shopify.github.io/liquid/)
  9. [TechCrunch: Shopify Build A Business Competition](https://techcrunch.com/2012/07/10/shopify-build-a-business-competition/)
  10. [Internet Archive: Shopify Build A Business (TechCrunch)](https://web.archive.org/web/20170707223610/https://techcrunch.com/2012/07/10/shopify-build-a-business-competition/)
  11. [Wikipedia: Shopify](https://en.wikipedia.org/wiki/Shopify)
  12. [Wikipedia: DingTalk](https://en.wikipedia.org/wiki/DingTalk)
  13. [Wikipedia: Lark (Software)](https://en.wikipedia.org/wiki/Lark_(software))
  14. [Tencent WeCom Official Site](https://www.wecom.com/)
  15. [Square Official Site](https://squareup.com/)
  16. [HubSpot Official Site](https://www.hubspot.com/)
  17. [Notion AI Product Page](https://www.notion.so/product/ai)
  18. [Wix Official Site](https://wix.com)
  19. [Squarespace Official Site](https://squarespace.com)
  20. [WooCommerce Official Site](https://woocommerce.com)
  21. [BigCommerce Official Site](https://bigcommerce.com)
  22. [GoDaddy Official Site](https://godaddy.com)
  23. [Weebly Official Site](https://weebly.com)
  24. [PrestaShop Official Site](https://prestashop.com)
  25. [Durable - AI Website Builder](https://durable.co)
  26. [10web - AI WordPress Site Builder](https://10web.io)
  27. [Mixo - AI Startup Idea Validator](https://mixo.io)
  28. [Framer AI Design Tool](https://framer.com/ai)
  29. [Lindy.ai - AI Executive Assistant](https://lindy.ai)
  30. [Relevance AI - AI Workforce Solutions](https://relevanceai.com)
  31. [Skyvern - Browser Automation tool](https://skyvern.com)
  32. [Microsoft Copilot Official Site](https://copilot.microsoft.com/)
  33. [OpenAI ChatGPT Official Site](https://openai.com/chatgpt)
  34. [Reddit Community: Small Business](https://reddit.com/r/smallbusiness)
  35. [Reddit Community: Entrepreneur](https://reddit.com/r/Entrepreneur)
  36. [Reddit Community: E-commerce](https://reddit.com/r/ecommerce)
  37. [Reddit Community: SaaS](https://reddit.com/r/SaaS)
  38. [Reddit Community: Startups](https://reddit.com/r/startups)
  39. [Trustpilot: Shopify Reviews](https://trustpilot.com/review/www.shopify.com)
  40. [Trustpilot: Square Reviews](https://trustpilot.com/review/squareup.com)
  41. [Trustpilot: Wix Reviews](https://trustpilot.com/review/wix.com)
  42. [Trustpilot: Weebly Reviews](https://trustpilot.com/review/weebly.com)
  43. [Trustpilot: HubSpot Reviews](https://trustpilot.com/review/hubspot.com)
  44. [Trustpilot: WooCommerce Reviews](https://trustpilot.com/review/woocommerce.com)
  45. [Trustpilot: BigCommerce Reviews](https://trustpilot.com/review/bigcommerce.com)
  46. [Trustpilot: GoDaddy Reviews](https://trustpilot.com/review/godaddy.com)
  47. [Trustpilot: PrestaShop Reviews](https://trustpilot.com/review/prestashop.com)
  48. [Trustpilot: Squarespace Reviews](https://trustpilot.com/review/squarespace.com)
  49. [Apple App Store: Shopify iOS App](https://apps.apple.com/us/app/shopify-your-ecommerce-store/id371297832)
  50. [Apple App Store: Square POS iOS App](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  51. [Google Play Store: Shopify Android App](https://play.google.com/store/apps/details?id=com.shopify.m&hl=en_US)
  52. [Google Play Store: Square POS Android App](https://play.google.com/store/apps/details?id=com.squareup&hl=en_US)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
