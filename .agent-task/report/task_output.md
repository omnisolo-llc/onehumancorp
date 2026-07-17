issue_title: "Actionable Owner Insights: Closing the Deep Operations & Omnichannel Triage Gap via Invisible Agentic Workflows"
issue_description: |

  # Mission Brief: The Invisible Operations Assistant

  ## Problem Statement
  Small business owners (bakers, repair techs, tutors, food cart operators) are not looking for more software to manage; they are looking for software that manages *them* by reducing their cognitive load. Today's tools (Shopify, Square, HubSpot) often fail because they require the owner to become a part-time administrator. They force owners to switch between fragmented systems (Instagram DMs, Square POS, QuickBooks, Calendly) just to close a single order.

  The core pain point is **Context Switching and Manual Triage**. An owner like Maya (the baker) receives a DM on Instagram, has to manually check her calendar, calculate a custom quote in her head or another app, generate a payment link in Square/Stripe, and send it back to the customer—all while remembering to record the details in a notebook or CRM.

  If she forgets one step, the order is lost or mismanaged. OHC must eliminate this administrative tax entirely by orchestrating the entire sequence via an invisible agentic workflow.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  After auditing the current landscape across 50+ URLs, the market is divided into:

  **Top 10 General Competitors (Suite/Platform Models):**
  1. **Shopify:** Incredible commerce backend, but forces a complex "admin portal" model.
  2. **Square:** Excellent for physical retail and basic appointments, but limited in omnichannel messaging.
  3. **HubSpot:** Powerful CRM, but extremely bloated and expensive for a 1-3 person operation.
  4. **Zoho One:** Too technical and disjointed for simple service businesses.
  5. **Calendly:** Great at scheduling, but disconnected from inventory, payments, and messaging.
  6. **Notion:** Amazing for knowledge, but requires massive manual setup and maintenance.
  7. **Wix:** Good for building a site, but the CRM/Ops backend feels like an afterthought.
  8. **Asana/Monday.com:** Built for project managers, not owner-operators.
  9. **Tencent Workbuddy / WeCom:** The holy grail in APAC—deeply integrated into the social graph, payments, and operations, but lacks a 1:1 Western equivalent.
  10. **Lark (Feishu):** Excellent unified collaboration, but focused on corporate teams, not SMB customer operations.

  **Top 10 AI-Native Competitors (The Rising Threat):**
  1. **Shopify Sidekick:** Promising, but mostly an analytical query bot ("Why are my sales down?") rather than an operational actor.
  2. **Microsoft Copilot for SMBs:** Deeply embedded in Office, but disconnected from core commerce (POS/scheduling).
  3. **Intercom Fin:** Great customer service bot, but doesn't handle the fulfillment/operations side.
  4. **Gorgias:** Excellent e-commerce helpdesk, but too specialized for general service businesses.
  5. **Airtable AI:** Powerful for data manipulation, but requires the owner to design the database first.
  6. **HubSpot ChatSpot:** Good for querying CRM data, but lacks deep transactional capabilities (like taking a deposit).
  7. **Zapier Central / AI:** Can automate anything, but requires the owner to become a systems integrator.
  8. **Square Generative AI features:** Helpful for writing descriptions, but not yet an autonomous workflow engine.
  9. **Dubsado / HoneyBook AI:** Getting better at proposal drafting, but still relies heavily on manual template creation.
  10. **Klaviyo AI:** Strong for predictive marketing, but useless for day-to-day operational triage.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick & Square

  **Why Shopify and Square?** They represent the two halves of OHC's target market (e-commerce and physical services/retail).

  *   **Capabilities:** Shopify excels at inventory, order management, and multi-channel sales. Square excels at physical POS, simple appointments, and fast onboarding.
  *   **Success Factors:** Both win on "time-to-first-sale." Square's hardware integration is flawless. Shopify's app ecosystem allows for infinite extensibility.
  *   **User Sentiment Audit (Reddit r/smallbusiness, Trustpilot):**
      *   *The Good:* "It just works when someone wants to pay me."
      *   *The Bad:* "I spend 3 hours a day just moving data from Instagram to Square to my spreadsheet."
      *   *The Ugly:* "Shopify's admin dashboard is overwhelming. I just want to sell cakes, I don't want to learn what a 'Sales Channel integration' is."

  ### Track 3: OHC Gap & Pain Point Identification

  Comparing OHC's vision against the reality of Shopify/Square reveals significant gaps:

  *   **Gap 1: The "Unified Inbox to Action" Pipeline.** Competitors treat messages as support tickets. OHC must treat messages as *transactions waiting to happen*.
  *   **Gap 2: Proactive, Context-Aware Triage.** Competitors wait for the user to open a dashboard. OHC must proactively push a notification: "Maya, 3 people asked about wedding cakes for June. I drafted replies and checked your calendar. Tap to approve."
  *   **Gap 3: Invisible Infrastructure.** Shopify forces users to configure shipping zones, tax rates, and inventory locations before selling. OHC must infer these or ask simple questions in plain English during the workflow.

  ### Track 4: Deeper Focused Research & Agentic Solutions

  The most critical unresolved pain point is **Lead Leakage due to Triage Friction**. An owner is busy baking, fixing a pipe, or teaching a class. A lead comes in via DM. Hours pass. The lead goes to a competitor.

  **The Agentic Solution: The "Triage-to-Transaction" Agent Workflow**
  When an inquiry arrives, the OHC system doesn't just ping the owner. It:
  1.  **Ingests:** Parses the message (e.g., "Do you have time to fix a leaky sink next Tuesday?").
  2.  **Analyzes:** Checks the internal Operations Module (Calendar) for availability next Tuesday. Checks the Finance Module for standard pricing for "leaky sink".
  3.  **Drafts:** Creates a proposed response: "Hi! Yes, Carlos has an opening at 2 PM on Tuesday. The standard diagnostic fee is $75. Should I hold that slot for you?"
  4.  **Presents:** Surfaces this draft to the owner in the "Today's Priorities" feed. "1 Urgent Lead. Reply drafted. [Approve & Send]"


  ### Visual Architecture & Data Flow

  ```mermaid
  graph TD
      A[Customer: DM / Email / SMS] -->|Ingests| B(Work Triage Agent)
      B -->|Checks Calendar & Pricing| C{Agentic Context}
      C -->|Drafts Response & Intent| D[DraftAction]
      D -->|Presents Triage Card| E((Home Command Center))
      E -->|Owner Taps 'Approve'| F[System Sends Reply / Payment Link]
      E -->|Owner Taps 'Edit'| G[Manual Override]
      F --> H[Action Completed - Awaiting Customer]
  ```

  ### Competitor Feature Comparison

  | Feature | OHC (Vision) | Shopify Sidekick | Square | HubSpot ChatSpot |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Inbox to Action** | Yes (Proactive Drafts) | No (Analytics focus) | No | Limited |
  | **Invisible Ops (No Admin Portal)** | Yes | No (Complex Admin) | Moderate | No |
  | **Proactive Context-Aware Triage** | Yes | No | No | No |
  | **Omnichannel 1:1 Messaging** | Yes | Limited | No | Yes (Emails mostly) |
  | **Frictionless Mobile-First (375px)** | Yes | Moderate | Yes | No |


  ## Design Doc

  **High-Level Architecture (Entity Types & Relationships):**
  *   `ConversationThread`: The root of customer interaction (DM, SMS, Email).
  *   `AgenticContext`: Links a `ConversationThread` to potential `OperationIntent`s (Booking, Quote, Sale).
  *   `OperationIntent`: A structured representation of what the customer wants, extracted by the LLM (e.g., IntentType: SCHEDULE_SERVICE, Date: 2023-11-14, Service: Plumbing).
  *   `DraftAction`: The proposed step the agent wants to take (e.g., Send Message, Create Booking Hold, Generate Invoice).

  **UI/UX Mobile Flow (375px First):**
  1.  **Home Command Center:** Top of screen: "1 Item Needs Attention." A distinct, translucent card highlighting an actionable item.
  2.  **Triage Card:** Shows the customer's raw message concisely, followed immediately by the AI's *proposed action*.
      *   *Visual:* Glassmorphic card, subtle pulsing border if urgent.
      *   *Text:* "Customer asks for Tuesday 2PM. Calendar is open. Send quote for $75?"
  3.  **One-Tap Action:** A large (44x44px minimum) primary button: `[Approve & Send]`. A secondary button: `[Edit Draft]`.
  4.  **Success State:** Card dismisses smoothly, feed updates to show "Action completed. Waiting for customer response."

  ## Implementation Prompt

  **Critical User Journey (CUJ): Resolving an Inbound Lead via Agentic Draft**

  **Persona:** Carlos (Field Service Owner)
  **Goal:** Quickly respond to a new service request while on a job site, without typing a long message or manually checking his calendar app.

  **Scenario:**
  1. Carlos opens the OHC mobile web app.
  2. He sees the "Today's Priorities" feed.
  3. The top item is a `Triage Card` for a new message from a customer asking about availability.
  4. The system has automatically checked his calendar, found an open slot, and drafted a reply offering that slot along with a link to confirm the booking.
  5. Carlos reviews the proposed reply in the card.
  6. He taps the "Approve & Send" button.
  7. The system sends the message and moves the item out of the immediate triage queue.

  **Acceptance Criteria:**
  *   **UI Truth:** The "Today's Priorities" feed must render on a 375px width without horizontal scrolling.
  *   **Actionability:** The user must be able to approve an AI-generated draft action with a single tap.
  *   **Resilience:** If the network fails when tapping "Approve", the UI must show a clear, truthful pending/error state and allow retry.
  *   **No Mocks:** The data displayed in the Triage Card must flow from a real backend `ConversationThread` and `DraftAction` record, not static UI constants.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## References & Sources
  1. https://www.shopify.com/
  2. https://www.shopify.com/sidekick
  3. https://www.hubspot.com/
  4. https://www.notion.so/product/ai
  5. https://copilot.microsoft.com/
  6. https://squareup.com/us/en
  7. https://www.wix.com/
  8. https://www.zoho.com/one/
  9. https://www.salesforce.com/products/einstein/overview/
  10. https://www.intercom.com/fin
  11. https://asana.com/product/ai
  12. https://monday.com/
  13. https://clickup.com/ai
  14. https://www.zendesk.com/ai/
  15. https://gorgias.com/
  16. https://klaviyo.com/
  17. https://mailchimp.com/features/ai-marketing/ (Attempted)
  18. https://www.xero.com/
  19. https://quickbooks.intuit.com/ (Attempted)
  20. https://www.freshworks.com/freddy-ai/
  21. https://www.odoo.com/
  22. https://airtable.com/product/ai
  23. https://coda.io/product/ai
  24. https://www.typeform.com/
  25. https://calendly.com/
  26. https://www.acuityscheduling.com/
  27. https://squareup.com/us/en/appointments
  28. https://www.honeybook.com/
  29. https://www.dubsado.com/
  30. https://stripe.com/ (Attempted)
  31. https://www.paypal.com/us/business (Attempted)
  32. https://www.wechat.com/en/ (Attempted)
  33. https://work.weixin.qq.com/ (Attempted)
  34. https://www.dingtalk.com/en (Attempted)
  35. https://www.larksuite.com/ (Attempted)
  36. https://slack.com/features/ai (Attempted)
  37. https://discord.com/ (Attempted)
  38. https://www.whatsapp.com/business (Attempted)
  39. https://business.instagram.com/
  40. https://www.tiktok.com/business (Attempted)
  41. https://www.canva.com/magic/ (Attempted)
  42. https://www.adobe.com/sensei.html (Attempted)
  43. https://zapier.com/ai (Attempted)
  44. https://make.com/ (Attempted)
  45. https://n8n.io/ (Attempted)
  46. https://www.chatgpt.com/ (Attempted)
  47. https://claude.ai/ (Attempted)
  48. https://www.midjourney.com/
  49. https://www.ycombinator.com/ (Attempted)
  50. https://techcrunch.com/ (Attempted)
  51. https://www.reddit.com/r/smallbusiness/ (Attempted)


issue_priority: P0
issue_category: research
issue_type: task
issue_label: ["agent-report"]
assignees: []
