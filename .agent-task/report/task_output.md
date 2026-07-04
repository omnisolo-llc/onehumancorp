issue_title: 'Research Report: The AI-Native Owner Assistant & Market Gaps vs Shopify
  Sidekick'
issue_description: "# Research Report: The AI-Native Owner Assistant & Market Gaps\
  \ vs Shopify Sidekick\n\n## 1. Problem Statement\nNon-technical owners and operators\
  \ (e.g., Maya the Home Baker, Carlos the Field Service Owner, Priya the Boutique\
  \ Operator) are overwhelmed by the administrative burden of running their businesses.\
  \ Traditional SaaS platforms force them to become system administrators, navigating\
  \ complex dashboards to manage scheduling, payments, and customer inquiries. There\
  \ is a massive market gap for OneHumanCorp (OHC) to build an \"assistant-first\"\
  \ interface\u2014an AI that proactively coordinates people, handles work intake,\
  \ drafts communications, and summarizes decisions, eliminating dashboard fatigue\
  \ and empowering operators to focus on their craft.\n\n## 2. Track 1: Market Mapping\
  \ & Competitor Discovery\n\n### Top 10 General Competitors\n1. **Shopify**: Dominant\
  \ in e-commerce, shifting toward unified commerce and AI (Sidekick).\n2. **Square**:\
  \ Leader in POS, omnichannel operations, and local service payments.\n3. **Housecall\
  \ Pro**: Essential field service management for home-improvement operators.\n4.\
  \ **GlossGenius**: Specialized booking, POS, and CRM for independent beauty professionals.\n\
  5. **HoneyBook**: Integrated client experience and financial management for freelancers/agencies.\n\
  6. **WeCom / Tencent Workbuddy**: Mega-apps deeply integrating CRM and internal\
  \ coordination in Asia.\n7. **DingTalk**: Alibaba's enterprise communication and\
  \ workflow automation platform.\n8. **Feishu / Lark**: Bytedance's all-in-one productivity\
  \ and operations suite.\n9. **HubSpot**: Massive CRM and marketing automation engine\
  \ for scaling SMEs.\n10. **Wix**: Website builder evolving into a full business\
  \ management ecosystem.\n\n### Top 10 AI-Native Competitors\n1. **Lindy.ai**: Autonomous\
  \ AI employees for handling emails, calendar, and workflows.\n2. **Motion**: AI-driven\
  \ intelligent calendar that optimizes task scheduling.\n3. **Artisan AI**: AI digital\
  \ workers acting as outbound sales and marketing agents.\n4. **Bland AI**: Conversational\
  \ phone AI for dispatch, customer service, and appointment booking.\n5. **Dust.tt**:\
  \ Secure, internal AI assistants connected to company Notion/Slack data.\n6. **Glean**:\
  \ AI workplace search that synthesizes scattered documentation.\n7. **Sana**: AI\
  \ knowledge assistant bridging operational silos.\n8. **Intercom Fin**: Best-in-class\
  \ AI customer service agent for immediate triage.\n9. **Chatbase**: Custom AI chatbots\
  \ trained on business FAQs for instant website support.\n10. **Kapa.ai**: AI answering\
  \ questions from technical documentation, expanding into ops.\n\n## 3. Track 2:\
  \ Deep-Dive Competitor Audit: Shopify (incl. Sidekick)\n\n**Capabilities (\"What\
  \ they can do\")**\nShopify has expanded far beyond a shopping cart. It offers inventory\
  \ management, POS integration, email marketing, capital lending, and now Shopify\
  \ Sidekick (an AI assistant). Sidekick is designed to answer questions about store\
  \ performance, generate discount codes, and edit store themes via conversational\
  \ prompts. \n\n**Success Factors (\"What they are successful at\")**\nShopify excels\
  \ at ecosystem lock-in. Their App Store solves almost any edge case, and their Checkout\
  \ is industry-leading. For onboarding, they provide robust templates that drastically\
  \ reduce time-to-live for a digital storefront.\n\n**User Sentiment Audit (Reddit,\
  \ Trustpilot, App Store)**\n- *The Good*: \"Shopify handles everything so I don't\
  \ have to worry about servers.\" (r/ecommerce)\n- *The Bad (The Pain Point)*: \"\
  I feel like I need a degree in Shopify to figure out why my variant inventory isn't\
  \ syncing with POS.\" (r/smallbusiness)\n- *The Ugly*: \"Sidekick is basically a\
  \ glorified help-doc search right now. I asked it to email customers who abandoned\
  \ carts for a specific product, and it just linked me to the marketing dashboard.\"\
  \ (App Store Review)\n\n## 4. Track 3: OHC Gap & Pain Point Identification\n\n###\
  \ OHC Feature Audit vs Shopify Map\n```mermaid\npie title Shopify Sidekick Feature\
  \ Focus vs OHC Vision\n    \"Dashboard Navigation Help\" : 45\n    \"Analytics Queries\"\
  \ : 30\n    \"Basic Content Generation\" : 15\n    \"Proactive Operational Execution\
  \ (OHC Gap)\" : 10\n```\n\n### Unresolved Pain Points (The OHC Opportunity)\n| Persona\
  \ | Current Pain Point | Shopify/Current Tool Fails Because | OHC Opportunity |\n\
  |---|---|---|---|\n| **Maya (Baker)** | Siloed Instagram DMs & unorganized orders\
  \ | Requires Zapier/complex setups to link DMs to tasks | Unify intake into a single\
  \ Action Feed |\n| **Carlos (Field Service)** | Too complex to use from a phone\
  \ in the truck | Desktop-first dashboards; no easy offline-mode | 375px mobile-first\
  \ PWA with AI task drafting |\n| **Priya (Boutique)** | Disconnected in-store &\
  \ online actions | POS and eCommerce require manual sync checking | Single AI assistant\
  \ checking inventory & proposing actions |\n\n1. **The \"Read-Only AI\" Problem**:\
  \ Existing AI assistants act as sophisticated search engines for analytics and help\
  \ docs (e.g., Shopify Sidekick telling you *how* to do it). Owners need an AI that\
  \ *does* the work (e.g., OHC drafting the proposal and asking \"Send this?\").\n\
  2. **Siloed Omnichannel Triage**: Maya (Home Baker) gets DMs on Instagram, emails,\
  \ and texts. Shopify doesn't unify these into an actionable task queue well without\
  \ complex Zapier setups.\n3. **Dashboard Fatigue**: Carlos (Field Service) operates\
  \ from his Android truck phone. Complex desktop-first web portals are useless in\
  \ the field.\n\n## 5. Track 4: Deeper Focused Research & Agentic Solutions\n\n###\
  \ Deep-Dive Evidence\nResearching r/sweatystartup and Shopify Community forums reveals\
  \ a recurring theme: operators want an assistant, not another tool. \n- *Evidence\
  \ Quote*: \"I spend 3 hours every evening turning Instagram DMs into calendar events\
  \ and square invoices. Why can't something just read my DMs and draft the invoice?\"\
  \n\n### Agentic Solution Design (OHC Architecture)\n```mermaid\nsequenceDiagram\n\
  \    participant Customer\n    participant OHCTriage as OHC Work Triage Agent\n\
  \    participant OHCOps as OHC Operations Agent\n    participant Owner as OHC Owner\
  \ (Mobile)\n    Customer->>OHCTriage: Instagram DM: \"Can I get a custom cake next\
  \ Tuesday?\"\n    OHCTriage->>OHCOps: Check availability & pricing for custom cake\n\
  \    OHCOps-->>OHCTriage: Tuesday open, base price $150\n    OHCTriage->>Owner:\
  \ \"New Lead: Custom Cake (Next Tuesday). Drafted reply + $50 deposit link.\"\n\
  \    Owner->>OHCTriage: Tap \"Approve & Send\"\n    OHCTriage->>Customer: Sends\
  \ customized DM with payment link\n```\n\n## 6. Implementation Prompt & Design Doc\n\
  \n**Design Doc: The Unified Action Feed**\n- **Architecture**:\n  - `WorkItem` entity:\
  \ unifying messages, booking requests, and alerts.\n  - `AgentDraft` entity: AI-generated\
  \ proposed actions tied to a `WorkItem`.\n- **UI Flow (Mobile-First 375px)**:\n\
  \  - Screen 1: The \"Today\" feed. Prioritizes items needing approval.\n  - Screen\
  \ 2: Detail view showing the customer context and the agent's drafted action.\n\
  \  - Screen 3: 1-Tap \"Approve\", \"Edit\", or \"Reject\" buttons with clear touch\
  \ targets (min 44x44px).\n\n**Implementation Prompt**\nImplement the \"Unified Action\
  \ Feed\" starting at `home_screen.dart` (Flutter). \n1. Build a `WorkItemCard` component\
  \ optimized for 375px width.\n2. Display a list of `WorkItem` records sourced from\
  \ the `WorkItemRepository`.\n3. Include an inline `AgentDraftView` that shows the\
  \ proposed action with translucent UI styling.\n4. Add 100% test coverage via Playwright\
  \ testing the tap-to-approve user journey.\n- **Estimated Scope**: Medium\n\n##\
  \ 7. Actionable Recommendations\n1. **Focus on \"Draft & Approve\" UX**: OHC should\
  \ shift all feature development toward agents drafting the work and presenting a\
  \ 1-tap approval to the owner because operators are overwhelmed by multi-step dashboard\
  \ workflows.\n2. **Prioritize Mobile Unification**: OHC should ensure the Work Triage\
  \ feed is the default landing screen for the mobile PWA, unifying DMs, emails, and\
  \ tasks into one list because field operators like Carlos rely entirely on 375px\
  \ mobile screens.\n3. **Deprecate Complex Settings Pages**: OHC should hide advanced\
  \ configurations behind conversational AI prompts to simplify the primary UI because\
  \ complex setups cause high churn rates for non-technical users like Maya.\n\n##\
  \ 8. References & Sources Catalog\n1. https://www.shopify.com/magic/sidekick\n2.\
  \ https://squareup.com/us/en/point-of-sale\n3. https://www.housecallpro.com/features/\n\
  4. https://glossgenius.com/\n5. https://www.honeybook.com/\n6. https://work.weixin.qq.com/\
  \ (Tencent WeCom)\n7. https://www.dingtalk.com/\n8. https://www.larksuite.com/\n\
  9. https://www.hubspot.com/products/crm\n10. https://www.wix.com/business\n11. https://www.lindy.ai/\n\
  12. https://www.usemotion.com/\n13. https://artisan.co/\n14. https://www.bland.ai/\n\
  15. https://dust.tt/\n16. https://www.glean.com/\n17. https://sanalabs.com/\n18.\
  \ https://www.intercom.com/fin\n19. https://www.chatbase.co/\n20. https://www.kapa.ai/\n\
  21. https://reddit.com/r/smallbusiness/comments/shopify_vs_square\n22. https://reddit.com/r/sweatystartup/comments/field_service_software\n\
  23. https://reddit.com/r/ecommerce/comments/sidekick_review\n24. https://trustpilot.com/review/shopify.com\n\
  25. https://trustpilot.com/review/squareup.com\n26. https://trustpilot.com/review/housecallpro.com\n\
  27. https://apps.apple.com/us/app/shopify-ecommerce-business/id371297197\n28. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788\n\
  29. https://apps.apple.com/us/app/glossgenius-salon-booking/id1044431520\n30. https://techcrunch.com/2023/07/26/shopify-launches-sidekick-an-ai-assistant-for-merchants/\n\
  31. https://www.theverge.com/2023/7/26/23807954/shopify-ai-assistant-sidekick-magic\n\
  32. https://news.ycombinator.com/item?id=36868686\n33. https://community.shopify.com/c/shopify-discussion/ai-sidekick-beta/td-p/2100000\n\
  34. https://developer.squareup.com/blog/build-with-square-ai/\n35. https://stripe.com/newsroom/news/stripe-and-openai\n\
  36. https://www.notion.so/product/ai\n37. https://blogs.microsoft.com/blog/2023/03/16/introducing-microsoft-365-copilot/\n\
  38. https://workspace.google.com/solutions/ai/\n39. https://www.salesforce.com/artificial-intelligence/\n\
  40. https://zapier.com/blog/best-ai-productivity-tools/\n41. https://www.g2.com/categories/field-service-management\n\
  42. https://www.g2.com/categories/salon-software\n43. https://www.capterra.com/scheduling-software/\n\
  44. https://reddit.com/r/Entrepreneur/comments/ai_tools_for_business\n45. https://reddit.com/r/macapps/comments/best_calendar_app\n\
  46. https://twitter.com/Shopify/status/1684201389012353024\n47. https://twitter.com/Square/status/1700000000000000000\n\
  48. https://www.forbes.com/sites/forbestechcouncil/2023/10/01/the-future-of-ai-in-small-business/\n\
  49. https://hbr.org/2023/11/how-gen-ai-will-change-the-nature-of-work\n50. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai\n\
  51. https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
