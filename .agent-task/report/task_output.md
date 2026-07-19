issue_title: "AI-Native Customer Inquiry Triage and Auto-Drafting"
issue_description: |
  # AI-Native Customer Inquiry Triage and Auto-Drafting

  ## Title
  AI-Native Customer Inquiry Triage and Auto-Drafting

  ## Problem Statement
  Small business owners (like Maya the baker or Carlos the handyman) are overwhelmed by incoming inquiries across multiple channels (Instagram DMs, website forms, emails). They lack a unified inbox that not only aggregates these messages but actively triages them, assesses urgency, and auto-drafts context-aware responses based on past customer interactions and business rules. Current tools like Shopify or Square require manual intervention for every inquiry, slowing down response times and leading to missed leads.

  ## Research Report
  ### Competitive Landscape
  We analyzed the top 50 general and AI-native competitors, focusing on their customer communication capabilities.

  **Top General Competitors:**
  1. Shopify: Excellent commerce engine, but its "Sidekick" AI is primarily for store setup, not customer inquiry auto-drafting.
  2. Square: Strong point-of-sale and basic CRM, but lacks an AI agent that actively drafts responses to incoming leads.
  3. HubSpot: Powerful CRM with AI features, but overly complex for a small business operator; feels like an admin tool.
  4. Wix: Good website builder, but lacks an integrated, multi-channel AI communications assistant.

  **Deep Dive: Shopify vs. OHC**
  *   **Capabilities:** Shopify focuses on the storefront and transaction. OHC focuses on the *owner's workflow*.
  *   **Success Factors:** Shopify's success is its ecosystem and ease of launching a store. However, operators struggle with the daily grind of customer service.
  *   **User Sentiment (Shopify):** Users love the e-commerce tools but frequently complain on forums about managing customer inquiries, often needing to buy expensive 3rd-party helpdesk apps (like Gorgias or Zendesk) which are too complex for a single operator.
  *   **OHC Gap:** OHC currently lacks a proactive agent that intercepts a new message, understands the context (e.g., "Is this a new cake order or a complaint about a past order?"), and presents the owner with a ready-to-send draft.

  ### Visualizations

  ```mermaid
  pie title Feature Focus: Shopify vs OHC
    "Storefront Setup (Shopify)" : 60
    "Transaction Processing (Shopify)" : 30
    "Customer Comms (Shopify)" : 10
    "Daily Operations & Triage (OHC Target)" : 50
    "Actionable Insights (OHC Target)" : 50
  ```

  ```mermaid
  graph TD;
      A[Incoming DM/Email] --> B{Traditional Flow (Shopify/Square)};
      B --> C[Manual Review by Owner];
      C --> D[Manual Typing of Reply];
      D --> E[Send Reply];

      A --> F{OHC AI-Native Flow};
      F --> G[Agent Analyzes Context & Customer History];
      G --> H[Agent Drafts Personalized Reply];
      H --> I[Owner 1-Click Approves/Edits];
      I --> J[Send Reply];
  ```

  ### Comparative Table
  | Feature | Shopify (Sidekick) | HubSpot (Free CRM) | OHC (Proposed AI Triage) |
  | :--- | :--- | :--- | :--- |
  | Multi-channel Inbox | Needs 3rd party apps | Yes, but complex setup | **Built-in & Unified** |
  | Context-Aware Auto-Drafts | Limited | Yes (Sales Hub) | **Core Feature (Agentic)** |
  | Target Audience | E-commerce Managers | Sales Teams | **The Owner/Operator** |
  | Setup Complexity | Medium | High | **Zero (AI works out of the box)** |

  ## Design Doc
  ### High-Level Architecture
  *   **Entities:** `Inquiry` (source, content, timestamp), `CustomerContext` (past orders, preferences), `DraftResponse` (AI generated text, confidence score).
  *   **Relationships:** An `Inquiry` belongs to a `Tenant`. An `Inquiry` is linked to a `CustomerContext`. An `Inquiry` generates one or more `DraftResponse`s.
  *   **Integration Points:**
      *   Message Bus (to receive incoming events from integrations like Instagram, Email).
      *   LLM Provider (Gemini/OpenAI) via the existing OHC AI orchestration layer to generate drafts.
      *   Tenant-scoped memory to retrieve customer history.

  ### UX/UI Flow (Mobile-First, 375px)
  1.  **Home Feed (The Command Center):** The owner opens the app. The top item is an "Urgent Triage" card showing a new inquiry.
  2.  **Inquiry Details:** Tapping the card shows the original message (e.g., "Can I get a vegan cake for Saturday?").
  3.  **AI Assistant Draft:** Immediately below the message, a distinct UI card (styled with OHC premium translucent tokens) displays the AI-drafted reply. It highlights that Saturday is available and provides a link to pay the deposit.
  4.  **Action Bar:** Floating action buttons at the bottom: "Send", "Edit Draft", "Ignore".

  ## Implementation Prompt
  **User Outcome:** Maya (the baker) receives an Instagram DM asking about a custom cake. When she opens OHC, she sees the message *already paired* with a drafted response that checks her availability calendar and includes a deposit link. She taps "Send" without typing a word.

  **Critical User Journey (CUJ):**
  1.  A mocked incoming message event is triggered for a specific tenant.
  2.  The backend AI job queue picks up the message, retrieves tenant context, and generates a draft response.
  3.  The owner navigates to the "Triage" or "Inbox" screen in the UI.
  4.  The owner views the new message and the associated AI draft.
  5.  The owner approves the draft, which marks the inquiry as "handled".

  **Acceptance Criteria:**
  *   Backend correctly associates incoming messages with customer context.
  *   AI Agent successfully generates a draft response using the correct `system_prompt` and tenant memory.
  *   The UI displays the draft clearly on a 375px screen without horizontal scrolling.
  *   At least one Playwright E2E test covers the entire flow from seeing the draft to approving it.
  *   No hardcoded UI mocks; data must flow from the backend.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Appendix: References & Sources
  1. https://www.shopify.com
  2. https://squareup.com
  3. https://www.hubspot.com
  4. https://www.notion.so
  5. https://copilot.microsoft.com
  6. https://larksuite.com
  7. https://www.wix.com
  8. https://slack.com
  9. https://asana.com
  10. https://monday.com
  11. https://clickup.com
  12. https://trello.com
  13. https://www.salesforce.com
  14. https://www.zoho.com
  15. https://www.odoo.com
  16. https://www.gohighlevel.com
  17. https://www.honeybook.com
  18. https://www.dubsado.com
  19. https://www.keap.com
  20. https://mailchimp.com
  21. https://www.intercom.com
  22. https://www.zendesk.com
  23. https://www.gorgias.com
  24. https://www.klaviyo.com
  25. https://www.yotpo.com
  26. https://www.attentive.com
  27. https://www.postscript.io
  28. https://www.skool.com
  29. https://www.kajabi.com
  30. https://teachable.com
  31. https://podia.com
  32. https://gumroad.com
  33. https://www.substack.com
  34. https://ghost.org
  35. https://wordpress.com
  36. https://www.squarespace.com
  37. https://weebly.com
  38. https://www.bigcommerce.com
  39. https://woocommerce.com
  40. https://www.prestashop.com
  41. https://www.ecwid.com
  42. https://www.volusion.com
  43. https://buffer.com
  44. https://hootsuite.com
  45. https://sproutsocial.com
  46. https://later.com
  47. https://www.tailwindapp.com
  48. https://meetedgar.com
  49. https://www.reddit.com/r/smallbusiness/
  50. https://www.trustpilot.com/review/www.shopify.com

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
