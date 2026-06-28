issue_title: "Agentic Unified Inbox & Work Feed: Research & Implementation Plan"
issue_description: |
  # Agentic Unified Inbox & Work Feed: Market Research & Deep Dive

  ## Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service) are overwhelmed by scattered communication channels. They juggle Instagram DMs, WhatsApp, email, SMS, and web forms. Managing this fragmented communication leads to dropped leads, missed context, slow response times, and cognitive overload. The current OHC system handles some elements of operations (booking, quoting), but fails to provide a unified "command center" that not only aggregates messages but actively drafts replies, resolves intents, and creates an actionable feed.

  ## Research Report
  Our competitive research highlights a significant gap in the market between traditional dashboards and true AI assistants:

  ### Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Shopify Sidekick
  2. Wix Studio AI
  3. Squarespace Blueprint
  4. Square AI
  5. HubSpot Breeze
  6. WooCommerce AI
  7. BigCommerce AI
  8. GoDaddy Airo
  9. Weebly
  10. PrestaShop

  **Top 10 AI-Native Competitors:**
  1. Durable
  2. 10web.io
  3. Mixo
  4. Framer AI
  5. Lindy.ai
  6. Relevance AI
  7. Skyvern
  8. 11x.ai
  9. Intercom Fin
  10. AGI (On-Device)

  ### Track 2: Deep-Dive Competitor Audit (HubSpot Breeze & Lindy.ai)
  - **Capabilities:** Lindy.ai and HubSpot Breeze act as true AI executive assistants, handling email triage and drafting replies with deep contextual awareness of CRM data.
  - **Success Factors:** A primary success factor for these platforms is "Vibe Coding" / conversational interfaces that hide technical complexity behind natural language.
  - **User Sentiment Audit:** Users express immense relief at having unified, auto-drafted responses. E.g., *"I love that my AI knows past conversations and drafts a polite refusal for out-of-scope work."* (Reddit r/smallbusiness). Conversely, users complain when tools fail to integrate with their specific channels (e.g., missing Instagram DM support).

  ### Track 3 & 4: OHC Gap Matrix & Agentic Solution
  Currently, OHC lacks a conversational feed. We need to implement an **Agentic Unified Inbox**.

  **Pain Point Analysis:** Maya receives a custom cake inquiry via Instagram DM. She has to switch from her kitchen prep to her phone, open Instagram, check her availability on a separate calendar, cross-reference pricing in a notebook, and manually reply.

  **Agentic Solution Design:** The **Agent Feed** intercepts the Instagram DM. The intent is classified by an LLM. It queries Maya's inventory and calendar. It drafts a response and pushes an Action Card to Maya's OHC app. Maya taps "Approve."

  ---

  ## Design Doc

  ### Architecture
  - **Core Entities:** `Message`, `Thread`, `ActionCard`, `DraftIntent`, `AgentRule`.
  - **Integration Points:** Instagram Graph API, Twilio (SMS), SendGrid (Email).
  - **UI/UX:** A Mobile-First (375px) "Work Feed." It replaces a traditional dashboard with a chronological feed of Action Cards. Each card contains context, a drafted reply, and primary actions (Approve, Edit, Discard). It employs translucent glass styling and clear typographic hierarchy.

  ### Mermaid.js Charts

  **Competitive Landscape**
  ```mermaid
  graph TD;
      OHC[OHC: Agentic Assistant] --> Traditional[Traditional CRM/Inbox];
      OHC --> AINative[AI-Native Assistants];

      Traditional --> HubSpot[HubSpot Breeze];
      Traditional --> Zendesk[Zendesk AI];

      AINative --> Lindy[Lindy.ai];
      AINative --> Intercom[Intercom Fin];

      OHCGap((OHC Gap: Unified Actionable Feed));
      OHC --> OHCGap;
  ```

  **Unified Inbox Workflow**
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Agent
      participant OwnerApp

      Customer->>OHC_Agent: Instagram DM: "Need vegan cake for Sat"
      OHC_Agent->>OHC_Agent: Classify Intent (Order Inquiry)
      OHC_Agent->>OHC_Agent: Query Inventory & Schedule
      OHC_Agent->>OwnerApp: Push Action Card (Draft Reply)
      OwnerApp-->>OHC_Agent: Owner taps "Approve"
      OHC_Agent->>Customer: Reply sent via Instagram
  ```

  **Feature Gap Heatmap**
  | Capability | OHC | HubSpot | Lindy | Zendesk |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Multi-Channel** | 🟡 | 🟢 | 🟡 | 🟢 |
  | **Auto-Draft Replies** | 🔴 | 🟢 | 🟢 | 🟡 |
  | **Inventory Aware** | 🟢 | 🔴 | 🔴 | 🔴 |
  | **Action-Card Feed** | 🔴 | 🔴 | 🟡 | 🔴 |

  ---

  ## Implementation Prompt
  **User-Facing Outcome:** When Maya opens the OHC app, the first screen is the "Work Feed". She sees a card stating: "New Inquiry from Sarah (Insta DM) for a Vegan Cake this Saturday. We have capacity." Below it is a drafted reply. Maya taps "Approve" to send it instantly.

  **Critical User Journey (CUJ):**
  1. User navigates to the Work Feed screen.
  2. User views an Action Card containing a customer message and an AI-drafted reply.
  3. User taps "Approve."
  4. System confirms the message was sent and removes/archives the Action Card from the feed.

  **Acceptance Criteria:**
  - The UI must render perfectly at 375px wide.
  - The feed must display empty states truthfully.
  - Approve/Edit/Discard buttons must be functional and trigger the respective state transitions.
  - Zero mock data in the UI; data must be fetched from backend APIs.

  ---

  ## Priority: P0
  ## Estimated Scope: Large

  ---

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.wix.com/ai-website-builder
  4. https://durable.co/
  5. https://www.10web.io/
  6. https://mixo.io/
  7. https://www.framer.com/ai/
  8. https://www.hubspot.com/products/ai
  9. https://squareups.com/us/en/software/ai
  10. https://www.intercom.com/fin
  11. https://www.lindy.ai/
  12. https://relevanceai.com/
  13. https://skyvern.com/
  14. https://www.11x.ai/
  15. https://www.agi.app/
  16. https://www.honeybook.com/ai
  17. https://www.dubsado.com/features/automation
  18. https://www.squarespace.com/design/ai-website-builder
  19. https://www.godaddy.com/ai
  20. https://www.bigcommerce.com/solutions/ai/
  21. https://www.reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/
  22. https://www.reddit.com/r/ecommerce/comments/1993296205/wix_ai_vs_shopify/
  23. https://www.trustpilot.com/review/durable.co
  24. https://www.trustpilot.com/review/10web.io
  25. https://www.g2.com/products/lindy-lindy/reviews
  26. https://www.forbes.com/sites/shopify-vs-competition-ai-2025/
  27. https://techcrunch.com/2024/02/22/10web-armenia/
  28. https://www.searchenginejournal.com/10web-releases-api-for-scaled-white-label-ai-website-building/
  29. https://www.latimes.com/b2b/ai-technology/agi-snapdragon-partnership/
  30. https://www.tomsguide.com/phones/future-of-siri-agi-android-app/
  31. https://uk.finance.yahoo.com/news/qualcomm-says-agentic-ai-turn-devices-into-operators/
  32. https://www.investing.com/news/stock-market-news/qualcomm-agentic-ai-mwc/
  33. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  34. https://www.deeplearning.ai/short-courses/building-ai-browser-agents/
  35. https://www.nytimes.com/2025/12/02/technology/artificial-intelligence-amazon-gmail.html
  36. https://www.relevanceai.com/customers/canva
  37. https://www.relevanceai.com/customers/kpmg
  38. https://www.11x.ai/customers
  39. https://www.11x.ai/blog/digital-workers-revenue
  40. https://fin.ai/cx-models
  41. https://www.intercom.com/blog/ai-agent-blueprint/
  42. https://www.hubspot.com/spotlight
  43. https://www.hubspot.com/new
  44. https://www.wix.com/blog/how-does-ai-work
  45. https://www.wix.com/blog/best-ai-website-builder
  46. https://durable.com/ai-website-builder
  47. https://durable.com/blog/durable-vs-squarespace
  48. https://www.lindy.ai/integrations
  49. https://www.lindy.ai/security
  50. https://skyvern.com/healthcare
  51. https://www.theagi.company/blog
  52. https://www.theagi.company/media-features
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
