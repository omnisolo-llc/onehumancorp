issue_title: "[Research] OHC Dynamic AI Conversational Form Engine (Agentic Intake)"
issue_description: |
  # OHC Dynamic AI Conversational Form Engine

  ## Problem Statement
  Small business owners and operators (like Carlos the handyman or Maya the baker) receive scattered requests across DMs, SMS, emails, and basic forms. Traditional static forms (e.g., Typeform, Google Forms) are rigid and impersonal, often leading to low conversion rates or incomplete information. Owners waste time going back and forth with potential clients to gather the necessary details for a quote or a booking. There is no intelligent, unified intake system that adapts to the customer's intent on the fly while instantly syncing with the owner's operations (CRM, quoting, scheduling).

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. **Shopify** (shopify.com)
  2. **Wix** (wix.com)
  3. **Squarespace** (squarespace.com)
  4. **Square** (squareup.com)
  5. **HubSpot** (hubspot.com)
  6. **WooCommerce** (woocommerce.com)
  7. **BigCommerce** (bigcommerce.com)
  8. **GoDaddy** (godaddy.com)
  9. **Weebly** (weebly.com)
  10. **PrestaShop** (prestashop.com)
  11. **Typeform** (typeform.com)
  12. **Calendly** (calendly.com)
  13. **Zendesk** (zendesk.com)
  14. **Freshworks** (freshworks.com)
  15. **Salesforce** (salesforce.com)

  **Top 10 AI-Native Competitors:**
  1. **Durable** (durable.co)
  2. **10Web** (10web.io)
  3. **Mixo** (mixo.io)
  4. **Framer AI** (framer.com/ai)
  5. **Lindy.ai** (lindy.ai)
  6. **Relevance AI** (relevanceai.com)
  7. **Skyvern** (skyvern.com)
  8. **Gorgias** (gorgias.com)
  9. **ClickUp AI** (clickup.com/ai)
  10. **Zapier AI** (zapier.com/ai)

  ### Track 2: Deep-Dive Competitor Audit - Typeform AI
  **Competitor:** Typeform AI (typeform.com/ai)
  **Capabilities:** Generates forms from natural language prompts, smart question logic, and conversational UI.
  **Success Factors:** Excellent UI/UX, smooth transitions, high completion rates compared to static forms.
  **User Sentiment:** Users love the aesthetics but complain about the steep pricing and lack of deep, native integration into operational workflows (it's mostly an intake tool, not an execution engine).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Gap:** OHC currently lacks an intelligent, conversational, and adaptive intake mechanism. We rely on standard static inputs or disconnected chat.
  **Pain Points:** Owners still manually follow up for missing information. Customers drop off if the initial intake feels too rigid or irrelevant.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Operators consistently cite "lead qualification and data collection" as a major time sink. A handyman needs specific photos and dimensions; a baker needs dietary restrictions and dates.
  **Agentic Solution:** **OHC Dynamic Conversational Form Engine (Agentic Intake)**. Instead of a static form, OHC provides an AI-driven, chat-like intake experience that starts with a simple prompt ("What do you need help with?") and dynamically asks follow-up questions based on the owner's predefined business logic (e.g., if cake, ask flavor; if repair, ask for photo).

  ### Comparison Chart
  ```mermaid
  graph TD
      A[Customer Intent] --> B{Static Form vs AI Intake}
      B -->|Static Form| C[Rigid Questions]
      B -->|AI Intake| D[Adaptive Conversation]
      C --> E[Incomplete Data / Drop-off]
      D --> F[Complete Context & Trust]
      E --> G[Owner Manual Follow-up]
      F --> H[Instant Quote/Booking Draft]
  ```

  ## Design Doc
  - **Entity:** `AgenticIntakeFlow`, `IntakeSession`, `IntakeMessage`.
  - **Architecture:** The Agentic Intake Engine (LLM-powered) receives customer input, evaluates it against the owner's required data fields (e.g., `service_type`, `date`, `budget`), and generates the next conversational prompt to gather missing info. Once complete, it creates an `Opportunity` or `Task` in OHC.
  - **Mobile UX (375px):** A clean, chat-style interface (similar to an iMessage thread or a clean Typeform). Large input areas, easy photo upload buttons.
  - **AI Integration:** Uses Gemini Pro to interpret intent and extract structured data from unstructured user responses.

  ## Implementation Prompt
  Implement the Dynamic Agentic Intake flow. Build the backend state machine that handles a conversational intake session, extracting structured data (like dates, preferences, and photos) using an LLM. Create the mobile-first (375px) chat-like UI where customers interact with the intake agent. Ensure that once the required fields are gathered, the system transitions the session to a completed state and generates a summary task for the owner. Provide full Playwright E2E coverage for the customer intake journey.

  ## Priority
  P1

  ## Estimated Scope
  Large

  ## References & Sources
  1. https://shopify.com
  2. https://wix.com
  3. https://squarespace.com
  4. https://squareup.com
  5. https://hubspot.com
  6. https://woocommerce.com
  7. https://bigcommerce.com
  8. https://godaddy.com
  9. https://weebly.com
  10. https://prestashop.com
  11. https://typeform.com
  12. https://calendly.com
  13. https://zendesk.com
  14. https://freshworks.com
  15. https://salesforce.com
  16. https://durable.co
  17. https://10web.io
  18. https://mixo.io
  19. https://framer.com/ai
  20. https://lindy.ai
  21. https://relevanceai.com
  22. https://skyvern.com
  23. https://gorgias.com
  24. https://clickup.com/ai
  25. https://zapier.com/ai
  26. https://notion.so/product/ai
  27. https://slack.com/features/ai
  28. https://asana.com/product/ai
  29. https://monday.com/platform/ai
  30. https://make.com
  31. https://airtable.com/platform/ai
  32. https://coda.io/product/ai
  33. https://basecamp.com
  34. https://trello.com
  35. https://zoho.com/zia
  36. https://intercom.com/fin
  37. https://klaviyo.com
  38. https://mailchimp.com/features/ai/
  39. https://omnisend.com
  40. https://attentive.com
  41. https://yotpo.com
  42. https://gorgias.com/product/ai
  43. https://typeform.com/ai/
  44. https://acuityscheduling.com
  45. https://honeybook.com
  46. https://dubsado.com
  47. https://thryv.com
  48. https://jobber.com
  49. https://housecallpro.com
  50. https://servicefusion.com
  51. https://servicetitan.com
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
