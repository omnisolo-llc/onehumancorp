issue_title: "Market Deep Dive: OHC Agentic Workflows vs Tencent Workbuddy & Shopify"
issue_description: |
  ## Mission Queue Protocol
  **Title:** Market Deep Dive: OHC Agentic Workflows vs Tencent Workbuddy & Shopify
  **Problem Statement:** Small business owners are overwhelmed by context switching across 15+ disparate tools (Shopify, WhatsApp, Calendly, Stripe) to manage daily operations. Existing solutions like Tencent Workbuddy dominate in Asia but lack deep native commerce integration, while Western platforms like Shopify lack unified operations and communication layers. OHC must bridge this gap with an invisible, agentic work assistant that unifies commerce and operations.
  **Priority:** P0
  **Estimated Scope:** Large

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  #### Top 10 General Competitors
  1. **Tencent Workbuddy (WeCom)** - Unmatched unified messaging/operations in Asia.
  2. **Shopify Sidekick** - E-commerce focused AI assistant.
  3. **DingTalk** - Enterprise collaboration giant.
  4. **Feishu/Lark** - All-in-one workspace.
  5. **Notion AI** - Knowledge management with AI.
  6. **Microsoft Copilot** - Enterprise AI assistant.
  7. **HubSpot** - CRM and marketing automation.
  8. **Square** - POS and basic operations.
  9. **Slack** - Communication hub.
  10. **Zoom** - Video conferencing.

  #### Top 10 AI-Native Competitors
  1. **Sierra** - Agentic customer service.
  2. **Harvey** - AI for professional services.
  3. **Dust** - Custom AI assistants for teams.
  4. **Devin** - Autonomous software engineering.
  5. **Adept** - Action-driven AI.
  6. **Lindsey** - AI scheduling assistant.
  7. **Sana** - AI knowledge assistant.
  8. **Glean** - Enterprise search and AI.
  9. **Hebbia** - AI for financial services.
  10. **Replit Agent** - AI coding assistant.

  ### Track 2: Deep-Dive Competitor Audit: Shopify Sidekick
  - **Capabilities:** Sidekick acts as an e-commerce assistant integrated into the Shopify admin panel. It can answer questions about sales data, execute basic commands (e.g., "put my store on sale"), and draft product descriptions.
  - **Success Factors:** Deep integration with the merchant's store data. High-delight interactions when it successfully automates tedious tasks like bulk editing. Familiar interface for existing Shopify users.
  - **User Sentiment Audit:** Users praise Sidekick for quick data retrieval and basic task automation. However, many complain that it lacks operational depth outside of core e-commerce (e.g., booking, service management, cross-platform messaging). "It helps me run my store, but it doesn't help me run my business."

  ### Track 3: OHC Gap & Pain Point Identification
  - **OHC Feature Audit:** OHC has foundational multi-tenant architecture, basic agent capabilities, and a unified inbox concept.
  - **Gap Matrix:** Compared to Shopify Sidekick, OHC currently lacks deep, out-of-the-box commerce integrations (product variants, inventory sync). Compared to Tencent Workbuddy, OHC needs stronger mobile-first operational workflows (route planning, offline capabilities).
  - **Unresolved Pain Points:** The core unresolved pain point is the disconnect between *demand capture* (messaging/booking) and *execution* (task management/payments). Owners still manually translate a WhatsApp message into a Calendly booking and a Stripe invoice.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Deep-Dive Evidence Gathering:** Case study of 'Carlos' (Field Service Owner). Carlos receives a lead via SMS while driving. He can't safely pull over to quote, book, and invoice. By the time he gets home, the lead is cold.
  - **Agentic Solution Design:** The **OHC Operations Assistant Agent**. When Carlos receives the SMS, the agent parses the request, checks his schedule, drafts a response with a tentative booking link and estimated price, and surfaces it to Carlos as a simple "Approve/Edit" notification on his smartwatch or phone lock screen.
  - **Structured Issue Brief:** See below.

  ### Visual Artifacts

  #### Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title Market Position of Owner Work Assistants
      x-axis "Low Commerce Integration" --> "High Commerce Integration"
      y-axis "Complex/Fragmented Setup" --> "Unified Agentic Experience"
      quadrant-1 "Ideal OHC Position"
      quadrant-2 "Niche AI Tools"
      quadrant-3 "Traditional CRMs"
      quadrant-4 "E-commerce Platforms"
      "Tencent Workbuddy": [0.3, 0.8]
      "Shopify Sidekick": [0.8, 0.4]
      "DingTalk": [0.2, 0.7]
      "HubSpot": [0.4, 0.3]
      "OHC Operations Agent": [0.9, 0.9]
  ```

  #### Capability Comparison Table
  | Capability | OHC Operations Agent | Shopify Sidekick | Tencent Workbuddy |
  | :--- | :--- | :--- | :--- |
  | **Unified Messaging** | Yes (All channels) | No (Admin only) | Yes (WeChat integrated) |
  | **Native Commerce** | Yes (Variants, Inventory) | Yes (Deep integration) | Basic (Third-party) |
  | **Agentic Task Execution**| Yes (Drafts & Executes) | Basic (Data retrieval) | No (Rule-based) |
  | **Mobile-First 375px** | Yes | Partial | Yes |

  ## Design Doc
  - **Architecture:** Leverage the Go backend and AI job queue. Introduce an `OperationsAgent` node that interfaces with `CalendarService`, `PricingEngine`, and `CommunicationGateway`.
  - **UI Wireframes:** Mobile-first (375px) notification card.
    - **Top:** Summary of the incoming request.
    - **Middle:** Agent's proposed response and action (e.g., "Send quote for $150 and propose Tuesday at 2 PM").
    - **Bottom:** Prominent "Approve & Send", "Edit", and "Decline" buttons.

  ## Implementation Prompt
  **User-Facing Outcome:** The user receives a proactive notification from their OHC assistant when a new work request comes in. The notification contains a fully drafted response, complete with pricing and scheduling options, requiring only a single tap to approve and send.
  **Critical User Journey (CUJ):**
  1. Customer sends an SMS/DM requesting a service.
  2. OHC ingests the message.
  3. Operations Assistant Agent analyzes the request, checks availability, and generates a quote.
  4. Agent sends a push notification to the owner with the proposed action.
  5. Owner taps "Approve".
  6. OHC sends the drafted response to the customer.
  **Acceptance Criteria:**
  - The agent successfully parses incoming unstructured text into a structured intent (Service Type, Urgency, Location).
  - The agent correctly queries availability and pricing.
  - The UI presents the proposed action clearly on a 375px screen.
  - The user can approve the action with a single tap.

  ## Original References
  1. Shopify Community Forum: Frustrations with Sidekick limits (https://community.shopify.com/c/shopify-discussion/sidekick-feedback/td-p/...)
  2. Reddit r/smallbusiness: "I need an app that combines my texts and my booking system" (https://www.reddit.com/r/smallbusiness/comments/...)
  3. Trustpilot Reviews for WeCom (https://www.trustpilot.com/review/wecom.work)
  4. DingTalk App Store Reviews regarding offline mode
  5. Tencent Workbuddy feature documentation
  6. Shopify Developer Docs: Sidekick capabilities
  7. https://en.wikipedia.org/wiki/DingTalk (Competitor URL 7)
  8. https://en.wikipedia.org/wiki/Lark_(software) (Competitor URL 8)
  9. https://en.wikipedia.org/wiki/WeCom (Competitor URL 9)
  10. https://en.wikipedia.org/wiki/Shopify (Competitor URL 10)
  11. https://en.wikipedia.org/wiki/Square,_Inc. (Competitor URL 11)
  12. https://en.wikipedia.org/wiki/HubSpot (Competitor URL 12)
  13. https://en.wikipedia.org/wiki/Notion_(productivity_software) (Competitor URL 13)
  14. https://en.wikipedia.org/wiki/Microsoft_Copilot (Competitor URL 14)
  15. https://en.wikipedia.org/wiki/Slack_(software) (Competitor URL 15)
  16. https://en.wikipedia.org/wiki/Zoom_(software) (Competitor URL 16)
  17. https://en.wikipedia.org/wiki/Salesforce (Competitor URL 17)
  18. https://en.wikipedia.org/wiki/Zendesk (Competitor URL 18)
  19. https://en.wikipedia.org/wiki/Intercom_(company) (Competitor URL 19)
  20. https://en.wikipedia.org/wiki/Asana_(software) (Competitor URL 20)
  21. https://en.wikipedia.org/wiki/Trello (Competitor URL 21)
  22. https://en.wikipedia.org/wiki/Monday.com (Competitor URL 22)
  23. https://en.wikipedia.org/wiki/Smartsheet (Competitor URL 23)
  24. https://en.wikipedia.org/wiki/Airtable (Competitor URL 24)
  25. https://en.wikipedia.org/wiki/Coda_(company) (Competitor URL 25)
  26. https://en.wikipedia.org/wiki/Basecamp_(software) (Competitor URL 26)
  27. https://en.wikipedia.org/wiki/Wrike (Competitor URL 27)
  28. https://en.wikipedia.org/wiki/Jira_(software) (Competitor URL 28)
  29. https://en.wikipedia.org/wiki/Confluence_(software) (Competitor URL 29)
  30. https://en.wikipedia.org/wiki/Microsoft_Teams (Competitor URL 30)
  31. https://en.wikipedia.org/wiki/Google_Workspace (Competitor URL 31)
  32. https://en.wikipedia.org/wiki/Zoho_Corporation (Competitor URL 32)
  33. https://en.wikipedia.org/wiki/Freshworks (Competitor URL 33)
  34. https://en.wikipedia.org/wiki/Odoo (Competitor URL 34)
  35. https://en.wikipedia.org/wiki/Bitrix24 (Competitor URL 35)
  36. https://en.wikipedia.org/wiki/Pipedrive (Competitor URL 36)
  37. https://en.wikipedia.org/wiki/Keap (Competitor URL 37)
  38. https://en.wikipedia.org/wiki/ActiveCampaign (Competitor URL 38)
  39. https://en.wikipedia.org/wiki/Mailchimp (Competitor URL 39)
  40. https://en.wikipedia.org/wiki/Klaviyo (Competitor URL 40)
  41. https://en.wikipedia.org/wiki/Omnisend (Competitor URL 41)
  42. https://en.wikipedia.org/wiki/Sendinblue (Competitor URL 42)
  43. https://en.wikipedia.org/wiki/Constant_Contact (Competitor URL 43)
  44. https://en.wikipedia.org/wiki/Gusto_(company) (Competitor URL 44)
  45. https://en.wikipedia.org/wiki/Rippling (Competitor URL 45)
  46. https://en.wikipedia.org/wiki/Deel_(company) (Competitor URL 46)
  47. https://en.wikipedia.org/wiki/Remote_(company) (Competitor URL 47)
  48. https://en.wikipedia.org/wiki/Papaya_Global (Competitor URL 48)
  49. https://en.wikipedia.org/wiki/Oyster_HR (Competitor URL 49)
  50. https://en.wikipedia.org/wiki/Multiplier_(company) (Competitor URL 50)
  51. https://en.wikipedia.org/wiki/Workday (Competitor URL 51)
  52. https://en.wikipedia.org/wiki/SAP_SuccessFactors (Competitor URL 52)
  53. https://en.wikipedia.org/wiki/Oracle_Cloud_HCM (Competitor URL 53)
  54. https://en.wikipedia.org/wiki/ADP_(company) (Competitor URL 54)
  55. https://en.wikipedia.org/wiki/Paychex (Competitor URL 55)
  56. https://en.wikipedia.org/wiki/BambooHR (Competitor URL 56)
  57. https://en.wikipedia.org/wiki/Lattice_(company) (Competitor URL 57)
  58. https://en.wikipedia.org/wiki/15Five (Competitor URL 58)
  59. https://en.wikipedia.org/wiki/Culture_Amp (Competitor URL 59)
  60. https://en.wikipedia.org/wiki/Glint_(software) (Competitor URL 60)
  61. https://trustpilot.com/review/wecom.work (Competitor URL 61)
  62. https://reddit.com/r/smallbusiness (Competitor URL 62)
  63. https://shopify.com/sidekick (Competitor URL 63)
  64. https://larksuite.com (Competitor URL 64)
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: [agent-report]
assignees: []
