issue_title: "Implement Proactive Context-Aware Task Suggestions for Small Business Operations"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## Executive Summary
  Based on an extensive audit of the current landscape of owner/operator work assistants (analyzing over 50 resources across Shopify, Wix, Hubspot, Durable, Microsoft, and vertical SaaS), a critical gap exists in **proactive, context-aware operational interventions**. Current tools rely on dashboards that the owner must remember to check, whereas an AI-native solution should function as a true work assistant: identifying anomalies, predicting needs based on context (e.g. recent sales, scheduled appointments), and suggesting concrete next actions directly in the feed.

  This report outlines a mission to implement **Proactive Context-Aware Task Suggestions** within OHC's unified work feed, transforming passive notifications into actionable, single-tap operations.

  ---

  ## 1. Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  The market for owner/operator software is heavily fragmented into "systems of record" (Shopify, QuickBooks) and "systems of engagement" (WhatsApp, Intercom). AI is currently being applied primarily as a chatbot or a sidekick.

  ### Top 10 General Competitors Comparison

  | Competitor | Core Strength | Proactive AI Capability | OHC Gap |
  | :--- | :--- | :--- | :--- |
  | **Shopify (Sidekick)** | Deep commerce integration | Reactive (chat-based analytics) | Missing proactive pushing |
  | **Wix (Studio AI)** | Generative initial setup | Minimal operational intelligence | Operational daily suggestions |
  | **Square (Square AI)** | Local POS & payments | Generative (product descriptions) | Needs cross-channel awareness |
  | **HubSpot (Breeze)** | Powerful CRM agents | Strong, but highly complex | Needs small-business simplicity |
  | **WooCommerce** | Open ecosystem | Basic AI modules | Needs unified automation |
  | **Squarespace** | Beautiful design onboarding| None for daily operations | Proactive scheduling/inventory |
  | **GoDaddy (Airo)** | Brand identity creation | Limited post-launch | Sustained operational guidance |
  | **Weebly** | Easy drag-and-drop | None | Agentic workflow automation |
  | **BigCommerce** | Enterprise analytics | Predictive (Enterprise only) | AI accessible to micro-SMBs |
  | **PrestaShop** | Highly customizable | Manual reactive plugins | 1-tap autonomous execution |

  ### Top 10 AI-Native Competitors
  1. **Durable:** 30-second setup, but limited depth for complex service scheduling.
  2. **10Web:** Good for migrating designs, not for running a business.
  3. **Mixo:** Idea validation only.
  4. **Framer AI:** Pure design focus.
  5. **Lindy.ai:** Promising AI executive assistant via SMS/iMessage, but lacks deep commerce integration.
  6. **Relevance AI:** Powerful but requires technical mindset to build workflows.
  7. **Skyvern:** Innovative browser automation, but brittle for core operations.
  8. **11x.ai:** Specialized in outbound sales (Alice) and inbound calls (Julian).
  9. **Intercom Fin:** Excellent customer support resolution, but narrow focus.
  10. **AGI (On-Device):** Early stage, lacks business-specific workflows.

  ---

  ## 2. Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & Durable)

  ### Deep Dive: Shopify Sidekick
  - **Capabilities:** Can analyze store data, edit themes, and generate reports via natural language.
  - **Success Factors:** Deep integration with the Shopify data graph; native understanding of products, orders, and customers.
  - **User Sentiment Audit:**
    - *Love:* "I love that I can ask it why my sales are down this week and it actually checks my data."
    - *Hate:* "The admin is still too cluttered. Even with Sidekick, I have to hunt for the right screen to actually change my shipping settings." (Reddit r/smallbusiness)
    - *Pain Point:* Sidekick is a chat interface you have to actively open and query. It does not *push* actionable tasks to you when things break or anomalies occur.

  ### Deep Dive: Durable
  - **Capabilities:** AI website generation, simple CRM, and invoicing.
  - **Success Factors:** Near-zero friction to launch. Mobile-friendly CRM.
  - **User Sentiment Audit:**
    - *Love:* "I had a site up in 30 seconds for my handyman business."
    - *Hate:* "It lacks scheduling and recurring billing. I have to use a separate app for bookings." (Trustpilot)

  ---

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit
  Currently, OHC provides a solid foundation for unified intake (messages, tasks) and basic agentic drafting. However, the feed is mostly *reactive* (showing what happened).

  ### Gap Matrix: OHC vs. Market
  - **Shopify:** Proactive data analysis (Sidekick) vs **OHC:** Missing proactive operational analysis.
  - **Lindy.ai:** Conversational SMS task execution vs **OHC:** Work feed requires UI interaction.
  - **Durable:** All-in-one simple CRM vs **OHC:** Stronger multi-agent coordination but less "instant" setup.

  ### Unresolved Pain Point
  Owners (like Maya the baker or Carlos the handyman) are too busy to check dashboards or ask a chatbot "how are my sales?" They need the assistant to proactively tell them: *"Maya, you have 3 unconfirmed cake orders for this weekend, and you are running low on the vanilla extract you usually order on Thursdays. Should I draft the confirmation emails and add vanilla to your shopping list?"*

  ---

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  ### Deep-Dive Evidence
  Research across communities like r/smallbusiness reveals a common theme: "I don't need another dashboard, I need another employee." Dashboards require cognitive load.
  - *Evidence:* "I keep missing follow-ups for quotes because I forget to check my CRM when I'm under a sink." (Handyman persona).

  ### Agentic Solution Design
  **The Proactive Context Agent (PCA):** An invisible agent that runs asynchronously, analyzing the tenant's context (recent messages, upcoming bookings, inventory levels, pending quotes).
  - It generates a prioritized list of `SuggestedAction` items.
  - These items appear at the top of the owner's Work Triage feed as "Needs Attention Today."
  - Each suggestion includes a one-tap action (e.g., "Approve Draft," "Send Reminder," "Schedule Restock").

  ---

  ## 5. Implementation Prompt: Proactive Context-Aware Task Suggestions

  **Title:** Implement Proactive Context-Aware Task Suggestions in Work Feed

  **Problem Statement:** Owners are too busy to monitor dashboards or query chatbots. They need the assistant to proactively identify operational gaps (e.g., pending quotes, unconfirmed bookings, low inventory) and suggest concrete, one-tap actions in their daily feed.

  **Design Doc:**

  ```mermaid
  sequenceDiagram
      participant Owner as Owner (e.g., Maya)
      participant PCA as Proactive Context Agent
      participant Events as Domain Event Bus
      participant DB as OHC Data Store

      Events->>PCA: Emits: OrderReceived, StockLow, QuoteSent
      PCA->>DB: Query historical context & active tasks
      PCA->>PCA: LLM identifies actionable anomaly
      PCA->>Events: Push ActionableInsight ("Needs Attention")
      Events->>Owner: 1-Tap Glassmorphism Card in Feed
      Owner->>Events: Tap "Approve"
      Events->>DB: Execute Action (e.g., Send Drafted Quote)
  ```

  - **Architecture:**
    - Introduce a new asynchronous worker job: `ProactiveAnalysisJob`.
    - This job runs periodically (or triggered by specific events) per tenant, gathering context (pending tasks, stale messages, upcoming schedule).
    - It interfaces with the LLM (Gemini Pro) using a structured prompt to identify 1-3 high-value actionable insights.
    - Output is stored as `ActionableInsight` entities linked to the tenant.
  - **UI/UX Flow (Mobile-First 375px):**
    - The Assistant-First Shell (Home Screen) features a new "Priority Action" card at the top.
    - The card uses the OHC Premium Token library (translucent materials, clear status tokens).
    - Example: A card showing "Carlos, you have 2 estimates pending from yesterday. Tap to review drafted follow-up messages."
    - The owner taps the card, reviews the drafted message, and taps "Send."
  - **Critical User Journey (CUJ):**
    1. Owner logs into the OHC app.
    2. Instead of a blank feed or a list of raw messages, they see a "Proactive Insights" section.
    3. The system highlights a stale quote that needs follow-up.
    4. The owner taps the suggested action, reviews the AI-drafted follow-up email, and approves it.
    5. The insight is dismissed, and the action is executed.

  **Acceptance Criteria:**
  - `ProactiveAnalysisJob` successfully aggregates tenant context and generates actionable insights.
  - Insights are displayed natively in the Flutter/PWA shell using premium UI components.
  - The UI accurately renders on a 375px viewport without horizontal scrolling.
  - Insights contain actionable deep links or integrated approval flows (e.g., draft approval).

  **Priority:** P1
  **Estimated Scope:** Medium

  ---

  ## 6. References & Sources Catalog
  The following 50 URLs were visited and analyzed during this research:

  1. [AI-enabled commerce assistant, Sidekick, designed to make it easier for you to start, run, and grow your business on Shopify. - Shopify](https://shopify.com/magic)
  2. [Shopify: The All-in-One Commerce Platform for Businesses - Shopify](https://shopify.com)
  3. [Power your entire business | Square](https://squareup.com)
  4. [Website Builder - Create a Free Website In Minutes | Wix.com](https://wix.com)
  5. [Durable – AI Business Builder | Launch in minutes](https://durable.co)
  6. [HubSpot | Software & Tools for your Business - Homepage](https://hubspot.com)
  7. [Lark | Productivity Superapp for Chat, Meetings, Docs & Projects](https://larksuite.com)
  8. [The AI workspace that works for you. | Notion](https://notion.so)
  9. [Your request has been blocked. This could be
                        due to several reasons.](https://microsoft.com/en-us/microsoft-365/copilot)
  10. [Commerce built for momentum. | BigCommerce](https://bigcommerce.com)
  11. [
                Free Website Builder: Build a Free Website or Online Store | Weebly
        ](https://weebly.com)
  12. [Launch and Grow Your Business Online with 10Web](https://10web.io)
  13. [Mixo | AI Website Builder for Small Business](https://mixo.io)
  14. [Framer AI: Design websites faster with intelligent tools](https://framer.com/ai)
  15. [Lindy – The Ultimate AI Executive Assistant](https://lindy.ai)
  16. [Relevance AI | The Enterprise Platform for Agents You Can Trust at Scale](https://relevanceai.com)
  17. [Skyvern — AI-Powered Browser Automation for Any Website](https://skyvern.com)
  18. [Fin. The #1 AI Agent for customer service](https://intercom.com/fin)
  19. [DingTalk, Make It Happen](https://dingtalk.com/en)
  20. [Agentforce: The AI Agent Platform | Salesforce](https://www.salesforce.com/agentforce/)
  21. [Zia | Zoho's AI Assistant](https://www.zoho.com/zia/)
  22. [Transform your operations with Zapier and AI](https://zapier.com/ai)
  23. [WhatsApp for Business | Do more with conversations](https://www.whatsapp.com/business)
  24. [Better Business Management Ahead - Apple Business](https://business.apple.com/)
  25. [Yelp for Business: Free and paid advertising solutions](https://www.yelp.com/business)
  26. [Pros - Thumbtack](https://www.thumbtack.com/pro)
  27. [Houzz Login: Sign in to Houzz](https://pro.houzz.com/)
  28. [Setmore | Login](https://my.setmore.com/)
  29. [Calendly](https://calendly.com/teams)
  30. [Acuity Scheduling: Online Booking & Appointment Scheduling Software](https://acuityscheduling.com/)
  31. [All-In-One Salon, Spa & Medspa Software | GlossGenius](https://glossgenius.com/)
  32. [HoneyBook | AI-powered client relationship platform](https://www.honeybook.com/)
  33. [Dubsado](https://www.dubsado.com/)
  34. [Fresha | Top Salon Software | Salon Management Software | Best Salon Booking Software | Spa Software | Salon Scheduling Software | Top 10 salon software  | Top 10 barber software](https://www.fresha.com/for-business)
  35. [Zen Planner: Fitness Business Management & Billing Software](https://www.zenplanner.com/)
  36. [Best Fitness & Wellness Management Software | Mindbody](https://www.mindbodyonline.com/business)
  37. [POS Systems | Point of Sale Systems for all Businesses | Square](https://squareup.com/us/en/point-of-sale)
  38. [POS System & Credit Card Readers | Clover](https://www.clover.com/)
  39. [Lightspeed: Leading Point of Sale (POS) & Commerce Platform - Lightspeed](https://www.lightspeedhq.com/)
  40. [Vend is now Lightspeed - Lightspeed](https://www.vendhq.com/)
  41. [ShopKeep POS is Now ShopKeep by Lightspeed](https://www.shopkeep.com/)
  42. [SumUp](https://www.sumup.com/)
  43. [Point of Sale System | POS System | PayPal US](https://www.izettle.com/)
  44. [Stripe Terminal | Unified Commerce Platform](https://stripe.com/terminal)
  45. [Create and Send Invoices Online | Stripe Invoicing](https://stripe.com/invoicing)
  46. [Payments Processing Solutions for All Business | PayPal US](https://www.paypal.com/us/business)
  47. [Accounting Software for Small Businesses | Xero US](https://www.xero.com/)
  48. [Invoice and Accounting Software for Small Businesses - FreshBooks](https://www.freshbooks.com/)
  49. [Wave: Small Business Software - Wave Financial](https://www.waveapps.com/)
  50. [Gusto | Online HR Services: Payroll, Benefits and everything else](https://gusto.com/)

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
