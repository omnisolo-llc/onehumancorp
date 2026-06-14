issue_title: "OHC Owner Work Assistant Market Research & Gap Analysis"
issue_description: |
  # OHC Owner Work Assistant Market Research & Gap Analysis

  ## Problem Statement
  Owners and operators of small businesses (like Maya the baker, Carlos the handyman, and Fatima the food cart owner) face significant challenges in managing their daily operations, customer interactions, and financial tracking. Existing tools are often fragmented, technically complex, or fail to provide a unified, actionable view of their business. OHC needs to bridge this gap by offering a cohesive AI work assistant that simplifies these tasks and proactively guides owners toward their next best action, without the steep learning curve of traditional software suites.

  ## Research Report

  ### Executive Summary
  This report investigates the competitive landscape of AI work assistants tailored for small business owners and operators. It analyzes both established platforms integrating AI features and emerging AI-native solutions to identify key market trends, successful capabilities, and critical gaps that OHC must address to fulfill its vision.

  ### Track 1: Market Mapping & Competitor Discovery
  We have evaluated a broad spectrum of tools serving the owner/operator segment:

  **Top 10 General Competitors:**
  1.  **Shopify:** E-commerce giant, recently introduced Shopify Magic and Sidekick for AI-assisted store management.
  2.  **Square:** Point-of-Sale leader, integrating AI for inventory, appointment scheduling, and customer insights.
  3.  **Wix:** Website builder with robust AI tools for design, content generation, and business management.
  4.  **HubSpot:** Comprehensive CRM with a growing suite of AI tools for marketing, sales, and customer service.
  5.  **Tencent Workbuddy / WeCom:** Enterprise-grade collaboration and operations management, strong in the Asian market.
  6.  **DingTalk:** Alibaba's communication and collaboration platform, heavily focused on organizational efficiency.
  7.  **Lark (Feishu):** ByteDance's unified suite combining chat, docs, and calendar, increasingly incorporating AI.
  8.  **Notion:** Flexible workspace integrating AI for writing, summarizing, and database management.
  9.  **Microsoft Copilot:** Deeply integrated AI assistant across the Microsoft 365 ecosystem.
  10. **Zendesk:** Customer service platform leveraging AI for automated responses and agent assistance.

  **Top Emerging AI-Native Competitors & Features:**
  1.  **Intercom Fin:** AI customer service bot capable of resolving complex queries based on knowledge bases.
  2.  **Shopify Sidekick:** Dedicated AI assistant for merchants to answer questions, modify store settings, and generate content.
  3.  **Square AI Features:** Automated product descriptions, predictive inventory, and smart scheduling.
  4.  **Wix AI Creator:** Generates complete websites, text, and images based on simple prompts.
  5.  **HubSpot Content Assistant:** AI-powered content creation integrated directly into the CRM.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick & Magic
  **Selection:** Shopify Sidekick represents the most direct attempt by a major platform to create an "owner assistant."

  *   **Capabilities:** Sidekick can answer questions about store performance (e.g., "Why are my sales down?"), perform actions (e.g., "Put all summer items on sale"), and generate content (product descriptions, emails). Magic focuses on text generation across the platform.
  *   **Success Factors:** Deep integration with the merchant's data. It understands the context of the store. The conversational interface lowers the barrier to executing complex bulk actions.
  *   **User Sentiment (Aggregated from Forums/Reviews):**
      *   *Positives:* "Saves hours on writing descriptions," "Helps me understand analytics without digging through reports."
      *   *Negatives/Pain Points:* "Still feels like a chatbot tacked onto a complex backend," "Doesn't proactively tell me what to do, I still have to ask," "Setup for the rest of the store is still overwhelming for a beginner."

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC vs. Shopify/Square/Wix Gap Matrix:**

  | Feature / Capability | Shopify Sidekick | Square AI | Wix AI | **OHC Target State** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Intake** | Partial (App dependent) | Partial | Partial | **Full (DMs, emails, calls unified)** |
  | **Proactive Suggestions** | Low (Reactive to queries) | Low | Low | **High (Daily work feed)** |
  | **Mobile-First Operations**| Medium | High (POS focus) | Medium | **Critical (Fully functional on 375px)** |
  | **Cross-Domain Action** | Medium (Store only) | Medium | Low | **High (Connects tasks, comms, payments)** |
  | **Setup Complexity** | High | Medium | Low/Medium | **Extremely Low (AI-guided setup)** |

  **Unresolved Pain Points:**
  1.  **The "Blank Dashboard" Problem:** Owners log in and see metrics but don't know what to *do* next. Tools are reactive, not proactive.
  2.  **Fragmented Context:** Customer conversations live in Instagram, orders in Shopify, tasks on paper. AI tools often only see one piece of the puzzle.
  3.  **Mobile Compromise:** Full management still requires a desktop. Mobile apps are often stripped-down versions or just dashboards.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Small business owners (e.g., on r/smallbusiness) frequently complain that while AI helps write emails, it doesn't solve the core issue of coordinating the *actual work*—connecting a customer DM to a quote, to an inventory check, to a scheduled task.

  **Agentic Solution Design for OHC:**
  Instead of a chatbot side-panel, OHC must be structured as a **proactive work feed**.
  *   **The Triage Engine:** An background agent constantly monitors connected channels (DMs, emails, forms). It doesn't just read them; it categorizes them into actionable items (e.g., "Maya: New cake inquiry from Sarah").
  *   **The Draft & Propose Model:** When an owner clicks an item, the Customer Assistant has already drafted a reply, and the Operations Assistant has pre-filled a quote based on the request. The owner's job is simply to review and approve.
  *   **The Daily Briefing:** Upon login, the Decision Assistant presents a natural language summary: "You have 3 urgent inquiries, 1 delivery today, and sales are up 10% this week. Here is your suggested priority list."


  ### Visual Analysis

  **Dynamic Competitive Landscape**
  ```mermaid
  quadrantChart
      title Market Positioning: AI Work Assistants
      x-axis "Traditional Suite" --> "AI-Native Assistant"
      y-axis "Enterprise Focus" --> "Owner/Operator Focus"
      quadrant-1 "Emerging Threat"
      quadrant-2 "OHC Target Zone"
      quadrant-3 "Traditional Enterprise"
      quadrant-4 "Complex Commerce Suite"
      "Shopify Sidekick": [0.6, 0.2]
      "Square AI": [0.5, 0.3]
      "Wix AI": [0.7, 0.4]
      "HubSpot": [0.3, 0.7]
      "Lark / Feishu": [0.2, 0.8]
      "Microsoft Copilot": [0.1, 0.9]
      "Intercom Fin": [0.8, 0.6]
      "OHC": [0.9, 0.1]
  ```

  **User Journey Comparison: Blank Dashboard vs. OHC Feed**
  ```mermaid
  journey
      title Daily Operations: Traditional vs. OHC
      section Traditional Tool (Shopify/Square)
        Log in to dashboard: 3: Owner
        See static charts: 2: Owner
        Search for new messages: 2: Owner
        Switch to email/Instagram: 1: Owner
        Manually draft reply & quote: 1: Owner
      section OHC Assistant Feed
        Open app to Action Feed: 5: Owner
        See prioritized DMs & Tasks: 5: Owner
        Tap to review AI-drafted reply: 4: Owner
        Approve reply and pre-filled quote: 5: Owner
        Return to feed for next action: 5: Owner
  ```

  **Persona Pain Point Matrix**
  ```mermaid
  mindmap
    root((Unresolved Pain Points))
      Fragmented Context
        Maya (Baker)
        Carlos (Handyman)
      Blank Dashboard
        Priya (Boutique)
        Leo (Tutor)
      Mobile Compromise
        Fatima (Food Cart)
        Carlos (Handyman)
      High Setup Complexity
        Maya (Baker)
        Fatima (Food Cart)
  ```

  ### Persona-Specific Pain Point Summaries

  *   **Maya (Baker):** Overwhelmed by the setup complexity of platforms like Shopify. She needs an assistant that guides her through creating custom-order deposit flows without needing a computer science degree. The current "blank dashboard" of ecommerce tools doesn't tell her which DMs need her immediate attention to secure a booking.
  *   **Carlos (Handyman):** Operates almost entirely from his Android phone in the field. Desktop-first tools are useless to him. His biggest pain point is the fragmented context—he loses track of word-of-mouth leads because he doesn't have time to manually log them into a CRM while working.
  *   **Priya (Boutique Operator):** Struggles with bridging her in-store POS context with her online efforts. She sees data in her Square dashboard but finds it difficult to translate those charts into actionable marketing emails or inventory decisions. She needs a tool that doesn't just show data, but drafts the email based on it.
  *   **Leo (Music Tutor):** Deals with scheduling chaos across text messages, emails, and phone calls. Existing tools force him to build a scheduling webpage, but his clients prefer to just ask him via text. He needs an AI that can ingest a text message like "Can we do Tuesday at 4?" and turn it into a calendar booking and payment link automatically.
  *   **Fatima (Food Cart Operator):** Faces language barriers and slow mobile data. English-first, desktop-heavy tools are inaccessible. She needs an extremely simple, offline-tolerant interface that just tells her what pre-orders to prep and when to hand them off.

  ## Design Doc

  **Architecture Overview:**
  *   **Frontend (Flutter):** A unified, mobile-first interface prioritizing the "Work Feed" over traditional navigation menus.
  *   **Backend (Go):** Event-driven architecture. Incoming webhooks (messages, payments) trigger the AI Job Queue.
  *   **AI Orchestration:** Agents (Triage, Customer, Ops, Finance) evaluate events. They communicate via distributed locks (Redis) to build a unified context before presenting it to the user.

  **UX Flow (Mobile 375px Focus):**
  1.  **Home Screen ("The Feed"):** A vertical list of cards prioritized by urgency. No charts initially.
      *   *Card Example:* "New Message: Custom Cake Inquiry (Instagram). [Review Draft]"
  2.  **Action Screen (Tapping a Card):**
      *   Top half: The customer's original message and vital context (past orders).
      *   Bottom half: AI-generated proposed actions (Draft Reply, Create Quote, Decline).
  3.  **Execution:** Tapping 'Draft Reply' opens an editable text field with the AI's suggestion. A large 'Send' button completes the action and returns the user to the Feed.

  ## Implementation Prompt

  **User-Facing Outcome:** Develop the core "Unified Work Feed" UI for the mobile app (375px width). When a user logs in, they should see a prioritized list of actionable items generated by the backend agents, rather than a traditional dashboard of metrics or a blank inbox.

  **Critical User Journey (CUJ):**
  1.  User (e.g., Maya) opens the OHC app on her phone.
  2.  The Home screen displays a prioritized list of tasks (e.g., "Respond to 2 new DMs", "Approve quote for John").
  3.  User taps a task card.
  4.  User is presented with the context and an AI-drafted response or action.
  5.  User approves or modifies the action, completing the task, and is returned to the feed.

  **Acceptance Criteria:**
  *   The UI must render perfectly on a 375px width screen without horizontal scrolling.
  *   Interactive elements (buttons, cards) must have a minimum touch target of 44x44px.
  *   The feed must visually distinguish between different types of tasks (messages, operations, finance) using the OHC Design System tokens.
  *   E2E Playwright tests must verify the flow from feed -> task view -> action completion.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources Catalog
  1.  Shopify Sidekick: https://www.shopify.com/sidekick
  2.  Shopify Magic: https://www.shopify.com/magic
  3.  Shopify News: https://news.shopify.com/shopify-magic-and-sidekick
  4.  Shopify AI Blog: https://www.shopify.com/blog/ai-ecommerce
  5.  Shopify Magic Blog: https://www.shopify.com/blog/shopify-magic
  6.  Square AI for Business: https://square.com/us/en/townsquare/ai-for-business
  7.  Square POS: https://squareup.com/us/en/point-of-sale
  8.  Square AI Features: https://squareup.com/us/en/features/ai
  9.  Square Online: https://squareup.com/us/en/online-store
  10. Square Appointments: https://squareup.com/us/en/appointments
  11. Wix AI: https://www.wix.com/about/ai
  12. Wix AI Website Builder: https://www.wix.com/blog/ai-website-builder
  13. Wix AI Tools: https://www.wix.com/blog/ai-tools-for-business
  14. Wix AI eCommerce: https://www.wix.com/ecommerce/ai
  15. Wix Studio AI: https://www.wix.com/studio/ai
  16. HubSpot AI: https://www.hubspot.com/artificial-intelligence
  17. HubSpot AI Products: https://www.hubspot.com/products/artificial-intelligence
  18. HubSpot Content Writer: https://www.hubspot.com/artificial-intelligence/content-writer
  19. HubSpot Campaign Assistant: https://www.hubspot.com/artificial-intelligence/campaign-assistant
  20. HubSpot Chat Bot: https://www.hubspot.com/artificial-intelligence/chat-bot
  21. Tencent Workbuddy/WeCom: https://work.weixin.qq.com/
  22. WeCom API: https://work.weixin.qq.com/api/doc/90000/90135/90664
  23. DingTalk: https://www.dingtalk.com/en
  24. DingTalk Products: https://www.dingtalk.com/en/product
  25. DingTalk Features: https://www.dingtalk.com/en/features
  26. Lark AI: https://www.larksuite.com/en_us/product/ai
  27. Lark Base: https://www.larksuite.com/en_us/product/base
  28. Lark Messenger: https://www.larksuite.com/en_us/product/messenger
  29. Lark Meetings: https://www.larksuite.com/en_us/product/meetings
  30. Lark Docs: https://www.larksuite.com/en_us/product/docs
  31. Notion AI: https://www.notion.so/product/ai
  32. Notion Wikis: https://www.notion.so/product/wikis
  33. Notion Projects: https://www.notion.so/product/projects
  34. Notion Docs: https://www.notion.so/product/docs
  35. Notion Calendar: https://www.notion.so/product/calendar
  36. Microsoft Copilot: https://copilot.microsoft.com/
  37. Copilot for M365: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  38. M365 SMB: https://www.microsoft.com/en-us/microsoft-365/business/small-business-solutions
  39. M365 Business Standard: https://www.microsoft.com/en-us/microsoft-365/business/microsoft-365-business-standard
  40. M365 Business Premium: https://www.microsoft.com/en-us/microsoft-365/business/microsoft-365-business-premium
  41. Intercom Fin: https://www.intercom.com/fin
  42. Intercom AI CS: https://www.intercom.com/ai-customer-service
  43. Intercom AI Bot: https://www.intercom.com/features/ai-bot
  44. Intercom Inbox: https://www.intercom.com/features/inbox
  45. Intercom Help Center: https://www.intercom.com/features/help-center
  46. Zendesk AI: https://www.zendesk.com/service/ai/
  47. Zendesk Messaging: https://www.zendesk.com/service/messaging/
  48. Zendesk Help Center: https://www.zendesk.com/service/help-center/
  49. Zendesk Sell: https://www.zendesk.com/sell/
  50. Zendesk Pricing: https://www.zendesk.com/pricing/

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
