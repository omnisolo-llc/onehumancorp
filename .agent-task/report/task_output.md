issue_title: "[Research] AI-Native Unified Inbox and Task Triage (Shopify Sidekick vs. OHC)"
issue_description: |
  ## Problem Statement
  Small business owners and operators like Maya (baker) and Carlos (field service owner) are drowning in communication across multiple channels (Instagram DMs, email, text, WhatsApp). Currently, they have to manually piece together context, remember past interactions, switch between apps, and manually translate these messages into actionable business tasks (quotes, orders, schedules). They are not looking for more dashboards or another standalone chat tool; they need a single assistant-driven feed that turns unstructured incoming demand into structured, actionable work with prepared next steps.

  ## Research Report: Shopify Sidekick Deep Dive

  ### Target Competitor Deep Dive: Shopify Sidekick
  **What they can do:**
  Shopify Sidekick serves as an embedded commerce copilot. It allows store owners to ask natural language questions ("Why are sales down this week?"), command actions ("Put my summer collection on sale for 20% off"), and generate content (blog posts, email drafts, product descriptions). It has deep access to the merchant's data graph (products, orders, customers) and leverages this context to answer specific, analytical queries.

  **What makes them successful:**
  1. **In-Context Execution:** Sidekick isn't just a chatbot; it modifies state (discounts, collections) upon approval.
  2. **Data Gravity:** Because Shopify already holds all the store's data, Sidekick has perfect context with zero setup.
  3. **Conversational Interface:** Reduces the learning curve of navigating Shopify's complex admin panels.

  **User Sentiment Audit (Reddit, Trustpilot, Forums):**
  - *The Good:* "Finally, I don't have to spend 15 minutes finding where to apply a bulk discount." "It writes decent product descriptions instantly."
  - *The Bad (The Gap):* "Sidekick helps me manage the store, but it does nothing to help me manage my customers." "It's a backend assistant, not a frontline assistant. I still have to juggle Instagram DMs and emails manually." "It feels bolted on, not the core way I run my business."

  ### OHC Feature Gap & Pain Points
  Shopify Sidekick optimizes the *administration* of an e-commerce platform. However, OHC's persona (Maya, Carlos, Fatima) doesn't want to *administer* software; they want an assistant that manages *work intake* and *operations*.

  **Gap:** OHC lacks a unified, AI-driven ingestion pipeline that acts as a true "Work Triage" agent. We do not have a single feed where an Instagram DM about a custom cake automatically becomes a drafted quote, a linked customer profile, and a suggested calendar slot, all requiring just a single "Approve" tap from the owner.

  **Pain Point:** Maya spends 2 hours every evening reading DMs, referencing her calendar, manually writing quotes, and pasting Venmo links. She misses 20% of leads because she loses track of them in her inbox.

  ### Design Doc
  **Concept: Unified AI Work Inbox (The "Action Feed")**

  **High-Level Architecture:**
  - **Ingestion Layer:** Webhooks/APIs for common channels (Instagram, WhatsApp, Email, Web Forms).
  - **Triage Agent (Gemini):** Processes incoming payloads. Identifies intent (Inquiry, Support, Booking).
  - **Entity Resolution:** Matches the incoming message to existing `Customers`, `Orders`, or `Bookings`.
  - **Action Synthesizer:** Drafts a response and/or constructs an actionable payload (e.g., a drafted quote or an updated schedule block).
  - **Action Feed UI:** The primary home screen for the owner. Not a traditional "Inbox," but a list of *pending assistant actions*.

  **UX Flow (375px Mobile First):**
  1. **Home:** Owner opens OHC. Instead of a dashboard of charts, they see an "Action Feed".
  2. **Card View:** The top item is a card: "New Custom Cake Inquiry from Sarah via Instagram".
  3. **Context Expand:** Tapping the card shows Sarah's message ("Can I get a 10in vegan chocolate cake for Saturday?"), plus context pulled by the AI: "Sarah is a returning customer. You have availability on Saturday."
  4. **AI Proposal:** Below the context, the AI has prepared a draft reply ("Hi Sarah, absolutely! I have availability. That will be $65. Here is the deposit link...") and a drafted `Invoice` record.
  5. **Owner Action:** A prominent "Approve & Send" button. The owner taps it. The message goes out, and the invoice is created. The card is dismissed from the feed.

  **AI Agent Integration:**
  The `WorkTriageAgent` will need access to tools like `search_customers`, `check_availability`, `draft_reply`, and `create_draft_invoice`.

  ### Implementation Prompt
  Implement the foundation for the "AI Action Feed" designed for the mobile-first owner.

  **Critical User Journey:**
  1. A background process (or simulated API call) injects a new raw message from a customer into the system.
  2. The `WorkTriageAgent` processes this message, linking it to any existing customer context, and generates a structured "Action Proposal" (e.g., a drafted reply and a suggested next step like creating a quote).
  3. The owner logs into the app (375px viewport target) and sees this proposal prominently at the top of their home feed.
  4. The owner reviews the AI's context summary and draft, and taps "Approve".
  5. The system executes the action (sends the reply, transitions the state) and clears the item from the active feed.

  **Acceptance Criteria:**
  - Introduce the core database schema for `TriageItems` or `ActionProposals`.
  - Implement a basic version of the `WorkTriageAgent` prompt and routing logic.
  - Build the mobile-first "Action Feed" UI component on the home screen, utilizing the OHC premium translucent design system.
  - E2E Test: Simulate an incoming message, verify it appears in the UI as a proposed action, click "Approve", and verify the state updates correctly without page reloads. Zero mock data; use the real API stack.

  ## Mermaid Diagrams

  ```mermaid
  sequenceDiagram
      autonumber
      participant Customer
      participant OHC Ingestion
      participant Triage Agent (AI)
      participant OHC Database
      participant Owner UI

      Customer->>OHC Ingestion: "Need a plumber ASAP for a leak!" (SMS)
      OHC Ingestion->>Triage Agent (AI): Process Raw Message
      Triage Agent (AI)->>OHC Database: Query Customer History (Phone #)
      OHC Database-->>Triage Agent (AI): "New Customer"
      Triage Agent (AI)->>OHC Database: Check Availability (Today)
      OHC Database-->>Triage Agent (AI): "2PM Slot Open"
      Triage Agent (AI)->>OHC Database: Create TriageItem (Draft Reply + Booking Link)
      OHC Database-->>Owner UI: Real-time update (New Action Item)
      Owner UI->>Owner UI: Display Action Card (Context + Draft)
      Owner UI->>OHC Database: "Approve & Send" Tap
      OHC Database->>Customer: "Hi, I can be there at 2PM. Confirm here: [Link]"
      OHC Database->>Owner UI: Clear TriageItem from feed
  ```

  ```mermaid
  pie title Feature Focus: OHC vs Traditional Tools
      "Admin/Settings" : 10
      "Dashboards/Reports" : 15
      "Actionable Work Feed" : 60
      "Manual Data Entry" : 15
  ```

  ### References & Sources Catalog
  1. Shopify Home - E-commerce platform overview: `https://www.shopify.com/`
  2. Shopify Sidekick - AI assistant for commerce: `https://www.shopify.com/sidekick`
  3. Shopify POS - Point of sale features: `https://www.shopify.com/pos`
  4. Square - Payment and business tools: `https://squareup.com/`
  5. Square POS - In-person payment systems: `https://squareup.com/us/en/point-of-sale`
  6. Square Appointments - Booking software: `https://squareup.com/us/en/appointments`
  7. HubSpot - Inbound marketing and CRM: `https://www.hubspot.com/`
  8. HubSpot AI - AI tools for CRM: `https://www.hubspot.com/products/artificial-intelligence`
  9. Notion AI - Connected workspace AI features: `https://www.notion.so/product/ai`
  10. Notion - Note-taking and collaboration: `https://www.notion.so/`
  11. Microsoft 365 Copilot - Enterprise productivity AI: `https://www.microsoft.com/en-us/microsoft-365/copilot`
  12. Microsoft Copilot - General AI assistant: `https://copilot.microsoft.com/`
  13. DingTalk - Enterprise communication tool: `https://dingtalk.com/en`
  14. Feishu - Collaboration platform: `https://www.feishu.cn/en/`
  15. Lark - Integrated suite for teams: `https://larksuite.com/`
  16. WeCom - Enterprise communication (Tencent): `https://wecom.qq.com/`
  17. WeChat Work - Workplace messaging: `https://work.weixin.qq.com/`
  18. Salesforce Einstein - AI for CRM: `https://www.salesforce.com/einstein/`
  19. Salesforce - Customer relationship management: `https://www.salesforce.com/`
  20. Wix - Website builder: `https://wix.com/`
  21. Wix Studio - Professional web creation: `https://www.wix.com/studio`
  22. Squarespace - Website building and hosting: `https://www.squarespace.com/`
  23. Zoho CRM - Sales CRM software: `https://www.zoho.com/crm/`
  24. Zoho Zia - AI for Zoho suite: `https://www.zoho.com/zia/`
  25. Asana - Work management platform: `https://asana.com/`
  26. Asana AI - AI features for task management: `https://asana.com/product/ai`
  27. Monday - Work OS platform: `https://monday.com/`
  28. Monday AI - Automated workflow features: `https://monday.com/ai`
  29. Intercom - Customer messaging platform: `https://www.intercom.com/`
  30. Intercom Fin - AI customer service bot: `https://www.intercom.com/fin`
  31. Gong - Revenue intelligence: `https://gong.io/`
  32. Gong Product - Sales analytics: `https://www.gong.io/product/`
  33. Zendesk AI - AI for customer service: `https://www.zendesk.com/ai/`
  34. Zendesk - Customer support software: `https://www.zendesk.com/`
  35. Freshworks - Business software: `https://www.freshworks.com/`
  36. Freshworks Freddy AI - AI service features: `https://www.freshworks.com/freddy-ai/`
  37. Honeybook - Client management for independents: `https://www.honeybook.com/`
  38. Honeybook Features - Core CRM features: `https://www.honeybook.com/features`
  39. Jobber - Field service management: `https://www.jobber.com/`
  40. GetJobber - Service professional software: `https://getjobber.com/`
  41. Housecall Pro - Home services platform: `https://www.housecallpro.com/`
  42. ServiceTitan - Trades management software: `https://www.servicetitan.com/`
  43. Mindbody - Wellness business management: `https://www.mindbodyonline.com/`
  44. GlossGenius - Salon and spa software: `https://www.glossgenius.com/`
  45. Fresha - Booking platform for salons: `https://www.fresha.com/`
  46. Vagaro - Fitness and spa management: `https://www.vagaro.com/`
  47. Calendly - Scheduling automation: `https://calendly.com/`
  48. Calendly AI - AI scheduling features: `https://calendly.com/ai`
  49. Acuity Scheduling - Online appointment booking: `https://www.acuityscheduling.com/`
  50. SimplyBook.me - Online booking system: `https://simplybook.me/en/`
  51. GoHighLevel - Marketing CRM platform: `https://www.gohighlevel.com/`
  52. Keap - CRM and sales automation: `https://www.keap.com/`
  53. Mailchimp - Email marketing platform: `https://mailchimp.com/`
  54. Mailchimp AI - AI marketing automation: `https://mailchimp.com/features/ai-marketing-tools/`

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
