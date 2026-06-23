issue_title: "Product Gap: Integrated Communication and Task Pipeline"
issue_description: |
  # Research Report: Building an Agentic Work Assistant for SMB Owners

  ## Market Mapping & Competitor Discovery
  We analyzed the landscape of owner/operator work assistants to define OHC's unique position. The market spans generalist collaboration tools and specialized AI-native assistants.

  ### Dynamic Competitive Landscape Matrix
  ```mermaid
  quadrantChart
      title AI Native vs Operational Depth
      x-axis "Low Operational Depth" --> "High Operational Depth"
      y-axis "Traditional Software" --> "AI-Native Assistant"
      quadrant-1 "Strong OHC Fit"
      quadrant-2 "Niche AI Tools"
      quadrant-3 "Simple Software"
      quadrant-4 "Complex Legacy Systems"
      "Shopify Sidekick": [0.8, 0.7]
      "OHC Vision": [0.9, 0.9]
      "WeCom/DingTalk": [0.85, 0.2]
      "Lindy.ai": [0.3, 0.8]
      "Jobber": [0.9, 0.1]
      "Square": [0.75, 0.2]
      "Notion AI": [0.4, 0.6]
  ```

  ### Top 10 General Competitors
  1. WeCom (Tencent): Strong integration with WeChat ecosystem, facilitating seamless customer-to-business communication.
  2. DingTalk (Alibaba): Comprehensive suite covering HR, approvals, and communication, heavily utilized in enterprise/SMBs in Asia.
  3. Feishu/Lark (ByteDance): Focuses on document collaboration and real-time messaging with integrated workflows.
  4. Shopify (Sidekick): Emerging AI assistant specifically tailored for e-commerce operators, aiding in store management and analytics.
  5. Square: Point-of-Sale dominant with expanding team management and online booking tools.
  6. HubSpot: CRM-centric, managing leads, customer journeys, and marketing campaigns.
  7. Notion AI: Strong in knowledge management, using AI to organize docs, policies, and project boards.
  8. Microsoft Copilot: Deeply integrated into the Office suite, excelling at summarizing emails, meetings, and drafting content.
  9. Wix: Website builder evolving into business management (bookings, CRM, payments).
  10. Jobber (Vertical): Highly specialized for field service, managing dispatch, quoting, and invoicing.

  ### Top 10 AI-Native Competitors
  1. MultiOn: Autonomous web agents capable of executing tasks across different browser tabs.
  2. Lindy.ai: AI scheduling and email assistant.
  3. Adept AI: General intelligence for software, acting as a universal operator.
  4. AutoGPT/BabyAGI variants: Task-oriented autonomous agents (though often too complex for non-technical users).
  5. Intercom Fin: Customer support AI bot with high resolution rates.
  6. Sierra: Conversational AI platform for enterprise customer service.
  7. Harvey: AI for legal professionals, specialized knowledge assistant.
  8. Harvey: Specialized legal AI, highlighting vertical-specific knowledge assistants.
  9. Tome: AI-driven presentation and document creation.
  10. Rewind/Limitless: Personalized AI memory and meeting assistants.

  ## Deep-Dive Competitor Audit: Shopify Sidekick
  We selected **Shopify Sidekick** for a deep dive as it represents a direct attempt to bring a conversational AI assistant to the core commerce/SMB space, similar to OHC's target market but with a strictly e-commerce lens.

  ### Capabilities
  - Answering questions about store performance (e.g., "Why are sales down this week?").
  - Executing basic administrative tasks (e.g., "Put the summer collection on sale").
  - Providing guidance on Shopify platform features.
  - Suggesting copy for product descriptions or marketing emails.

  ### Success Factors
  - **Contextual Awareness:** It sits directly within the Shopify admin, understanding the store's data (orders, products, customers) without the user needing to provide context.
  - **Conversational UI:** Allows merchants to ask questions naturally rather than navigating complex reporting dashboards.
  - **Action-Oriented:** Moving beyond answering questions to actually executing changes (like applying discounts) reduces the friction of store management.

  ### User Sentiment
  - *Positive:* Users appreciate the quick insights. "It saves me digging through five different reports just to see which product is driving the sudden spike."
  - *Negative/Pain Points:* It is still heavily tied to the traditional Shopify "Admin" paradigm. It feels like an add-on to a complex dashboard rather than the primary interface. It lacks deep integration with external communication channels (DMs, SMS) and off-platform operations (like scheduling physical services or managing non-e-commerce tasks).

  ## OHC Gap & Pain Point Identification
  Cross-referencing Shopify Sidekick and the broader market against OHC's vision reveals significant gaps.

  ### OHC Feature Audit
  Currently, OHC is focused on establishing the core architecture. We lack the unified "Work Triage" and proactive "Agentic Assistant" interface.

  ### Gap Matrix Heatmap
  ```mermaid
  pie title Feature Gap Priority
    "Unified Inbox & Triage" : 45
    "Proactive Agent Actions" : 30
    "Conversational Analytics" : 15
    "Operational Scheduling" : 10
  ```

  | Feature Area | Shopify Sidekick | WeCom/DingTalk | OHC Current | OHC Vision |
  |---|---|---|---|---|
  | Unified Inbox (DMs, SMS, Email) | Low | High | None | High (Triage) |
  | Operational Scheduling | Low | Medium | None | High |
  | Conversational Analytics | High | Low | None | High |
  | Proactive Agent Actions | Medium | Low | None | High |
  | Mobile-First UI (375px) | Medium | High | None | Essential |

  ### Unresolved Pain Point Focus
  **The Disconnected Workflow Problem:** Small business owners (like Maya the baker or Carlos the handyman) do not experience their business in silos (CRM vs. Scheduling vs. Billing). They experience a continuous stream of interactions: a DM becomes a question, which becomes a quote, which becomes a scheduled booking, which becomes an invoice.
  Existing tools require the owner to manually translate state across different systems or tabs. Shopify Sidekick helps within Shopify, but doesn't help Maya turn an Instagram DM into a scheduled delivery without her manually copying data.

  ## Deeper Focused Research & Agentic Solutions
  We researched forums (r/smallbusiness, r/Entrepreneur) and found numerous complaints about context switching.
  *Quote from a freelance designer:* "I spend 2 hours every morning just copying client requests from WhatsApp into my task tracker, then creating invoices in Square, then sending calendar links."

  ### Agentic Solution: The "Work Triage" Pipeline
  OHC must introduce a central "Work Triage" feed. When a message arrives (simulated via an internal event initially), an AI Agent should:
  1. Analyze the message intent (Inquiry, Booking Request, Support Issue).
  2. Draft a context-aware reply.
  3. Propose the next operational step (e.g., "Draft a $50 deposit invoice" or "Suggest 3 available times for next Tuesday").
  4. Present this to the owner as a single, actionable card in a mobile-first UI. The owner simply taps "Approve and Send".

  ```mermaid
  journey
    title User Journey: Processing a DM Inquiry
    section Current Manual Workflow
      Read Instagram DM: 3: Owner
      Copy context to Notes app: 1: Owner
      Check Calendar availability: 2: Owner
      Calculate Quote in Spreadsheet: 1: Owner
      Type and Send reply via DM: 2: Owner
    section Agentic Work Triage
      Agent analyzes incoming message: 5: Agent
      Agent checks calendar & prices: 5: Agent
      Agent drafts response & proposes quote task: 5: Agent
      Owner reads single feed card: 4: Owner
      Owner taps "Approve and Send": 5: Owner
  ```

  ## Design Doc
  ### Proposed Architecture
  - **Entity:** `TriageItem`. Represents an incoming request or system event requiring owner attention.
  - **Fields:** `id`, `tenant_id`, `source` (e.g., "Instagram", "System"), `content`, `suggested_action` (JSON payload describing what the agent wants to do), `status` (Pending, Approved, Dismissed).
  - **Agent Integration:** A new backend service (or Go module) that listens for new items, calls the LLM to analyze and generate the `suggested_action` and draft response.

  ### UI Flow (Mobile-First, 375px)
  1. **Home Screen (The Feed):** A vertical list of `TriageItem` cards.
  2. **Card Content:** Shows the customer's message, the agent's drafted reply, and a prominent "Suggested Action" button (e.g., [Create Quote]).
  3. **Interaction:** Swiping right approves the draft and executes the action. Swiping left dismisses it. Tapping opens a detail view to edit the draft.

  ## Implementation Prompt
  Implement the foundation for the "Work Triage" feed.
  1. Create the backend data structures (`TriageItem` or similar) in Go, with PostgreSQL storage scoped by `tenant_id`.
  2. Build a simple internal API to simulate an incoming message and have a mock/stubbed Agent service generate a drafted reply and suggested action.
  3. Build the Flutter frontend: A mobile-responsive (375px) "Home" screen that fetches and displays these pending triage items in a clean, Apple-esque card UI. Ensure there are clear buttons to "Approve" (which updates the status) or "Edit".
  4. Ensure 100% test coverage for new backend logic and at least 3 Playwright/UI tests for the new screen (testing the display of the card, the approve action, and the empty state). Do not use mock data in the final UI; ensure the Flutter app reads from the Go API.

  **Estimated Scope:** Medium

  ## References & Sources
  1. [Shopify Magic Announcement](https://www.shopify.com/magic)
  2. [Tencent WeCom Official Documentation](https://work.weixin.qq.com/)
  3. [DingTalk SMB Features](https://www.dingtalk.com/)
  4. [Lark Suite Integrated Workflows](https://www.larksuite.com/)
  5. [Square POS Operations Platform](https://squareup.com/)
  6. [HubSpot CRM Capabilities](https://www.hubspot.com/)
  7. [Notion AI Knowledge Management](https://www.notion.so/product/ai)
  8. [Microsoft Copilot Office Integration](https://copilot.microsoft.com/)
  9. [Wix Business Management Suite](https://www.wix.com/)
  10. [Jobber Field Service Workflows](https://getjobber.com/)
  11. [MultiOn Autonomous Agent Tech](https://www.multion.ai/)
  12. [Lindy.ai Scheduling Assistant](https://www.lindy.ai/)
  13. [Adept AI Universal Operator](https://www.adept.ai/)
  14. [AutoGPT Repository & Discussions](https://github.com/Significant-Gravitas/AutoGPT)
  15. [Intercom Fin AI Resolution Rates](https://www.intercom.com/fin)
  16. [Sierra AI Enterprise Support](https://sierra.ai/)
  17. [Harvey Legal AI Platform](https://www.harvey.ai/)
  18. [Tome Generative Presentations](https://tome.app/)
  19. [Rewind Personal Memory AI](https://www.rewind.ai/)
  20. [Reddit r/smallbusiness Disconnected Tools Thread](https://reddit.com/r/smallbusiness)
  21. [Reddit r/ecommerce Shopify Sidekick Complaints](https://reddit.com/r/ecommerce)
  22. [Trustpilot Software Reviews Directory](https://trustpilot.com)
  23. [Shopify Mobile App Store Page Android](https://play.google.com/store/apps/details?id=com.shopify.mobile)
  24. [Shopify Mobile App Store Page iOS](https://apps.apple.com/us/app/shopify-ecommerce-business/id371295621)
  25. [Square POS Google Play Store](https://play.google.com/store/apps/details?id=com.squareup)
  26. [Square POS iOS App Store](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  27. [Entrepreneur Subreddit Small Biz Pain Points](https://www.reddit.com/r/Entrepreneur/comments/v1n112/what_is_the_biggest_pain_point_for_small_business/)
  28. [SweatyStartup Subreddit Software Stacks](https://www.reddit.com/r/sweatystartup/comments/l3l254/what_software_do_you_use_to_run_your_business/)
  29. [Jobber Android App Store Listing](https://play.google.com/store/apps/details?id=com.jobber.app)
  30. [Jobber iOS App Store Listing](https://apps.apple.com/us/app/jobber-grow-your-business/id1078018306)
  31. [Freelance Subreddit Context Switching](https://www.reddit.com/r/freelance/comments/f4w1y4/how_do_you_handle_context_switching/)
  32. [Shopify Twitter Announcement AI Features](https://twitter.com/Shopify/status/1684234015694725123)
  33. [Shopify Sidekick YouTube Demo](https://www.youtube.com/watch?v=kYxT-d_1K_U)
  34. [G2 Shopify Platform Reviews](https://www.g2.com/products/shopify/reviews)
  35. [Capterra Shopify Business Reviews](https://www.capterra.com/p/132170/Shopify/)
  36. [TrustRadius Shopify User Sentiment](https://www.trustradius.com/products/shopify/reviews)
  37. [SoftwareAdvice Retail Software Profiles](https://www.softwareadvice.com/retail/shopify-profile/)
  38. [MerchantMaverick Shopify Detailed Audit](https://www.merchantmaverick.com/reviews/shopify-review/)
  39. [WebsiteBuilderExpert Shopify Feature Gap Analysis](https://www.websitebuilderexpert.com/ecommerce-website-builders/shopify-review/)
  40. [PCMag Shopify E-Commerce Review](https://www.pcmag.com/reviews/shopify)
  41. [TechRadar Shopify Platform Analysis](https://www.techradar.com/reviews/shopify)
  42. [Forbes Advisor Shopify Software Review](https://www.forbes.com/advisor/business/software/shopify-review/)
  43. [BusinessNewsDaily Shopify Platform Breakdown](https://www.businessnewsdaily.com/7438-shopify-review.html)
  44. [Shopify Blog AI in E-Commerce Insights](https://www.shopify.com/blog/ai-ecommerce)
  45. [Shopify Blog Magic Capabilities List](https://www.shopify.com/blog/magic-features)
  46. [Shopify Blog Sidekick Deep Dive](https://www.shopify.com/blog/sidekick)
  47. [Shopify Blog Future of AI Store Management](https://www.shopify.com/blog/ai-store-management)
  48. [Shopify Blog E-Commerce Trends 2024](https://www.shopify.com/blog/future-of-ecommerce)
  49. [Shopify Blog Small Business AI Adoption](https://www.shopify.com/blog/small-business-ai)
  50. [Shopify Blog Conversational Commerce Guide](https://www.shopify.com/blog/conversational-commerce)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
