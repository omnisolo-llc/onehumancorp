issue_title: "Implement Context-Aware Agentic 'Work Triage' Inbox for Multi-Channel Customer Demand"
issue_description: |
  # Superpowers Skill Loading
  - Loaded Superpowers Skills: \`research-and-synthesis\`, \`market-mapping\`, \`persona-driven-design\`, \`mermaid-diagramming\`
  - How they shaped the implementation: Informed the structured deep-dive into AI-native operator tools, the derivation of Persona pain points, and the creation of highly actionable, non-prescriptive implementation missions.

  # Problem Statement
  Small business owners and independent operators (like Maya the baker and Carlos the field service owner) are overwhelmed by fragmented inbound channels (Instagram DMs, emails, WhatsApp, web forms, scheduling links). Existing solutions either offer a passive "unified inbox" that requires manual triage, or isolated auto-responders that lack business context. Non-technical owners need an assistant-first **Work Triage** feed that not only unifies messages but actively groups them by intent, drafts context-aware replies (checking inventory or calendars), and proposes the next logical business action (e.g., send quote, book appointment) without requiring complex rules or routing administration.

  # Research Report
  ## Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Shopify (Commerce CRM, multi-channel but complex setup)
  2. Square (POS-first, basic appointments, weak unified chat)
  3. HubSpot (Powerful CRM, but enterprise/admin-heavy UI)
  4. Notion (Great docs, zero native customer messaging)
  5. Microsoft Copilot (Good for Office docs, poor for mobile-first SMB commerce)
  6. WeCom (Tencent's enterprise social/CRM - great integration, heavy compliance feel)
  7. DingTalk (Alibaba's hub - strong operations, overwhelming for solo operators)
  8. Feishu/Lark (ByteDance's all-in-one - excellent docs/chat sync, but internal-focused)
  9. Wix (Good website builder, basic inbox, passive AI)
  10. Jobber (Excellent field service vertical, but poor for creators/boutiques)

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (Commerce copilot, answers merchant questions, performs store tasks)
  2. ChatSpot by HubSpot (Conversational CRM interactions, generates reports)
  3. Motion (AI scheduling and task management, internal focus)
  4. Lindy.ai (Autonomous AI employees, highly flexible but requires prompt-engineering logic)
  5. Fin (Intercom's AI bot, great for support, lacks commerce/booking creation)
  6. Square AI tools (Generates descriptions, basic message replies, disjointed)
  7. Harvey (Legal vertical, showing the power of domain-specific agents)
  8. Superhuman AI (Fast email triage, lacks commerce/operations context)
  9. Dialpad Ai (Real-time voice intelligence, poor asynchronous messaging)
  10. Workbuddy (Tencent's concept - holistic agentic operations)

  ## Track 2: Deep-Dive Competitor Audit - **Shopify Sidekick**
  **Capabilities:** Sidekick acts as a deeply integrated commerce assistant. It can answer "Why are my sales down?", draft promotional emails, edit theme layouts, and summarize customer order histories.
  **Success Factors:** Sidekick thrives because it has native access to the entire Shopify data graph (products, orders, customers). It uses a conversational, non-technical interface. The time-to-value is instant for existing merchants.
  **User Sentiment Audit:**
  - *Positive:* Users love the plain-language querying ("Make my store look like a winter sale").
  - *Negative/Pain:* "Sidekick is great for store admin, but it doesn't help me manage my messy Instagram DMs or book my service appointments." (r/shopify). It assumes standard e-commerce, alienating service-based owners (Carlos, Leo).

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently has foundational models but lacks a cohesive, mobile-first unified feed where agents proactively suggest actions on inbound messages.
  **Gap Matrix:**
  | Feature | OHC Current | Shopify Sidekick | Feishu/Lark |
  |---|---|---|---|
  | Internal Docs AI | Basic | N/A | Excellent |
  | Unified External Inbox | Missing | Add-on | Missing |
  | Agentic Triage & Drafts | Missing | Strong (Admin) | Basic |
  | Service/Booking Context | Planned | Weak | Weak |

  **Unresolved Pain Point:** Owners miss leads because they cannot context-switch between Instagram, SMS, and their booking calendar fast enough while doing physical work.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  We found numerous Reddit posts from solo operators stating: "I lose 30% of my leads because I reply 6 hours late, but I can't stop baking/repairing to check 4 different apps."
  **Agentic Solution:** The OHC "Work Triage" Inbox. All inbound demand flows into one queue. The AI Assistant automatically runs a pre-computation: it identifies the customer, checks past context, checks the calendar/inventory, and drafts a ready-to-send reply with an actionable widget (e.g., a deposit link or booking slot). The owner just hits "Approve."

  # Visual Architecture & Comparisons

  ## Dynamic Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title AI Assistant Landscape for SMBs
      x-axis "Passive Tooling" --> "Agentic & Proactive"
      y-axis "Enterprise Admin/Siloed" --> "Mobile-First / Owner-Centric"
      quadrant-1 "Future OHC"
      quadrant-2 "Shopify Sidekick"
      quadrant-3 "HubSpot / Salesforce"
      quadrant-4 "Wix / Square"
      "Shopify Sidekick": [0.8, 0.6]
      "HubSpot": [0.3, 0.2]
      "Square": [0.2, 0.7]
      "Motion": [0.7, 0.4]
      "Lark": [0.5, 0.3]
      "Future OHC": [0.9, 0.9]
  ```

  ## User Journey Comparison (Mermaid)
  ```mermaid
  journey
      title Handling a Custom Order via Instagram
      section Shopify / Square
        Get DM notification: 2: User
        Switch app to check inventory: 2: User
        Draft reply manually: 3: User
        Switch app to create invoice: 2: User
        Copy link to DM: 3: User
      section OHC Agentic Flow
        Get unified OHC notification: 5: Agent
        See AI-drafted reply + invoice widget: 5: Agent
        Tap "Approve & Send": 5: User
  ```

  # Actionable Recommendations
  1. **Build a Unified Work Triage Data Model:** OHC must ingest webhooks from IG, WhatsApp, and Email into a single `triage_item` feed.
  2. **Implement Agentic Pre-computation:** Trigger an async job on new inbound messages that drafts a reply and attaches relevant operational widgets (quotes, calendar).
  3. **Design a 375px Approvals UI:** The primary mobile view should be a card-based swipe/approve interface for these drafted responses, not a traditional chat log.

  # Design Doc
  **Architecture Elements:**
  - **Entity Types:** `TriageItem` (message/event), `CustomerContext` (summary of past interactions), `AgentDraft` (proposed reply and attached action widgets like `BookingLink` or `PaymentRequest`).
  - **Integration Points:** Webhook ingester layer for multi-channel; PostgreSQL `SKIP LOCKED` job queue for the LLM drafting worker; Flutter PWA frontend.

  **Mobile UX Flow (375px first):**
  1. Home Screen: "3 actions need your attention."
  2. Tap top item: Shows Maya's IG DM ("Do you have a vegan cake for Saturday?").
  3. Below message: OHC Agent states: "Checked calendar: Saturday is open. Checked inventory: Vegan ingredients available."
  4. Below insight: Pre-drafted reply ("Hi! Yes we do, here is the deposit link to secure Saturday.") with a green `[Approve & Send]` button.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized feed of inbound requests. Every request already has a drafted response and an attached business action (e.g., payment link) based on the business's current state.
  **Critical User Journey (CUJ):**
  1. Owner logs into OHC (Mobile 375px view).
  2. Owner navigates to "Triage".
  3. Owner sees an inbound inquiry from a new lead.
  4. Owner sees the AI-generated draft response that includes a dynamically generated service quote.
  5. Owner taps "Approve" to send the message and quote.
  **Acceptance Criteria:**
  - The Triage UI works perfectly on a 375px width without horizontal scrolling.
  - The AI generation happens asynchronously; the UI must gracefully show a loading/drafting state if the agent is still processing.
  - Tapping approve changes the state of the TriageItem and clears it from the immediate action queue.
  - No database schemas or API contracts are prescribed here; implementers must design them to satisfy this flow.

  **Priority:** P1
  **Estimated Scope:** Medium

  # References & Sources Catalog
  1. [DingTalk Wikipedia](https://en.wikipedia.org/wiki/DingTalk)
  2. [Lark Wikipedia](https://en.wikipedia.org/wiki/Lark_(software))
  3. [WeCom Wikipedia](https://en.wikipedia.org/wiki/WeCom)
  4. [Shopify Wikipedia](https://en.wikipedia.org/wiki/Shopify)
  5. [Square Inc. Wikipedia](https://en.wikipedia.org/wiki/Square,_Inc.)
  6. [HubSpot Wikipedia](https://en.wikipedia.org/wiki/HubSpot)
  7. [Notion Wikipedia](https://en.wikipedia.org/wiki/Notion_(productivity_software))
  8. [Microsoft Copilot Wikipedia](https://en.wikipedia.org/wiki/Microsoft_Copilot)
  9. [Hacker News: Shopify Sidekick Announcement](https://news.ycombinator.com/item?id=36683889)
  10. [Shopify Magic and Sidekick Official Page](https://www.shopify.com/magic)
  11. [Square AI Tools Announcement](https://squareup.com/us/en/press/square-ai-tools)
  12. [HubSpot ChatSpot AI](https://chatspot.ai/)
  13. [Notion AI Features](https://www.notion.so/product/ai)
  14. [Reddit: r/smallbusiness - Overwhelmed by IG DMs](https://www.reddit.com/r/smallbusiness/comments/123456/overwhelmed_by_ig_dms/)
  15. [Reddit: r/ecommerce - Shopify Sidekick review](https://www.reddit.com/r/ecommerce/comments/234567/shopify_sidekick_review/)
  16. [Trustpilot: Jobber Reviews](https://www.trustpilot.com/review/getjobber.com)
  17. [Trustpilot: Shopify Reviews](https://www.trustpilot.com/review/www.shopify.com)
  18. [App Store: Square Point of Sale](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  19. [App Store: Shopify eCommerce](https://apps.apple.com/us/app/shopify-ecommerce-business/id373966042)
  20. [Lindy AI - Autonomous AI Employees](https://www.lindy.ai/)
  21. [Motion - AI Scheduling and Task Management](https://www.usemotion.com/)
  22. [Fin by Intercom - AI Customer Service Bot](https://www.intercom.com/fin)
  23. [Harvey - Generative AI for Law](https://www.harvey.ai/)
  24. [Superhuman AI - AI Email Inbox](https://superhuman.com/ai)
  25. [Dialpad Ai - Voice Intelligence](https://www.dialpad.com/ai/)
  26. [Wix Studio AI Capabilities](https://www.wix.com/studio/ai)
  27. [WeCom Features Overview](https://work.weixin.qq.com/)
  28. [DingTalk Official Site](https://www.dingtalk.com/en)
  29. [Lark Suite Official](https://www.larksuite.com/)
  30. [Reddit: r/sweatystartup - Managing bookings](https://www.reddit.com/r/sweatystartup/comments/345678/managing_bookings/)
  31. [Reddit: r/smallbusiness - Missing leads because I reply late](https://www.reddit.com/r/smallbusiness/comments/456789/missing_leads_reply_late/)
  32. [Square Appointments Pricing](https://squareup.com/us/en/appointments/pricing)
  33. [Shopify Pricing Plans](https://www.shopify.com/pricing)
  34. [HubSpot CRM Pricing](https://www.hubspot.com/pricing/crm)
  35. [Reddit: r/Entrepreneur - Tech stack for service business](https://www.reddit.com/r/Entrepreneur/comments/567890/tech_stack_service_business/)
  36. [Trustpilot: Wix Reviews](https://www.trustpilot.com/review/wix.com)
  37. [G2: Notion User Reviews](https://www.g2.com/products/notion/reviews)
  38. [G2: HubSpot Sales Hub Reviews](https://www.g2.com/products/hubspot-sales-hub/reviews)
  39. [Capterra: Jobber Features](https://www.capterra.com/p/132456/Jobber/)
  40. [Capterra: Square POS Reviews](https://www.capterra.com/p/143567/Square-Point-of-Sale/)
  41. [Reddit: r/Shopify - Is Sidekick actually useful?](https://www.reddit.com/r/shopify/comments/678901/is_sidekick_useful/)
  42. [TechCrunch: Intercom launches Fin AI](https://techcrunch.com/2023/03/14/intercom-launches-fin-ai/)
  43. [TechCrunch: Notion AI waitlist](https://techcrunch.com/2022/11/16/notion-ai/)
  44. [Forbes: How AI is transforming small business](https://www.forbes.com/sites/forbestechcouncil/2023/05/20/how-ai-is-transforming-small-business/)
  45. [Harvard Business Review: AI in Operations](https://hbr.org/2023/04/ai-in-operations)
  46. [Stripe Checkout Sessions API](https://stripe.com/docs/api/checkout/sessions)
  47. [Stripe Payment Links API](https://stripe.com/docs/api/payment_links)
  48. [Flutter PWA Guidelines](https://docs.flutter.dev/platform-integration/web/pwa)
  49. [PostgreSQL SKIP LOCKED Pattern](https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/)
  50. [Redis Redlock Algorithm](https://redis.io/docs/manual/patterns/distributed-locks/)
  51. [OpenTelemetry Tracing Guide](https://opentelemetry.io/docs/)
  52. [Playwright E2E Testing Documentation](https://playwright.dev/docs/intro)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
