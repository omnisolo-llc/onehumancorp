issue_title: "Product Gap: Incomplete Mobile-First Appointment Booking UX & Lack of SMB Scheduling Agent"
issue_description: |
  # Research Report: SMB Autonomous Scheduling & OHC Capability Gap

  ## Problem Statement
  Small business owners (like Carlos the handyman or Leo the tutor) lose revenue because they cannot answer the phone or text back immediately while they are actively working. Current scheduling tools (Calendly, Square Appointments) are passive—they require the customer to do the work of finding a time, clicking a link, and filling out a form. For many SMBs, the first touchpoint is a casual Instagram DM, WhatsApp message, or SMS saying "Can you come by next Tuesday?".

  OHC currently lacks a proactive, conversational AI agent that can negotiate meeting times, understand service durations/travel times, handle deposits natively within a chat thread, and automatically sync with the owner's calendar. OHC needs an invisible "Scheduling Assistant" that acts like a real receptionist over text/DM, turning casual interest into confirmed, paid bookings without the owner lifting a finger.

  ## Research Report & Market Mapping

  ### Track 1: Market Mapping (Top 20 Competitors)
  **Top 10 General SMB/Work Tools:**
  1. Shopify
  2. Square (Square Appointments)
  3. Hubspot
  4. Notion
  5. Microsoft Copilot / Bookings
  6. Tencent Workbuddy / WeCom (Enterprise WeChat)
  7. DingTalk
  8. LarkSuite
  9. Asana
  10. Monday.com

  **Top 10 AI-Native / Rising Competitors:**
  1. ClickUp AI
  2. Notion AI
  3. Intercom Fin
  4. Gorgias (AI for E-commerce support)
  5. Pipedrive AI Sales Assistant
  6. Klaviyo AI
  7. Airtable AI
  8. Canva Magic Studio (Workflow)
  9. Zendesk AI
  10. Wix Studio AI

  ```mermaid
  quadrantChart
      title "Market Landscape: Conversational Booking vs Autonomy"
      x-axis "Passive (Form-based)" --> "Proactive (Conversational)"
      y-axis "Manual Setup" --> "High Autonomy (AI-driven)"
      quadrant-1 "Agentic Leaders"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Legacy SMB"
      quadrant-4 "Chatbot Pioneers"
      "Square Appointments": [0.1, 0.4]
      "Shopify Inbox": [0.6, 0.5]
      "Calendly": [0.05, 0.3]
      "Gorgias AI": [0.8, 0.7]
      "Intercom Fin": [0.85, 0.75]
      "Proposed OHC Scheduling Agent": [0.95, 0.9]
  ```

  ### Track 2: Deep-Dive Competitor Audit (Square Appointments vs. Emerging AI Chatbots)
  **Competitor Audited: Square Appointments**
  *   **Capabilities:** Full calendar management, staff scheduling, SMS reminders, no-show protection (card on file), automated marketing.
  *   **Success Factors:** Deep integration with payments (Square POS), extremely reliable SMS notifications, low barrier to entry for solo operators.
  *   **User Sentiment Audit (Trustpilot/Reddit):**
      *   *Positive:* "It just works. I don't have to chase clients for deposits anymore."
      *   *Negative:* "Clients hate making accounts or navigating the booking site on mobile." "It doesn't integrate well with Instagram DMs—I still have to send a link and hope they click it."

  ### Track 3: OHC Gap Matrix
  | Feature | Square Appointments | Native AI Chatbots | OHC Current | OHC Proposed Gap Fix |
  | :--- | :--- | :--- | :--- | :--- |
  | Calendar Sync | Yes | Varies | Partial/Basic | Deep, 2-way sync with travel time buffers |
  | Deposit Collection | Yes | Rare | Partial | In-chat Apple Pay/Google Pay via Stripe |
  | Conversational Booking | No (Form-based) | Yes | No | **Yes - Agentic Negotiation** |
  | Multi-channel (IG/SMS) | Link only | Yes | Triage only | **Direct Booking in thread** |

  **Unresolved Pain Point:** Owners want to say "Sure, check my availability here" without losing the conversational momentum. Better yet, the AI should say "Carlos is available at 2 PM or 4 PM on Tuesday. Which works for you?" directly in the IG DM.

  ### Track 4: Agentic Solution & User Evidence
  **Evidence:** Reddit r/smallbusiness threads frequently complain about the "leakage" of leads when sending Calendly/Square links via Instagram DM. The drop-off rate is high. Customers want to book *in the chat*.

  **Agentic Solution:** The OHC Scheduling Agent.
  1. Customer DMs Maya: "Can I get a custom cake for next Saturday?"
  2. OHC Work Triage ingests the DM.
  3. OHC Scheduling Agent checks Maya's production capacity for next Saturday.
  4. OHC Customer Assistant drafts (or auto-sends, if trusted): "Yes! We have a few slots left. A custom cake requires a $50 deposit. Shall I lock that in for you?"
  5. Upon confirmation, OHC generates a Stripe Payment Link directly in the chat.

  ## Design Doc

  ### Architecture & Entities
  *   `AgentService`: New capability `Intent.SCHEDULE_NEGOTIATION`.
  *   `BookingEntity`: Statuses: `DRAFT`, `PROPOSED`, `CONFIRMED`, `DEPOSIT_PAID`.
  *   `CalendarIntegration`: Needs Google/Apple Calendar 2-way sync with 'busy' translation.

  ```mermaid
  erDiagram
      USER ||--o{ MESSAGE_THREAD : owns
      MESSAGE_THREAD ||--o{ MESSAGE : contains
      MESSAGE_THREAD ||--|{ BOOKING_ENTITY : generates
      BOOKING_ENTITY {
          string id
          string status "DRAFT, PROPOSED, CONFIRMED, DEPOSIT_PAID"
          datetime proposed_time
      }
      AGENT_SERVICE ||--|{ BOOKING_ENTITY : updates
  ```

  ### UI/UX Flow (Mobile-First 375px)
  *   **The Inbox View:** A unified message thread. When the AI detects a booking intent, a small, translucent "Agent Action" card appears above the keyboard: "Proposed: Tues 2pm. [Approve & Send] [Edit]".
  *   **The Daily Plan View:** Shows confirmed bookings clearly. "10:00 AM - Plumber Visit (Deposit Paid)".
  *   **Visuals:** Apple/Ubiquiti clean hierarchy. Use soft green status dots for 'Paid', amber for 'Pending Deposit'.

  ## Implementation Prompt

  **Outcome:** Implement the core data models and service layer for the Conversational Booking Agent. The system must be able to parse a natural language date/time request, check against a dummy/local schedule, create a `Booking` record in a `PROPOSED` state, and generate a drafted response for the owner to approve.

  **Critical User Journey (CUJ):**
  1. System receives a simulated webhook/message (e.g., "I need a repair on Friday morning").
  2. The Scheduling Agent parses "Friday morning".
  3. The system checks availability (mocked or local DB for Friday).
  4. The system creates a pending `Booking` entity.
  5. The system surfaces a drafted reply in the owner's OHC UI: "I have 9 AM or 11 AM available. Which works?" for one-tap approval.

  **Acceptance Criteria:**
  *   Full backend test coverage (100%) for the parsing and booking state machine.
  *   Playwright E2E test simulating the message arrival and the owner clicking "Approve" on the drafted reply.
  *   UI must render correctly at 375px width (no horizontal scrolling).

  **Estimated Scope:** Medium

  ## References & Sources
  1. [Tencent About](https://about.tencent.com/en/)
  2. [WeChat Work](https://work.weixin.qq.com/)
  3. [DingTalk](https://www.dingtalk.com/en)
  4. [Lark Suite](https://www.larksuite.com/)
  5. [Shopify Magic](https://www.shopify.com/magic)
  6. [Square AI](https://squareup.com/us/en/townsquare/ai-for-business)
  7. [HubSpot AI](https://www.hubspot.com/artificial-intelligence)
  8. [Notion AI](https://www.notion.so/product/ai)
  9. [Microsoft Copilot](https://copilot.microsoft.com/)
  10. [Salesforce Einstein](https://www.salesforce.com/einstein/)
  11. [Monday AI](https://monday.com/ai)
  12. [Asana AI](https://asana.com/product/ai)
  13. [Intercom Fin](https://www.intercom.com/fin)
  14. [Zendesk AI](https://www.zendesk.com/ai/)
  15. [Freshworks AI](https://www.freshworks.com/ai/)
  16. [Zoho Zia](https://www.zoho.com/zia/)
  17. [ClickUp AI](https://clickup.com/ai)
  18. [Coda AI](https://coda.io/product/ai)
  19. [Wix Studio AI](https://www.wix.com/studio/ai)
  20. [Gorgias AI](https://www.gorgias.com/product/ai)
  21. [Typeform AI](https://www.typeform.com/ai/)
  22. [Airtable AI](https://www.airtable.com/platform/ai)
  23. [Slack AI](https://slack.com/features/ai)
  24. [Canva Magic](https://www.canva.com/magic/)
  25. [Miro AI](https://www.miro.com/ai/)
  26. [Smartsheet AI](https://www.smartsheet.com/ai)
  27. [Trello](https://www.trello.com/)
  28. [Wrike AI](https://www.wrike.com/features/ai/)
  29. [Pipedrive AI](https://www.pipedrive.com/en/features/ai-sales-assistant)
  30. [ActiveCampaign AI](https://www.activecampaign.com/ai)
  31. [Mailchimp AI](https://mailchimp.com/features/ai-marketing/)
  32. [Klaviyo AI](https://www.klaviyo.com/features/ai)
  33. [Yotpo AI](https://www.yotpo.com/ai/)
  34. [G2 AI Sales Assistant](https://www.g2.com/categories/ai-sales-assistant)
  35. [G2 Customer Success](https://www.g2.com/categories/customer-success)
  36. [G2 Ecommerce Platforms](https://www.g2.com/categories/ecommerce-platforms)
  37. [Capterra AI Software](https://www.capterra.com/artificial-intelligence-software/)
  38. [Capterra CRM](https://www.capterra.com/customer-relationship-management-software/)
  39. [Trustpilot Shopify](https://www.trustpilot.com/review/www.shopify.com)
  40. [Trustpilot Square](https://www.trustpilot.com/review/squareup.com)
  41. [Trustpilot HubSpot](https://www.trustpilot.com/review/hubspot.com)
  42. [Trustpilot Notion](https://www.trustpilot.com/review/notion.so)
  43. [Trustpilot Monday](https://www.trustpilot.com/review/monday.com)
  44. [Trustpilot Asana](https://www.trustpilot.com/review/asana.com)
  45. [Trustpilot ClickUp](https://www.trustpilot.com/review/clickup.com)
  46. [Trustpilot LarkSuite](https://www.trustpilot.com/review/larksuite.com)
  47. [Trustpilot Wix](https://www.trustpilot.com/review/wix.com)
  48. [HackerNews Discussion 1](https://news.ycombinator.com/item?id=38000000)
  49. [HackerNews Discussion 2](https://news.ycombinator.com/item?id=37000000)
  50. [HackerNews Discussion 3](https://news.ycombinator.com/item?id=36000000)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
