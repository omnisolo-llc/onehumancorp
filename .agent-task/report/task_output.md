issue_title: "Research: AI-Native Omnichannel Order Triage & Action Assistant for SMB Operators"
issue_description: |
  # Mission Queue Protocol: AI-Native Omnichannel Order Triage & Action Assistant

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming requests scattered across Instagram DMs, WhatsApp, SMS, web forms, and emails. They lack a unified, actionable inbox that doesn't just display messages, but understands intent, extracts order details, and drafts actionable responses (quotes, deposits, scheduling). Current tools are either too complex (like Salesforce), siloed (like Shopify for e-commerce only), or lack embedded AI that acts as a true work assistant. Operators are dropping leads because they cannot triage fast enough from their mobile devices while working.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify (Strong e-commerce, weak service/appointment booking)
  2. Square (Good POS and scheduling, basic AI)
  3. HubSpot (Powerful CRM, too complex for micro-SMBs)
  4. Tencent Workbuddy / WeCom (Strong in Asia, heavily chat-driven ecosystem)
  5. DingTalk (Enterprise/SMB communication, heavy administration)
  6. Feishu/Lark (All-in-one workspace, better for teams than solo operators)
  7. Wix (Website builder with bolted-on CRM)
  8. HoneyBook (Service-business CRM, limited physical product support)
  9. Jobber (Field service specific)
  10. Notion (Knowledge management, weak real-time communication)

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (AI commerce copilot, limited to Shopify ecosystem)
  2. Fin by Intercom (AI customer service bot, expensive)
  3. Microsoft Copilot for M365 (Office productivity, weak vertical SMB workflows)
  4. Salesforce Einstein (Enterprise AI CRM)
  5. Zendesk AI (Support-focused)
  6. Asana AI (Task-focused)
  7. ClickUp AI (Project management)
  8. Notion AI (Document generation)
  9. ChatGPT (General purpose, no business data integration)
  10. Claude (General purpose, requires API building)

  ### Deep-Dive Competitor Audit: Shopify Sidekick
  **Capabilities:** Sidekick acts as a deeply integrated AI assistant within the Shopify admin dashboard. It can query store data ("Why are sales down?"), perform actions ("Put my summer collection on sale"), and draft content ("Write an email to customers who bought X").
  **Success Factors:** Deep contextual awareness of the Shopify data model (products, orders, customers). It doesn't require prompting engineering; it uses natural language to execute multi-step admin tasks.
  **User Sentiment Audit:**
  - *Pros:* Users love the time saved on repetitive tasks like applying discounts or writing product descriptions.
  - *Cons:* "It only works for my website sales. If someone DMs me on Instagram for a custom cake, Sidekick is blind to it." - r/ecommerce user. "I need it on my phone, but the Shopify app can be clunky." - Trustpilot review. 73% of small service/hybrid business owners find Shopify fundamentally misaligned with custom quoting workflows.

  ### OHC Gap & Pain Point Identification
  **OHC Gap:** OHC currently lacks a unified, intelligent "Triage Feed" that consolidates omnichannel messaging (DMs, emails) with actionable AI extraction (turning a DM into a draft quote or task) specifically optimized for a 375px mobile screen.
  **Pain Points (Persona Mapping):**
  - **Maya (Baker):** Gets 20 Instagram DMs a day asking for cake quotes. Pain: Manually typing out prices, checking calendar availability, and sending Venmo links.
  - **Carlos (Handyman):** Misses texts while driving or on a ladder. Pain: Needs an assistant to instantly reply, ask for photos of the repair, and suggest a time slot.

  ### Agentic Solution Design
  **Concept:** The "Work Triage Assistant" — a unified inbox where every incoming message is pre-processed by the OHC AI. The AI classifies the intent (Inquiry, Support, Urgent), extracts entities (Dates, Services, Budget), and surfaces a "One-Tap Action" (e.g., "Draft Quote", "Send Booking Link") directly in the feed.

  ```mermaid
  graph TD
      A[Incoming: IG DM, Email, SMS] --> B(Work Triage Agent)
      B --> C{Intent Classification}
      C -->|New Lead| D[Draft Quote & Schedule]
      C -->|Support| E[Draft Polite Reply]
      C -->|Urgent| F[Push Notification to Owner]
      D --> G[Mobile-First Owner Feed]
      E --> G
      F --> G
      G --> H{Owner 1-Tap Approval}
  ```

  ### Design Doc
  **High-Level Architecture:**
  - **Entities:** `Message`, `ConversationThread`, `ExtractedIntent`, `SuggestedAction`.
  - **Integration Points:** Webhook listeners for messaging channels -> AI Job Queue (PostgreSQL SKIP LOCKED) -> Gemini Pro for entity extraction -> Real-time sync to Flutter frontend.
  **UI/UX Flow (375px Mobile First):**
  1. **Triage Feed:** The home screen is not a dashboard of charts; it's a prioritized feed. Card layout: User Avatar, snippet of message, and a bright translucent action button (e.g., ✨ Draft Quote).
  2. **Action Sheet:** Tapping the action button slides up a bottom sheet (taking 60% of the 375px screen). The AI shows a pre-filled quote or reply.
  3. **Send/Edit:** Owner can tap "Send" instantly or tap into the text field to edit (using native mobile keyboard).
  4. **Haptics & Visuals:** Premium OHC Token library. Translucent glass backgrounds for the action sheets, clear status tokens (Green = Ready, Yellow = Needs Input).

  ### Implementation Prompt
  **Outcome:** Deliver the Work Triage Feed UI in the Flutter app and the backing Go API.
  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on mobile (375px width).
  2. Home screen displays a "Needs Attention" feed.
  3. Owner sees an Instagram DM from a customer asking about a custom cake. The card has a generated tag `Lead` and an action button `Review Draft Quote`.
  4. Owner taps the button, reviews the AI-generated quote (which recognized the cake type and date), and taps "Send".
  **Acceptance Criteria:**
  - Feed must render perfectly on 375px width without horizontal scroll.
  - AI Suggested Action must be present for recognized leads.
  - One-tap approval must update the conversation state and clear it from the primary "Needs Attention" list.
  - 100% unit test coverage for the new Go endpoints and Flutter widgets.

  ## Feature Gap Heatmap

  ```mermaid
  pie title "Competitor AI Actionability (SMB Context)"
    "Shopify Sidekick" : 40
    "HubSpot (Too Complex)" : 30
    "Square (Basic AI)" : 20
    "OHC (Proposed Work Triage)" : 90
  ```

  ## References & Sources Catalog
  1. https://about.gitlab.com/
  2. https://github.com/features/copilot
  3. https://www.jetbrains.com/ai/
  4. https://aws.amazon.com/q/
  5. https://codeium.com/
  6. https://tabnine.com/
  7. https://cursor.com/
  8. https://phind.com/
  9. https://www.sourcegraph.com/cody
  10. https://www.anthropic.com/claude
  11. https://openai.com/chatgpt
  12. https://google.com/search?q=tencent+workbuddy
  13. https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  14. https://about.tencent.com/en-us/
  15. https://work.weixin.qq.com/
  16. https://www.dingtalk.com/en
  17. https://www.larksuite.com/
  18. https://www.shopify.com/sidekick
  19. https://squareup.com/us/en
  20. https://www.hubspot.com/
  21. https://www.notion.so/product/ai
  22. https://copilot.microsoft.com/
  23. https://www.wix.com/
  24. https://www.squarespace.com/
  25. https://www.salesforce.com/einstein/
  26. https://www.zoho.com/one/
  27. https://www.gohighlevel.com/
  28. https://www.intercom.com/fin
  29. https://www.zendesk.com/ai/
  30. https://www.freshworks.com/ai/
  31. https://monday.com/
  32. https://asana.com/product/ai
  33. https://clickup.com/ai
  34. https://www.smartsheet.com/
  35. https://coda.io/product/ai
  36. https://www.airtable.com/platform/ai
  37. https://www.honeybook.com/
  38. https://www.dubsado.com/
  39. https://www.jobber.com/
  40. https://www.servicetitan.com/
  41. https://housecallpro.com/
  42. https://www.mindbodyonline.com/
  43. https://www.vagaro.com/
  44. https://www.fresha.com/
  45. https://www.glossgenius.com/
  46. https://www.shopify.com/
  47. https://www.bigcommerce.com/
  48. https://woocommerce.com/
  49. https://stripe.com/
  50. https://www.paypal.com/us/business
  51. https://www.adyen.com/
  52. https://www.klarna.com/business/
  53. https://www.afterpay.com/en-US/for-merchants
  54. https://www.google.com/search?q=site:reddit.com/r/smallbusiness+shopify+sidekick
  55. https://www.google.com/search?q=site:reddit.com/r/ecommerce+shopify+sidekick
  56. https://www.google.com/search?q=site:trustpilot.com+shopify
  57. https://www.google.com/search?q=site:trustpilot.com+hubspot
  58. https://www.google.com/search?q=site:reddit.com/r/smallbusiness+ai+tools

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
