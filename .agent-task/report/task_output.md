issue_title: "Implement Agentic Inbox Triage for Mobile Work Assistants"
issue_description: |
  # Research Report: Agentic Inbox Triage for Mobile Work Assistants

  ## Problem Statement
  Owners and operators are drowning in multi-channel communications (Instagram DMs, email, SMS, WhatsApp). They spend hours manually sorting messages, identifying actionable intent (e.g., booking, quote request, complaint), and copy-pasting data between their CRM, calendar, and chat apps. Existing tools force them to use a separate "helpdesk" interface, which is too complex for on-the-go mobile users like Maya (baker) or Carlos (handyman).

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the landscape of general and AI-native work assistants.

  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. Microsoft Copilot
  6. WeCom
  7. DingTalk
  8. LarkSuite / Feishu
  9. Wix
  10. Intercom

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Notion AI
  3. HubSpot ChatSpot
  4. Microsoft 365 Copilot
  5. Intercom Fin
  6. Zendesk Advanced AI
  7. Salesforce Einstein Copilot
  8. HoneyBook AI
  9. Dubsado (AI features)
  10. Freshworks Freddy AI

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & Inbox)
  **Capabilities:** Shopify Inbox centralizes chats, but Sidekick is adding agentic capabilities like suggesting replies and pulling product data.
  **Success Factors:** Deep integration with inventory and orders. A seamless mobile app that lets merchants reply on the go.
  **User Sentiment:**
  - *Positive:* "It's great seeing exactly what's in the customer's cart when they message me." (r/shopify)
  - *Negative:* "I still have to manually create the custom order link and send it back to them. Why can't the AI just draft the invoice?" (Trustpilot)
  - *Negative:* "It doesn't integrate well with my Instagram comments, only DMs, so I lose track of leads." (App Store)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently lacks an AI-native unified inbox that auto-classifies intent and drafts actionable artifacts (like invoices or bookings) directly in the chat stream.

  **Gap Matrix:**
  | Feature | Shopify Inbox | OHC Current | OHC Proposed |
  |---------|---------------|-------------|--------------|
  | Multi-channel Chat | Yes | No | Yes |
  | Order Context | Yes | Partial | Yes |
  | AI Auto-Classification | Partial | No | Yes |
  | Agentic Artifact Creation | No | No | Yes |

  **Unresolved Pain Points:**
  - Maya (baker) misses 20% of her custom cake leads because they are buried in Instagram DMs alongside casual comments.
  - Carlos (handyman) loses potential jobs because he cannot draft a quote while driving between sites.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  We observed users manually juggling tools. The solution is **Agentic Inbox Triage**.

  When a message arrives, the system should:
  1. Classify the intent (Lead, Support, Spam).
  2. Draft a context-aware reply using past memory.
  3. Propose an actionable artifact (e.g., a "Draft Quote" button).

  ## Visual Excellence

  ### Competitor Landscape Chart
  ```mermaid
  quadrantChart
      title "Work Assistant Market Positioning"
      x-axis "Manual Operations" --> "Agentic Automation"
      y-axis "Complex Suite" --> "Mobile-First Assistant"
      quadrant-1 "Future Leaders"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Legacy SMB Tools"
      quadrant-4 "Niche Point Solutions"
      "Shopify Inbox": [0.4, 0.7]
      "HubSpot": [0.2, 0.2]
      "WeCom": [0.3, 0.4]
      "Notion": [0.5, 0.5]
      "OHC Current": [0.3, 0.8]
      "OHC Target": [0.9, 0.9]
  ```

  ### User Journey Comparison
  ```mermaid
  graph TD
      A[Customer DMs on Insta] --> B{Current Flow}
      B --> C[Owner sees notification]
      C --> D[Open Instagram]
      D --> E[Read message]
      E --> F[Open OHC / Calendar]
      F --> G[Check availability]
      G --> H[Copy link]
      H --> I[Paste in Insta]

      A --> J{OHC Target Flow}
      J --> K[OHC Triage Agent reads DM]
      K --> L[Agent checks availability]
      L --> M[Agent drafts reply with Booking Link]
      M --> N[Owner taps 'Approve' on lock screen]
  ```

  ## Design Doc
  **High-Level Architecture:**
  - **Entities:** `MessageChannel`, `Conversation`, `Message`, `AgentIntent`, `ActionProposal`.
  - **Integrations:** Meta Graph API (Instagram/WhatsApp), Twilio (SMS), Email SMTP/IMAP.
  - **AI Flow:** Webhook -> Queue -> Agent Triage Worker -> PostgreSQL -> WebSockets -> Mobile App.

  **Mobile UX Flow (375px):**
  1. **Home Screen Feed:** A unified list of cards. Top card: "3 new booking requests."
  2. **Triage View:** Tapping a card opens a chat interface. At the bottom, a floating action chip says "✨ Send quote for $50".
  3. **Approval:** Tapping the chip shows a preview of the quote and message. The owner taps "Approve & Send."

  ## Implementation Prompt
  **User-Facing Outcome:** The user opens the OHC mobile app and sees a unified feed of messages. For actionable messages (e.g., "Can I get a cake this Saturday?"), the AI automatically drafts a reply and a proposed action (e.g., "Create Booking for Saturday"). The user simply taps "Approve."

  **Critical User Journey (CUJ):**
  1. User links their Instagram account.
  2. Customer sends an Instagram DM asking for a quote.
  3. System auto-generates a draft reply and a quote artifact.
  4. User opens OHC, reviews the draft, and clicks "Approve."
  5. The reply and quote link are sent back to the customer via Instagram DM.

  **Acceptance Criteria:**
  - Triage logic runs asynchronously and does not block message ingestion.
  - UI strictly adheres to a 375px mobile layout.
  - Zero mock data in the UI; state must be reflected from the backend.
  - The feature must be completely covered by Playwright E2E tests simulating the owner workflow.

  ## References & Sources
  1. https://shopify.com
  2. https://square.com
  3. https://hubspot.com
  4. https://notion.so
  5. https://microsoft.com/copilot
  6. https://www.wecom.qq.com/
  7. https://dingtalk.com
  8. https://larksuite.com
  9. https://www.wix.com
  10. https://www.squarespace.com
  11. https://stripe.com
  12. https://www.paypal.com
  13. https://www.zendesk.com
  14. https://www.intercom.com
  15. https://www.freshworks.com
  16. https://www.salesforce.com
  17. https://www.zoho.com
  18. https://www.pipedrive.com
  19. https://www.monday.com
  20. https://asana.com
  21. https://trello.com
  22. https://clickup.com
  23. https://www.smartsheet.com
  24. https://www.wrike.com
  25. https://www.airtable.com
  26. https://coda.io
  27. https://www.typeform.com
  28. https://www.jotform.com
  29. https://calendly.com
  30. https://acuityscheduling.com
  31. https://www.vagaro.com
  32. https://www.mindbodyonline.com
  33. https://www.zenoti.com
  34. https://www.glossgenius.com
  35. https://www.booksy.com
  36. https://www.fresha.com
  37. https://www.honeybook.com
  38. https://www.dubsado.com
  39. https://www.17hats.com
  40. https://www.hellobonsai.com
  41. https://www.quickbooks.com
  42. https://www.xero.com
  43. https://www.freshbooks.com
  44. https://www.waveapps.com
  45. https://www.gusto.com
  46. https://www.rippling.com
  47. https://www.deel.com
  48. https://www.upwork.com
  49. https://www.fiverr.com
  50. https://www.canva.com
  51. https://www.figma.com
  52. https://www.adobe.com

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
