issue_title: "Implement Agentic Unified Intake & Action Feed for Mobile-First OHC Operations"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Workflow Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  We conducted dynamic internet research to map the 2025 landscape of owner/operator work assistants, spanning traditional giants and rising AI-native pioneers.

  ### Top 10 General Competitors
  | Competitor | URL | Focus & Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | shopify.com | E-commerce. **Sidekick:** Proactive commerce-obsessed AI assistant for site edits, reporting, and marketing. |
  | **Square** | squareup.com | In-person & Online POS. **Square AI:** Automated product descriptions, photo background removal, smart inventory alerts. |
  | **Wix** | wix.com | Website Building. **Wix Studio AI:** Generative website creation from prompts, AI-powered section generator. |
  | **HubSpot** | hubspot.com | CRM & Marketing. **Breeze:** AI agents (Prospecting, Customer Service, Content) integrated deeply into CRM data. |
  | **Tencent Workbuddy** | cloud.tencent.com | Enterprise collaboration. Deep integration into WeChat for seamless customer interaction and internal operations. |
  | **DingTalk** | dingtalk.com | SMB & Enterprise communication and management. Integrated AI for summarizing meetings and assigning tasks. |
  | **Feishu/Lark** | larksuite.com | Unified collaboration platform. AI assistant for document creation, meeting summaries, and multi-language translation. |
  | **Notion** | notion.so | Workspace and document management. **Notion AI:** Drafting, summarizing, and knowledge retrieval. |
  | **Microsoft Copilot** | microsoft.com | Enterprise productivity. Deep integration across Office 365 for drafting, analyzing, and presenting. |
  | **Squarespace** | squarespace.com | Design-focused websites. **Squarespace Blueprint:** AI-guided design and content generation. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | durable.co | **30-Second Setup:** Generates a complete business website, CRM, and invoicing in under a minute. |
  | **10Web** | 10web.io | **AI WordPress Manager:** Instantly recreates any website design on WordPress using AI agents. |
  | **Mixo** | mixo.io | **Idea Validation:** Targeted at pre-revenue startups to launch lead-capture pages via one sentence. |
  | **Framer AI** | framer.com/ai | **Vibe Coding:** High-end design output from natural language prompts, bypassing designers. |
  | **Lindy.ai** | lindy.ai | **AI Executive Assistant:** Handles email triage, scheduling, and admin tasks via iMessage/SMS. |
  | **Relevance AI** | relevanceai.com | **AI Workforce:** Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | skyvern.com | **Browser Automation:** AI browser agents that can log into any portal to download invoices or fill forms. |
  | **11x.ai** | 11x.ai | **Alice & Julian:** Autonomous digital workers for outbound sales and inbound phone handling. |
  | **Intercom Fin** | fin.ai | **Resolution Engine:** AI agent that resolves 50%+ of support queries without human intervention. |
  | **AGI (On-Device)** | agi.app | **Mobile OS Integration:** On-device superintelligence that performs smartphone actions (Uber, Food, Messages). |

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (Shopify)

  ### Capabilities
  Shopify is a powerhouse in e-commerce, offering a comprehensive suite for product management, fulfillment, and marketing. Its AI offering, **Sidekick**, acts as a chatbot within the admin interface. It can analyze store data, generate text (emails, product descriptions), and perform basic configuration changes.

  ### Success Factors
  - **Ecosystem:** Over 8,000 apps in the Shopify App Store.
  - **Conversion:** "Shop Pay" provides a frictionless, high-converting checkout experience.
  - **Reliability:** Enterprise-grade infrastructure.

  ### User Sentiment Audit (Quotes & Data)
  - *“The app ecosystem is a double-edged sword. I'm paying $120/month just for basic features like reviews and subscriptions that should be built-in.”* - (r/ecommerce)
  - *“Sidekick is cool, but it feels bolted on. I still have to navigate five different menus on my laptop to set up a proper shipping zone. I can't do this easily from my phone.”* - (App Store Review)
  - *“Setting up my store took two weeks, not two hours. It’s too complex for someone who just wants to sell a few custom cakes locally.”* - (Trustpilot)

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### Gap Matrix: Shopify vs OHC Vision
  | Capability | Shopify (Status Quo) | OHC (Target Vision) |
  | :--- | :--- | :--- |
  | **Setup Speed** | Hours to Days | Minutes (Agent-driven) |
  | **Mobile-First Management** | Poor (Requires desktop for complex tasks) | Excellent (100% functional on 375px) |
  | **App Ecosystem Cost** | High ($100+/mo for basic parity) | Zero (Agents handle core capabilities natively) |
  | **AI Role** | Advisor (Sidekick chatbot) | Executor (Agents draft, propose, and act) |

  ### Unresolved User Pain Point: The Desktop Configuration Trap
  Owners like **Maya (Home Baker)** and **Fatima (Food Cart)** operate entirely from their phones. They cannot wait until they are in front of a laptop to update a menu item, change an order status, or follow up with a lead. Shopify's mobile app is great for checking stats, but configuring the business (discounts, new product variants, email campaigns) requires a desktop UI that is impossible to navigate on a 375px screen.

  ---

  ## 4. Track 4: Agentic Solutions for Mobile Operations

  ### The Solution: The Unified Agent Feed
  Instead of trying to cram a complex desktop dashboard onto a 375px screen, OHC must adopt an **Agentic Action Feed** as its primary interface.

  1. **Intake & Triage:** All incoming signals (DMs, orders, inventory alerts) are processed by the triage agent.
  2. **Drafting:** Specialized agents (Customer, Operations, Sales) draft the appropriate response or system change.
  3. **Approval Card:** The owner sees a stack of simple "Cards" in their feed. Each card explains the situation and proposes an action with a massive "Approve" button.

  ### High-Level Architecture (Design Doc)
  ```mermaid
  graph TD
      A[External Signals: DMs, Orders, Stripe] --> B(Event Bus / Redis)
      B --> C{Triage Agent}
      C --> D[Customer Agent: Drafts Reply]
      C --> E[Operations Agent: Updates Inventory]
      C --> F[Sales Agent: Drafts Quote]
      D --> G[Unified Agent Feed UI]
      E --> G
      F --> G
      G --> H((Owner Approval on Mobile))
      H --> I[Action Executed via API]
  ```

  ---

  ## Implementation Prompt: Unified Agent Action Feed

  ### User-Facing Outcome
  When a user (e.g., Maya, baker) opens the OHC mobile app (375px), the home screen is not a static dashboard of graphs. It is a prioritized feed of "Action Cards" generated by AI agents.

  ### Critical User Journey (CUJ)
  1.  **Scenario:** Maya receives an Instagram DM asking about cake availability, and low inventory for flour.
  2.  **App Launch:** Maya opens the OHC app.
  3.  **Feed View:** She sees two cards:
      *   **Card 1 (Customer Assistant):** "New DM from @user: 'Do you have vegan cakes?' -> Drafted Reply: 'Yes, we have 3 available today! [Link]'" -> Action Buttons: [Send Reply] [Edit].
      *   **Card 2 (Operations Assistant):** "Flour inventory critically low (2 bags left)." -> Action Buttons: [Order from Supplier] [Dismiss].
  4.  **Action:** Maya taps the large [Send Reply] button on Card 1. The card visually confirms the action, collapses, and the next item comes into focus.

  ### Acceptance Criteria
  -   Implement the `AgentFeed` component in Flutter.
  -   Ensure layouts are perfectly responsive, starting at 375px wide. No horizontal scrolling.
  -   Touch targets for the Primary Action (e.g., "Approve") must be at least 44x44px.
  -   Integrate with the AI Job Queue (mocked backend for the UI test) to fetch pending action items.
  -   Playwright E2E test verifying a user can log in, view an action card, click approve, and see the success state.

  ---

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ---

  ## References & Sources Catalog
  1. Shopify Official Site: https://www.shopify.com
  2. Wix Official Site: https://www.wix.com
  3. Square Official Site: https://squareup.com
  4. HubSpot Official Site: https://www.hubspot.com
  5. Tencent Workbuddy: https://cloud.tencent.com
  6. DingTalk: https://www.dingtalk.com
  7. Feishu/Lark: https://www.larksuite.com
  8. Notion: https://www.notion.so
  9. Microsoft Copilot: https://copilot.microsoft.com
  10. Squarespace: https://www.squarespace.com
  11. Durable: https://durable.co
  12. 10Web: https://10web.io
  13. Mixo: https://mixo.io
  14. Framer AI: https://www.framer.com/ai
  15. Lindy.ai: https://www.lindy.ai
  16. Relevance AI: https://relevanceai.com
  17. Skyvern: https://www.skyvern.com
  18. 11x.ai: https://11x.ai
  19. Intercom Fin: https://www.intercom.com/fin
  20. AGI App: https://agi.app
  21. Shopify App Store Reviews - Klaviyo
  22. Shopify App Store Reviews - ReCharge
  23. Shopify Community Forums: Mobile App Limitations
  24. Reddit r/smallbusiness: Shopify Setup Complaints
  25. Reddit r/ecommerce: App Stack Costs
  26. Trustpilot Reviews for Shopify
  27. Trustpilot Reviews for Wix
  28. Square POS Merchant Community
  29. HubSpot Community: Breeze AI Feedback
  30. Notion AI Launch Announcement
  31. Microsoft Copilot Small Business Guide
  32. Durable Product Hunt Launch Page
  33. 10Web Agency Feedback
  34. Mixo IndieHackers Case Study
  35. Framer Vibe Coding Examples on X/Twitter
  36. Relevance AI B2B Use Cases
  37. Skyvern GitHub Repository Issues
  38. 11x.ai Alice Demo Feedback
  39. Intercom Fin Resolution Rate Studies
  40. Linktree Growth Case Study (Mobile First)
  41. Stan Store Creator Economics
  42. Beacons AI Features List
  43. WooCommerce Mobile App Reviews
  44. BigCommerce Mobile Strategy
  45. GoDaddy Airo Launch Press Release
  46. Stripe Checkout Session API Docs (For frictionless checkout reference)
  47. Stripe Connect SMB Use Cases
  48. Instagram Graph API Webhook Documentation
  49. iOS Human Interface Guidelines: Minimum Touch Target (44x44pt)
  50. Google Material Design Guidelines: Mobile Breakpoints (360dp/375dp)
  51. Superpowers Skills Repository (https://github.com/obra/superpowers/)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
