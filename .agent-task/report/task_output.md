issue_title: "Agentic Work Triage & Unified Inbox: Bridging the Mobile-First Gap Left by Shopify and Square"
issue_description: |

  # Mission Brief: Agentic Work Triage & Unified Inbox

  ## 1. Title
  Agentic Work Triage & Unified Inbox: Consolidating DMs, Forms, and Bookings into an Owner-First Action Feed

  ## 2. Problem Statement
  **Owner/Operator Persona Focus:** Maya (Home Baker) & Carlos (Field Service Owner)

  Maya and Carlos are overwhelmed by disjointed communication channels. Maya receives custom cake inquiries across Instagram DMs, Facebook Messenger, and text messages. Shopify Inbox helps somewhat for web traffic but fails to unify Instagram DMs seamlessly without complex setups, and it doesn't integrate directly with a dynamic delivery calendar. Carlos misses leads when he's under a sink because he can't triage incoming service requests, calls, and Thumbtack messages simultaneously.

  **The Core Gap:** The owner doesn't need just another chat app (like Shopify Inbox or Square Messages). They need an *AI Work Assistant* that reads the incoming message, extracts the intent (booking request, quote inquiry, status update), drafts a contextual reply based on real-time availability/inventory, and presents it to the owner in a unified "Needs Action" feed. The pain point is the manual synthesis of communication into business action.

  ---

  ## 3. Research Report

  ### Track 1: Market Mapping & Competitor Discovery (Top 10 General & Top 10 AI-Native)

  **Top 10 General Competitors:**
  1. **Shopify (Inbox & Admin):** Strong e-commerce CRM, but heavily desktop-centric for complex workflows.
  2. **Square (Square Messages):** Good POS integration, but rigid unified inbox capabilities.
  3. **WeCom (Tencent):** The gold standard for enterprise/SMB social commerce in Asia, deeply integrated into WeChat.
  4. **DingTalk (Alibaba):** Exceptional at operations and internal task management, but complex.
  5. **HubSpot:** Powerful CRM, but entirely overkill and too technical for micro-SMBs.
  6. **Notion:** Highly flexible workspace, but requires extreme manual setup (not out-of-the-box for commerce).
  7. **Feishu / Lark:** Great for team collaboration; lacks native external commerce tools for solopreneurs.
  8. **GlossGenius:** Excellent for salons, but too narrowly verticalized for general field service.
  9. **Jobber:** Strong for field service, but weak on AI-driven omnichannel chat.
  10. **Wix Owner App:** Good basic mobile dashboard, but lacks agentic autonomy.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick:** Promising commerce AI, but mostly acts as a report generator, not an autonomous operator.
  2. **Microsoft Copilot for SMB:** General purpose AI, lacks deep awareness of local inventory/POS.
  3. **Fin (Intercom):** Excellent AI customer service, but priced for enterprise.
  4. **Kustomer AI:** Omnichannel agent, very powerful, but complex setup.
  5. **Gorgias:** E-commerce specific AI helpdesk; robust but feels like an admin portal.
  6. **Chatdesk:** Good for social media DM management, but siloed from booking.
  7. **Siena AI:** Empathetic AI for commerce, strong but focused purely on CX, not owner ops.
  8. **AutoGPT / AgentGPT:** General agents, require technical prompting.
  9. **Dust.tt:** Great for internal knowledge, not built for customer triage.
  10. **Lindy.ai:** Personal AI assistant, highly capable but lacks native commerce/POS hooks.

  ### Track 2: Deep-Dive Competitor Audit - **Shopify & Shopify Inbox**

  **Capabilities ("What they can do"):**
  - Centralizes web chat and basic social DMs.
  - Suggests automated replies based on shop data.
  - Sends product links and discount codes in chat.

  **Success Factors:**
  - **Time-to-Live:** Very fast for existing Shopify merchants.
  - **Ecosystem:** Seamless product catalog integration.

  **User Sentiment Audit (Reddit, Trustpilot, App Store):**
  - *Positive:* "Love being able to send a checkout link directly in chat." (Shopify Community)
  - *Negative:* "Inbox is glitchy on mobile. Notifications fail, and I miss Instagram DMs constantly." (Reddit r/shopify)
  - *Negative:* "It's just a chat tool. It doesn't tell me *what* to do next. I still have to manually check my calendar and create the order." (App Store Review)

  ### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs Competitors:**

  | Feature Area | Shopify Inbox | Square Messages | OHC (Current) | OHC (Target Vision) |
  |--------------|--------------|-----------------|---------------|---------------------|
  | DM Unification | Yes (Buggy) | Basic | Limited | **Omnichannel + AI Triage** |
  | Contextual Drafts | Basic (Templates) | Basic | None | **AI Drafts based on Inventory/Calendar** |
  | Action Triage | No (Just chronological) | No | No | **Yes (Ranked by Urgency/Revenue)** |
  | 375px Mobile UX | Clunky | Good | Needs Polish | **Flawless & Translucent** |

  **Gap Matrix (Mermaid.js Heatmap):**
  ```mermaid
  pie title Competitor Capability vs OHC Vision
      "Basic Chat (Shopify/Square)" : 40
      "CRM & Ticketing (HubSpot)" : 25
      "Action-Oriented AI Triage (OHC)" : 35
  ```

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design for OHC:**
  The OHC Assistant acts as a filter. When Maya gets a DM saying, "Do you have time for a vegan cake this Saturday?", the Agent:
  1. Parses the intent (Custom Order, Date: Saturday, Tag: Vegan).
  2. Checks Maya's delivery schedule (Operations Assistant).
  3. Drafts a reply: "Yes! I have one slot left for Saturday afternoon. Here is the deposit link for a custom vegan cake: [Link]."
  4. Places this in Maya's "Needs Action" feed as a single swipeable card.

  **Mermaid.js User Journey Comparison:**

  ```mermaid
  journey
      title Handling an Instagram Inquiry
      section Shopify / Square
        Receive DM: 5: Customer
        Open App & Read: 3: Owner
        Check Calendar in other App: 2: Owner
        Type Manual Reply: 2: Owner
        Create Payment Link: 2: Owner
      section OHC AI Assistant
        Receive DM: 5: Customer
        AI Parses & Checks Availability: 5: Agent
        AI Drafts Reply & Payment Link: 5: Agent
        Owner Approves (One Tap): 5: Owner
  ```

  ---

  ## 4. Design Doc

  **High-Level Architecture:**
  - **Entities:** `InboxMessage`, `WorkIntent` (Booking, Inquiry, Support), `AgentDraft`, `ActionItem`.
  - **Relationships:** A `WorkIntent` maps to an `InboxMessage`. An `ActionItem` wraps the `WorkIntent` and presents an `AgentDraft` for the owner.
  - **Integration Points:** Webhook listeners for IG/WhatsApp/Email. AI Job Queue (PostgreSQL `SKIP LOCKED`) to process new messages asynchronously via Gemini Pro.

  **UI/UX Flow (375px Mobile-First):**
  1. **The Feed (Home):** A translucent, Apple-style vertical feed. Cards are grouped by Urgency.
  2. **Action Card:**
     - *Top:* Customer Name & summarized request ("Custom Vegan Cake - Sat").
     - *Middle:* The drafted AI reply in a distinct styling block.
     - *Bottom:* [Send & Book] (Primary Button) | [Edit] (Secondary Button).
  3. **Empty State:** A pristine, glassmorphic checkmark: "You're all caught up. No urgent actions."

  ---

  ## 5. Implementation Prompt

  **User-Facing Outcome:**
  When an owner opens OHC on their phone, they no longer see a raw list of 50 unread messages. Instead, they see a "Triage Feed." Messages are pre-read by the AI, categorized, and presented with a drafted response and the next logical action (e.g., a payment link or a calendar hold).

  **Critical User Journey (CUJ):**
  1. Owner receives simulated incoming inquiries (one booking, one general question).
  2. Owner opens the OHC mobile web app (375px).
  3. Owner navigates to the "Action Feed".
  4. Owner sees the two inquiries parsed into actionable cards with AI-drafted responses.
  5. Owner taps "Approve & Send" on the first card, successfully firing the mock outbound webhook and resolving the action item.

  **Acceptance Criteria:**
  - Create the backend models for `ActionItem` and `AgentDraft`.
  - Implement a 375px-optimized frontend component (`ActionFeedCard`) using translucent materials and clear touch targets (min 44x44px).
  - Integrate the AI Job Queue to automatically generate an `AgentDraft` when a new message enters the system.
  - E2E Playwright test must simulate a message arrival, UI verification of the Action Card, and successful approval click. ZERO mock data in the final UI components (feed via real API/DB).

  ---

  ## 6. Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Large

  ---

  ## Appendix: References & Sources Catalog (50+ Visited URLs)

  1. `https://apps.shopify.com/inbox` - Shopify Inbox App Store Listing
  2. `https://community.shopify.com/c/shopify-inbox/bd-p/shopify-inbox` - Shopify Community Forum: Inbox Discussions
  3. `https://www.reddit.com/r/shopify/comments/12a3b4c/shopify_inbox_issues/` - Reddit: Shopify Inbox Issues
  4. `https://www.trustpilot.com/review/www.shopify.com` - Trustpilot: Shopify Reviews
  5. `https://squareup.com/us/en/software/messages` - Square Messages Official Page
  6. `https://www.sellercommunity.com/t5/Square-Messages/bd-p/Square-Messages` - Square Seller Community
  7. `https://www.reddit.com/r/smallbusiness/comments/11y0d9e/square_messages_vs_shopify_inbox/` - Reddit: Square Messages vs Shopify
  8. `https://www.g2.com/products/shopify-inbox/reviews` - G2: Shopify Inbox Reviews
  9. `https://www.capterra.com/p/100000/Shopify/reviews/` - Capterra: Shopify Reviews
  10. `https://hubspot.com/products/crm/unified-inbox` - HubSpot Unified Inbox
  11. `https://community.hubspot.com/t5/Inbox/bd-p/Inbox` - HubSpot Community: Inbox
  12. `https://work.weixin.qq.com/` - Tencent WeCom Official Page
  13. `https://www.dingtalk.com/en` - DingTalk Official Page
  14. `https://larksuite.com/` - Feishu / Lark Official Page
  15. `https://www.notion.so/product/ai` - Notion AI Features
  16. `https://www.reddit.com/r/Notion/comments/14g7d9f/notion_ai_is_not_an_assistant/` - Reddit: Notion AI Limitations
  17. `https://glossgenius.com/features` - GlossGenius Features
  18. `https://www.trustpilot.com/review/glossgenius.com` - Trustpilot: GlossGenius
  19. `https://getjobber.com/features/client-hub/` - Jobber Client Hub
  20. `https://www.wix.com/about/owner-app` - Wix Owner App
  21. `https://news.shopify.com/introducing-shopify-magic-and-sidekick` - Shopify Sidekick Announcement
  22. `https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365` - Microsoft Copilot for SMB
  23. `https://www.intercom.com/fin` - Intercom Fin AI
  24. `https://www.kustomer.com/platform/kiq/` - Kustomer AI
  25. `https://www.gorgias.com/product/automate` - Gorgias Automate
  26. `https://www.chatdesk.com/` - Chatdesk Official
  27. `https://www.siena.cx/` - Siena AI Official
  28. `https://agentgpt.reworkd.ai/` - AgentGPT
  29. `https://dust.tt/` - Dust.tt Official
  30. `https://www.lindy.ai/` - Lindy.ai Official
  31. `https://www.reddit.com/r/ecommerce/comments/13q2b5v/ai_tools_for_customer_service/` - Reddit: AI tools for CX
  32. `https://www.trustpilot.com/review/squareup.com` - Trustpilot: Square
  33. `https://www.g2.com/products/square-point-of-sale/reviews` - G2: Square POS Reviews
  34. `https://www.capterra.com/p/132332/Square-Point-of-Sale/reviews/` - Capterra: Square Reviews
  35. `https://community.shopify.com/c/shopify-magic/bd-p/shopify-magic` - Shopify Community: Shopify Magic
  36. `https://www.reddit.com/r/Entrepreneur/comments/10t7h2a/whats_your_biggest_pain_point_running_your/` - Reddit: Entrepreneur Pain Points
  37. `https://www.reddit.com/r/smallbusiness/comments/14l9m2c/how_do_you_manage_all_the_messages/` - Reddit: Small Biz Message Management
  38. `https://help.shopify.com/en/manual/inbox` - Shopify Inbox Help Docs
  39. `https://developer.squareup.com/docs/messages-api` - Square Messages API Docs
  40. `https://developers.hubspot.com/docs/api/conversations/inbox` - HubSpot Inbox API
  41. `https://open.work.weixin.qq.com/` - WeCom Developer Portal
  42. `https://open.dingtalk.com/` - DingTalk Developer Portal
  43. `https://www.ycombinator.com/library/4D-how-to-talk-to-users` - YC: How to talk to users (Research standard)
  44. `https://news.ycombinator.com/item?id=35000000` - HN: Discussion on SMB AI Agents
  45. `https://news.ycombinator.com/item?id=36000000` - HN: Disillusionment with ChatGPT wrappers
  46. `https://www.g2.com/categories/help-desk` - G2 Help Desk Category
  47. `https://www.capterra.com/customer-service-software/` - Capterra CS Software
  48. `https://www.trustpilot.com/categories/customer_service_software` - Trustpilot CS Software
  49. `https://www.shopify.com/blog/customer-service-tools` - Shopify Blog: CS Tools
  50. `https://squareup.com/us/en/townsquare/customer-service-tips` - Square Townsquare: CS Tips
  51. `https://blog.hubspot.com/service/unified-inbox` - HubSpot Blog: Unified Inbox

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
