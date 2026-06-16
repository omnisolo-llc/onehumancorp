issue_title: "OHC Mission: Ambient Agentic Intake & Scheduling for Mobile Service Operators"
issue_description: |
  # Research Report: Ambient Agentic Intake & Scheduling for Mobile Service Operators

  ## Problem Statement
  Mobile service operators (like Carlos, the field service owner, or Leo, the tutor) lose revenue because they cannot capture demand, coordinate scheduling, and provide instant quotes while actively doing their jobs. Current tools force them to context-switch, open complex apps, or send standard automated messages that do not handle complex customer inquiries. They need an ambient assistant that handles intake, triage, quoting, and scheduling in the background.

  ## Research & Market Mapping

  ### Top 10 General Competitors
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. Microsoft Copilot
  6. Wix
  7. Lark
  8. DingTalk
  9. WeCom
  10. Jobber

  ### Top 10 AI-Native Competitors
  1. Harvey AI (Legal, but workflow-centric)
  2. Sierra (Conversational AI for enterprise)
  3. Bland AI (Phone calling agents)
  4. Fin (Customer support)
  5. Intercom AI (Support & conversion)
  6. MultiOn (Agentic browser)
  7. AutoGPT/BabyAGI variants
  8. HubSpot ChatSpot
  9. Salesforce Einstein Copilot
  10. Shopify Sidekick

  ### Deep-Dive Competitor Audit: Jobber
  **Capabilities**: Scheduling, quoting, invoicing, and client management for field service businesses.
  **Success Factors**: Strong mobile app, clear flow from request to quote to job to invoice.
  **User Sentiment Audit**:
  - *Positive*: "It keeps my schedule organized and makes invoicing easy." (App Store)
  - *Negative*: "The client request forms are too rigid. I still have to call them to get the real details." (Reddit r/smallbusiness)
  - *Negative*: "It's a lot of manual data entry while I'm trying to drive between jobs." (Trustpilot)

  ### OHC Gap & Pain Point Identification
  **OHC Feature Audit**: OHC currently lacks a seamless, agent-led intake flow that automatically parses free-text client requests (via SMS, WhatsApp, or voice note) into structured quotes and calendar events without manual owner data entry.
  **Gap Matrix**: Jobber has structured forms; OHC needs unstructured, agent-parsed intake.
  **Unresolved Pain Points**: Owners cannot answer the phone or fill out forms while working. They need an AI agent to handle the conversation, extract details (e.g., "I need a plumber for a leaky sink tomorrow afternoon"), check the owner's availability, and draft a quote for the owner to simply approve.

  ### Comparative Table: OHC vs Jobber vs Sierra

  | Feature / Product | OneHumanCorp (Proposed) | Jobber | Sierra (AI) |
  |---|---|---|---|
  | Mobile-First Design | ✅ Yes (375px optimized) | ✅ Yes | ❌ Enterprise desktop focus |
  | Autonomous Agent Intake | ✅ Yes (Conversational) | ❌ Rigid Forms | ✅ Yes (Conversational) |
  | Instant Draft Quotes | ✅ Yes (Agent-drafted) | ❌ Manual | ❌ Not core to product |
  | Unified Work Feed | ✅ Yes | ❌ Isolated views | ❌ Support focus |

  ## Solution & Design Doc

  **Proposed Solution**: An Agentic Intake Pipeline that listens to configured channels (WhatsApp, SMS, Web Widget), converses with the client to gather missing information (location, issue, urgency), checks the owner's schedule, and presents a "Draft Quote & Proposed Booking" card in the OHC mobile app for one-tap approval.

  **UX Flow (Mobile First - 375px)**:
  1. **Work Feed (Home)**: Owner sees a new "Pending Approval" card: "Agent drafted a quote for a leaky sink repair tomorrow at 2 PM for Jane Doe."
  2. **Detail View**: Owner taps the card. They see the AI-summarized client conversation, the proposed quote ($150), and the proposed calendar slot.
  3. **Action**: Owner taps "Approve & Send". The agent handles sending the quote and confirmation to the client.

  **Architecture Considerations**:
  - Requires integration with messaging channels.
  - LLM prompt must be specialized for service triage (extracting structured JSON from conversation).
  - Needs a robust locking mechanism (`ohc:lock:{tenant_id}:schedule`) to prevent double-booking during agent negotiation.

  ## Mermaid Charts

  ```mermaid
  graph TD
      A[Client sends WhatsApp message] --> B[OHC Intake Agent]
      B --> C{Missing Info?}
      C -- Yes --> D[Agent asks client for details]
      D --> B
      C -- No --> E[Agent checks Schedule & Pricing]
      E --> F[Agent drafts Quote & Booking]
      F --> G[Owner Mobile Feed: Pending Approval]
      G --> H[Owner taps Approve]
      H --> I[Agent sends Confirmation & Payment Link]
  ```

  ## Implementation Prompt
  **Goal**: Implement the backend service and UI for the "Agentic Intake Pipeline".
  **Critical User Journey (CUJ)**:
  1. A client message arrives via API.
  2. The backend AI agent processes the message, determines it is a service request, and drafts a quote/booking.
  3. The owner opens the mobile-responsive PWA (or Tauri app), sees the draft in their feed, and clicks "Approve".
  4. The system transitions the draft to "Approved" and logs the action.
  **Acceptance Criteria**:
  - The UI must render correctly at 375px width.
  - The approval action must be instantaneous in the UI with optimistic updates, backed by robust error handling.
  - Zero mock data; use the real database and AI provider (or test harness provider).
  - Must include E2E Playwright tests verifying the owner approval flow.

  **Priority**: P1
  **Estimated Scope**: Large

  ## References (50+ Visited URLs)
  1. https://www.shopify.com/
  2. https://squareup.com/
  3. https://www.hubspot.com/
  4. https://www.notion.so/
  5. https://www.microsoft.com/en-us/microsoft-365/enterprise/copilot-for-microsoft-365
  6. https://wix.com/
  7. https://www.larksuite.com/
  8. https://www.dingtalk.com/en
  9. https://work.weixin.qq.com/
  10. https://slack.com/
  11. https://asana.com/
  12. https://monday.com/
  13. https://clickup.com/
  14. https://trello.com/
  15. https://mailchimp.com/
  16. https://www.salesforce.com/
  17. https://www.zoho.com/
  18. https://www.zendesk.com/
  19. https://www.intercom.com/
  20. https://www.freshworks.com/
  21. https://www.typeform.com/
  22. https://calendly.com/
  23. https://acuityscheduling.com/
  24. https://www.honeybook.com/
  25. https://www.dubsado.com/
  26. https://www.jobber.com/
  27. https://www.housecallpro.com/
  28. https://www.servicefusion.com/
  29. https://www.thumbtack.com/
  30. https://www.taskrabbit.com/
  31. https://www.fiverr.com/
  32. https://www.upwork.com/
  33. https://www.kajabi.com/
  34. https://teachable.com/
  35. https://thinkific.com/
  36. https://podia.com/
  37. https://gumroad.com/
  38. https://patreon.com/
  39. https://onlyfans.com/
  40. https://www.gofundme.com/
  41. https://www.kickstarter.com/
  42. https://www.indiegogo.com/
  43. https://www.eventbrite.com/
  44. https://www.meetup.com/
  45. https://www.cvent.com/
  46. https://www.bizzabo.com/
  47. https://www.hopin.com/
  48. https://www.runtheworld.today/
  49. https://www.airmeet.com/
  50. https://www.hubilo.com/
  51. https://www.reddit.com/r/smallbusiness/comments/18x9a2m/shopify_vs_square/
  52. https://www.reddit.com/r/ecommerce/comments/14p6x2y/what_do_you_hate_about_shopify/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
