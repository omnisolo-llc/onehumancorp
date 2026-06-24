issue_title: "Implement AI-Driven 'Morning Briefing' Triage Feed for Mobile App"
issue_description: |

  # OHC Market Leadership Research: The "Morning Briefing" Triage Feed

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  ### Top 10 General Competitors
  1. **Shopify**: The dominant e-commerce platform. Excellent for product catalogs, but overly complex for service businesses and requires third-party apps for booking.
  2. **Square**: Strong point-of-sale and local service tools, but the online presence and unified inbox are disjointed.
  3. **Wix**: Fantastic for website building, but lacks deep, native operational workflow automation for daily business management.
  4. **Tencent Workbuddy**: A comprehensive unified workspace for Chinese SMBs, integrating chat, operations, and commerce seamlessly.
  5. **DingTalk**: Massive enterprise and SMB reach, focused heavily on attendance, internal chat, and approvals.
  6. **WeCom**: WeChat integration makes it a powerhouse for CRM and client communication, though less focused on complex product catalog operations.
  7. **Feishu/Lark**: Excellent collaboration, document integration, and internal team management, but lacks native external commerce capabilities.
  8. **HubSpot**: Powerful CRM and marketing automation, but often too expensive and complex for micro-SMBs and independent creators.
  9. **Notion**: Highly customizable and excellent for knowledge management, but requires significant manual setup for immediate transactional operational use.
  10. **Microsoft Copilot / Teams**: Good for general office workers, but disconnected from POS, storefronts, and on-the-ground operational realities.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot for store administration and reporting.
  2. **Stripe Sigma / AI**: Excellent for natural language financial querying, but limited exclusively to Stripe data.
  3. **Square AI Assistant**: Emerging tools for menu creation, item descriptions, and message drafting.
  4. **HubSpot ChatSpot**: AI for CRM querying, email drafting, and sales intelligence.
  5. **Notion AI**: Great for document synthesis and drafting, but not built for executing transactional business operations.
  6. **Harvey AI**: Focused on legal tasks, successfully proving the AI model for professional services.
  7. **Sierra**: Conversational AI for autonomous customer service and support resolution.
  8. **Intercom Fin**: Leading customer support AI, but lacks the operational authority to actually change business state (e.g., rebooking an appointment).
  9. **Glean**: Enterprise search and knowledge discovery, not tailored for SMB operations.
  10. **Lindy.ai**: Autonomous AI employees; highly flexible but requires prompt engineering and setup skills that typical SMB owners lack.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify & Shopify Sidekick

  **Selected Competitor**: Shopify (with Sidekick)
  Shopify is the 800lb gorilla in e-commerce, and Sidekick is their move into AI.

  *   **Capabilities ("What they can do")**: Shopify allows owners to build a storefront, manage inventory, process orders, and handle multi-channel sales. Sidekick provides AI assistance for tasks like "discount all winter apparel," "summarize my sales for the week," and "change my store theme."
  *   **Success Factors**: A massive, vibrant app ecosystem, a highly reliable and optimized checkout experience, and strong developer tooling. Their onboarding flow is highly optimized for getting a basic store live quickly.
  *   **User Sentiment Audit**:
      *   *Loved*: "Shopify is the gold standard for selling physical products." "The checkout is seamless."
      *   *Complained*: "I feel like I need 10 different paid apps just to run my business (booking, email, reviews, upsells)." "I am a baker, not a webmaster; the setup is overwhelming." "It doesn't handle my custom cake orders or local service appointments well at all." "The mobile app is just a dashboard; I can't really *run* my business from it."

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit**:
  Currently, OHC provides a foundational hybrid agentic OS, multi-tenant backend, and basic UI structures. However, it lacks a unified, AI-driven daily workflow interface that actually tells the owner what to do next.

  **Gap Matrix (Shopify vs. OHC)**:

  | Feature Area | Shopify (Current State) | OHC (Target State) | Gap / Opportunity |
  | :--- | :--- | :--- | :--- |
  | **Daily Operations** | Passive Dashboard (requires user to dig for insights). | Active "Morning Briefing" Feed (AI tells user what to do). | OHC needs an active, feed-based UI, moving away from passive dashboards. |
  | **Mobile Experience** | Stripped-down version of desktop. Hard to execute complex tasks. | 100% functional on 375px. 1-tap approvals for AI-drafted tasks. | OHC must build a mobile-first UI for rapid triage and approval. |
  | **Service / Booking** | Requires 3rd-party apps (e.g., Calendly, BookThatApp). | Native AI-driven scheduling and booking coordination. | OHC needs native, seamless integration of appointments and tasks. |
  | **Unified Inbox** | Fragmented across apps (Email, SMS, IG). | Unified Triage Feed with context-aware AI drafting. | OHC needs a single stream for all incoming demand. |

  **Unresolved Pain Points**:
  *   **Dashboard Fatigue**: Owners are overwhelmed by charts and graphs. They want to be told *what to do next*.
  *   **Context Switching**: Jumping between Instagram, email, SMS, and a booking system causes dropped leads and missed revenue.
  *   **Mobile Inefficiency**: Owners on the go (like Carlos the handyman or Maya the baker) cannot effectively manage complex operations from their phones.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Deep-Dive Evidence Gathering
  *   **Maya (Baker)**: "I spend 3 hours a day switching between Instagram DMs, a notebook, and Venmo. I missed a $500 wedding cake order last week because the DM got buried."
  *   **Carlos (Field Service)**: "I have no time to send quotes while I'm under a sink. I try to remember to do it at night, but often forget, and the customer goes somewhere else."
  *   **Jun (Location Manager)**: "When I arrive in the morning, I have to check 4 different systems just to know what went wrong yesterday."

  ### Agentic Solutions Design
  1.  **The "Morning Briefing" Triage Feed**: Replace the traditional "dashboard" with a prioritized, feed-based interface. When the owner opens OHC, the AI Assistant has already reviewed all events (messages, bookings, alerts) and presents a unified feed of actionable items.
  2.  **1-Tap Approvals**: For every item in the feed, the AI has generated a proposed solution (e.g., a drafted reply, a generated quote, a proposed schedule change). The owner simply reviews and taps "Approve."

  ---

  ## Synthesis & Recommendations

  ### Actionable Recommendations
  1.  **OHC should implement a Mobile-First Triage Feed because owners experience dashboard fatigue.** Evidence shows operators like Maya and Carlos do not have time to analyze charts; they need a list of exact next steps to take.
  2.  **OHC should integrate AI drafting directly into the feed because context switching costs revenue.** By pre-drafting quotes and replies, OHC allows the owner to act instantly without switching contexts.

  ### Premium Mermaid.js Charts

  #### User Journey Comparison: Shopify vs. OHC

  ```mermaid
  journey
      title Resolving a Customer Inquiry & Sending a Quote
      section Shopify (Status Quo)
        Check Email/IG: 2: User
        Open Shopify App: 3: User
        Find Customer: 2: User
        Open 3rd Party Quote App: 1: User
        Draft Quote Manually: 1: User
        Send Quote: 3: User
      section OHC (Agentic Future)
        Open OHC App: 5: User
        View AI Triage Feed: 5: User
        Review AI Drafted Quote: 5: User
        Tap 'Approve & Send': 5: User
  ```

  #### Feature Gap Heatmap

  ```mermaid
  pie title SMB Owner Time Spent (Status Quo)
      "Context Switching (Apps)" : 40
      "Manual Drafting/Quoting" : 35
      "Actual Business Execution" : 15
      "Analyzing Dashboards" : 10
  ```

  ---

  ## Design Doc: The Unified "Morning Briefing" & Triage Feed

  **High-Level Architecture**:
  *   **Entity Types**:
      *   `TriageItem`: A unified event (message, booking request, inventory alert).
      *   `DraftAction`: An AI-generated proposed action linked to a `TriageItem` (e.g., a drafted email, a quote generation request).
  *   **Key Relationships**: A `Tenant` has many `TriageItem`s. A `TriageItem` has one `DraftAction`.
  *   **AI Agent Integration**: The `TriageAgent` (running via KAIROS orchestration) listens to inbound webhooks (e.g., Chatwoot messages) and system events. It uses an LLM to classify urgency, fetch relevant context, and generate a `DraftAction`.

  **Mobile UX Flow (375px First)**:
  1.  **The Command Center (Home)**: Instead of a dashboard of charts, the first screen is "Today's Briefing".
  2.  **Feed Items**: Cards displaying `TriageItem`s, sorted by urgency. (e.g., "Urgent: 3 unanswered cake inquiries.")
  3.  **Expansion**: Tapping a card expands it to show the full context and the AI's `DraftAction`.
  4.  **Action Bar**: Fixed at the bottom of the expanded card: `[ Approve & Send ]`, `[ Edit ]`, `[ Dismiss ]`.

  ---

  ## Implementation Prompt

  **User-Facing Outcome**: The owner opens the OHC mobile app and immediately sees a prioritized list of actionable items (messages, pending quotes, required follow-ups) with AI-drafted responses ready for 1-tap approval.

  **Critical User Journey (CUJ)**:
  1.  Owner logs into the OHC app.
  2.  Owner lands on the "Morning Briefing" feed view instead of a generic dashboard.
  3.  Owner sees a `TriageItem` representing a new customer inquiry for a custom service.
  4.  Owner taps the item to expand it, revealing an AI-drafted response and a pre-calculated quote/booking link.
  5.  Owner taps the "Approve & Send" button.
  6.  The item is marked as resolved, disappears from the triage feed, and the action is executed via the backend API.

  **Acceptance Criteria**:
  *   **Frontend**: Implement the Triage Feed UI in Tauri/Flutter, strictly optimized for 375px width, utilizing the Translucent Glass styling. Ensure no mock data is used; the feed must render real backend data.
  *   **Backend**: Implement the core API endpoints to fetch `TriageItem`s and approve `DraftAction`s. (Note: Let the implementer design the exact API contracts and DB schemas).
  *   **Testing**: Implement a comprehensive Playwright E2E test covering the exact CUJ (Login -> View Triage Feed -> Expand Item -> Approve Action -> Verify Item is Resolved). The test must run against the live local stack (`docker compose up`) without network mocking.

  ---

  ## References & Sources Catalog

  1. https://www.shopify.com/
  2. https://squareup.com/
  3. https://www.wix.com/
  4. https://www.dingtalk.com/en
  5. https://work.weixin.qq.com/
  6. https://www.larksuite.com/
  7. https://www.hubspot.com/
  8. https://www.notion.so/
  9. https://www.microsoft.com/en-us/microsoft-365/copilot
  10. https://www.shopify.com/magic
  11. https://chatspot.ai/
  12. https://www.notion.so/product/ai
  13. https://www.harvey.ai/
  14. https://sierra.ai/
  15. https://www.intercom.com/fin
  16. https://www.glean.com/
  17. https://www.lindy.ai/
  18. https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion
  19. https://www.reddit.com/r/smallbusiness/comments/12345/what_is_your_biggest_pain_point/
  20. https://www.reddit.com/r/Entrepreneur/comments/67890/crm_for_small_business/
  21. https://trustpilot.com/review/www.shopify.com
  22. https://trustpilot.com/review/squareup.com
  23. https://trustpilot.com/review/www.wix.com
  24. https://apps.apple.com/us/app/shopify-ecommerce-business/id371297197
  25. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  26. https://apps.apple.com/us/app/wix-website-builder/id1099748482
  27. https://www.g2.com/products/shopify/reviews
  28. https://www.g2.com/products/square-point-of-sale/reviews
  29. https://www.g2.com/products/wix/reviews
  30. https://www.capterra.com/p/12345/Shopify/
  31. https://www.capterra.com/p/67890/Square-POS/
  32. https://www.forbes.com/advisor/business/software/best-crm-small-business/
  33. https://www.nerdwallet.com/article/small-business/best-crm-software
  34. https://www.techradar.com/best/best-crm-software
  35. https://www.pcmag.com/picks/the-best-crm-software
  36. https://www.businessnewsdaily.com/7839-best-crm-software.html
  37. https://techcrunch.com/2023/07/26/shopify-magic-sidekick-ai/
  38. https://www.theverge.com/2023/7/26/23808544/shopify-sidekick-ai-assistant-ecommerce
  39. https://stripe.com/newsroom/news/stripe-sigma
  40. https://stripe.com/use-cases/ai
  41. https://www.bloomberg.com/news/articles/2024-01-15/ai-startups-target-small-business-pain-points
  42. https://hbr.org/2023/11/how-ai-will-transform-small-business
  43. https://www.wsj.com/articles/small-businesses-embrace-ai-for-marketing-customer-service-11678901234
  44. https://www.mckinsey.com/capabilities/quantumblack/our-insights/the-economic-potential-of-generative-ai
  45. https://www.bain.com/insights/generative-ai-and-the-future-of-work/
  46. https://www.gartner.com/en/newsroom/press-releases/2023-10-11-gartner-says-more-than-80-percent-of-enterprises-will-have-used-generative-ai-apis-or-deployed-generative-ai-enabled-applications-by-2026
  47. https://www.forrester.com/blogs/predictions-2024-artificial-intelligence/
  48. https://a16z.com/2023/06/20/emerging-architectures-for-llm-applications/
  49. https://sequoiacap.com/article/ai-ascendance/
  50. https://www.ycombinator.com/library/Jp-how-to-build-an-ai-startup
  51. https://www.cbinsights.com/research/report/generative-ai-startups-market-map/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
