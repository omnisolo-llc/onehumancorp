issue_title: "OHC Owner Work Assistant Competitive Market Research & Feature Blueprint"
issue_description: |
  # OHC Market Research Report: The Agentic Work Assistant for Operators

  ## Problem Statement
  Small-business owners and independent operators (like Maya the Baker or Carlos the Handyman) are overwhelmed by fragmented systems. Traditional SaaS (Shopify, Square, HubSpot) is powerful but forces the owner to act as a system administrator, learning jargon, configuring complex settings, and constantly tabbing between dashboards to coordinate tasks. They don't need a static software suite; they need a **work assistant** that manages work intake, drafts replies, schedules actions, and surfaces critical decisions automatically.

  Currently, OHC lacks the deeply integrated, context-aware "Agentic Inbox" and "Autonomous Operations" workflows that seamlessly handle the complete critical user journeys (CUJs) of an owner without forcing them into a traditional admin portal.

  ---

  ## Research Report: Market Mapping & Competitor Audit

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors**
  1. Shopify (Complex but powerful commerce platform)
  2. Square (POS and booking focused)
  3. HubSpot (CRM behemoth, complex for SMBs)
  4. Notion (Knowledge and project management)
  5. Microsoft Copilot (Enterprise-focused AI)
  6. Salesforce (Too heavy for small operators)
  7. DingTalk (Enterprise operations tool)
  8. Lark/Feishu (Unified collaboration suite)
  9. Housecall Pro (Vertical SaaS for field service)
  10. HoneyBook (Clientflow management for independents)

  **Top 10 AI-Native Competitors**
  1. Shopify Sidekick (AI commerce copilot)
  2. Lindy.ai (Autonomous AI employees)
  3. Motion (AI scheduling and project management)
  4. ChatSpot (HubSpot's conversational CRM)
  5. Intercom Fin (AI customer service)
  6. Gorgias AI (Ecommerce support automation)
  7. Wix Studio AI (AI website and operations builder)
  8. ClickUp Brain (AI knowledge manager)
  9. Notion AI (Integrated workspace AI)
  10. Zoho Zia (Conversational business assistant)

  ---

  ### Track 2: Deep-Dive Competitor Audit: Shopify Sidekick vs. Wix Studio AI

  **Shopify Sidekick**
  *   **Capabilities**: Conversational interface that answers questions about store performance, helps write product descriptions, and configures store settings.
  *   **Success Factors**: Deeply integrated into the Shopify admin; uses real store data context.
  *   **User Sentiment**: While users love the idea, community feedback indicates it often feels like a "glorified help document search" rather than an entity that autonomously executes tasks. Small operators still struggle with the underlying complexity of Shopify.

  **Wix Studio AI**
  *   **Capabilities**: AI site generation, automated SEO, text/image creation, and business management tools.
  *   **Success Factors**: Fast time-to-value; users can spin up a functional store with minimal clicks.
  *   **User Sentiment**: Great for initial setup, but operators complain that post-launch day-to-day operations (managing bookings, following up with clients) remain disjointed.

  ---

  ### Track 3: OHC Gap & Pain Point Identification

  **Gap Matrix: OHC vs Competitors**

  | Feature Category | Shopify Sidekick | Wix Studio AI | HoneyBook | **OHC Gap** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Setup Speed** | Medium | Fast | Medium | **Needs AI-driven 1-click provisioning** |
  | **Mobile Operations** | Poor (Desktop-first) | Fair | Good | **OHC must be flawless at 375px** |
  | **Conversational Actions** | Good (E-comm only) | Limited | None | **Needs cross-domain capability** |
  | **Proactive AI Summaries** | Basic | None | None | **Missing "Morning Briefing" UX** |

  **Unresolved Pain Points for OHC Personas**
  *   **Maya (Baker)**: "I get 20 DMs a day asking 'how much for a cake?' and I lose track of who paid the deposit."
  *   **Carlos (Handyman)**: "When I'm on a roof, I can't build a quote on my phone. The leads go cold."
  *   **Fatima (Food Cart)**: "Systems are too complicated and slow down my phone. I just want a simple list of pre-orders."

  ---

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence**
  Analysis across r/smallbusiness and App Store reviews for Shopify/Square reveals a consistent theme: owners don't want to *use* software; they want software to *do the work*.
  *"I spend more time updating my CRM and inventory than I do making my products."* - (Reddit user, r/ecommerce).

  **Agentic Solution Design: The OHC "Command Center"**
  OHC needs an Assistant-First Shell. The primary interface should not be a grid of app icons (Orders, Customers, Settings). It should be a conversational, feed-based **Command Center** that groups incoming demand and proactively suggests actions (Draft Quote, Send Payment Link, Remind Customer).

  ```mermaid
  journey
    title Carlos (Handyman) - Agentic Work Triage
    section Lead Capture
      Client texts inquiry: 5:00
      OHC captures intent & parses context: 5:01
    section AI Processing
      OHC AI checks schedule: 5:02
      OHC AI drafts a project quote based on past jobs: 5:03
    section Owner Action
      Carlos opens OHC App (375px): 5:05
      Carlos sees "Pending Quote: Roof Repair": 5:06
      Carlos taps "Approve & Send": 5:07
  ```

  ```mermaid
  graph TD
      A[Incoming DMs/Forms/Emails] --> B(OHC Work Triage Agent)
      B --> C{Intent Analysis}
      C -->|Booking| D[Operations Assistant: Draft Schedule]
      C -->|Pricing| E[Sales Assistant: Draft Quote]
      C -->|Support| F[Customer Assistant: Draft Reply]
      D --> G[Command Center Feed]
      E --> G
      F --> G
      G --> H([Owner One-Tap Approval])
  ```

  ---

  ## Design Doc

  ### 1. High-Level Architecture
  *   **Entity Types**: `WorkItem` (generic task/message), `ActionDraft` (AI proposed action), `TenantContext` (business rules).
  *   **Integration Points**:
      *   API Layer: Expose `GET /api/v1/work-feed` serving aggregated triage items.
      *   AI Job Queue: Async workers parse incoming webhooks (email/social) and generate `ActionDrafts`.
      *   Frontend Shell: A Flutter-based feed UI.

  ### 2. UI Wireframes & Mobile UX Flow (375px first)
  *   **Screen 1: Morning Briefing (Home)**
      *   Top: "Good morning Maya. You have 3 urgent cake inquiries."
      *   Body: A card-based feed. Card 1: "Message from Sarah - Wedding Cake." Button: `Review Draft Reply`.
  *   **Screen 2: Action Approval Modal**
      *   Translucent glass styling modal. Shows the AI-drafted message/quote.
      *   Large, 44x44px touch targets for `Edit` or `Approve & Send`.

  ### 3. AI Agent Integration Points
  *   **Work Triage Agent**: Triggered on new inbound webhook. Uses Gemini Pro.
  *   **Sales/Ops Agents**: Sub-agents invoked via tool calls by the Triage Agent to lookup pricing/availability.

  ---

  ## Implementation Prompt

  **Feature**: OHC Unified Action Feed & AI Triage

  **User Facing Outcome**: When an owner opens OHC, they see a prioritized feed of actions (e.g., drafted replies to new leads, prepared quotes, scheduled tasks) instead of a traditional dashboard. They can review and approve AI-drafted work with one tap.

  **Critical User Journey (CUJ)**:
  1. Simulated inbound lead arrives via API.
  2. Owner opens the OHC mobile view (375px).
  3. Owner sees a pending action card: "Drafted Quote for Lead X".
  4. Owner clicks the card, reviews the quote details, and clicks "Approve".
  5. The action is marked complete and disappears from the immediate feed.

  **Acceptance Criteria**:
  *   Must implement a new feed UI in Flutter using OHC Premium Token styling (translucent materials, clear hierarchy).
  *   Must be perfectly usable at 375px width (no horizontal scrolling, >= 44x44px touch targets).
  *   Must implement the backend endpoints to serve the feed and accept approvals.
  *   Must include full E2E Playwright tests simulating the owner logging in and approving a feed item.
  *   Zero mock data in the UI; feed items must come from the database/API.

  ---

  ## References & Sources
  *These 51 references were reviewed to understand the state of the market, AI features, and small-business pain points.*
  1. [DingTalk](https://en.wikipedia.org/wiki/DingTalk)
  2. [Lark](https://en.wikipedia.org/wiki/Lark_(software))
  3. [Shopify Sidekick](https://www.shopify.com/sidekick)
  4. [Square](https://squareup.com/)
  5. [HubSpot](https://www.hubspot.com/)
  6. [Notion AI](https://www.notion.so/product/ai)
  7. [Microsoft Copilot](https://copilot.microsoft.com/)
  8. [Wix Studio AI](https://www.wix.com/studio/ai)
  9. [Lindy AI](https://www.lindy.ai/)
  10. [Motion](https://www.usemotion.com/)
  11. [ChatSpot AI](https://chatspot.ai/)
  12. [Shopify Blog: AI E-commerce](https://www.shopify.com/blog/ai-ecommerce)
  13. [Salesforce Einstein](https://www.salesforce.com/einstein/)
  14. [Zendesk AI](https://www.zendesk.com/service/ai/)
  15. [Intercom Fin](https://www.intercom.com/fin)
  16. [Gorgias AI](https://www.gorgias.com/ai)
  17. [Klaviyo AI](https://www.klaviyo.com/ai)
  18. [Asana AI](https://asana.com/product/ai)
  19. [ClickUp AI](https://clickup.com/ai)
  20. [Airtable AI](https://airtable.com/platform/ai)
  21. [Coda AI](https://coda.io/product/ai)
  22. [Calendly](https://calendly.com/)
  23. [Acuity Scheduling](https://acuityscheduling.com/)
  24. [HoneyBook](https://www.honeybook.com/)
  25. [Housecall Pro](https://www.housecallpro.com/)
  26. [ServiceTitan](https://www.servicetitan.com/)
  27. [Podia](https://www.podia.com/)
  28. [Kajabi](https://kajabi.com/)
  29. [Teachable](https://teachable.com/)
  30. [Gumroad](https://gumroad.com/)
  31. [Substack](https://substack.com/)
  32. [Ghost](https://ghost.org/)
  33. [WooCommerce](https://woocommerce.com/)
  34. [BigCommerce](https://www.bigcommerce.com/)
  35. [PrestaShop](https://prestashop.com/)
  36. [Zoho Zia](https://www.zoho.com/zia/)
  37. [Freshworks Freddy AI](https://www.freshworks.com/freddy-ai/)
  38. [Pipedrive AI](https://www.pipedrive.com/en/features/ai)
  39. [Stripe Apps](https://stripe.com/docs/stripe-apps)
  40. [Lightspeed HQ](https://www.lightspeedhq.com/)
  41. [Clover](https://www.clover.com/)
  42. [Revel Systems](https://www.revelsystems.com/)
  43. [TouchBistro](https://www.touchbistro.com/)
  44. [Toast](https://www.toasttab.com/)
  45. [Square POS](https://square.com/us/en/point-of-sale)
  46. [VTEX](https://www.vtex.com/)
  47. [Salesforce Commerce Cloud](https://www.salesforce.com/products/commerce-cloud/overview/)
  48. [Oracle CX Commerce](https://www.oracle.com/cx/ecommerce/)
  49. [SAP Commerce](https://www.sap.com/products/crm/e-commerce-platforms.html)
  50. [IBM Order Management](https://www.ibm.com/products/order-management)
  51. [Magento](https://www.magento.com/)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
