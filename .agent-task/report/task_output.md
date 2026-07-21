issue_title: "Mission: Agentic Operations Assistant for Unified Inbox and Booking Recovery"
issue_description: |
  # OHC Market Research & Mission Brief: Agentic Operations Assistant for Unified Inbox and Booking Recovery

  ## 1. Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service) are overwhelmed by fragmented communication channels (Instagram DMs, WhatsApp, SMS, Web Forms) and disjointed scheduling systems. Traditional tools like Shopify, Square, or HoneyBook force them into complex setup workflows, admin portals, and manual triage. When they are busy executing their craft, they miss leads, fail to collect deposits, and lose context on customer preferences. This leads to lost revenue, degraded customer experience, and operator burnout. OHC needs to bridge this gap by acting as a proactive, unified assistant that autonomously captures intent, drafts responses, and orchestrates bookings across channels.

  ## 2. Research Report
  ### Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  **Top 10 General Competitors:**
  1. **Shopify**: Dominant in e-commerce, but overwhelming for service-based or hybrid operators.
  2. **Square**: Strong POS and booking, but lacks deep conversational AI workflows.
  3. **HoneyBook**: Excellent for independent professionals, but highly manual pipeline management.
  4. **Tencent Workbuddy / WeCom**: Powerful enterprise/SMB chat operations, less suited for global small business non-technical owners.
  5. **DingTalk**: Robust operations, but heavy administrative overhead.
  6. **Feishu / Lark**: Great for internal collaboration, less focused on external B2C conversational commerce.
  7. **HubSpot**: Powerful CRM, but too complex and expensive for micro-businesses.
  8. **GlossGenius**: Incredible mobile-first UX for beauty, but vertically constrained.
  9. **Jobber**: Strong field service dispatch, but lacks generative AI conversational triage.
  10. **Wix**: General purpose builder, but feels like an admin portal rather than an assistant.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick**: AI assistant for merchants, mostly focused on store admin and analytics.
  2. **Intercom Fin**: AI customer service agent, excellent for support but lacks operational execution (booking/quoting).
  3. **Gorgias AI**: E-commerce focused support automation.
  4. **Stripe AI**: AI for payments and fraud, lacks customer-facing scheduling workflows.
  5. **Notion AI**: Great for knowledge, lacks external customer operation workflows.
  6. **Microsoft Copilot**: General productivity, not tuned for SMB field/commerce workflows.
  7. **Salesforce Einstein**: Enterprise CRM AI, misaligned for 1-person operators.
  8. **Zendesk AI**: Ticketing-focused, not revenue-generation focused.
  9. **ClickUp Brain**: Project management AI, no customer-facing capabilities.
  10. **Klaviyo AI**: Marketing automation, lacks real-time conversational booking.

  ### Track 2: Deep-Dive Competitor Audit - HoneyBook
  **Capabilities ("What they can do"):**
  - Project pipelines, invoicing, contract signing, and scheduling.
  - Automations (smart files) that trigger based on pipeline stage.
  - Client portal for communication and payment.

  **Success Factors ("What they are successful at"):**
  - **All-in-one workflow**: Consolidating the quote-to-cash process for freelancers.
  - **Professionalism**: Makes solo operators look like established agencies.
  - **Delightful onboarding**: High-touch guidance to set up templates.

  **User Sentiment Audit (Reddit, Trustpilot, App Store):**
  - *Positive:* "Saves me 10 hours a week not chasing invoices." (Source: Trustpilot)
  - *Negative:* "The mobile app is severely lacking compared to desktop. I can't easily edit smart files on my phone." (Source: r/freelance)
  - *Negative:* "I still have to manually move clients through stages, if I forget, they don't get the follow-up." (Source: r/smallbusiness)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:**
  - OHC currently has a foundation for tenant isolation, backend AI job queues, and basic Flutter UI shell.

  **Gap Matrix (OHC vs HoneyBook vs Shopify):**
  | Feature | HoneyBook | Shopify | OHC (Current) | OHC (Target) |
  |---------|-----------|---------|---------------|--------------|
  | Mobile-first quote editing | Poor | Average | Missing | **Excellent (375px native)** |
  | Conversational Lead Capture | Manual form | Chatbot | Missing | **Agentic unified inbox** |
  | Automated Follow-up | Rule-based | App-based | Missing | **Context-aware LLM drafts** |

  **Unresolved Pain Points:**
  - *Maya (Baker)*: "I have 5 unread DMs asking for quotes. If I don't reply in an hour, they go elsewhere."
  - *Carlos (Field Service)*: "I'm on a ladder, I can't type out a quote on my phone. I need to just hit 'approve'."

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering:**
  Small business owners consistently cite "communication overhead" as their #1 bottleneck. The inability to rapidly transition from a chat/DM to a structured quote/booking on a mobile device leads to a 30%+ drop-off in lead conversion (Sources: Reddit r/sweatystartup, r/smallbusiness discussions).

  **Agentic Solution Design:**
  - **The "Work Triage" Agent**: Continuously monitors incoming channels (mocked as unified inbox items for now). When a DM arrives ("How much for a custom cake next Tuesday?"), the agent:
    1. Parses intent (Quote Request + Date).
    2. Checks Maya's availability calendar.
    3. Drafts a response: "Hi! I have availability on Tuesday. A custom cake starts at $50. Should I send over a deposit link?"
    4. Presents the draft to Maya in a 375px mobile UI with a single tap "Approve & Send" button.

  ### Visual & Strategic Assets

  #### Competitive Landscape (Mermaid)
  ```mermaid
  quadrantChart
      title Market Positioning: Automation vs Mobile-First Simplicity
      x-axis "Heavy Admin Portal" --> "Assistant-First Mobile UI"
      y-axis "Rule-based Workflows" --> "Agentic Generative Actions"
      quadrant-1 "Visionary Leaders"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Traditional SMB Tools"
      quadrant-4 "Consumer/Light Tools"
      "Shopify": [0.3, 0.4]
      "HoneyBook": [0.4, 0.3]
      "HubSpot": [0.1, 0.5]
      "WeCom": [0.6, 0.6]
      "GlossGenius": [0.7, 0.2]
      "OHC (Target)": [0.9, 0.9]
  ```

  #### User Journey Comparison
  ```mermaid
  journey
      title Capturing a Lead and Sending a Quote
      section Traditional Tool (e.g., HoneyBook)
        Receive DM: 3: User
        Switch to App: 2: User
        Create new project: 2: User
        Type client details: 1: User
        Build quote on phone: 1: User
        Send link: 3: User
      section OHC Agentic Flow
        Receive DM: 3: User
        Notification with Draft: 5: Agent
        Tap 'Approve & Send Quote': 5: User
  ```

  ## 3. Design Doc
  **High-Level Architecture:**
  - `InboxItem`: Entity representing an incoming message across channels.
  - `AgentDraft`: Entity representing the AI-proposed response or action (e.g., Send Quote, Schedule Visit).
  - **Integration Points**: Backend AI Job Queue (PostgreSQL `SKIP LOCKED`) processes `InboxItem`s and generates `AgentDraft`s via Gemini Pro.
  - **Mobile UX Flow (375px first)**:
    - **Screen 1: Triage Feed**: A vertically scrolling list of cards. Each card shows the customer's message, context (e.g., "New Customer", "Past Spender"), and a prominent translucent glass AI Draft button.
    - **Screen 2: Draft Review**: Tapping a card opens a bottom sheet. Shows the exact drafted text and the attached action (e.g., Payment Link for $50). Native keyboard available for quick edits.
    - **Action**: One-tap "Approve". Uses optimistic UI updates to instantly clear the item from the feed.

  ## 4. Implementation Prompt
  **Critical User Journey (CUJ):**
  1. The operator (e.g., Maya) opens the OHC mobile app (375px view).
  2. The home screen "Work Triage" feed shows a new inquiry from a customer asking for a booking/quote.
  3. Attached to the inquiry is an AI-generated draft reply and a prepared action (e.g., a quote/deposit request).
  4. Maya reviews the draft, makes zero edits, and taps "Approve & Send".
  5. The inquiry is marked as resolved and moves out of the active triage feed, visually updating the day's priority list.

  **Acceptance Criteria:**
  - Build the "Work Triage" UI components using the OHC Premium Token library (translucent materials, clear typography).
  - The UI MUST be fully responsive, perfectly functioning on a 375px width without horizontal scroll.
  - Implement a mock backend/local state to simulate the arrival of a new `InboxItem` and its corresponding `AgentDraft` to demonstrate the flow end-to-end in Playwright.
  - E2E Playwright test must navigate the feed, click the inquiry, review the draft, click "Approve", and verify the item is removed from the active feed.
  - ZERO hardcoded demo data in the final production UI state (mock data only injected via test setup/seed paths).

  ## References & Sources
  1. https://www.wecom.qq.com/
  2. https://www.dingtalk.com/en
  3. https://www.larksuite.com/
  4. https://www.shopify.com/magic
  5. https://squareup.com/us/en/townsquare/square-artificial-intelligence
  6. https://www.hubspot.com/artificial-intelligence
  7. https://www.notion.so/product/ai
  8. https://copilot.microsoft.com/
  9. https://www.honeybook.com/
  10. https://getjobber.com/
  11. https://www.housecallpro.com/
  12. https://glossgenius.com/
  13. https://www.wix.com/studio/ai
  14. https://www.salesforce.com/einstein/
  15. https://zoho.com/zia/
  16. https://clickup.com/ai
  17. https://monday.com/ai
  18. https://asana.com/product/ai
  19. https://www.intercom.com/fin
  20. https://www.gorgias.com/product/ai-support
  21. https://www.klaviyo.com/features/ai
  22. https://mailchimp.com/features/ai-marketing/
  23. https://stripe.com/newsroom/news/stripe-ai
  24. https://www.fresha.com/
  25. https://www.mindbodyonline.com/
  26. https://www.vagaro.com/
  27. https://www.thryv.com/
  28. https://www.podium.com/
  29. https://www.birdeye.com/
  30. https://www.zendesk.com/ai/
  31. https://www.freshworks.com/ai/
  32. https://www.typeform.com/ai/
  33. https://www.calendly.com/
  34. https://acuityscheduling.com/
  35. https://simplybook.me/
  36. https://setmore.com/
  37. https://www.xero.com/
  38. https://www.quickbooks.intuit.com/
  39. https://www.waveapps.com/
  40. https://www.gusto.com/
  41. https://www.rippling.com/
  42. https://www.deel.com/
  43. https://www.brex.com/
  44. https://www.ramp.com/
  45. https://www.bill.com/
  46. https://www.expensify.com/
  47. https://www.toasttab.com/
  48. https://www.lightspeedhq.com/
  49. https://www.clover.com/
  50. https://www.touchbistro.com/
  51. Reddit r/smallbusiness - "HoneyBook vs Dubsado for independent operators"
  52. Reddit r/ecommerce - "Shopify is too complex for my 3-product bakery"
  53. Trustpilot HoneyBook Reviews
  54. Trustpilot Jobber Reviews
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
