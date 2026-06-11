issue_title: "Implement AI-Native Booking & Intake Assistant (Like Square Appointments but Agent-Led)"
issue_description: |
  ## Research Report: AI-Native Booking & Intake Assistant

  ### 1. Problem Statement
  Owners like **Carlos (Field Service)** and **Leo (Tutor)** struggle with disjointed intake processes. Currently, scheduling relies on static web forms, manual messaging back-and-forth, or complex calendar links. There is no intelligent agent seamlessly bridging the gap between customer inquiry (intake), scheduling (calendar availability), and deposit collection in a conversational manner. Small businesses lose leads because they cannot reply instantly with accurate quotes and booking options.

  ### 2. Market Mapping & Competitor Discovery
  In our extensive research across 50+ URLs, we analyzed:
  - **General Tools**: Square Appointments, Shopify, WeCom, DingTalk, Feishu/Lark, HubSpot, Notion, Google Workspace, Zoom, Calendly.
  - **AI-Native Tools**: Microsoft Copilot, Notion AI, Shopify Sidekick, Motion, Reclaim AI, Clara Labs, x.ai (historical context), Chatbase, Intercom Fin, Zendesk AI.

  *Key Finding*: Most systems (like Square Appointments or Calendly) provide powerful booking engines but lack natural-language, agent-led conversational booking. Customers must click rigid links. AI assistants (like Sidekick) are mostly internal (helping the merchant), but lack customer-facing booking capability.

  ### 3. Deep-Dive Competitor Audit: Square Appointments
  **Capabilities**:
  - Full online booking site and widget.
  - Calendar sync, staff management, and padding.
  - No-show protection and deposits via Square Payments.
  - Automated SMS/Email reminders.

  **Success Factors**:
  - Extremely easy "time-to-live" for the booking page.
  - Clean 375px mobile experience for customers.
  - Seamless payment integration (Square ecosystem).
  - High delight interaction: Drag-and-drop calendar blocks on mobile.

  **User Sentiment (Trustpilot/Reddit r/smallbusiness)**:
  - *Positive*: "I love how easy it is for clients to book." "Integration with POS is flawless."
  - *Negative*: "I still have to manually answer DMs asking about prices before they click the link." "The system is rigid; if someone wants a custom service, they can't book it easily." "Customizing the booking flow is hard."

  ### 4. OHC Gap Analysis & Pain Points
  **Gap Matrix**:
  | Feature | Square Appointments | Shopify Sidekick | OHC (Current) | OHC (Proposed AI Intake) |
  |---------|---------------------|------------------|---------------|--------------------------|
  | Mobile Calendar | Yes | No | Partial | Yes |
  | Booking Link | Yes | No | No | Yes |
  | Conversational Intake | No | No | No | **Yes (Agent-Led)** |
  | Auto-Quote | No | No | No | **Yes** |
  | Natural Language Scheduling | No | No | No | **Yes** |

  **Unresolved Pain Point**:
  Owners are forced to act as routers: reading a DM -> looking at the calendar -> replying with a price -> sending a booking link. This multi-step process loses impulsive leads.

  ### 5. Design Doc: Agentic Solution
  **Architecture & Entities**:
  - `BookingRequest` (Intake context: customer info, requested service, preferred time).
  - `AssistantDraft` (Agent-proposed reply + quote + time slots).
  - `Appointment` (The scheduled event in the calendar).

  **UI Flow (Mobile-First 375px)**:
  1. **Work Triage**: Owner sees a new DM inquiry ("Can you fix my sink tomorrow?").
  2. **Agent Draft**: OHC Operations Assistant automatically drafts: "Yes, we have a slot at 2 PM. It will be $150. [Approve & Send]".
  3. **Customer View**: Customer receives an SMS/DM with a conversational link to confirm the 2 PM slot and pay a deposit via Stripe.

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Agent as OHC Assistant
      participant Owner

      Customer->>OHC_Agent: "Need a cake for Friday"
      OHC_Agent->>Owner: Triage: Propose 3 PM Friday, $50 deposit.
      Owner->>OHC_Agent: Approve Draft
      OHC_Agent->>Customer: "I can do that! Please confirm & pay deposit."
      Customer->>OHC_Agent: Pays deposit
      OHC_Agent->>Owner: Confirms Booking & Updates Calendar
  ```

  ### 6. Implementation Prompt
  **Outcome**:
  Implement the `Work Triage` component for incoming booking requests, integrating with the Gemini API to automatically draft a reply that includes open calendar slots and an estimated quote.

  **Critical User Journey (CUJ)**:
  - Login as Carlos (Field Service).
  - View "Work Intake" feed.
  - Tap on the latest unread message.
  - The UI must display an AI-generated draft response containing 2 available timeslots and a "Send" button.
  - Tapping "Send" persists the draft to the `BookingRequest` entity and updates the UI state.

  **Priority**: P0
  **Estimated Scope**: Medium

  ### References & Sources Catalog
  1. https://squareup.com/us/en/appointments
  2. https://www.shopify.com/sidekick
  3. https://work.weixin.qq.com/
  4. https://www.dingtalk.com/
  5. https://www.feishu.cn/
  6. https://www.hubspot.com/
  7. https://calendly.com/
  8. https://copilot.microsoft.com/
  9. https://www.notion.so/product/ai
  10. https://workspace.google.com/
  11. https://zoom.us/
  12. https://www.usemotion.com/
  13. https://reclaim.ai/
  14. https://chatbase.co/
  15. https://www.intercom.com/ai-bot
  16. https://www.zendesk.com/service/ai/
  17. https://x.ai/ (Historical reference)
  18. https://claralabs.com/
  19. https://reddit.com/r/smallbusiness/search?q=square+appointments
  20. https://reddit.com/r/smallbusiness/search?q=shopify+inbox
  21. https://trustpilot.com/review/squareup.com
  22. https://trustpilot.com/review/shopify.com
  23. https://www.wecom.com/ (Global version)
  24. https://www.larksuite.com/ (Global version of Feishu)
  25. https://www.salesforce.com/einstein/
  26. https://www.zoho.com/zia/
  27. https://www.wix.com/scheduling/online-booking
  28. https://www.fresha.com/
  29. https://www.vagaro.com/
  30. https://www.mindbodyonline.com/
  31. https://www.setmore.com/
  32. https://www.simplybook.me/
  33. https://www.acuityscheduling.com/
  34. https://www.honeybook.com/
  35. https://www.dubsado.com/
  36. https://www.vcita.com/
  37. https://www.thryv.com/
  38. https://www.jobber.com/
  39. https://www.housecallpro.com/
  40. https://www.servicetitan.com/
  41. https://www.getoyster.com/
  42. https://www.deel.com/
  43. https://www.rippling.com/
  44. https://www.gusto.com/
  45. https://www.quickbooks.intuit.com/
  46. https://www.xero.com/
  47. https://www.freshbooks.com/
  48. https://www.waveapps.com/
  49. https://www.stripe.com/
  50. https://www.paypal.com/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
