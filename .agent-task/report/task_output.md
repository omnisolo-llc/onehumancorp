issue_title: Implement Unified Mobile Action Center for Triage
issue_description: "\n# OHC Market Research: Competitor Analysis & AI Agentic Solutions\
  \ for SMBs\n\n## 1. Title\nImplement Unified Mobile Action Center for Triage\n\n\
  ## 2. Problem Statement\n**The \"Dashboard Deluge\" & Fragmented Workflows:** Small\
  \ business owners (like Maya the baker or Carlos the handyman) suffer from tool\
  \ fatigue. Existing solutions (like Shopify, Square, or Hubspot) provide powerful\
  \ dashboards but fail to deliver actionable, synthesized next steps. A booking arrives\
  \ via Instagram DM, a payment via Venmo, and a schedule update in Google Calendar.\
  \ This fragmentation leads to dropped balls, lost revenue, and extreme operational\
  \ friction, especially on mobile devices where field operators require full capabilities\
  \ on a 375px screen.\n\n### Persona-Specific Pain Point Summary\n*   **Maya (Baker):**\
  \ Fragmented DMs. Needs order categorization and automated deposit links.\n*   **Carlos\
  \ (Handyman):** Manual quoting. Needs booking and estimating tools integrated with\
  \ text alerts.\n*   **Priya (Boutique):** Inventory sync. Needs unified view of\
  \ POS and online stock.\n*   **Leo (Tutor):** Booking chaos. Needs simple subscription\
  \ billing.\n*   **Fatima (Food Cart):** Slow mobile data. Needs reliable offline-capable,\
  \ one-screen pre-order lists.\n\n## 3. Research Report\nThis research evaluates\
  \ established industry giants and emerging AI-native platforms to identify key gaps\
  \ in serving non-technical SMB owners.\n\n### 3.1 Market Mapping (Top 10 General\
  \ Competitors)\n*   **Shopify:** Focuses on E-commerce/POS with \"Sidekick\" AI\
  \ for analytics. (https://www.shopify.com)\n*   **Square:** POS and local biz with\
  \ AI-generated product descriptions. (https://squareup.com)\n*   **HubSpot:** CRM\
  \ with \"ChatSpot\" AI integration. (https://www.hubspot.com)\n*   **Wix:** Website\
  \ builder with AI site generation. (https://www.wix.com)\n*   **Tencent Workbuddy\
  \ (WeCom):** Deep ecosystem integration for enterprise/SMB comms. (https://work.weixin.qq.com)\n\
  *   **DingTalk (Alibaba):** Collaboration with \"DingTalk AI\" for summaries. (https://www.dingtalk.com)\n\
  *   **Feishu / Lark (ByteDance):** Unified collaboration with strong AI summaries.\
  \ (https://www.larksuite.com)\n*   **Notion:** Knowledge management with \"Notion\
  \ AI\". (https://www.notion.so)\n*   **Microsoft Copilot:** Ubiquitous AI across\
  \ Office suite. (https://www.microsoft.com/en-us/microsoft-365/copilot)\n*   **ServiceTitan:**\
  \ Vertical SaaS for home services. (https://www.servicetitan.com)\n\n### 3.2 Emerging\
  \ AI-Native Competitors (Top 10)\n*   **Motion:** AI Scheduling. (https://www.usemotion.com)\n\
  *   **Reclaim.ai:** Smart Calendar Assistant. (https://reclaim.ai)\n*   **Lindsey\
  \ AI:** AI Receptionist. (https://lindsey.ai)\n*   **Gong:** Revenue Intelligence.\
  \ (https://www.gong.io)\n*   **Intercom (Fin):** AI Customer Service. (https://www.intercom.com)\n\
  *   **Harvey:** Legal AI. (https://www.harvey.ai)\n*   **Copy.ai / Jasper:** AI\
  \ Marketing Copy. (https://www.copy.ai)\n*   **Devin:** Autonomous AI Software Engineer.\
  \ (https://www.cognition-labs.com)\n*   **MultiOn:** Autonomous Web Agent. (https://www.multion.ai)\n\
  *   **Adept:** Desktop Agent. (https://www.adept.ai)\n\n### 3.3 Dynamic Competitive\
  \ Landscape (Mermaid)\n\n```mermaid\nquadrantChart\n    title Market Positioning:\
  \ Intelligence vs. Complexity\n    x-axis \"Traditional/Reactive\" --> \"Agentic/Proactive\"\
  \n    y-axis \"High Complexity (Enterprise)\" --> \"Owner-Centered (SMB)\"\n   \
  \ quadrant-1 \"Agentic SMB Leaders\"\n    quadrant-2 \"Agentic Enterprise\"\n  \
  \  quadrant-3 \"Traditional Enterprise\"\n    quadrant-4 \"Traditional SMB\"\n \
  \   \"Shopify\": [0.3, 0.4]\n    \"HubSpot\": [0.4, 0.8]\n    \"Motion\": [0.8,\
  \ 0.6]\n    \"Tencent Workbuddy\": [0.6, 0.7]\n    \"OHC (Target)\": [0.9, 0.2]\n\
  \    \"Reclaim.ai\": [0.7, 0.5]\n    \"Intercom Fin\": [0.8, 0.7]\n    \"ServiceTitan\"\
  : [0.2, 0.8]\n```\n\n### 3.4 Deep-Dive Audit: Shopify\n*   **Capabilities:** Omnichannel\
  \ commerce, inventory, fulfillment, payments. AI (Sidekick) queries store data and\
  \ answers \"how-to\" questions.\n*   **Success Factors:** Extremely fast onboarding,\
  \ massive app ecosystem, highly optimized checkout (Shop Pay).\n*   **User Sentiment\
  \ (Reddit/Trustpilot):** Users love the reliability and checkout experience but\
  \ suffer from \"App fatigue\" (needing 10 different apps for basic functions). Crucially,\
  \ Sidekick is seen as a data query tool, not an agent that *executes work*.\n\n\
  ### 3.5 OHC Gap & Unresolved Pain Points\n\n| Feature Category | Shopify | OHC Vision\
  \ | OHC Current Gap |\n| :--- | :--- | :--- | :--- |\n| **Focus** | E-commerce /\
  \ POS | Unified Owner Action Hub | Needs a centralized \"Today\" view that aggregates\
  \ all work types. |\n| **AI Paradigm** | Chatbot (Sidekick) | Agentic Co-worker\
  \ | AI must proactively draft, coordinate, and suggest, not just answer queries.\
  \ |\n| **Business Model Support**| Product-centric | Product, Service, Booking,\
  \ Creator | Needs unified support for varied service types (e.g., Carlos the Handyman).\
  \ |\n\n```mermaid\njourney\n    title Critical User Journey: The Fragmented Inbox\
  \ vs OHC Triage\n    section Traditional Setup (Fragmented)\n      Receive IG DM:\
  \ 5: Customer\n      Check email for booking: 2: Owner\n      Log into Square to\
  \ invoice: 1: Owner\n      Miss lead response time: 1: Customer\n    section OHC\
  \ Triage (Unified)\n      Triage Agent centralizes messages: 5: System\n      Triage\
  \ Agent drafts reply & invoice: 5: System\n      Owner taps Approve from one feed:\
  \ 5: Owner\n```\n\n## 4. Design Doc\n### High-Level Architecture\n*   **Frontend\
  \ (Flutter):** A central `ActionFeed` widget displaying interactive, swipable `ActionCards`.\
  \ The UI is strictly mobile-first (375px), utilizing the OHC Premium Token library\
  \ with translucent materials.\n*   **Backend (Go):** A `TriageService` that acts\
  \ as the ingestion point for various events (communications, payments, bookings).\n\
  *   **AI Integration:** The Work Triage Agent evaluates incoming events, categorizes\
  \ them, assigns priority, and drafts suggested actions via the LLM provider (Gemini\
  \ Pro/GPT-4o), feeding them into a PostgreSQL `SKIP LOCKED` job queue.\n\n### Feature\
  \ Gap Heatmap (Mermaid)\n\n```mermaid\npie title Required Feature Focus Areas for\
  \ Action Center\n    \"Triage Feed UI\" : 40\n    \"Agentic API Integrations\" :\
  \ 35\n    \"Cross-Channel Auth\" : 15\n    \"Analytics/Reporting\" : 10\n```\n\n\
  ### Mobile UX Flow (375px First)\n1.  **Feed View:** The owner opens the app to\
  \ a prioritized, single-column vertical list of action cards.\n2.  **Card Interaction:**\
  \ A card displays context (e.g., \"High priority: Lead waiting > 2 hours\") and\
  \ a drafted reply. The owner taps \"Approve\" (one-tap execution) or \"Edit\".\n\
  3.  **Completion:** Upon action execution, the card animates away, and the underlying\
  \ state (e.g., message sent, booking confirmed) is updated.\n\n## 5. Implementation\
  \ Prompt\nImplement the `ActionFeed` and `ActionCard` components in Flutter, connected\
  \ to a mocked (for now) `TriageService` endpoint. The Critical User Journey (CUJ)\
  \ involves an owner opening the app, viewing a high-priority \"Drafted Reply\" card\
  \ for an Instagram DM, reviewing the context, and tapping \"Approve\" to send the\
  \ message. Acceptance criteria:\n1.  The UI must perfectly fit a 375px width without\
  \ horizontal scrolling.\n2.  The `ActionCard` must display the customer context,\
  \ the AI-drafted reply, and actionable buttons (\"Approve\", \"Edit\", \"Dismiss\"\
  ).\n3.  Tapping \"Approve\" must trigger an API call and remove the card from the\
  \ feed with a smooth animation.\n4.  The components must adhere to the OHC Premium\
  \ Token library styling.\n\n## 6. Priority\nP1\n\n## 7. Estimated Scope\nMedium\n\
  \n## 8. References & Sources Catalog\n1.  https://www.shopify.com\n2.  https://www.shopify.com/editions/summer2023#sidekick\n\
  3.  https://squareup.com/us/en/campaign/ai\n4.  https://chatspot.ai/\n5.  https://www.wix.com/about/ai\n\
  6.  https://work.weixin.qq.com\n7.  https://www.dingtalk.com\n8.  https://www.larksuite.com\n\
  9.  https://www.notion.so/product/ai\n10. https://www.microsoft.com/en-us/microsoft-365/copilot\n\
  11. https://www.servicetitan.com\n12. https://www.usemotion.com\n13. https://reclaim.ai\n\
  14. https://lindsey.ai\n15. https://www.gong.io\n16. https://www.intercom.com/fin\n\
  17. https://www.harvey.ai\n18. https://www.copy.ai\n19. https://www.cognition-labs.com/introducing-devin\n\
  20. https://www.multion.ai\n21. https://www.adept.ai/blog/act-1\n22. https://www.reddit.com/r/smallbusiness/\n\
  23. https://www.reddit.com/r/ecommerce/\n24. https://community.shopify.com/\n25.\
  \ https://www.trustpilot.com/review/servicetitan.com\n26. https://zapier.com/blog/\n\
  27. https://www.forbes.com/\n28. https://techcrunch.com/\n29. https://www.bloomberg.com/\n\
  30. https://www.ycombinator.com/\n31. https://www.producthunt.com/\n32. https://www.g2.com/\n\
  33. https://www.capterra.com/\n34. https://www.getapp.com/\n35. https://www.softwareadvice.com/\n\
  36. https://apps.shopify.com/\n37. https://stripe.com/\n38. https://squareup.com/us/en/app-marketplace\n\
  39. https://www.zendesk.com/\n40. https://www.freshworks.com/\n41. https://www.salesforce.com/\n\
  42. https://www.adobe.com/\n43. https://www.canva.com/\n44. https://asana.com/\n\
  45. https://monday.com/\n46. https://clickup.com/\n47. https://www.smartsheet.com/\n\
  48. https://www.airtable.com/\n49. https://webflow.com/\n50. https://www.framer.com/\n\
  51. https://www.web.com/\n"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
