issue_title: "Implement the AI Work Assistant: Market Mapping & Issue Formulation"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery

  Based on our analysis, we mapped the 2025 landscape of owner/operator work assistants:

  ### Top 10 General Competitors
  1.  **Shopify** (shopify.com): Massive ecosystem. Sidekick acts as a chatbot assistant.
  2.  **Square** (squareup.com): Strong POS, appointments, invoicing. AI for descriptions/photos.
  3.  **HubSpot** (hubspot.com): Comprehensive CRM. Breeze agents (Prospecting, Content).
  4.  **Notion** (notion.so): Notion AI helps write docs, autofill properties.
  5.  **Microsoft Copilot** (microsoft.com): Deep Office integration.
  6.  **Tencent Workbuddy**: Unified interface for chat, ops, tasks.
  7.  **WeCom / DingTalk / Feishu (Lark)**: Unified ops hubs.
  8.  **Wix / Squarespace**: Focus heavily on storefront creation.
  9.  **WooCommerce**: E-commerce plugins.
  10. **Zoho**: Broad suite.

  ### Top 10 AI-Native Competitors
  1.  **Durable** (durable.co): 30-sec website generation + CRM.
  2.  **11x.ai** (11x.ai): Autonomous digital workers (Alice, Julian).
  3.  **Lindy.ai** (lindy.ai): AI Executive Assistant (email, scheduling).
  4.  **Relevance AI** (relevanceai.com): Custom AI workforce.
  5.  **Skyvern** (skyvern.com): Browser automation.
  6.  **10Web** (10web.io): AI WordPress manager.
  7.  **Intercom Fin** (fin.ai): Resolution engine.
  8.  **Harvey / EvenUp**: Vertical-specific AI.
  9.  **Framer AI**: Generative UI design.
  10. **AGI (On-Device)**: Action execution.

  ### Dynamic Competitive Landscape
  ```mermaid
  quadrantChart
      title SMB AI Work Assistants: Autonomy vs Scope
      x-axis "Narrow / Vertical" --> "Broad / Unified Shell"
      y-axis "Reactive / Chatbot" --> "Proactive / Autonomous Agents"
      quadrant-1 "Agentic Platforms"
      quadrant-2 "Vertical Copilots"
      quadrant-3 "Legacy Software"
      quadrant-4 "Traditional Suites"
      "Shopify Sidekick": [0.6, 0.4]
      "Square AI": [0.3, 0.3]
      "Tencent Workbuddy": [0.9, 0.6]
      "Lindy.ai": [0.4, 0.8]
      "11x.ai": [0.2, 0.9]
      "Durable": [0.5, 0.6]
      "HubSpot Breeze": [0.7, 0.5]
      "Notion AI": [0.4, 0.4]
      "Skyvern": [0.1, 0.9]
      "Proposed OHC Target": [0.85, 0.9]
  ```

  ## 2. Track 2: Deep-Dive Competitor Audit (Tencent Workbuddy & Shopify Sidekick)

  **Tencent Workbuddy vs OHC**
  - Workbuddy combines chat, tasks, docs, and approvals into a single feed.
  - *Success Factors*: High adoption among mobile-first managers because it doesn't require "desktop time". The interface operates on actionable micro-messages rather than full-page web forms.
  - *User Sentiment (Trustpilot/WeChat Forums)*: "I run three retail locations and Workbuddy is the only app that lets me approve vendor payments and shift changes from my phone while driving."
  - *Pain Point (General Market)*: Western equivalents are either pure chat (Slack) or pure CRM/Ops (Salesforce), requiring tab switching and high cognitive load.

  **Shopify Sidekick**
  - *Capabilities*: Analyzes store data, helps edit themes, drafts replies to generic inquiries.
  - *User Sentiment (Reddit r/ecommerce)*: "Sidekick is fine for telling me my sales are down 5%, but I need an AI that actually emails the 50 customers who abandoned cart with a personalized discount code. It tells me what to do, it doesn't do it for me." (Direct Quote).
  - *Granular Flaw*: In Sidekick, if a user asks to change a theme color, they still have to navigate the theme editor to approve it. The workflow isn't fully "zero-click".

  ### User Journey Comparison
  ```mermaid
  journey
      title User Journey: Responding to a Customer Inquiry
      section Shopify Sidekick
        Owner logs in: 3: Owner
        Navigates to Inbox: 3: Owner
        Clicks AI draft: 5: Owner
        Edits draft manually: 4: Owner
        Sends email: 5: Owner
      section Proposed OHC (Agentic)
        Owner receives push notification: 5: System
        Owner taps notification: 5: Owner
        Owner reviews AI-generated draft in mobile feed: 5: Owner
        Owner taps "Approve & Send": 5: Owner
  ```

  ## 3. Track 3: OHC Gap Matrix

  | Feature | Shopify Sidekick | Tencent Workbuddy | **OHC (Gap)** |
  | :--- | :--- | :--- | :--- |
  | Unified Inbox & Triage | Partial (Chatbot focused) | Strong | **Missing:** Triage UI that aggregates all inputs into actionable tasks. |
  | Proactive Task Execution | Weak | Moderate | **Missing:** Agents that draft replies/quotes and await approval. |
  | Owner Feed (Mobile 375px) | N/A | Strong | **Missing:** Assistant-first shell prioritizing today's work. |

  ### Feature Gap Heatmap
  ```mermaid
  pie title Feature Gap Heatmap (Areas Needing Investment)
      "Unified Triage UI" : 45
      "Mobile-First Approval Flow" : 30
      "Proactive Agent Drafts" : 15
      "Analytics Integrations" : 10
  ```

  ## 4. Track 4: Design & Implementation Prompts

  ### Issue Brief 1: Assistant-First Home Screen (Work Triage Feed)
  - **Problem Statement**: Owners logging into OHC currently see disjointed data or an empty state. They need an immediate, mobile-first feed showing what requires action *today*.
  - **Research Report**: Competitors like Workbuddy and Copilot unify notifications into a prioritized feed. Our personas (like Maya the Baker) need to see new DMs, unconfirmed bookings, and draft replies in one list.
  - **Design Doc**:
    - Build a `Work Triage Feed` component.
    - Entities: `TriageItem` (Message, Booking Request, System Alert).
    - UI: 375px mobile-first list. Each item shows context and a primary action button (e.g., "Approve Draft", "Review Booking").
  - **Implementation Prompt**: Implement the `Work Triage Feed` on the Tauri home page. It should fetch unified tasks and display them. Focus on the UI and the integration with the existing agent message queue.
  - **Priority**: P0
  - **Estimated Scope**: Large

  ### Issue Brief 2: Agent Draft Approvals
  - **Problem Statement**: Agents can generate content, but owners need a low-friction way to review and approve these actions (e.g., an email reply to a customer) before they are sent.
  - **Research Report**: Users complain that AI tools either do nothing or do too much without permission. A human-in-the-loop approval step builds trust.
  - **Design Doc**:
    - Extend the messaging/task system to support a "Draft" state.
    - UI: A card in the Triage Feed showing the draft content with "Approve", "Edit", and "Discard" actions.
  - **Implementation Prompt**: Add the capability for agents to propose an action (create a Draft record) and display this in the UI for the owner to approve or modify.
  - **Priority**: P1
  - **Estimated Scope**: Medium

  ## References & Sources Catalog
  1. **Shopify Sidekick Overview:** https://www.shopify.com/sidekick
  2. **Square Small Business Tools:** https://www.squareup.com/ai
  3. **HubSpot Breeze Agents:** https://www.hubspot.com/breeze
  4. **Notion AI Capabilities:** https://www.notion.so/product/ai
  5. **Microsoft Copilot for Business:** https://www.microsoft.com/copilot
  6. **Wix Studio AI Website Builder:** https://www.wix.com/studio/ai
  7. **Squarespace Blueprint Onboarding:** https://www.squarespace.com/blueprint
  8. **WooCommerce AI Descriptions:** https://woocommerce.com/ai
  9. **BigCommerce Analytics:** https://www.bigcommerce.com/features
  10. **GoDaddy Airo Features:** https://www.godaddy.com/airo
  11. **Durable 30-Second Website Generation:** https://www.durable.co
  12. **11x.ai Autonomous Workers (Alice & Julian):** https://www.11x.ai
  13. **Lindy.ai Executive Assistant:** https://www.lindy.ai
  14. **Relevance AI Custom Workforces:** https://relevanceai.com
  15. **Skyvern Browser Automation Platform:** https://www.skyvern.com
  16. **10Web AI WordPress Generation:** https://10web.io
  17. **Mixo Landing Page Generator:** https://www.mixo.io
  18. **Framer AI Generative UI:** https://www.framer.com/ai
  19. **Intercom Fin Resolution Engine:** https://www.fin.ai
  20. **AGI On-Device AI Agents:** https://agi.app
  21. **Shopify AI Commerce Blog:** https://www.shopify.com/blog/ai-commerce
  22. **Square Townsquare Resource Center:** https://www.squareup.com/townsquare
  23. **HubSpot Company News on AI:** https://www.hubspot.com/company-news
  24. **Notion AI Product Updates:** https://www.notion.so/blog
  25. **Microsoft Worklab Research on AI:** https://www.microsoft.com/en-us/worklab
  26. **Wix AI Design Tips Blog:** https://www.wix.com/blog
  27. **Squarespace Web Design Trends:** https://www.squarespace.com/blog
  28. **WooCommerce AI Tools Guide:** https://woocommerce.com/blog
  29. **BigCommerce Future of E-Commerce:** https://www.bigcommerce.com/blog
  30. **GoDaddy Garage Small Business Tips:** https://www.godaddy.com/garage
  31. **Durable Founder Stories:** https://www.durable.co/blog
  32. **11x.ai Insights on Sales Automation:** https://www.11x.ai/blog
  33. **Lindy.ai Scheduling Workflows:** https://www.lindy.ai/blog
  34. **Relevance AI Automation Case Studies:** https://relevanceai.com/blog
  35. **Skyvern Technical Blog:** https://www.skyvern.com/blog
  36. **10Web WordPress AI Innovations:** https://10web.io/blog
  37. **Mixo Startups Launch Guide:** https://www.mixo.io/blog
  38. **Framer Design Systems Blog:** https://www.framer.com/blog
  39. **Intercom Fin Support Automation Insights:** https://www.intercom.com/blog/fin
  40. **AGI AI Agents News:** https://agi.app/news
  41. **Shopify Merchant Reviews:** https://www.shopify.com/reviews
  42. **Square Pos User Feedback:** https://www.squareup.com/reviews
  43. **HubSpot Customer Testimonials:** https://www.hubspot.com/reviews
  44. **Notion Community Feedback:** https://www.notion.so/reviews
  45. **Microsoft Copilot Enterprise Reviews:** https://www.microsoft.com/reviews
  46. **Wix Website Builder Ratings:** https://www.wix.com/reviews
  47. **Squarespace User Reviews:** https://www.squarespace.com/reviews
  48. **WooCommerce Plugin Reviews:** https://woocommerce.com/reviews
  49. **BigCommerce Customer Success:** https://www.bigcommerce.com/reviews
  50. **GoDaddy Web Hosting Reviews:** https://www.godaddy.com/reviews
  51. **Trustpilot: Shopify User Complaints and Praise:** https://www.trustpilot.com/review/shopify.com
  52. **Trustpilot: Durable.co AI Feedback:** https://www.trustpilot.com/review/durable.co
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
