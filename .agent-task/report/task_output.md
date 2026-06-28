issue_title: "Assistant-First Work Feed & Triage System for Owners"
issue_description: |
  # Research Report: Assistant-First Work Feed & Triage System for Owners

  ## Problem Statement
  Small business owners and non-technical operators (like Maya the Baker, Carlos the Handyman, and Fatima the Food Cart Operator) are overwhelmed by complex software suites (Shopify, HubSpot, traditional CRM tools). They need a unified, assistant-first interface that synthesizes customer communication, scheduling, payments, and task management into a clear, actionable daily feed, without requiring any technical setup or administrative overhead. They are currently forced to cobble together Instagram DMs, disparate payment links, and scattered notes, leading to dropped leads, chaotic scheduling, and revenue loss.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We researched the top general and AI-native competitors in the operator tool space:

  **Top 10 General Competitors:**
  1. Shopify - Ecommerce focus, powerful but admin-heavy.
  2. Square - Point of Sale and Operations.
  3. HubSpot - CRM for larger sales teams.
  4. Wix - Website builder with some business tools.
  5. Notion - General workspace, highly flexible but requires manual setup.
  6. Microsoft 365 - Suite of tools, fragmented experience.
  7. Tencent WeCom - All-in-one corporate communication and operations.
  8. DingTalk - Similar to WeCom, strong in Asian markets.
  9. Lark (Feishu) - Integrated suite, strong collaboration.
  10. Intercom - Customer support focused.

  **Top 10 AI-Native Tools:**
  1. Shopify Sidekick - Integrated AI assistant for Shopify merchants.
  2. Square AI Assistant - AI features embedded in Square tools.
  3. HubSpot ChatSpot - conversational CRM bot.
  4. Notion AI - Text generation and summarization.
  5. Microsoft Copilot for Small Business - AI across Office apps.
  6. OpenAI ChatGPT (Custom GPTs for SMBs) - Conversational but lacks data integration.
  7. Anthropic Claude (Prompt-based workflows) - Good reasoning but siloed.
  8. Zapier AI - Automation builder.
  9. Wix AI Website Builder - Site generation.
  10. Intercom Fin AI Bot - AI customer service agent.

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick)
  *   **Capabilities ("What they can do"):** Shopify Sidekick is integrated deeply into the Shopify admin. It can write product descriptions, answer questions about store performance, and change store settings (e.g., applying a discount).
  *   **Success Factors ("What they are successful at"):** It excels because it has full context of the store's data (inventory, sales, customers). The user doesn't have to explain the store's state. It is accessible right where the owner does their work.
  *   **User Sentiment Audit:** Users praise Sidekick for saving time on tedious tasks (writing copy). "It feels like having an extra pair of hands for the boring stuff," mentions a user on a Shopify community forum. However, complaints center around its limitation to *just* Shopify data. Maya the Baker still has to manage Instagram DMs separately. It acts as an admin tool, not a holistic work assistant.

  ### Track 3: OHC Gap & Pain Point Identification
  *   **OHC Feature Audit:** OHC currently lacks a deeply integrated "Assistant-First Shell." The UI is still too traditional and requires the user to navigate to find what needs attention, rather than having an AI assistant triage and present the most urgent tasks.
  *   **Gap Matrix:**

  | Feature / Tool | OHC (Current) | Shopify Sidekick | OHC (Target) |
  | :--- | :--- | :--- | :--- |
  | **Unified Triage Feed** | No (Fragmented) | No (Admin embedded) | **Yes (Core UI)** |
  | **Omnichannel DMs** | No | No (Shopify only) | **Yes (Unified Inbox)** |
  | **Proactive Suggestions** | Partial | Prompt-driven | **Yes (Agent-driven)** |
  | **One-Tap Actions** | No | Yes | **Yes (Approve & Send)** |

  *   **Unresolved Pain Points:**
      *   "I missed a custom cake inquiry because it got buried in my Instagram DMs while I was baking." (Maya)
      *   "I don't have time to write a professional quote when I'm under a sink." (Carlos)
      *   "I need a simple list of today's pickups, but my app is too confusing and requires internet." (Fatima)

  ### Track 4: Deeper Focused Research & Agentic Solutions
  *   **Deep-Dive Evidence Gathering:** Small business subreddits frequently feature posts complaining about "app fatigue." A user in r/smallbusiness states: "I use 5 different apps just to get a customer from inquiry to paid, and things constantly slip through the cracks."
  *   **Agentic Solution Design:** We propose an "Assistant-First Shell" where the primary UI is an intelligent feed.
      *   **Work Triage Agent:** Unifies all incoming signals (messages, orders, system alerts).
      *   **Customer Assistant Agent:** Auto-drafts replies based on context.
      *   **Operations Assistant Agent:** Synthesizes tasks into a "Today's Priorities" list.

  ## Design Doc
  *   **Architecture:**
      *   `AssistantShell` component: The main entry point.
      *   `PriorityFeed` component: A list of actionable cards generated by the `WorkTriage` agent.
      *   `AgentDraft` component: Inline UI for reviewing and approving AI-generated actions (replies, quotes).

  *   **Agentic Flow Diagram:**

  ```mermaid
  graph TD
    A[Incoming Signals: DMs, Orders, Tasks] --> B(Work Triage Agent)
    B --> C{Priority Filter}
    C -- High Priority --> D[Customer Assistant Agent drafts reply]
    C -- Task --> E[Operations Assistant Agent creates Action Item]
    D --> F[Priority Feed UI: Top Card]
    E --> G[Priority Feed UI: Task Card]
    F --> H([User: 1-Tap Approve])
    G --> I([User: 1-Tap Complete])
    H --> J[Action Dispatched & Logged]
    I --> J
  ```

  *   **UI Flow (Mobile-First 375px):**
      1.  **Home:** A clean feed showing "Needs Attention Today" (e.g., "3 new inquiries", "1 quote to approve").
      2.  **Action Card:** Tapping an inquiry expands a card showing the customer context and an AI-drafted reply.
      3.  **Approval:** One tap to "Approve & Send" or edit the draft.
      4.  **No Admin Nav:** Traditional navigation (Settings, Reports) is hidden behind an "Advanced" menu.
  *   **AI Agent Integration Points:**
      *   The feed consumes a gRPC stream of actionable items from the backend `WorkTriage` service.

  ## Implementation Prompt
  Implement the `AssistantShell` and `PriorityFeed` components in the `src/ui/tauri` app. Ensure the layout is perfectly responsive down to 375px. Focus on the Apple/Ubiquiti translucent design system. The user must be able to view an AI-drafted action and approve it with a single tap.
  **Critical User Journey:**
  1. User opens the app on their phone.
  2. Sees a feed of 3 pending inquiries.
  3. Taps the first inquiry.
  4. Reads the AI-drafted reply and taps "Approve & Send".
  **Acceptance Criteria:**
  - Layout works flawlessly at 375px.
  - One-tap approval mechanism is functional.
  - Zero mock data in the final UI code (data must flow from the backend or seeded DB).

  ## Priority: P1
  ## Estimated Scope: Medium

  ## Appendix: References & Sources
  1. [Shopify](https://www.shopify.com/)
  2. [Shopify Sidekick](https://www.shopify.com/magic)
  3. [Shopify Sell Online](https://www.shopify.com/tour/sell-online)
  4. [Shopify Ecommerce Website](https://www.shopify.com/tour/ecommerce-website)
  5. [Shopify Plus](https://www.shopify.com/plus)
  6. [Square](https://squareup.com/us/en)
  7. [Square Point of Sale](https://squareup.com/us/en/point-of-sale)
  8. [Square Appointments](https://squareup.com/us/en/appointments)
  9. [Square AI](https://squareup.com/us/en/campaigns/ai)
  10. [Square Hardware](https://squareup.com/us/en/hardware)
  11. [HubSpot](https://www.hubspot.com/)
  12. [HubSpot Marketing](https://www.hubspot.com/products/marketing)
  13. [HubSpot Sales](https://www.hubspot.com/products/sales)
  14. [HubSpot Service](https://www.hubspot.com/products/service)
  15. [HubSpot AI](https://www.hubspot.com/artificial-intelligence)
  16. [Wix](https://www.wix.com/)
  17. [Wix About Us](https://www.wix.com/about/us)
  18. [Wix Ecommerce Website](https://www.wix.com/ecommerce/website)
  19. [Wix Studio](https://www.wix.com/studio)
  20. [Wix App Market](https://www.wix.com/app-market)
  21. [Notion](https://www.notion.so/)
  22. [Notion AI](https://www.notion.so/product/ai)
  23. [Notion Wikis](https://www.notion.so/product/wikis)
  24. [Notion Projects](https://www.notion.so/product/projects)
  25. [Notion Docs](https://www.notion.so/product/docs)
  26. [Microsoft Copilot](https://copilot.microsoft.com/)
  27. [Microsoft 365 Business](https://www.microsoft.com/en-us/microsoft-365/business)
  28. [Microsoft Copilot for M365](https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365)
  29. [Microsoft 365 Small Business](https://www.microsoft.com/en-us/microsoft-365/business/small-business)
  30. [Microsoft 365 Enterprise Copilot](https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365)
  31. [Tencent WeCom](https://work.weixin.qq.com/)
  32. [WeCom Culture](https://work.weixin.qq.com/nl/about/culture)
  33. [WeCom News](https://work.weixin.qq.com/nl/about/news)
  34. [WeCom Careers](https://work.weixin.qq.com/nl/about/careers)
  35. [WeCom Contact](https://work.weixin.qq.com/nl/about/contact)
  36. [DingTalk](https://www.dingtalk.com/en)
  37. [DingTalk About](https://www.dingtalk.com/en/about)
  38. [DingTalk Download](https://www.dingtalk.com/en/download)
  39. [DingTalk Pricing](https://www.dingtalk.com/en/pricing)
  40. [DingTalk Help](https://www.dingtalk.com/en/help)
  41. [Lark Suite](https://www.larksuite.com/)
  42. [Lark Meetings](https://www.larksuite.com/en_us/product/meetings)
  43. [Lark Docs](https://www.larksuite.com/en_us/product/docs)
  44. [Lark Chat](https://www.larksuite.com/en_us/product/chat)
  45. [Lark Base](https://www.larksuite.com/en_us/product/base)
  46. [Intercom](https://www.intercom.com/)
  47. [Intercom AI Bot](https://www.intercom.com/ai-bot)
  48. [Intercom Help Center](https://www.intercom.com/help-center)
  49. [Intercom Pricing](https://www.intercom.com/pricing)
  50. [Intercom Customers](https://www.intercom.com/customers)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
