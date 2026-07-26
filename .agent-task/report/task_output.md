issue_title: "OHC Mission: Build a Native Rust Omnichannel Customer & AI Agent System to Replace External Dependencies"
issue_description: |
  # OHC Native Rust Omnichannel Customer Assistant Mission

  ## Mission Output Contract
  **Mission:** Develop OHC's custom omnichannel AI chat and operations engine natively in Rust (within `onehumancorp/mono`), retiring Chatwoot entirely, and differentiating from enterprise heavyweights (like Shopify Sidekick, Intercom Fin, MS Copilot, and Zoho Zia) by prioritizing non-technical owner clarity and mobile-first operations.

  ## 1. Problem Statement
  Small business owners and operators (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented tools. They use one tool for DMs, another for bookings, another for invoicing, and struggle to manage everything from their 375px mobile screens. Current enterprise AI solutions (like Intercom Fin or MS 365 Copilot) feel too much like administrative portals—they are complex, expensive, and require significant setup. External dependencies like Chatwoot add infrastructure overhead and fail to deliver the seamless "assistant-first" experience OHC promises. Owners need one unified assistant that understands their business and takes action across all channels natively.

  ## 2. Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the current landscape across 50+ visited pages:
  - **General Competitors (Top 10):** WeCom, DingTalk, Shopify, Square, HubSpot, Notion, Microsoft Copilot, Zendesk, Freshdesk, Zoho.
  - **AI-Native Competitors (Top 10):** Intercom Fin, Shopify Sidekick (Magic), Sierra AI, Chatwoot Captain, Zoho Zia, Notion AI Agents, Dynamics 365 Copilot, Einstein (Salesforce), Kustomer Freddy, Gorgias AI.

  ### Track 2: Deep-Dive Competitor Audit - Intercom Fin & Chatwoot
  - **Intercom Fin:** Highly advanced AI customer service agent running on custom Apex models. Excellent at complex multi-step procedures and handing off to human agents. However, its pricing ($29/seat + $0.99 per resolution) and enterprise-heavy interface make it inaccessible and overly complex for small SMBs (like Fatima's food cart).
  - **Chatwoot (Source Code Audit):** We performed a deep dive into the Chatwoot GitHub repository (Ruby on Rails backend, Vue.js frontend). It offers excellent omnichannel support (WhatsApp, Messenger, IG, Email, SMS, Telegram) and recently launched Captain AI. However, relying on it as a 3rd party dependency creates tenant isolation risks and breaks OHC's unified assistant shell experience.

  ### Track 3: OHC Gap & Pain Point Identification
  - **Missing Native Omnichannel:** OHC currently lacks native WhatsApp, Instagram, and SMS webhook ingestion built directly into our Rust gRPC/PostgreSQL backend.
  - **Missing "Agentic Shell" Integration:** Current tools require the user to switch context to an inbox. OHC needs triage directly in the mobile-first "assistant shell".

  ## 3. Visual Comparisons & Mermaid Charts

  ### Dynamic Competitive Landscape
  ```mermaid
  quadrantChart
      title AI Assistant Complexity vs. SMB Focus
      x-axis "Enterprise Admin Focus" --> "SMB Owner/Operator Focus"
      y-axis "Traditional Tool Suite" --> "Agentic & Autonomous"
      quadrant-1 "Target OHC Position"
      quadrant-2 "Heavy Enterprise AI"
      quadrant-3 "Legacy CRM"
      quadrant-4 "Basic SMB Tools"
      "Intercom Fin": [0.2, 0.9]
      "Shopify Sidekick": [0.6, 0.8]
      "MS 365 Copilot": [0.1, 0.85]
      "WeCom": [0.7, 0.3]
      "HubSpot": [0.2, 0.5]
      "Square": [0.8, 0.4]
      "Chatwoot": [0.4, 0.5]
      "OHC (Future)": [0.9, 0.95]
  ```

  ### Feature Gap Heatmap
  | Feature | OHC Current | Intercom Fin | Shopify Sidekick | Chatwoot | OHC Target |
  |---------|------------|--------------|------------------|----------|------------|
  | Mobile-first Assistant UI | Partial | No (Admin heavy)| Yes | Partial | **Yes (375px)**|
  | Native Rust Omnichannel | **Gap** | N/A | N/A | Ruby/Vue | **Yes** |
  | Outcome-based AI Actions | Drafts only| Yes | Yes | Yes (Captain) | **Yes** |
  | Price Accessibility | Free/Low | High ($$$) | Included | Free/Open | **Low (SaaS)** |

  ### User Journey Comparison
  ```mermaid
  journey
    title Responding to an Instagram DM for a Custom Order (Maya)
    section Legacy Suite (e.g., Chatwoot/HubSpot)
      Receive notification: 5: Maya
      Open separate Inbox App: 3: Maya
      Read context manually: 2: Maya
      Type reply & send payment link: 2: Maya
    section OHC Target Assistant
      AI Triage flags priority in Main Feed: 5: OHC Agent
      Review AI-drafted reply & quote: 5: Maya
      Tap 'Approve & Send': 5: Maya
  ```

  ## 4. Design Doc & Architecture Recommendations
  **High-Level Architecture (Native Rust Omnichannel Engine)**
  - **Entity Types:** `Conversation`, `Message`, `Channel` (WhatsApp, IG, Web), `Contact`, `AgentDraft`.
  - **Integration Points:**
    - Natively ingest webhooks from Meta Cloud API (WhatsApp/IG) via Axum Rust routes.
    - Tie messages to OHC's `tenant_id` with Row-Level Security in PostgreSQL.
    - Utilize OHC's AI Job Queue (PostgreSQL `SKIP LOCKED` or Redis) to trigger the `Customer & Relationship Assistant` LLM prompt on every incoming message.
  - **Mobile UX Flow (375px):** The user does not see a traditional 3-pane inbox. They see a "Work Triage" feed card: "New custom cake request from Sarah (IG). Draft reply ready." Tapping it expands the context inline with an "Approve & Send" button.

  ## 5. Implementation Prompt
  **User-Facing Outcome:** Maya receives an IG DM. Instead of opening an inbox, her OHC home screen shows a triage card. She taps the card to see an AI-drafted reply and a pre-calculated deposit link. She hits "Approve," and the Rust backend dispatches the message back to IG.

  **Acceptance Criteria:**
  1. Create the backend data models (`Conversation`, `Message`) in Rust/PostgreSQL with strict `tenant_id` RLS.
  2. Implement an Axum HTTP webhook receiver for a generic channel (e.g., mock Meta API) to ingest messages.
  3. Wire the incoming message to the internal AI agent to auto-generate a draft reply.
  4. Build a Flutter/PWA UI component (375px width optimized) that displays the pending draft as an actionable feed card, not a traditional inbox table.
  5. Ensure E2E Playwright tests cover receiving a message, viewing the draft in the UI, and approving it.
  6. Completely remove any existing Chatwoot third-party dependencies from `package.json` or `docker-compose`.

  ## 6. Priority and Scope
  - **Priority:** P0 (Core capability for Work Triage and Chatwoot retirement)
  - **Estimated Scope:** Large

  ## 7. References & Sources Catalog
  1. Shopify Sidekick Homepage: https://www.shopify.com/sidekick
  2. WeCom Overview: https://work.weixin.qq.com/
  3. HubSpot CRM Platform: https://www.hubspot.com/
  4. Square POS & Payments: https://squareup.com/us/en
  5. Notion AI Productivity: https://www.notion.com/product/ai
  6. Microsoft 365 Copilot for Business: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  7. Chatwoot Platform Homepage: https://www.chatwoot.com/
  8. Chatwoot Features Overview: https://www.chatwoot.com/features
  9. Chatwoot Open Source Repository: https://github.com/chatwoot/chatwoot
  10. Zoho Zia AI Assistant: https://www.zoho.com/zia/
  11. Intercom Fin AI Agent: https://fin.ai/
  12. Sierra AI Conversational AI: https://sierra.ai/
  13. Intercom Helpdesk & Customer Support: https://www.intercom.com/
  14. Shopify Magic Features: https://www.shopify.com/magic
  15. Microsoft Copilot Pricing & Plans: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365#Pricing
  16. Microsoft Copilot Features Overview: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365#features
  17. Chatwoot Captain AI Documentation: https://chwt.app/captain-docs
  18. HubSpot Pricing & Packages: https://www.hubspot.com/pricing
  19. Zoho Zia Skills & Use Cases: https://www.zoho.com/zia/skills.html
  20. Intercom Fin Pricing: https://fin.ai/pricing
  21. Intercom Omnichannel Support: https://www.intercom.com/helpdesk/omnichannel
  22. Square POS Hardware & Solutions: https://squareup.com/us/en/hardware
  23. Square App Marketplace: https://squareup.com/us/en/app-marketplace
  24. Notion AI Meeting Notes: https://www.notion.com/product/ai-meeting-notes
  25. Notion AI Agents: https://www.notion.com/product/agents
  26. DingTalk Global Operations: https://www.dingtalk.com/en
  27. Microsoft Copilot ROI Insights: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365#ROI
  28. Chatwoot API Documentation: https://www.chatwoot.com/developers/api
  29. HubSpot Startups & SMB Solutions: https://www.hubspot.com/startups
  30. Square POS Features for Restaurants: https://squareup.com/us/en/restaurants
  31. Sierra AI Customers & Success Stories: https://sierra.ai/customers
  32. Intercom Fin Trust & Reliability: https://fin.ai/trust-reliability
  33. Zoho Privacy Commitment: https://www.zoho.com/privacy-commitment.html
  34. Chatwoot GitHub Commit History: https://github.com/chatwoot/chatwoot/commits/develop/
  35. Notion Custom Agents Pricing: https://www.notion.com/help/custom-agent-pricing
  36. Microsoft AI Tour & Readiness Quiz: https://go.microsoft.com/fwlink/?linkid=2346043
  37. Intercom 2026 Transformation Report: https://www.intercom.com/customer-transformation-report
  38. Intercom Solution Partner Program: https://www.intercom.com/solution-partner-program
  39. Sierra AI Agent Studio: https://sierra.ai/product/agent-studio
  40. Zoho Zia Customer Support Integration: https://www.zoho.com/zia/ask.html
  41. Shopify App Store Integrations: https://apps.shopify.com/
  42. HubSpot App Marketplace: https://ecosystem.hubspot.com/marketplace/apps
  43. Notion Template Gallery: https://www.notion.com/templates
  44. Square Business Types Overview: https://squareup.com/us/en/industry
  45. Chatwoot Integrations Catalog: https://www.chatwoot.com/features/integrations
  46. Intercom Live Intelligence: https://www.intercom.com/helpdesk/knowledge-hub
  47. Microsoft 365 Small Business Features: https://www.microsoft.com/en-us/microsoft-365/business
  48. WeCom Admin Portal: https://work.weixin.qq.com/wework_admin/loginpage_wx?from=myhome
  49. Zoho Desk AI Integration: https://www.zoho.com/desk/
  50. Shopify Editions & Updates: https://www.shopify.com/editions
  51. Intercom Fin Agent Blueprint: https://fin.ai/blueprint
  52. Microsoft 365 Copilot FAQ: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365#FAQ

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
