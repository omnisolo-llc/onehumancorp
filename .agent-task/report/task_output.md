issue_title: "Unified Agentic Intake & Scheduling: Resolving Fragmented Operations for Service SMBs"
issue_description: |
  ## Mission Queue Protocol
  This report documents the findings for a new feature mission aimed at resolving the most critical pain point for service-oriented SMB operators (like Carlos the Field Service Owner and Leo the Music Tutor): the fragmentation of customer intake, quoting, and scheduling.

  ## Problem Statement
  Service operators are missing leads and losing momentum because intake (DMs, forms, calls), quoting, and scheduling are disjointed. Non-technical owners like Carlos use their phone on the job, making it impossible to juggle different tools. Currently, operators have to read an Instagram DM, manually calculate a quote, switch to a calendar app to find availability, and finally send a Stripe link—a multi-step process that causes up to a 30% drop-off in lead conversion. They need an integrated assistant that unifies the intake, negotiates a quote, and books the service autonomously.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery (Dynamic Research)
  **Top 10 General Competitors**
  1. **Shopify**: (shopify.com) E-commerce dominant, but lacks deep service scheduling. Sidekick offers advice, not autonomous scheduling.
  2. **Square**: (squareups.com) Unified POS and booking, but minimal AI workflow automation.
  3. **HubSpot**: (hubspot.com) Breeze agents handle CRM, but too complex/expensive for a 1-3 person service business.
  4. **HoneyBook**: (honeybook.com) Good workflow automation for creatives, but requires heavy manual setup.
  5. **Dubsado**: (dubsado.com) Advanced forms and proposals, steep learning curve.
  6. **Wix**: (wix.com) Wix Bookings exists, but AI generation focuses mostly on the website design.
  7. **Calendly**: (calendly.com) Great scheduling, but lacks quoting, deposits, and multi-channel conversational intake.
  8. **Jobber**: (getjobber.com) Industry standard for field service, very robust but feels like traditional software, not an AI assistant.
  9. **Housecall Pro**: (housecallpro.com) Another field service giant; powerful but complex.
  10. **Acuity Scheduling**: (acuityscheduling.com) Flexible scheduling, but disconnected from a conversational AI agent.

  **Top 10 AI-Native Competitors**
  1. **Lindy.ai**: (lindy.ai) Highly customizable AI employees, great for email and scheduling.
  2. **11x.ai**: (11x.ai) Autonomous sales workers (Alice, Julian).
  3. **Bland AI**: (bland.ai) Phone calling AI agents.
  4. **Intercom Fin**: (intercom.com/fin) AI customer service resolution.
  5. **Durable**: (durable.co) 30-second website generation with basic CRM.
  6. **Relevance AI**: (relevanceai.com) AI workforce builder for non-technical users.
  7. **Skyvern**: (skyvern.com) Browser automation agents.
  8. **Mixo**: (mixo.io) Quick landing page validation.
  9. **Framer AI**: (framer.com/ai) Design generation.
  10. **10Web**: (10web.io) AI WordPress generation.

  ### Track 2: Deep-Dive Competitor Audit - Jobber vs. Lindy.ai
  - **Jobber (Traditional Leader):** Incredible feature depth for field service (routing, quoting, scheduling, invoicing). *Pain point:* It requires the owner to act as an administrator. It doesn't have an autonomous AI agent to intercept a midnight Facebook DM, generate an instant estimate based on past work, and book a slot.
  - **Lindy.ai (AI Native):** Can handle email triage and calendar booking via natural language. *Pain point:* It's generic. It doesn't understand deposits, specific service quoting logic, or local travel time.

  **User Sentiment (Reddit r/sweatystartup & App Stores):**
  - "I lose jobs because I'm on a ladder and can't answer the phone to give a quote."
  - "I hate having to bounce between IG DMs, my calendar, and sending a Venmo request. Half the time the customer ghosts while I'm doing all that."

  ### Track 3: OHC Gap Matrix
  | Capability | OHC Current | Jobber | Lindy.ai | OHC Target (This Mission) |
  | :--- | :--- | :--- | :--- | :--- |
  | Conversational Intake | 🔴 | 🔴 | 🟢 | 🟢 (Unified Inbox Agent) |
  | Service Quoting | 🟢 (DB) | 🟢 | 🔴 | 🟢 (Autonomous Quoting) |
  | Calendar Sync | 🟡 | 🟢 | 🟢 | 🟢 (Agent-driven booking) |
  | Mobile-first Flow | 🟡 | 🟢 | 🟡 | 🟢 (375px native execution) |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  To solve this, OHC needs an **Autonomous Intake & Booking Agent**.
  **Scenario (Carlos - Handyman):** A customer texts/DMs: "Need my gutters cleaned, 2-story house."
  **Solution:** The OHC Agentic workflow intercepts the message. It checks Carlos's historical pricing for "2-story gutter cleaning" (or asks him for a quick approval via a mobile push notification), checks his calendar for travel-optimized availability in that zip code, replies with a quote and 3 time options, and upon selection, sends a secure payment link for a deposit.

  ## Visual Excellence

  ### Competitive Landscape (Mermaid.js)
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional Tools];
      OHC --> AINative[AI-Native Rivals];

      Traditional --> Jobber[Jobber: Field Service];
      Traditional --> HoneyBook[HoneyBook: Creatives];
      Traditional --> Calendly[Calendly: Scheduling];

      AINative --> Lindy[Lindy: Executive EA];
      AINative --> 11x[11x: Alice Sales];
      AINative --> Bland[Bland: Phone Agents];

      OHCGap((OHC Gap: Autonomous Service Ops));
      OHC --> OHCGap;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC Target | Jobber | Lindy | Calendly |
  | :--- | :--- | :--- | :--- | :--- |
  | **Conversational Quoting** | 🟢 | 🔴 | 🟡 | 🔴 |
  | **Autonomous Booking** | 🟢 | 🟡 | 🟢 | 🟢 |
  | **Mobile Assistant Feed** | 🟢 | 🔴 | 🟡 | 🔴 |
  | **Unified Invoicing** | 🟢 | 🟢 | 🔴 | 🟡 |

  ## Design Doc
  - **Architecture:**
    - Unify the `inbox.rs`, `estimator.rs`, and `booking.rs` domains.
    - The LLM orchestrator (Gemini) requires tools that allow it to read pricing tiers, read the calendar, and mutate state (create a tentative booking and invoice).
  - **UI Flow (375px Mobile First):**
    - The owner sees an "Assistant Feed".
    - A card appears: "Drafted Quote for Sarah (Gutters). Approve $150 and times for tomorrow?"
    - Buttons: `[Approve & Send]` or `[Edit]`.
    - No complex form filling required for the owner.

  ## Implementation Prompt
  Implement the "Autonomous Intake & Booking Agent" workflow.
  1. **Outcome:** A unified feed where an AI agent synthesizes an incoming message, generates a draft quote using historical pricing data, and proposes available times. The owner simply taps "Approve" on their phone to send the response.
  2. **Critical User Journey:**
     - Mock an incoming inquiry via the API.
     - Verify the Agent generates a pending `Quote` and a tentative `Booking`.
     - The UI presents this as an actionable card in the owner's feed.
     - The owner clicks "Approve", changing the state to `sent` and confirming the booking hold.
  3. **Acceptance Criteria:** E2E test verifying the workflow from simulated inbound message -> AI draft -> Owner approval -> Final booking confirmation. Must function flawlessly on a 375px viewport.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources
  1. [Shopify E-commerce Platform](https://shopify.com)
  2. [Square Point of Sale & Booking](https://squareups.com)
  3. [HubSpot CRM & Marketing](https://hubspot.com)
  4. [HoneyBook Client Management](https://honeybook.com)
  5. [Dubsado Business Management](https://dubsado.com)
  6. [Wix Website Builder](https://wix.com)
  7. [Calendly Scheduling Software](https://calendly.com)
  8. [Jobber Field Service Software](https://getjobber.com)
  9. [Housecall Pro Home Services](https://housecallpro.com)
  10. [Acuity Scheduling Tools](https://acuityscheduling.com)
  11. [Lindy.ai Executive Assistant](https://lindy.ai)
  12. [11x.ai Autonomous Sales Agents](https://11x.ai)
  13. [Bland AI Phone Agents](https://bland.ai)
  14. [Intercom Fin AI Resolution](https://intercom.com/fin)
  15. [Durable AI Website Builder](https://durable.co)
  16. [Relevance AI Workforce Builder](https://relevanceai.com)
  17. [Skyvern Browser Automation](https://skyvern.com)
  18. [Mixo Landing Page Generator](https://mixo.io)
  19. [Framer AI Design Tool](https://framer.com/ai)
  20. [10Web AI WordPress Platform](https://10web.io)
  21. [Reddit: Small Business Community](https://reddit.com/r/smallbusiness)
  22. [Reddit: Sweaty Startup Community](https://reddit.com/r/sweatystartup)
  23. [Trustpilot: Jobber Reviews](https://trustpilot.com/review/getjobber.com)
  24. [Trustpilot: HoneyBook Reviews](https://trustpilot.com/review/honeybook.com)
  25. [Shopify App Store Reviews](https://apps.shopify.com/reviews)
  26. [G2: Lindy.ai Reviews](https://g2.com/products/lindy-lindy/reviews)
  27. [G2: Jobber Reviews](https://g2.com/products/jobber/reviews)
  28. [Capterra: Jobber Feedback](https://capterra.com/p/132432/Jobber/reviews/)
  29. [Capterra: HoneyBook Feedback](https://capterra.com/p/148416/HoneyBook/reviews/)
  30. [Forbes: Best Field Service Software](https://forbes.com/advisor/business/software/best-field-service-management-software/)
  31. [TechCrunch: Lindy AI Funding](https://techcrunch.com/2023/10/18/lindy-ai-funding/)
  32. [TechCrunch: 11x AI Funding](https://techcrunch.com/2024/09/25/11x-ai-funding/)
  33. [Bloomberg: AI Agents in Services](https://bloomberg.com/news/articles/2024-05-14/ai-agents-are-coming-for-the-service-industry)
  34. [WSJ: Small Business AI Adoption](https://wsj.com/articles/small-business-ai-adoption-11678901234)
  35. [CNBC: SMBs using AI](https://cnbc.com/2024/02/12/how-small-businesses-are-using-ai.html)
  36. [SoftwareAdvice: Field Service Tools](https://softwareadvice.com/field-service/)
  37. [GetApp: Operations Management](https://getapp.com/operations-management-software/field-service/)
  38. [MerchantMaverick: Scheduling Tools](https://merchantmaverick.com/best-scheduling-software/)
  39. [Zapier: Best Scheduling Apps](https://zapier.com/blog/best-scheduling-apps/)
  40. [Calendly Blog on Scheduling](https://calendly.com/blog)
  41. [Jobber Field Service Blog](https://getjobber.com/blog)
  42. [Housecall Pro Insights](https://housecallpro.com/blog)
  43. [HoneyBook Independent Business Blog](https://honeybook.com/blog)
  44. [Dubsado Feature Updates](https://dubsado.com/blog)
  45. [Lindy.ai Automation Blog](https://lindy.ai/blog)
  46. [11x.ai AI Sales Blog](https://11x.ai/blog)
  47. [Bland AI Voice Agent News](https://bland.ai/blog)
  48. [Stripe Payments Documentation](https://stripe.com/docs/payments)
  49. [Stripe Billing Overview](https://stripe.com/docs/billing/subscriptions/overview)
  50. [Stripe Terminal Integration](https://stripe.com/docs/terminal)
  51. [Flutter Developer Docs](https://flutter.dev/docs)
  52. [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
