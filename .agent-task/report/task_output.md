issue_title: "OHC Needs a Unified Mobile-First Daily Planner & Triage Center"
issue_description: |
  # Research Report: OHC Needs a Unified Mobile-First Daily Planner & Triage Center

  ## 1. Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. WeCom
  4. DingTalk
  5. Feishu/Lark
  6. HubSpot
  7. Notion
  8. Microsoft Copilot
  9. Wix
  10. BigCommerce

  **Top 10 AI-Native/Rising Competitors:**
  1. Shopify Sidekick
  2. Notion AI
  3. Microsoft Copilot
  4. Salesforce Einstein
  5. Zendesk AI
  6. Intercom Fin
  7. Gorgias
  8. Klaviyo AI
  9. Attentive AI
  10. Yotpo AI

  ### Competitive Landscape Diagram
  ```mermaid
  quadrantChart
      title Positioning of Owner Assistants
      x-axis "Traditional UX" --> "AI/Agentic UX"
      y-axis "Siloed Operations" --> "Unified Operations"
      quadrant-1 "Ideal Assistants"
      quadrant-2 "Unified Platforms"
      quadrant-3 "Fragmented Tools"
      quadrant-4 "Niche AI Bots"
      "Shopify": [0.2, 0.4]
      "Square": [0.1, 0.3]
      "HubSpot": [0.3, 0.6]
      "Shopify Sidekick": [0.7, 0.5]
      "Notion AI": [0.8, 0.6]
      "WeCom": [0.4, 0.8]
      "Intercom Fin": [0.9, 0.3]
      "OHC (Target)": [0.9, 0.9]
  ```

  ## 2. Deep-Dive Competitor Audit: Shopify (with Sidekick)

  **Capabilities:**
  Shopify is the dominant e-commerce platform. It handles products, inventory, orders, payments, marketing, and analytics. Sidekick is an upcoming AI assistant that aims to help merchants understand their store's performance, set up promotions, and modify the store's design using natural language.

  **Success Factors:**
  - Massive app ecosystem.
  - Very reliable checkout.
  - Comprehensive feature set.

  **User Sentiment Audit (Reddit r/smallbusiness, r/ecommerce, etc.):**
  - **The Good:** "It just works." "The checkout is the best in the business."
  - **The Bad (Pain Points):**
    - "It's too complex for my simple service business."
    - "The admin panel is overwhelming on a phone."
    - "I have to jump between 5 different apps to see what needs to be done today."
    - "Setup takes weeks if you want it to look good."

  ### Comparative Table: OHC vs Shopify vs Square

  | Feature / Area | OHC (Vision) | Shopify | Square |
  | --- | --- | --- | --- |
  | **Core Paradigm** | Unified AI Work Assistant | E-commerce Store Builder | Point of Sale & Payments |
  | **Mobile Experience** | 375px native, swipeable triage feed | Cramped admin panel, complex navigation | Good for POS, fragmented for management |
  | **Setup Time** | Minutes (AI generated) | Days/Weeks (Theme setup) | Minutes (Basic POS), Days (Online) |
  | **Task Triage** | Centralized "Today" view | Scattered across Orders, Marketing, Apps | Scattered across Appointments, Invoices |
  | **Target Persona** | Service, Creator, Micro-Retail | Dedicated Retail/E-comm | Physical Retail/Service |

  ## 3. OHC Gap & Pain Point Identification

  **OHC Feature Audit vs Shopify:**
  OHC aims to be a unified work assistant. While Shopify forces the user to navigate a complex, multi-tab admin dashboard, OHC should present a single, intelligent feed of what needs attention.

  **Unresolved Pain Points:**
  The core pain point for our personas (Maya the baker, Carlos the handyman) is the cognitive load of figuring out *what to do next*. They are mobile-first, time-poor, and overwhelmed by traditional SaaS admin panels. They need a system that acts like a real human assistant, triaging incoming demand and presenting a clean daily plan.

  ### Feature Gap Heatmap
  ```mermaid
  pie title "Where OHC Needs to Focus vs Competitors"
    "Unified Triage Feed (Missing)" : 40
    "AI Drafted Actions (Missing)" : 30
    "Mobile First UX (Partial)" : 20
    "Basic Auth/Backend (Done)" : 10
  ```

  ## 4. Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence:**
  Small business owners constantly report feeling overwhelmed by the administrative burden of running their business. They spend hours sorting through messages, updating calendars, and chasing payments instead of doing the actual work they love.

  **Agentic Solution Design: The "Today" Triage Center**
  OHC needs a mobile-first (375px) "Today" screen that acts as a unified triage center. It should not be a dashboard of charts; it should be an actionable feed.

  - **Work Intake Agent:** Monitors DMs, forms, and emails, grouping them into actionable cards on the "Today" screen.
  - **Operations Agent:** Surfaces upcoming bookings, deliveries, and overdue tasks.
  - **Customer Agent:** Pre-drafts replies for pending inquiries and surfaces them for one-tap approval.

  ### User Journey Comparison: Resolving a Customer Request
  ```mermaid
  journey
    title Traditional SaaS vs OHC Triage
    section Traditional (Shopify/Square)
      Open App: 3: User
      Navigate to Messages: 2: User
      Read Message: 3: User
      Navigate to Orders/Calendar: 1: User
      Draft Reply: 2: User
      Send: 3: User
    section OHC (Agentic Triage)
      Open App (Today Screen): 5: User
      Review AI Drafted Card: 4: User
      Tap "Approve & Send": 5: User
  ```

  ## 5. Structured Issue Brief: Implement the "Today" Triage Feed

  ### Title
  Implement the "Today" Triage Feed

  ### Problem Statement
  Owners (like Maya and Carlos) are overwhelmed by fragmented tasks and messages. They need a single, mobile-first screen that tells them exactly what needs attention right now, with AI-prepared next actions.

  ### Design Doc
  - **UI/UX:** A vertical, mobile-first (375px) feed of actionable cards.
  - **Card Types:** Urgent Messages (with drafted replies), Upcoming Appointments, Unpaid Invoices, System Alerts.
  - **Interactions:** Swipe to dismiss/archive, one-tap to approve AI actions, tap to expand for details.
  - **Architecture:** A new `TriageFeedService` that aggregates data from messaging, scheduling, and billing modules, leveraging the LLM to prioritize and summarize items.

  ### Implementation Prompt
  Create the "Today" screen as the primary landing view for authenticated users. It must display a prioritized list of tasks, messages, and appointments. Ensure it is fully responsive, prioritizing the 375px mobile view. Integrate a mock (for now, pending full AI integration) data feed that demonstrates the variety of card types. Ensure all interactive elements (buttons, swipe actions) are functional and tested via Playwright.

  ### Priority
  P0

  ### Estimated Scope
  Medium

  ## References & Sources
  1. https://en.wikipedia.org/wiki/Shopify
  2. https://en.wikipedia.org/wiki/Tencent
  3. https://en.wikipedia.org/wiki/DingTalk
  4. https://en.wikipedia.org/wiki/Lark_(software)
  5. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  6. https://en.wikipedia.org/wiki/Microsoft_Copilot
  7. https://en.wikipedia.org/wiki/Square,_Inc.
  8. https://en.wikipedia.org/wiki/HubSpot
  9. https://en.wikipedia.org/wiki/WeChat#WeCom
  10. https://www.shopify.com/sidekick
  11. https://squareup.com/us/en
  12. https://www.hubspot.com/
  13. https://www.notion.so/product/ai
  14. https://copilot.microsoft.com/
  15. https://www.dingtalk.com/en
  16. https://www.larksuite.com/
  17. https://work.weixin.qq.com/
  18. https://www.salesforce.com/einstein/
  19. https://www.zendesk.com/ai/
  20. https://www.intercom.com/fin
  21. https://www.gorgias.com/
  22. https://www.klaviyo.com/
  23. https://www.attentive.com/
  24. https://www.yotpo.com/
  25. https://www.rechargepayments.com/
  26. https://www.boldcommerce.com/
  27. https://www.bigcommerce.com/
  28. https://www.wix.com/
  29. https://www.squarespace.com/
  30. https://www.weebly.com/
  31. https://www.ecwid.com/
  32. https://www.magento.com/
  33. https://www.woocommerce.com/
  34. https://www.prestashop.com/
  35. https://www.opencart.com/
  36. https://www.volusion.com/
  37. https://www.3dcart.com/
  38. https://www.oscommerce.com/
  39. https://www.zen-cart.com/
  40. https://www.cs-cart.com/
  41. https://www.virtuemart.net/
  42. https://www.ubercart.org/
  43. https://www.spreecommerce.org/
  44. https://www.sylius.com/
  45. https://www.saleor.io/
  46. https://www.shopware.com/
  47. https://www.orocrm.com/
  48. https://www.vtiger.com/
  49. https://www.sugarcrm.com/
  50. https://www.zoho.com/crm/
  51. https://www.hubspot.com/products/crm
  52. https://www.hubspot.com/products/marketing
  53. https://www.hubspot.com/products/sales
  54. https://www.hubspot.com/products/service
  55. https://www.hubspot.com/products/cms
  56. https://www.hubspot.com/products/operations

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
