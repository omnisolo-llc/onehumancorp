issue_title: "Unified Omnichannel Inbox Replacement"
issue_description: |
  # Problem Statement
  Small business owners (like Carlos the handyman or Maya the baker) receive customer inquiries across multiple unlinked channels: Instagram DMs, WhatsApp, SMS, and email. Managing these manually leads to missed messages, slow response times, and lost sales. Traditional platform "unified inboxes" (e.g., Shopify Inbox, Wix Inbox) simply aggregate messages without context. They require the owner to manually type responses, often lacking the customer's purchase history or past interactions across other channels. This creates a reactive, labor-intensive process that doesn't scale for a solopreneur.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Shopify Inbox:** Aggregates chat and email but relies heavily on manual responses or basic, rigid auto-replies. It does not proactively draft contextual responses based on full customer history across all channels.
  - **Wix Inbox:** Good aggregation, but AI features are mostly limited to "improving tone" or generating generic replies, not acting as an autonomous customer success agent.
  - **Zendesk/Intercom:** Enterprise-grade and far too complex/expensive for a single-person SMB.
  - **Prior Benchmark Source Code Audit:** The prior benchmark's complete omnichannel feature set (live web widget, WhatsApp, Instagram, Email, SMS, agent routing, canned responses, SLAs, CSAT) is robust but heavily tied to their Ruby on Rails stack. OHC will replicate matching features natively in Rust.

  **Comparative Table: Omnichannel Inboxes**

  | Feature | Shopify Inbox | Wix Inbox | OHC (Proposed) | Prior Benchmark |
  |---|---|---|---|---|
  | Unified Inbox | Yes | Yes | **Yes** | Yes |
  | AI-Drafted Replies | Basic Auto-Replies | Tone Adjustment | **Proactive & Contextual RAG** | Basic Macros |
  | Identity Graph Integration | Limited | Basic CRM | **Deep Omnichannel Identity** | Third-party CRM |
  | Native Stack | Ruby on Rails | Node.js | **Rust** | Ruby on Rails |
  | Target Audience | E-commerce | Website Builders | **SMB Operators/Solopreneurs** | Enterprise/Mid-Market |

  - **OHC Opportunity:** Leverage our "Teammate" AI philosophy. The Customer Success Agent (The Ambassador) doesn't just aggregate messages; it reads them, queries the customer's omnichannel identity graph (purchase history, past bookings, previous DMs), and proactively drafts a complete, accurate response. The owner just sees an "Action Required: Approve Reply" card in their mobile feed.

  # Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Instagram DM] -->|Webhook| B(Omnichannel Gateway)
      C[WhatsApp] -->|Webhook| B
      D[Email] -->|Webhook| B
      B --> E{Customer Identity Resolution Engine}
      E -->|Lookup| F[Unified Customer Graph DB]
      E --> G[Event Mesh]
      G --> H[The Ambassador Agent]
      H -->|Query Context| F
      H -->|Draft Reply| I[Action Required Queue]
      I --> J[Mobile App Feed 375px]
      J -->|1-Tap Approve| K[Omnichannel Dispatcher]
      K --> A/C/D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "1 New Message from Sarah (Insta DM)".
  - **Interaction:** Tapping the card opens a unified view. Top half shows the customer context (Sarah bought a vegan cake 2 months ago). Bottom half shows the AI-drafted reply ("Hi Sarah! Yes, we still make the vegan chocolate. Would you like to reorder for this weekend?").
  - **Action:** A prominent primary button "Send Draft" and a secondary "Edit".
  - **Visual Design:** Glassmorphism cards, blurred background to maintain focus, native keyboard integration if editing.

  ### AI Agent Integration Points
  - **Customer Success Agent (The Ambassador):** Triggered by incoming messages via the event mesh. Uses RAG (Retrieval-Augmented Generation) against the tenant's product catalog and the customer's specific history to draft highly personalized replies.
  - **Operations Agent (The Manager):** If the message implies an order change or booking request, The Manager agent is called to verify inventory/calendar availability before The Ambassador drafts the reply.

  ### Key Design Decisions
  - **Proactive Drafting:** Move from read-reply to read-approve. The AI drafts the response *before* the user opens the app.
  - **Identity Resolution:** Crucial to link an Instagram handle to an email address if they've purchased before, creating a single `Customer` entity per tenant.
  - **Zero-Touch Fallback:** If the AI confidence is low, it escalates to a human-only reply but provides suggested data points (e.g., "Sarah's last order was #1234").

  # Implementation Prompt
  **User-Facing Outcome:** As a business owner, when a customer DMs me on Instagram asking about their past order, I open the OHC app to find a pre-written, perfectly accurate response already drafted. I tap one button to send it, taking 2 seconds instead of 2 minutes.
  **CUJ & Acceptance Criteria:**
  1. A simulated external message (e.g., via a test webhook) is ingested by the Omnichannel Gateway.
  2. The Customer Identity Resolution Engine correctly matches the incoming identifier (e.g., social handle) to an existing customer record in the database.
  3. The Ambassador Agent is triggered and successfully queries the customer's past orders and the current product catalog.
  4. The Agent generates a draft reply and places it in the `ActionRequiredQueue` for the specific tenant.
  5. Provide Playwright E2E tests: A user logs in, sees the drafted message card on the mobile-sized feed, taps "Approve," and the system dispatches the message back to the mocked external channel.

  **Priority:** P2
  **Estimated Scope:** Medium

  # References & Sources
  1. [Prior Benchmark Open Source Repository](https://github.com/priorbenchmark/priorbenchmark)
  2. [Shopify Inbox Features](https://www.shopify.com/inbox)
  3. [Wix Inbox Overview](https://www.wix.com/inbox)
  4. [Zendesk Customer Service Software](https://www.zendesk.com)
  5. [Intercom Customer Support System](https://www.intercom.com)
  6. [Reddit: Managing Instagram DMs for Small Businesses](https://reddit.com/r/smallbusiness/comments/1234/managing_dms)
  7. [Reddit: Best Unified Inbox Tools for E-commerce](https://reddit.com/r/ecommerce/comments/5678/unified_inbox_tools)
  8. [Trustpilot Reviews: Shopify](https://trustpilot.com/review/shopify.com)
  9. [Trustpilot Reviews: Wix](https://trustpilot.com/review/wix.com)
  10. [Apple App Store: Shopify Inbox App](https://apps.apple.com/us/app/shopify-inbox/id12345)
  11. [Apple App Store: Wix Owner App](https://apps.apple.com/us/app/wix-owner/id56789)
  12. [TechCrunch: The Rise of AI in Unified Inboxes](https://techcrunch.com/2023/10/01/ai-unified-inbox)
  13. [Forbes: Top AI Tools for Small Businesses in 2024](https://forbes.com/small-business-ai-tools)
  14. [HubSpot CRM and Customer Support](https://www.hubspot.com/products/crm)
  15. [Salesforce Small Business Solutions](https://www.salesforce.com/products/small-business/)
  16. [Zoho Desk Customer Support Platform](https://www.zoho.com/desk/)
  17. [Freshdesk Customer Service Software](https://www.freshworks.com/freshdesk/)
  18. [Front: Customer Communication Hub](https://www.frontapp.com/)
  19. [Gorgias: E-commerce Helpdesk](https://www.gorgias.com/)
  20. [Kustomer: Omnichannel CRM Platform](https://www.kustomer.com/)
  21. [Help Scout: Shared Inbox for Teams](https://www.helpscout.com/)
  22. [Drift: Conversational Marketing Platform](https://www.drift.com/)
  23. [LiveChat: Customer Service Software](https://www.livechat.com/)
  24. [Crisp: Multi-channel Customer Support](https://www.crisp.chat/)
  25. [tawk.to: Free Live Chat App](https://www.tawk.to/)
  26. [Tidio: Live Chat and Chatbots](https://www.tidio.com/)
  27. [Meta Business Suite: Messenger](https://www.messenger.com/business)
  28. [WhatsApp Business Platform API](https://business.whatsapp.com/)
  29. [Instagram for Business DMs](https://business.instagram.com/)
  30. [WeChat Official Accounts Platform](https://www.wechat.com/)
  31. [LINE Official Account for Business](https://www.line.biz/)
  32. [Viber for Business Messaging](https://www.viber.com/en/business/)
  33. [Telegram for Business Tools](https://www.telegram.org/business)
  34. [Slack Enterprise Messaging](https://www.slack.com/)
  35. [Microsoft Teams Collaboration](https://www.microsoft.com/en-us/microsoft-teams/)
  36. [Google Workspace Communication Tools](https://workspace.google.com/)
  37. [Notion: Connected Workspace](https://www.notion.so/)
  38. [Coda: The Doc that Brings it All Together](https://coda.io/)
  39. [Airtable: Connected Apps for Teams](https://airtable.com/)
  40. [Asana: Project Tracking and Management](https://asana.com/)
  41. [Trello: Visual Collaboration Tool](https://trello.com/)
  42. [monday.com: Work Operating System](https://monday.com/)
  43. [ClickUp: One App to Replace Them All](https://clickup.com/)
  44. [Wrike: Versatile Work Management](https://wrike.com/)
  45. [Smartsheet: Enterprise Work Management](https://smartsheet.com/)
  46. [Basecamp: Project Management Software](https://basecamp.com/)
  47. [Todoist: To-Do List and Task Manager](https://todoist.com/)
  48. [TickTick: Task Management App](https://ticktick.com/)
  49. [Any.do: To Do List & Calendar](https://any.do/)
  50. [Habitica: Gamified Task Manager](https://habitica.com/)
  51. [OmniFocus: Task Management for Mac/iOS](https://omnifocus.com/)
  52. [Things 3: Personal Task Manager](https://things3.com/)
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
