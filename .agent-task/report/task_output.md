issue_title: "OHC Work Triage: Unified Inbox & Action Feed"
issue_description: |
  # Research Report: The Missing Center of Small Business Operations

  ## 1. Problem Statement
  Owners like Maya (Baker) and Carlos (Handyman) are overwhelmed by fragmented channels. They receive inquiries via Instagram DMs, WhatsApp, and web forms, manage scheduling on a separate calendar, and handle payments via Stripe or Square. None of these tools talk to each other. Shopify Sidekick is too e-commerce heavy and ignores service scheduling. Square is too POS-heavy and ignores conversational commerce. OHC lacks a unified "Work Triage" feed that centralizes demand and turns it into actionable, AI-assisted tasks.

  ## 2. Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. Shopify - E-commerce giant, complex for service/custom businesses.
  2. Square - Heavy on POS and simple booking, weak on complex CRM/DMs.
  3. HubSpot - Powerful CRM, but too enterprise/B2B for micro-operators.
  4. Notion - Great for knowledge, lacks native commerce/transaction workflows.
  5. Wix - Website builder with bolted-on scheduling/store, disjointed UX.
  6. Intercom - Excellent customer support, not designed for operations/booking.
  7. Monday.com - Project management, lacks native payment/point-of-sale.
  8. Asana - Pure project management, no commerce.
  9. Salesforce - Overkill for small operators, steep learning curve.
  10. Zoho One - Comprehensive but clunky interface, lacks consumer-grade UX.

  **Top 10 AI-Native/Feature Competitors:**
  1. Shopify Sidekick - AI assistant for store management and analytics.
  2. Square AI - Generates item descriptions and basic messaging.
  3. Notion AI - Document generation and summarization.
  4. HubSpot ChatSpot - AI CRM query tool.
  5. Intercom Fin - AI customer service bot.
  6. ClickUp AI - Project management text generation.
  7. Monday AI - Workflow automation and summarization.
  8. Wix AI - Website and text generation.
  9. Microsoft Copilot - General office and productivity assistant.
  10. Salesforce Einstein - Enterprise sales analytics and forecasting.

  ## 3. Deep-Dive Competitor Audit: Shopify Sidekick

  **Capabilities:** Sidekick is an embedded conversational agent in the Shopify admin panel. It can answer questions about store performance, execute simple tasks (e.g., "Create a 10% discount code for holiday"), and summarize sales data.
  **Success Factors:** It lives directly where the user works, requires no complex prompting, and understands the store's data context perfectly.
  **User Sentiment Audit:**
  - *Praise:* "It saves me clicking through 5 menus to make a discount."
  - *Complaints (Reddit r/ecommerce):* "Sidekick can't manage my Instagram DMs where 80% of my custom orders happen." "It's completely useless for my service-based bookings."

  ## 4. OHC Gap & Pain Point Identification

  **OHC Feature Audit:** OHC currently has foundational models but lacks the central nervous system to pull inbound demand into one view.
  **Gap Matrix:**
  - *Shopify:* Has commerce AI, lacks conversational DM triage.
  - *Square:* Has POS/booking, lacks intelligent follow-up and proposal drafting.
  - *OHC:* Needs the **Work Triage Feed** - a single, unified inbox that combines messages, booking requests, and system alerts, layered with an AI assistant to draft next actions.
  **Unresolved Pain Point:** Custom orders (like Maya's cakes) or field services (like Carlos's repairs) start as conversations, not cart checkouts. Owners lose leads because they forget to reply or follow up.

  ## 5. Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design:**
  - A "Work Triage" screen where every inbound interaction (DM, form, call summary) lands.
  - The AI "Customer Assistant" reads the context and attaches a proposed action (e.g., "Draft Reply: Quote for $150", "Action: Schedule Visit").
  - The owner just taps "Approve & Send" or modifies the draft.

  ```mermaid
  graph TD
      A[Inbound: IG DM, Email, Form] --> B(Work Triage Engine)
      B --> C{AI Intent Recognition}
      C -->|Question| D[Draft Reply]
      C -->|Booking| E[Draft Schedule Proposal]
      C -->|Custom Order| F[Draft Quote/Deposit Link]
      D --> G((Owner Approves))
      E --> G
      F --> G
      G --> H[Action Executed via API]
  ```

  ### Design Doc

  **Architecture:**
  - `TriageItem` entity: ID, source, context, timestamp, status (pending, actioned).
  - `AgentDraft` entity: Linked to `TriageItem`, contains proposed text, proposed structured action (e.g., create invoice).
  **UX Flow (375px Mobile First):**
  - **Screen 1 (Home):** Feed of pending `TriageItem` cards. Each card shows the sender, a 1-line summary of the request, and a highlighted "Suggested Action" button.
  - **Screen 2 (Action Detail):** Tapping the card opens a bottom sheet with the full message thread and the AI's drafted response/action. The user can tap "Edit" or swipe to "Approve".

  ### Implementation Prompt

  Build the `Work Triage` mobile-first feed and backend capability.
  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on their mobile device.
  2. The home screen displays a prioritized feed of inbound requests (e.g., 3 pending DMs).
  3. Owner taps the top request ("Need a vegan cake for Saturday").
  4. The UI displays the message and an AI-generated draft response containing a proposed deposit link.
  5. Owner taps "Send". The item is marked completed and disappears from the feed.
  **Acceptance Criteria:**
  - UI is perfectly responsive at 375px wide.
  - `TriageItem` feed is implemented using Flutter/PWA components.
  - The feed updates correctly when an item is actioned.
  - The feature MUST be fully tested via Playwright E2E ensuring all buttons and links in the feed are interactive and functionally sound.

  ## References & Sources
  1. https://www.shopify.com
  2. https://www.shopify.com/pricing
  3. https://www.shopify.com/features
  4. https://www.shopify.com/pos
  5. https://www.shopify.com/sidekick
  6. https://squareup.com/us/en
  7. https://squareup.com/us/en/pricing
  8. https://squareup.com/us/en/point-of-sale
  9. https://squareup.com/us/en/appointments
  10. https://www.hubspot.com
  11. https://www.hubspot.com/pricing/crm
  12. https://www.hubspot.com/products/marketing
  13. https://www.hubspot.com/products/sales
  14. https://www.notion.so
  15. https://www.notion.so/pricing
  16. https://www.notion.so/product/ai
  17. https://www.microsoft.com/en-us/microsoft-365/copilot
  18. https://www.wix.com
  19. https://www.wix.com/pricing
  20. https://www.intercom.com
  21. https://www.intercom.com/pricing
  22. https://www.intercom.com/ai-bot
  23. https://monday.com
  24. https://monday.com/pricing
  25. https://asana.com
  26. https://asana.com/pricing
  27. https://www.salesforce.com/products/einstein/overview/
  28. https://www.zoho.com/one/
  29. https://www.zoho.com/one/pricing/
  30. https://www.freshworks.com/crm/
  31. https://www.freshworks.com/crm/pricing/
  32. https://clickup.com/
  33. https://clickup.com/pricing
  34. https://clickup.com/ai
  35. https://www.xero.com/us/
  36. https://www.xero.com/us/pricing/
  37. https://quickbooks.intuit.com/
  38. https://quickbooks.intuit.com/pricing/
  39. https://www.gusto.com/
  40. https://www.gusto.com/pricing
  41. https://stripe.com/
  42. https://stripe.com/pricing
  43. https://stripe.com/payments/checkout
  44. https://www.honeybook.com/
  45. https://www.honeybook.com/pricing
  46. https://www.jobber.com/
  47. https://www.jobber.com/pricing
  48. https://www.housecallpro.com/
  49. https://www.housecallpro.com/pricing
  50. https://larksuite.com/
  51. https://larksuite.com/pricing
  52. https://reddit.com/r/smallbusiness
  53. https://reddit.com/r/ecommerce
  54. https://trustpilot.com/review/shopify.com
  55. https://trustpilot.com/review/squareup.com

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
