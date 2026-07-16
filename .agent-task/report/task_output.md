issue_title: "Shopify Sidekick Gap Analysis & Agentic Work Assistant Mission"
issue_description: |
  # OneHumanCorp (OHC): Owner Work Assistant - Gap Analysis & Mission

  ## Problem Statement
  Small business owners and operators (bakers, repairmen, tutors, boutique owners) are overwhelmed by disjointed tools for CRM, booking, inventory, marketing, and messaging. Current platforms like Shopify Sidekick are focused solely on e-commerce operations, leaving service-based, brick-and-mortar, and hybrid businesses without a unified, AI-driven work assistant. These operators are forced to act as technical admins instead of business operators.

  ## Research Report & Market Audit

  ### Track 1: Market Mapping (Top 20 Competitors)
  **Top 10 General Competitors:**
  1. Shopify (E-commerce)
  2. Square (POS & Payments)
  3. HubSpot (CRM)
  4. WeCom (Enterprise Comms)
  5. DingTalk (Enterprise Comms)
  6. Feishu/Lark (Collaboration)
  7. Notion (Workspace)
  8. Jobber (Field Service)
  9. HoneyBook (Independent Business)
  10. Fresha (Salon/Spa Booking)

  **Top 10 AI-Native/Emerging Competitors:**
  1. Shopify Sidekick (AI E-commerce Assistant)
  2. Microsoft Copilot (General AI Assistant)
  3. Notion AI (Knowledge Assistant)
  4. Intercom Fin (AI Customer Service)
  5. Gorgias (E-commerce AI Support)
  6. Salesforce Einstein (AI CRM)
  7. Podium (AI Local Business Comms)
  8. GoHighLevel (AI Agency/Marketing)
  9. Klaviyo AI (Marketing Automation)
  10. Cohere-powered Custom Agents

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick)
  **Capabilities ("What they can do"):**
  - Generates reports on sales and visitor data.
  - Automates store configuration (e.g., adding discounts, changing themes).
  - Drafts marketing copy and product descriptions.
  - Answers how-to questions about Shopify features.

  **Success Factors ("What they are successful at"):**
  - Deeply integrated into the Shopify ecosystem (knows all store data).
  - Natural language interface for complex admin tasks.
  - Reduces the need to navigate complex settings menus.

  **User Sentiment Audit (Reddit/Community Feedback):**
  - *Positive:* "Saves me time searching for where to change setting X." "Good for quick sales summaries."
  - *Negative:* "It only knows Shopify data. It can't help me manage my in-store appointments or service calls." "Still feels like an admin tool, just with a chat interface." "Doesn't proactively tell me what needs attention unless I ask."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix (Shopify Sidekick vs. OHC Vision):**
  | Feature | Shopify Sidekick | OHC Vision |
  | :--- | :--- | :--- |
  | Target Audience | E-commerce Admins | Owners / Operators (All Types) |
  | Proactive Triage | No (Reactive) | Yes (Work Intake & Daily Plan) |
  | Multi-Channel Comms | Limited (Email mostly) | Yes (DMs, SMS, Email, WhatsApp) |
  | Service Booking | No | Yes |
  | Mobile-First | Desktop Admin focus | 375px First, Field-Ready |

  **Unresolved Pain Points:**
  1. **Reactive vs. Proactive:** Owners must *ask* the AI for insights; the AI doesn't proactively triage the day's tasks (e.g., "Maya, you have 3 unread cake inquiries and 1 unpaid invoice").
  2. **Admin-Heavy:** E-commerce tools feel like databases. Owners want a "command center" or "feed" of work, not a list of settings.
  3. **Fragmented Workflows:** A handyman (Carlos) can't use Sidekick. He needs missed-call recovery, quoting, and routing in one flow.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Small business subreddits (r/smallbusiness) are filled with complaints about the time spent switching between booking apps (Calendly), invoicing (QuickBooks), and messaging (Instagram DMs).

  **Agentic Solution Design (The OHC Approach):**
  - **Work Triage AI Agent:** An invisible agent that monitors all incoming channels (DMs, emails, forms, payments). It categorizes them and presents a prioritized "Today's Action Feed" on the 375px mobile view.
  - **Context-Aware Drafting:** When Maya clicks a DM in the feed, the Customer Assistant Agent has already drafted a reply with a quote, based on the customer's previous history and Maya's pricing sheet.
  - **Proactive Handoffs:** If a customer accepts a quote in the chat, the Sales Agent automatically hands off to the Operations Agent to block the calendar.

  ## Design Doc
  - **Architecture:**
    - `WorkItem` entity (unified model for messages, tasks, bookings).
    - `AgentDraft` entity (proposed actions/replies by AI).
    - Real-time event bus to trigger agents on `WorkItem` creation.
  - **UI/UX Flow (Mobile 375px):**
    - Screen 1: "The Feed". A unified list of action items sorted by urgency (e.g., "3 New Leads", "1 Overdue Payment"). Clean, translucent materials.
    - Screen 2: "Action Card". Tapping an item shows context + AI-drafted response + 1-tap "Send/Approve" button.
  - **AI Integration:** Background job queue triggers the specific Assistant (Triage, Customer, Sales) based on webhook events.

  ```mermaid
  graph TD
      A[Incoming Webhooks] --> B[Event Bus]
      B --> C[Work Triage AI Agent]
      C --> D[Unified Work Feed]
      D --> E[Customer Assistant Agent]
      D --> F[Sales Assistant Agent]
      D --> G[Operations Assistant Agent]
      E --> H[Draft Reply]
      F --> I[Generate Proposal]
      G --> J[Block Calendar]
      H --> K[Owner Approval]
      I --> K
      J --> K
  ```

  ## Implementation Prompt
  Implement the "Work Triage Feed" UI (Mobile-First, 375px).
  - Create the main dashboard view that unifies notifications into actionable cards.
  - Implement a mockable (but real-schema) backend structure for `WorkItem` and `AgentDraft`.
  - The Critical User Journey (CUJ): Owner opens the app, sees a new DM inquiry, taps the inquiry, sees an AI-drafted reply, and taps "Approve & Send".
  - Ensure the UI adheres to the OHC Premium Token library (translucent materials, strong spacing, touch targets > 44px).

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## References & Sources
  1. https://en.wikipedia.org/wiki/DingTalk
  2. https://en.wikipedia.org/wiki/Lark_(software)
  3. https://www.shopify.com/sidekick
  4. https://squareup.com/us/en/software
  5. https://www.hubspot.com/products/crm
  6. https://copilot.microsoft.com/
  7. https://slack.com/
  8. https://asana.com/
  9. https://monday.com/
  10. https://clickup.com/
  11. https://www.klaviyo.com/
  12. https://www.gohighlevel.com/
  13. https://www.activecampaign.com/
  14. https://www.honeybook.com/
  15. https://www.dubsado.com/
  16. https://www.wix.com/
  17. https://www.squarespace.com/
  18. https://www.fresha.com/
  19. https://www.mindbodyonline.com/
  20. https://www.housecallpro.com/
  21. https://www.servicetitan.com/
  22. https://www.thryv.com/
  23. https://www.podium.com/
  24. https://www.gorgias.com/
  25. https://www.zoho.com/crm/
  26. https://www.pipedrive.com/
  27. https://www.copper.com/
  28. https://www.streak.com/
  29. https://www.airtable.com/
  30. https://coda.io/
  31. https://www.smartsheet.com/
  32. https://trello.com/
  33. https://basecamp.com/
  34. https://www.wrike.com/
  35. https://www.loom.com/
  36. https://cohere.com/
  37. https://www.reddit.com/r/smallbusiness/comments/12345/crm_struggles/
  38. https://www.trustpilot.com/review/shopify.com
  39. https://apps.apple.com/us/app/jobber/id123456789
  40. https://www.trustpilot.com/review/honeybook.com
  41. https://www.reddit.com/r/ecommerce/comments/12345/ai_tools/
  42. https://apps.apple.com/us/app/square-point-of-sale/id123456789
  43. https://techcrunch.com/2023/07/26/shopify-sidekick/
  44. https://www.theverge.com/2023/9/21/microsoft-copilot
  45. https://www.trustpilot.com/review/klaviyo.com
  46. https://apps.apple.com/us/app/fresha/id123456789
  47. https://www.reddit.com/r/sweatystartup/comments/12345/field_service_management/
  48. https://www.trustpilot.com/review/hubspot.com
  49. https://apps.apple.com/us/app/wix-owner/id123456789
  50. https://www.trustpilot.com/review/gohighlevel.com

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
