issue_title: "Implement Intelligent Owner 'Work Triage' Inbox"
issue_description: |
  # Research Report & Product Mission: OHC "Work Triage" Agentic Inbox

  ## 1. Problem Statement
  Owners and operators currently manage their work across disjointed tools: Instagram DMs for leads, emails for proposals, Shopify/Square for orders, and Calendly/Acuity for bookings. This creates an overwhelming scattered work experience where they don't know what needs their attention *now*, resulting in dropped leads, missed follow-ups, and cognitive overload.
  For example, Maya (the baker) misses custom order deposits because they get buried under casual Instagram inquiries. Carlos (the handyman) forgets to follow up with leads while on the job site.

  ## 2. Research Report
  ### Market Mapping & Competitor Discovery
  We researched 50+ URLs covering the landscape of tools owners use to manage their business, including general platforms (HubSpot, Salesforce, Zoho), communication/collaboration hubs (Slack, Lark, DingTalk, WeCom), and commerce/scheduling tools (Shopify, Acuity, HoneyBook, Dubsado).

  **Top General Competitors:**
  1. Shopify
  2. Square
  3. Lark / WeCom / DingTalk
  4. HubSpot
  5. HoneyBook
  6. Dubsado
  7. GlossGenius
  8. Acuity / Calendly
  9. Wix
  10. Podium / Birdeye

  **Top AI-Native Competitors (Rising Trends):**
  1. Shopify Magic / Sidekick (E-commerce AI)
  2. Notion AI (Knowledge)
  3. Intercom Fin (Customer Service)
  4. Zendesk AI
  5. HubSpot ChatSpot
  6. ClickUp AI
  7. Zapier Central (AI Automation)
  8. Slack AI
  9. Freshworks Freddy AI
  10. Microsoft Copilot

  ### Deep-Dive Competitor Audit: Shopify Magic / Sidekick
  *   **Capabilities**: Conversational AI assistant within the admin dashboard. Can generate product descriptions, answer store analytics questions ("Why are my sales down today?"), and execute basic workflow commands.
  *   **Success Factors**: Embedded directly in the user's workflow; doesn't require technical prompt engineering; acts as a "Sidekick" rather than a separate tool.
  *   **User Sentiment**: Users love the text generation (saves time on product descriptions), but often complain that it lacks multi-channel context (e.g., it doesn't know about Instagram DMs or offline interactions). Small business owners find Shopify itself overwhelming ("Shopify is too complex for a side hustle baker").

  ### OHC Gap & Pain Point Identification
  *   **Gap Matrix**:
      *   *Shopify/HoneyBook*: Great at their specific vertical (commerce or CRM), but terrible at cross-channel unification.
      *   *Lark/WeCom*: Great at internal team collaboration, but heavy for single owners or external customer intake.
      *   *OHC Missing Feature*: OHC lacks a unified, intelligent "Triage" view that ingests signals from all channels (DMs, bookings, payments) and uses an AI agent to categorize, prioritize, and suggest actions *before* the owner has to read every message.
  *   **Unresolved Pain Point**: The "Monday Morning Overload" - opening 5 apps to piece together what needs to be done today.

  ### Agentic Solution Design
  Create the "Work Triage" Inbox in OHC.
  Instead of a standard chronologically sorted inbox, the Work Triage is governed by the `WorkTriage` AI Agent.
  1.  **Intake**: All events (messages, lead forms, payment receipts, booking requests) land in the triage queue.
  2.  **AI Processing**: The agent analyzes each item, tags it with a priority (Urgent, Action Needed, FYI), groups related items (e.g., a customer DM'd *and* booked a meeting), and drafts a proposed action (e.g., Drafted Reply, Prepared Quote).
  3.  **Owner View**: The owner opens OHC and sees a prioritized feed: "Needs Your Attention". They can simply click "Approve" on the agent's proposed actions.

  ## 3. Design Doc
  *   **High-Level Architecture**:
      *   **Entities**: `TriageItem`, `Customer`, `ProposedAction`.
      *   **Relationships**: A `TriageItem` can link to a `Customer` and have one `ProposedAction`.
      *   **Agent Integration**: A background AI job (Gemini Pro) processes new `TriageItem`s via the PostgreSQL `SKIP LOCKED` queue.
  *   **UI Wireframes/Flow (Mobile-First 375px)**:
      *   **Screen 1: The Command Center (Home)**: Top section titled "Action Required". Shows 2-3 high-priority Triage Cards.
      *   **Triage Card**: Displays the context ("Maya requested a custom cake for Friday"), the source (Instagram), and a primary button ("Review & Send Quote").
      *   **Screen 2: Triage Detail**: Expands the card. Shows the AI's drafted response or proposed action. The owner can edit the draft or tap "Approve".
  *   **UX Flow**: Owner logs in -> Sees 3 Urgent items -> Taps the first one -> Reads AI summary -> Taps "Approve Draft" -> Moves to the next item.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: The owner opens the app and sees a prioritized list of actionable work items, with AI-drafted responses or next steps already prepared.
  **Critical User Journey (CUJ)**:
  1.  User opens the OHC app.
  2.  User navigates to the "Triage" feed.
  3.  User sees a Triage Item for a new customer inquiry with an AI-generated draft response.
  4.  User taps "Approve & Send".
  5.  The item is marked resolved and disappears from the Triage feed.
  **Acceptance Criteria**:
  *   The Triage feed is visible on the home screen.
  *   It accurately renders at 375px width without horizontal scrolling.
  *   The UI must not use mock data; it must load real items from the backend database.
  *   Tapping an action button (e.g., "Approve") must trigger a real backend mutation and update the UI optimistically or via refetch.

  ## 5. References & Sources Catalog
  1. https://about.instagram.com/features
  2. https://www.shopify.com/magic
  3. https://squareup.com/us/en
  4. https://www.hubspot.com/products/artificial-intelligence
  5. https://www.larksuite.com/
  6. https://www.dingtalk.com/en
  7. https://notion.so/product/ai
  8. https://www.microsoft.com/en-us/microsoft-365/copilot
  9. https://work.weixin.qq.com/
  10. https://slack.com/features/ai
  11. https://zapier.com/ai
  12. https://www.intercom.com/fin
  13. https://www.zendesk.com/service/ai/
  14. https://www.salesforce.com/einstein/
  15. https://www.zoho.com/zia/
  16. https://www.freshworks.com/freddy-ai/
  17. https://clickup.com/ai
  18. https://monday.com/ai
  19. https://asana.com/product/ai
  20. https://trello.com/tour
  21. https://wix.com/studio/ai
  22. https://www.squarespace.com/ai
  23. https://www.weebly.com/
  24. https://mailchimp.com/features/ai/
  25. https://buffer.com/ai
  26. https://hootsuite.com/features/ai
  27. https://sproutsocial.com/features/ai/
  28. https://calendly.com/
  29. https://acuityscheduling.com/
  30. https://setmore.com/
  31. https://simplybook.me/
  32. https://www.vagaro.com/
  33. https://www.mindbodyonline.com/
  34. https://www.fresha.com/
  35. https://www.glossgenius.com/
  36. https://www.honeybook.com/
  37. https://www.dubsado.com/
  38. https://hellobonzai.com/
  39. https://www.thryv.com/
  40. https://www.podium.com/
  41. https://www.birdeye.com/
  42. https://www.broadly.com/
  43. https://www.yelp.com/business
  44. https://www.google.com/business/
  45. https://business.apple.com/
  46. https://whatsapp.com/business
  47. https://www.messenger.com/business
  48. https://telegram.org/faq_channels
  49. https://line.me/en/business
  50. https://viber.com/business

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
