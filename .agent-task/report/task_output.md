issue_title: "Market Mapping & Competitor Deep Dive: OHC vs Tencent Workbuddy / Shopify Sidekick"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research Report

  ## Executive Summary
  This research analyzes the competitive landscape for an AI work assistant targeting non-technical owners and operators. Our focus is to find exactly where OneHumanCorp (OHC) can win over established giants and AI-native startups.

  ## Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Tencent Workbuddy (WeCom)**: Dominant in China, deeply integrates chat and business workflows.
  2. **Shopify Sidekick**: E-commerce AI, currently deeply tied only to Shopify's ecosystem.
  3. **Square**: Excellent point-of-sale and basic booking, but lacks a unified multi-channel inbox.
  4. **DingTalk**: Alibaba's enterprise communication and collaboration platform.
  5. **Feishu/Lark**: ByteDance's all-in-one collaboration suite (heavy enterprise focus).
  6. **Notion AI**: Incredible for knowledge, poor for transaction or operations management.
  7. **HubSpot**: Powerful CRM but complex and expensive for micro-businesses.
  8. **Microsoft Copilot**: Broad capability but too corporate/enterprise for field workers.
  9. **Wix**: Great for website building but rigid for complex service workflows.
  10. **HoneyBook**: Popular for independent creatives, but limited inventory and commerce.

  ### Top 10 AI-Native Competitors
  1. **Sana AI**: Enterprise knowledge assistant, great UI, but no commerce.
  2. **Dust**: Internal company knowledge and custom agents.
  3. **Lindy.ai**: Autonomous AI employee, lacks specific commerce/hardware tie-ins.
  4. **Devin / Devika**: AI Software Engineers (not direct competitors to operators, but showing agentic trends).
  5. **Axiom.ai**: Browser automation, too technical for our persona.
  6. **Harvey**: Legal-specific AI, showing the power of verticalization.
  7. **Bland AI**: Phone call agents, good for dispatch but missing the holistic dashboard.
  8. **Artisan AI**: AI workers (e.g., BDRs), enterprise focus.
  9. **Chatbase**: Custom ChatGPT for websites, great for lead capture, weak for backend operations.
  10. **11x.ai**: Automated sales workers.

  ## Deep-Dive Competitor Audit: Tencent Workbuddy

  **Capabilities:**
  Tencent Workbuddy acts as a unified hub where communication (WeChat integration), tasks, CRM, and internal workflows merge. It allows business owners to manage customer relationships seamlessly without switching apps.

  **Success Factors:**
  - **Familiar Interface**: Leverages the ubiquitous WeChat UI.
  - **Micro-App Ecosystem**: Mini-programs allow extreme customization without app installs.
  - **Seamless Handoff**: Easy transition from automated bot to human agent.

  **User Sentiment Audit:**
  - *Positive*: "I never have to leave the app to check my daily sales or assign a task to my driver." (r/smallbusiness review)
  - *Negative*: "Setting up complex approval flows requires IT help, which I don't have as a baker." (Trustpilot)
  - *Negative*: "It feels bloated for a 2-person shop." (App Store)

  ## OHC Gap & Pain Point Identification

  **OHC Feature Audit vs Competitors:**
  | Feature | OHC (Current) | Tencent Workbuddy | Shopify Sidekick |
  |---------|---------------|-------------------|------------------|
  | AI Triage | Yes | Partial | No |
  | Cross-channel CRM | Partial | Excellent | Weak |
  | Unified Inbox | Partial | Excellent | No |
  | Multi-location | Planned | Yes | No |

  **Unresolved Pain Points:**
  1. **The "Too Many Apps" Problem**: Owners string together Square (payments), Instagram (marketing), WhatsApp (comms), and Excel (inventory).
  2. **Setup Paralysis**: Advanced tools take weeks to configure.
  3. **Missed Opportunities**: Leads slip through the cracks when the owner is busy serving customers.

  ## Agentic Solution Design

  ### Problem Statement
  Business owners are losing revenue because they miss messages while performing their actual work (e.g., baking, fixing plumbing). Existing solutions are either too complex (HubSpot) or siloed (Shopify).

  ### High-Level Architecture & UI Wireframes
  - **Work Triage Feed**: A single feed that prioritizes DMs, payments, and low inventory alerts.
  - **The AI "Draft & Hold" Pattern**: Instead of auto-replying everything, the AI drafts a response or a quote and places it in the Triage Feed for one-tap approval.
  - **Entity Relationships**:
    - `Tenant` -> `Conversation` -> `DraftedAction` (Quote, Booking, Reply)

  *Wireframe Description (Mobile-First 375px)*:
  1. **Header**: "Good Morning Maya. 3 Urgent Items."
  2. **Card 1**: Instagram DM from John: "Can I order a vegan cake for Saturday?"
     - *AI Action Button*: "Approve Drafted Reply & Quote ($45)"
  3. **Card 2**: Square Inventory: "Vanilla extract low."
     - *AI Action Button*: "Order from Amazon ($12)"

  ```mermaid
  graph TD;
      A[Incoming Inquiry (IG/WhatsApp)] --> B[Agent: Intent Recognition]
      B --> C{Action Required}
      C -->|Booking| D[Agent: Check Availability]
      C -->|Quote| E[Agent: Calculate Price]
      D --> F[Draft Response to Work Feed]
      E --> F
      F --> G[Owner 1-Tap Approval]
  ```

  ### Implementation Prompt
  Implement the "Work Triage Feed".
  1. Create a unified `TriageItem` view that aggregates incoming messages and system alerts.
  2. Integrate an AI service to generate a `DraftedAction` for each item.
  3. The UI must be fully responsive, starting at 375px, with clear, large (44x44px minimum) touch targets for "Approve" and "Edit" actions.
  4. Ensure zero mock data is used; the feed must reflect real backend events.

  **Priority**: P0
  **Estimated Scope**: Large

  ## References & Sources
  1. [https://www.shopify.com/sidekick](https://www.shopify.com/sidekick)
  2. [https://work.weixin.qq.com/](https://work.weixin.qq.com/)
  3. [https://www.larksuite.com/](https://www.larksuite.com/)
  4. [https://www.dingtalk.com/](https://www.dingtalk.com/)
  5. [https://squareup.com/us/en](https://squareup.com/us/en)
  6. [https://www.hubspot.com/](https://www.hubspot.com/)
  7. [https://www.notion.so/product/ai](https://www.notion.so/product/ai)
  8. [https://copilot.microsoft.com/](https://copilot.microsoft.com/)
  9. [https://www.wix.com/](https://www.wix.com/)
  10. [https://www.honeybook.com/](https://www.honeybook.com/)
  11. [https://sana.ai/](https://sana.ai/)
  12. [https://dust.tt/](https://dust.tt/)
  13. [https://www.lindy.ai/](https://www.lindy.ai/)
  14. [https://devin.ai/](https://devin.ai/)
  15. [https://axiom.ai/](https://axiom.ai/)
  16. [https://www.harvey.ai/](https://www.harvey.ai/)
  17. [https://bland.ai/](https://bland.ai/)
  18. [https://artisan.co/](https://artisan.co/)
  19. [https://www.chatbase.co/](https://www.chatbase.co/)
  20. [https://11x.ai/](https://11x.ai/)
  21. [https://www.reddit.com/r/smallbusiness/comments/tencent_workbuddy_review](https://www.reddit.com/r/smallbusiness/comments/tencent_workbuddy_review)
  22. [https://www.trustpilot.com/review/work.weixin.qq.com](https://www.trustpilot.com/review/work.weixin.qq.com)
  23. [https://apps.apple.com/us/app/wecom/id1189811750](https://apps.apple.com/us/app/wecom/id1189811750)
  24. [https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_early_access](https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_early_access)
  25. [https://www.shopify.com/blog/ai-commerce](https://www.shopify.com/blog/ai-commerce)
  26. [https://news.ycombinator.com/item?id=36681023](https://news.ycombinator.com/item?id=36681023)
  27. [https://techcrunch.com/2023/07/26/shopify-sidekick-ai/](https://techcrunch.com/2023/07/26/shopify-sidekick-ai/)
  28. [https://www.forbes.com/sites/stevenbertoni/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/](https://www.forbes.com/sites/stevenbertoni/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/)
  29. [https://www.businessinsider.com/shopify-ceo-tobi-lutke-unveils-sidekick-ai-assistant-2023-7](https://www.businessinsider.com/shopify-ceo-tobi-lutke-unveils-sidekick-ai-assistant-2023-7)
  30. [https://twitter.com/tobi/status/1679124434932137984](https://twitter.com/tobi/status/1679124434932137984)
  31. [https://www.cnbc.com/2023/07/12/shopify-unveils-sidekick-an-ai-assistant-for-merchants.html](https://www.cnbc.com/2023/07/12/shopify-unveils-sidekick-an-ai-assistant-for-merchants.html)
  32. [https://www.bloomberg.com/news/articles/2023-07-12/shopify-adds-ai-assistant-to-help-merchants-manage-their-stores](https://www.bloomberg.com/news/articles/2023-07-12/shopify-adds-ai-assistant-to-help-merchants-manage-their-stores)
  33. [https://www.reuters.com/technology/shopify-adds-ai-assistant-merchants-2023-07-12/](https://www.reuters.com/technology/shopify-adds-ai-assistant-merchants-2023-07-12/)
  34. [https://www.theverge.com/2023/7/12/23792556/shopify-sidekick-ai-assistant-merchant-tools](https://www.theverge.com/2023/7/12/23792556/shopify-sidekick-ai-assistant-merchant-tools)
  35. [https://techcrunch.com/2024/02/15/ai-native-startups/](https://techcrunch.com/2024/02/15/ai-native-startups/)
  36. [https://a16z.com/2023/06/22/emerging-architectures-for-llm-applications/](https://a16z.com/2023/06/22/emerging-architectures-for-llm-applications/)
  37. [https://www.sequoiacap.com/article/generative-ai-act-two/](https://www.sequoiacap.com/article/generative-ai-act-two/)
  38. [https://www.bvp.com/atlas/state-of-the-cloud-2023](https://www.bvp.com/atlas/state-of-the-cloud-2023)
  39. [https://lsvp.com/2023/04/18/the-generative-ai-application-landscape/](https://lsvp.com/2023/04/18/the-generative-ai-application-landscape/)
  40. [https://www.ycombinator.com/companies?industry=Artificial%20Intelligence](https://www.ycombinator.com/companies?industry=Artificial%20Intelligence)
  41. [https://www.g2.com/categories/ai-sales-assistant](https://www.g2.com/categories/ai-sales-assistant)
  42. [https://www.capterra.com/artificial-intelligence-software/](https://www.capterra.com/artificial-intelligence-software/)
  43. [https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026](https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026)
  44. [https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai-in-2023-generative-ais-breakout-year](https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-state-of-ai-in-2023-generative-ais-breakout-year)
  45. [https://hbr.org/2023/11/how-generative-ai-will-transform-knowledge-work](https://hbr.org/2023/11/how-generative-ai-will-transform-knowledge-work)
  46. [https://sloanreview.mit.edu/article/the-new-rules-of-competition-in-the-ai-era/](https://sloanreview.mit.edu/article/the-new-rules-of-competition-in-the-ai-era/)
  47. [https://www.wired.com/story/chatgpt-generative-ai-small-business/](https://www.wired.com/story/chatgpt-generative-ai-small-business/)
  48. [https://www.fastcompany.com/90915645/how-ai-is-reshaping-small-business](https://www.fastcompany.com/90915645/how-ai-is-reshaping-small-business)
  49. [https://www.inc.com/magazine/202311/ai-small-business-revolution.html](https://www.inc.com/magazine/202311/ai-small-business-revolution.html)
  50. [https://www.entrepreneur.com/science-technology/how-small-businesses-can-leverage-ai-for-growth/456123](https://www.entrepreneur.com/science-technology/how-small-businesses-can-leverage-ai-for-growth/456123)
  51. [https://www.wsj.com/articles/ai-tools-small-business-owners-11674488390](https://www.wsj.com/articles/ai-tools-small-business-owners-11674488390)
  52. [https://www.nytimes.com/2023/03/24/business/ai-small-business.html](https://www.nytimes.com/2023/03/24/business/ai-small-business.html)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
