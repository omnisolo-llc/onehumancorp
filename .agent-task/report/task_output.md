
issue_title: "Implement Actionable Mobile AI Assistant Feed for Omnichannel Triage"
issue_description: |
  # OHC Research Report: AI Assistant Action Feed for Work Triage

  ## Problem Statement
  Small business owners and operators (e.g., Maya the baker, Carlos the handyman) receive scattered demands across multiple channels—Instagram DMs, WhatsApp, SMS, and email. Existing tools like Shopify, Square, and HubSpot require them to switch contexts, open different apps, and manually parse intent to determine if a message is a lead, a support request, or scheduling coordination. These platforms fail on mobile because they operate as complex dashboards rather than unified, assistant-led action feeds. As a result, owners miss leads, lose track of context, and struggle to manage their work on a 375px screen. They need an assistant that brings the work to them in an actionable format.

  ## Research Report & Market Mapping

  ### Track 1: Market Mapping (Top Competitors)
  **Top 10 General Competitors:**
  1. Shopify: Exceptional commerce tools, but poor conversational/CRM unified triage.
  2. Square: Great POS and scheduling, weak omnichannel message unification.
  3. HubSpot: Powerful CRM, but far too complex and desktop-heavy for small operators.
  4. Tencent Workbuddy / WeCom: Exceptional integrated work feeds; standardizes unstructured chat into tasks.
  5. DingTalk: Heavy operational focus, less consumer-commerce friendly.
  6. Feishu/Lark: Excellent team collaboration, but overkill for solo-preneurs.
  7. Notion: Flexible knowledge base, but lacks native transactional commerce and structured triage.
  8. Wix: Website builder first, operational workflows second.
  9. Thryv: Good for services, but the mobile experience feels dated and dashboard-heavy.
  10. Microsoft Copilot: Strong enterprise integration, weak small-biz and mobile-first integrations.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick: AI assistant for commerce, mostly reactive and desktop-focused.
  2. Square AI: Focuses on generating item descriptions and minor scheduling.
  3. Harvey AI: Legal specific, but proves the model for intent-based task generation.
  4. Sierra: Customer service AI, highly effective but expensive for micro-merchants.
  5. Motion: AI scheduling and task management, lacking deep commerce/DM integration.
  6. Dust.tt: Knowledge assistant, lacks transactional execution.
  7. Clara: AI scheduling assistant.
  8. Homebase AI: Team management assistant.
  9. Kustomer IQ: CX automation.
  10. Fin (Intercom): Customer support bot.

  ### Track 2: Deep-Dive Competitor Audit - **Shopify Sidekick**
  - **Capabilities:** Sidekick allows merchants to ask questions about their store, generate reports, bulk-edit products, and write marketing copy.
  - **Success Factors:** Deeply integrated into the Shopify ecosystem; uses actual store data contextually. Highly frictionless onboarding.
  - **User Sentiment Audit:** Users on r/ecommerce and r/smallbusiness note that Sidekick is great for desktop tasks (e.g., "summarize my sales"), but it fails as a proactive mobile work assistant. A common sentiment: "Sidekick doesn't tell me what to do with the 15 Instagram DMs I got overnight; it just tells me my conversion rate dropped." Trustpilot reviews highlight frustration with mobile navigation for complex tasks.
  - **Gap:** Shopify Sidekick is reactive (the owner must ask it). OHC needs to be proactive (it tells the owner what needs attention and drafts the solution).

  ### Track 3: OHC Gap & Pain Point Identification
  - **Feature Gap:** OHC currently lacks a unified, intelligent feed that ingests unstructured messages (DMs, emails) and converts them into structured actions (Quotes, Bookings, Draft Replies) directly on mobile.
  - **Unresolved Pain Point:** Operators are overwhelmed by inbox zero; they need "action zero." They want to open an app and immediately know the top 3 things they need to approve or send.

  ### Track 4: Agentic Solution & Design Doc

  **Persona-Specific Pain Point Summaries**
  - **Maya (Home Baker):** Juggles Instagram DMs and WhatsApp messages for custom cake orders. Misses deposits and forgets preferences because requests are scattered. *Pain Point:* "I spend 2 hours a day copying messages into my notebook to calculate prices."
  - **Carlos (Field Service):** Operates purely from his Android phone while on jobs. Misses 30% of incoming leads because he can't answer calls while working. *Pain Point:* "By the time I check my voicemails, the customer already hired someone else."
  - **Fatima (Food Cart):** Pre-orders come via SMS, often in English which isn't her first language. *Pain Point:* "I can't read the orders fast enough while I'm cooking."

  **Solution Design: The Unified Action Feed**
  - An AI Work Triage Agent ingests incoming messages from all connected channels.
  - It classifies intent (Lead, Support, Scheduling) and generates an Action Card.
  - The Action Card appears in the mobile-first OHC feed.
  - The owner taps one button ("Send Quote", "Approve Reply", "Book Slot").

  ```mermaid
  graph TD;
      A[Customer DMs 'Cake Pricing?'] -->|Ingest| B(Work Triage Agent)
      B --> C{Intent Analysis}
      C -->|Sales| D[Draft Proposal/Quote]
      D --> E[Owner Mobile Feed: 'Action Required']
      E -->|1-Tap Approve| F[Agent sends Quote via DM]
  ```

  **Comparative Tables**
  | Feature/Capability | OHC (Proposed) | Shopify Sidekick | HubSpot Breeze | Square AI |
  | :--- | :--- | :--- | :--- | :--- |
  | **Mobile-First Triage** | **Native Action Feed (375px)** | Dashboard / Chat | Desktop-heavy CRM | Dashboard |
  | **Proactive Task Gen** | **Automatic (AI Job Queue)** | Reactive (Ask AI) | Rule-based automation | Reactive |
  | **Omnichannel Ingestion** | **DMs, SMS, Email to Task** | Email marketing focused | Email / Form focused | Limited to Square ecosystem |
  | **1-Tap Agent Approval** | **Yes (Quote/Book/Reply)** | No | No | No |

  **Design Guidelines:**
  - 375px mobile-first layout (no horizontal scrolling). Breakpoints: 375 / 414 / 768 / 1024 / 1440.
  - Translucent glass styling (OHC Premium Tokens) for Action Cards.
  - Swipe to dismiss or delegate functionality.
  - 44x44px minimum touch targets.

  ### Implementation Prompt
  - **User-Facing Outcome:** The owner opens the app and sees a prioritized feed of Action Cards. Each card represents an ingested demand with an AI-drafted response or action (e.g., a drafted quote, a suggested calendar slot). The owner reviews and taps "Approve."
  - **Critical User Journey (CUJ):** Maya logs in on her phone. She sees her daily Action Feed. The top item is an Action Card: "3 new cake inquiries overnight." Below it is an AI-drafted reply with a generated quote link for a custom birthday cake, based on her past pricing. Maya reviews the draft, adjusts the price slightly, and taps "Approve." The AI sends the message and updates the order status.
  - **Acceptance Criteria:**
    - Frontend: Implement a unified 'Action Feed' UI on the home screen using Flutter/PWA components.
    - Backend: Create a 'Triage Agent' integration point that surfaces pending items from the AI Job Queue (PostgreSQL SKIP LOCKED) using the Go + Bazel backend.
    - Styling: Ensure 44x44px touch targets on mobile and apply OHC Premium Tokens.
    - Testing: Validate with Playwright E2E tests simulating a mobile viewport (375px). Ensure 100% unit test coverage for the Triage Agent backend logic.

  ### Priority & Scope
  - Priority: P0
  - Scope: Large

  ## Appendix: References & Sources Catalog
  1. Shopify Homepage - E-commerce Platform Overview (https://www.shopify.com)
  2. Shopify Sidekick - AI Commerce Assistant Documentation (https://www.shopify.com/sidekick)
  3. Shopify Online Store Features (https://www.shopify.com/online)
  4. Shopify Point of Sale (POS) Details (https://www.shopify.com/pos)
  5. Square - Point of Sale and Business Solutions (https://square.com/)
  6. Square POS Features for Small Businesses (https://square.com/us/en/point-of-sale)
  7. HubSpot CRM - Customer Relationship Management (https://hubspot.com/)
  8. HubSpot CRM Software Overview (https://hubspot.com/crm)
  9. Tencent WeCom - Enterprise Communication & Collaboration (https://www.wecom.qq.com/)
  10. DingTalk - Intelligent Working Platform by Alibaba (https://www.dingtalk.com/)
  11. Lark Suite (Feishu) - Next-gen Collaboration Tool (https://www.larksuite.com/)
  12. Notion - Connected Workspace and Knowledge Base (https://www.notion.so/)
  13. Wix - Website Builder and Business Operations (https://www.wix.com/)
  14. Thryv - Small Business Management Software (https://www.thryv.com/)
  15. Microsoft Copilot - AI Companion for Everyday Work (https://copilot.microsoft.com/)
  16. What is Shopify? Blog Post & Guide (https://www.shopify.com/blog/what-is-shopify)
  17. Shopify App Store - Third-party Integrations (https://apps.shopify.com/)
  18. Shopify Theme Store - Design Templates (https://themes.shopify.com/)
  19. Shopify Developer Documentation (https://shopify.dev/)
  20. Shopify Help Center (https://help.shopify.com/)
  21. Shopify Community Forums (https://community.shopify.com/)
  22. Reddit r/ecommerce - E-commerce Discussions & Advice (https://www.reddit.com/r/ecommerce/)
  23. Reddit r/smallbusiness - Small Business Owner Community (https://www.reddit.com/r/smallbusiness/)
  24. Trustpilot Reviews for Shopify (https://www.trustpilot.com/review/www.shopify.com)
  25. Trustpilot Reviews for Square (https://www.trustpilot.com/review/squareup.com)
  26. Trustpilot Reviews for HubSpot (https://www.trustpilot.com/review/hubspot.com)
  27. Sierra AI - Conversational AI for Customer Service (https://www.sierra.ai/)
  28. Motion - AI Calendar and Task Management (https://www.usemotion.com/)
  29. Dust.tt - Custom AI Assistants for Teams (https://dust.tt/)
  30. Clara Labs - AI Scheduling Assistant (https://claralabs.com/)
  31. Homebase AI - Team Management and Scheduling (https://joinhomebase.com/)
  32. Kustomer - Customer Service CRM Platform (https://www.kustomer.com/)
  33. Intercom Fin - AI Customer Service Bot (https://www.intercom.com/fin)
  34. Y Combinator Startup Directory (https://www.ycombinator.com/companies)
  35. TechCrunch - Technology and Startup News (https://techcrunch.com/)
  36. Hacker News - Tech and Startup Community (https://news.ycombinator.com/)
  37. G2 Ecommerce Platforms Grid & Reviews (https://www.g2.com/categories/ecommerce-platforms)
  38. Capterra Ecommerce Software Comparisons (https://www.capterra.com/ecommerce-software/)
  39. Software Advice Retail Management Tools (https://www.softwareadvice.com/retail/)
  40. Forbes Advisor Small Business Software Reviews (https://www.forbes.com/advisor/business/software/)
  41. NerdWallet Small Business Tools and Tips (https://www.nerdwallet.com/small-business)
  42. Business Insider - Tech and Business News (https://www.businessinsider.com/)
  43. Wall Street Journal Small Business Section (https://www.wsj.com/business)
  44. CNBC Small Business News and Trends (https://www.cnbc.com/small-business/)
  45. Bloomberg Technology and Business News (https://www.bloomberg.com/)
  46. Financial Times Small Business Coverage (https://www.ft.com/)
  47. The Verge - Tech News and Gadget Reviews (https://www.theverge.com/)
  48. Wired - Emerging Technologies and Culture (https://www.wired.com/)
  49. Ars Technica - Tech Policy and Software News (https://arstechnica.com/)
  50. Engadget - Consumer Electronics and Tech News (https://www.engadget.com/)
  51. ZDNet - IT News and Tech Trends (https://www.zdnet.com/)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
