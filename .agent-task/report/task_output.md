issue_title: "Market Leadership Research: OHC Owner Work Assistant vs AI Competitors"
issue_description: |
  # OHC Owner Work Assistant: Competitive Market Analysis & Feature Missions

  ## 1. Problem Statement
  Non-technical small business owners, operators, and creators (e.g., bakers, handymen, tutors) are overwhelmed by "admin work." They currently string together fragmented tools (Instagram DMs, Apple Notes, Square, spreadsheets). Traditional CRMs (like HubSpot or Salesforce) are too complex, requiring an "admin" mindset. They don't need a dashboard of analytics; they need a **Workbuddy**—an AI assistant that triages tasks, drafts replies, coordinates schedules, and executes actions autonomously while hiding the technical complexity.

  ## 2. Research Report & Competitive Deep Dive

  ### Market Mapping
  We mapped out 20 major players, spanning traditional giants and AI-native upstarts.
  *(For complete sources, see the References Catalog at the end).*

  **Top General Competitors:** HubSpot, Salesforce, Zoho, Tencent Workbuddy, Lark/Feishu, DingTalk, Square, Wix, Asana, Notion.
  **Top AI-Native Competitors:** Shopify Sidekick, Microsoft Copilot, Intercom Fin, Gorgias AI, Notion AI, Klaviyo AI, Freshworks Freddy, Zendesk AI, ClickUp Brain, Square AI.

  ### Deep Dive: Shopify Sidekick vs. General Tools
  *   **Shopify Sidekick:** Excels at conversational execution ("Do it for me"). It hides complex variant/inventory logic behind a chat interface. It has high user delight because it reduces "time to live" for a store. However, it is strictly siloed to commerce.
  *   **HubSpot / Salesforce:** Extremely powerful but suffers from steep learning curves. Trustpilot and Reddit reviews frequently cite "too complex for a 2-person team" and "I just need a simple way to track leads, not a 10-week onboarding."
  *   **Tencent Workbuddy / Lark:** These excel at unified communication—bringing chat, approvals, and tasks into one feed. However, they lack native, seamless integration into Western commerce/payment flows out of the box.

  ### OHC Gap Analysis
  OHC's backend (KAIROS Orchestration, Teammate Mesh, Hybrid Agentic OS) is incredibly robust. But the frontend lacks consumer-grade translation:
  1.  **No Unified Work Triage:** We have shared task lists in PostgreSQL, but no mobile-first "Inbox" that merges DMs, AI-drafted tasks, and payment alerts.
  2.  **No Conversational "Do-It-For-Me" Actions:** Users cannot just type "Send Carlos a quote for roof repair."
  3.  **Missing "Autopilot" for Missed Leads:** No autonomous recovery for missed inquiries when the operator is busy.

  ---

  ## 3. Visual Comparisons & Mermaid Charts

  ```mermaid
  quadrantChart
      title Market Positioning: OHC vs Competitors
      x-axis "Manual Admin" --> "Agentic (Do-It-For-Me)"
      y-axis "Siloed Functionality" --> "Unified Work Triage"
      quadrant-1 "Ideal Operator Tool"
      quadrant-2 "All-In-One, Manual"
      quadrant-3 "Fragmented, Manual"
      quadrant-4 "Smart but Siloed"
      "HubSpot": [0.2, 0.7]
      "Salesforce": [0.1, 0.8]
      "Square": [0.3, 0.3]
      "Shopify Sidekick": [0.8, 0.4]
      "Notion AI": [0.7, 0.5]
      "Tencent Workbuddy": [0.4, 0.8]
      "OneHumanCorp (Target)": [0.9, 0.9]
  ```

  ### Feature Comparison Table

  | Feature | OHC (Target) | Shopify Sidekick | HubSpot | Tencent Workbuddy |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Work Triage** | Yes (Agentic Feed) | No (Commerce Only) | Yes (Manual CRM) | Yes (Chat/Ops) |
  | **Conversational Action** | Yes (Draft/Execute) | Yes | Limited | Limited |
  | **Offline/Mobile First** | Yes (Hybrid OS) | No | Mobile App | Yes |
  | **Setup Complexity** | Low (Conversational) | Low | High | Medium |

  ---

  ## 4. Design Doc & Agentic Solutions

  ### Entity Architecture & High-Level Design
  *   **Core Entities:** `WorkItem` (polymorphic: Message, Booking, Quote, Alert), `CustomerContext` (unified CRM view), `AgentDraft` (pending AI actions awaiting owner approval).
  *   **Agentic Flow:**
      1.  *Intake:* Omnichannel webhook (Instagram, Email, Web) feeds into the `Work Triage Agent`.
      2.  *Contextualization:* Agent retrieves `CustomerContext` and past `WorkItems`.
      3.  *Drafting:* AI drafts a response or generates a pending action (e.g., "Drafted Quote for $500").
      4.  *Owner Approval:* The owner sees a Tinder-like mobile card stack in the 375px UI: "Approve," "Edit," "Reject."

  ### Mobile UX Wireframe Flow (375px First)
  1.  **The "Today" Screen (Command Center):** A clean, Apple-esque list of prioritized items. Top item: "🚨 3 Unanswered Cake Inquiries. Agent drafted replies."
  2.  **Action Detail Screen:** Tapping an inquiry shows the customer message and the AI-generated draft. Buttons: [Send] [Edit] [Remind Me].
  3.  **Conversational Overlay:** A floating action button opens the Workbuddy chat. "Schedule a delivery for Maya on Tuesday." The UI instantly renders a native calendar confirmation card in the chat.

  ---

  ## 5. Implementation Prompts (Missions for the Swarm)

  ### Mission 1: Unified Work Triage Feed (P0)
  **Problem Statement:** Owners switch between 5 apps to see what needs to be done.
  **Outcome:** Implement a mobile-first (375px) "Command Center" feed. It must consume the existing `Shared Task List` backend but render it as a unified, prioritized inbox of messages, alerts, and pending agent actions.
  **Acceptance Criteria:**
  *   Renders flawlessly on a 375px viewport.
  *   Displays different card types (Message, Alert, Draft) with clear visual hierarchy.
  *   Includes a Playwright E2E test verifying a user can log in and view a populated feed without hardcoded mock data.
  **Estimated Scope:** Large

  ### Mission 2: Conversational "Do-It-For-Me" Action Overlay (P1)
  **Problem Statement:** Owners don't want to navigate 4 menus to create a quote.
  **Outcome:** Build a global conversational chat overlay that interprets natural language (e.g., "Create a $50 quote for Leo") and utilizes KAIROS orchestration to generate the entity.
  **Acceptance Criteria:**
  *   The UI must render the resulting "Quote" as a structured native card inside the chat stream, not just text.
  *   Integrates with the `OHC_LLM_PROVIDER` for intent parsing.
  **Estimated Scope:** Medium

  ### Mission 3: Autonomous "Missed Lead" Recovery Agent (P2)
  **Problem Statement:** Field operators miss leads when they are on the job.
  **Outcome:** An agentic workflow that monitors unread incoming inquiries. If unread for >10 mins, the agent automatically drafts and sends a friendly holding message ("Hi, Carlos is on a roof right now but will get back to you by 4 PM. Do you need an estimate?").
  **Acceptance Criteria:**
  *   Background job utilizing PostgreSQL `SKIP LOCKED`.
  *   Owner can toggle "Auto-pilot" mode on/off from the mobile UI.
  **Estimated Scope:** Small

  ---

  ## 6. References & Sources Catalog
  The following 56 URLs were scraped and analyzed to build this report:

  1. https://www.shopify.com/sidekick
  2. https://square.com/us/en/townsquare/ai-for-small-business
  3. https://www.hubspot.com/artificial-intelligence
  4. https://www.notion.so/product/ai
  5. https://www.microsoft.com/en-us/microsoft-copilot
  6. https://larksuite.com/
  7. https://work.weixin.qq.com/
  8. https://www.dingtalk.com/en
  9. https://www.salesforce.com/einstein/
  10. https://www.zoho.com/zia/
  11. https://www.intercom.com/ai-bot
  12. https://gorgias.com/product/ai
  13. https://www.klaviyo.com/features/ai
  14. https://www.wix.com/about/ai
  15. https://squareup.com/us/en/townsquare/ai-tools-for-small-business
  16. https://www.freshworks.com/freddy-ai/
  17. https://www.zendesk.com/service/ai/
  18. https://asana.com/product/ai
  19. https://monday.com/work-os/ai
  20. https://clickup.com/ai
  21. https://www.reddit.com/r/smallbusiness/search/?q=ai+assistant
  22. https://www.reddit.com/r/smallbusiness/search/?q=automation
  23. https://www.reddit.com/r/smallbusiness/search/?q=crm+struggle
  24. https://www.reddit.com/r/ecommerce/search/?q=shopify+sidekick
  25. https://www.reddit.com/r/smallbusiness/search/?q=hubspot+alternative
  26. https://www.trustpilot.com/review/www.hubspot.com
  27. https://www.trustpilot.com/review/www.shopify.com
  28. https://www.trustpilot.com/review/www.notion.so
  29. https://www.trustpilot.com/review/www.salesforce.com
  30. https://www.trustpilot.com/review/www.zoho.com
  31. https://apps.shopify.com/search?q=ai+assistant
  32. https://apps.apple.com/us/app/hubspot/id1105311053
  33. https://apps.apple.com/us/app/shopify-ecommerce-business/id371295624
  34. https://play.google.com/store/apps/details?id=com.hubspot.android
  35. https://play.google.com/store/apps/details?id=com.shopify.m
  36. https://techcrunch.com/tag/ai-assistant/
  37. https://techcrunch.com/tag/small-business-ai/
  38. https://www.theverge.com/ai-artificial-intelligence
  39. https://www.wired.com/tag/artificial-intelligence/
  40. https://www.forbes.com/ai/
  41. https://www.bloomberg.com/technology
  42. https://www.cnbc.com/technology/
  43. https://www.wsj.com/tech/ai
  44. https://hbr.org/topic/artificial-intelligence
  45. https://sloanreview.mit.edu/topic/artificial-intelligence/
  46. https://www.mckinsey.com/capabilities/quantumblack/our-insights
  47. https://www.bain.com/insights/topics/artificial-intelligence/
  48. https://www.bcg.com/capabilities/artificial-intelligence/insights
  49. https://www.gartner.com/en/topics/generative-ai
  50. https://www.forrester.com/bold/generative-ai/
  51. https://www.g2.com/categories/ai-chatbots
  52. https://www.g2.com/categories/ai-sales-assistant
  53. https://www.g2.com/categories/intelligent-virtual-assistants
  54. https://www.capterra.com/artificial-intelligence-software/
  55. https://www.getapp.com/it-management-software/artificial-intelligence/
  56. https://www.softwareadvice.com/artificial-intelligence/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
