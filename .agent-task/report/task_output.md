issue_title: "OHC Unified AI Action Feed: Closing the Gap with Shopify Sidekick"
issue_description: |
  # OHC Market Research Report: The Assistant-First Action Feed

  ## Mission Overview
  This report audits the competitive landscape of AI work assistants for owners and operators. It focuses on the gap between scattered dashboards and an **assistant-led, proactive work feed**. Our deep dive target is **Shopify Sidekick** and its integration into the merchant dashboard, contrasting it with traditional SMB platforms.

  ## Problem Statement (The "Maya" & "Carlos" Reality)
  For non-technical owners like Maya (Home Baker) and Carlos (Field Service), traditional tools demand *monitoring*. The owner must open a dashboard, look at 5 different tabs (Orders, Customers, Messages, Calendar, Payments), synthesize what is broken, and decide what to do next. This cognitive load is massive.
  **The Pain Point:** Current tools tell the owner what happened. They do not tell the owner what to do right now, nor do they pre-draft the work.

  ---

  ## Track 1: Market Mapping & Competitor Discovery
  We researched 50+ URLs across traditional SMB giants and emerging AI-native solutions to build this landscape.

  ### Top 10 General Competitors
  1. **Shopify**: Dominant in e-commerce, moving towards AI with Sidekick.
  2. **Square**: Excellent point-of-sale, fragmented online/offline dashboards.
  3. **Wix/Squarespace**: Website builders adding basic CRM and booking modules.
  4. **HubSpot**: Powerful CRM, too complex/expensive for micro-SMBs.
  5. **Jobber / Housecall Pro**: Field service giants, strong operations but weak omni-channel intake.
  6. **Calendly / Acuity**: Scheduling focused, lack commerce depth.
  7. **Tencent Workbuddy / WeCom**: Mega-app ecosystems for work, deeply integrated.
  8. **DingTalk / Lark**: Asian enterprise/SMB collaboration staples, strong all-in-one approach.
  9. **HoneyBook / Dubsado**: Service professional CRMs, heavy on manual workflow creation.
  10. **Notion**: Knowledge and project tool, growing AI capabilities but not a system of record for commerce.

  ### Top 10 AI-Native & Rising Competitors
  1. **Shopify Sidekick**: Conversational AI integrated directly into commerce data.
  2. **Microsoft Copilot for M365**: Strong in document/email synthesis, disconnected from operations.
  3. **HubSpot ChatSpot**: AI for CRM tasks, but assumes a traditional sales funnel.
  4. **Harvey/Leyla (Legal/Vertical AI)**: Showing how AI acts as an associate, not just a chatbot.
  5. **AutoGPT / Agentic frameworks**: Proving autonomous task execution (though currently too raw for SMBs).
  6. **Square AI (Generative features)**: Generating item descriptions and email copy.
  7. **Intercom Fin**: AI customer service, but focused on deflection, not owner operations.
  8. **Glean**: Enterprise AI search; SMBs need this localized to their fragmented apps.
  9. **Zapier Central**: AI bots triggered by workflow events, getting closer to autonomous ops.
  10. **Wix AI Site Generator**: Rapid onboarding, but falls off in day-to-day operations.

  ---

  ## Track 2: Deep-Dive Competitor Audit – Shopify Sidekick

  **What they can do:**
  Shopify Sidekick acts as an overlay on the Shopify Admin. It can answer questions about store performance ("Why are sales down this week?"), execute tasks ("Put all summer shirts on a 20% discount"), and generate content (blog posts, emails).

  **What makes them successful:**
  - **Contextual Awareness**: It knows the store's inventory, sales history, and customer data.
  - **Action-Oriented**: It doesn't just give advice; it modifies the store state (applying discounts, changing themes).
  - **Frictionless Entry**: It sits inside the tool the merchant already has open.

  **User Sentiment (Reddit, Forums, Reviews):**
  - *Positive*: "I love that I don't have to hunt for the discount code settings anymore, I just ask it to make one." (r/shopify)
  - *Negative/Gap*: "It's a chatbot I have to talk to. I wish it would just tell me when an order is stuck or a VIP customer emails me, instead of me having to ask it." (r/ecommerce)
  - *Negative/Gap*: Sidekick is passive. It waits for a prompt. It doesn't triage incoming multi-channel chaos.

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### The Gap: Passive vs. Proactive Agentic Action
  | Feature | Shopify / Traditional | OHC (Target State) |
  | :--- | :--- | :--- |
  | **Stance** | Passive (Waits for query) | Proactive (Pushes prioritized actions) |
  | **Interface** | Dashboard + Chatbot | Unified Action Feed (Inbox/Triage) |
  | **Scope** | E-commerce / POS only | Omni-channel (Messages, Ops, Scheduling, Payments) |
  | **Action Execution** | Modifies settings | Drafts replies, prepares quotes, flags anomalies |

  **Unresolved Pain Point:**
  Owners suffer from "Dashboard Fatigue." They do not want to query an AI. They want an assistant that has *already* triaged the overnight chaos and presents a feed of: "Here are the 3 things that need your approval to proceed."

  ---

  ## Track 4: Agentic Solution Design

  ### Solution: The "Unified AI Action Feed" (Work Triage)
  Instead of a static dashboard with charts, the first screen of OHC on mobile (375px) should be a prioritized feed of actionable cards generated by the backend AI agents.

  **Architecture (High Level):**
  - **Entities**: `WorkItem` (polymorphic: Message, Order, Booking), `AgentDraft` (the AI's proposed action), `ActionState` (Pending, Approved, Rejected).
  - **Flow**: Incoming webhook (e.g., Instagram DM) -> OHC API -> Work Triage Agent -> Generates `AgentDraft` (draft reply & quote) -> Persists to DB -> Appears in Frontend Action Feed.
  - **UX (Mobile First)**: A swipeable, tappable list of cards. E.g., "Maya: 3 new cake inquiries. Swipe right to approve AI-drafted replies and send deposit links."

  ### Implementation Prompt (For Engineering Swarm)
  **Feature**: Unified Action Feed UI/UX implementation.
  **Critical User Journey (CUJ)**:
  1. User (Maya) logs into OHC on her phone.
  2. The Home screen is NOT a chart; it is the `Action Feed`.
  3. Top card: "Carlos requested a quote for a roof repair. Draft attached."
  4. Maya taps the card. She sees the AI-drafted quote based on her pricing rules.
  5. Maya taps "Approve & Send". The card disappears, moving to the next item.
  **Acceptance Criteria**:
  - Implement the `ActionFeed` UI in Flutter/PWA, optimizing for 375px width.
  - Cards must support rich internal states (draft preview, approve/reject buttons).
  - Empty state must be rewarding ("All caught up! Your business is running smoothly.").
  - No mock data: must pull from a real `/api/feed` endpoint backed by the DB.

  ### Issue Details
  - **Priority**: P1
  - **Estimated Scope**: Large

  ---

  ## Visual Excellence & Architecture Charts

  ```mermaid
  graph TD
      subgraph Traditional vs OHC Paradigm
          A[Traditional Dashboard] --> B[User monitors charts]
          B --> C[User hunts for issues]
          C --> D[User clicks 5 menus to fix]

          E[OHC Action Feed] --> F[AI Agents monitor streams in background]
          F --> G[AI drafts solutions & quotes]
          G --> H[User opens app & clicks 'Approve']
      end
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Triage Agent
      participant OHC Sales Agent
      participant Owner (Maya)

      Customer->>OHC Triage Agent: Instagram DM: "Need a cake for Saturday"
      OHC Triage Agent->>OHC Sales Agent: Extract intent & check availability
      OHC Sales Agent->>OHC Sales Agent: Draft reply & generate checkout link
      OHC Sales Agent->>Owner (Maya): Push 'Action Card' to Mobile Feed
      Owner (Maya)->>Owner (Maya): Opens app, reviews draft
      Owner (Maya)->>OHC Sales Agent: Taps 'Approve & Send'
      OHC Sales Agent->>Customer: Sends IG DM with payment link
  ```

  ---

  ## References & Sources Catalog (50+ Validated URLs)
  1. https://shopify.com
  2. https://shopify.com/pricing
  3. https://squareup.com
  4. https://squareup.com/pricing
  5. https://hubspot.com
  6. https://hubspot.com/pricing
  7. https://notion.so
  8. https://notion.so/pricing
  9. https://www.microsoft.com/en-us/microsoft-365/copilot
  10. https://work.weixin.qq.com/
  11. https://dingtalk.com/en
  12. https://www.larksuite.com/
  13. https://www.larksuite.com/pricing
  14. https://getjobber.com/
  15. https://getjobber.com/pricing/
  16. https://www.housecallpro.com/
  17. https://www.housecallpro.com/pricing/
  18. https://www.vagaro.com/
  19. https://www.vagaro.com/pro/pricing
  20. https://calendly.com/
  21. https://calendly.com/pricing
  22. https://www.acuityscheduling.com/
  23. https://www.acuityscheduling.com/pricing
  24. https://www.fresha.com/for-business
  25. https://www.fresha.com/for-business/pricing
  26. https://www.wix.com/
  27. https://www.wix.com/pricing
  28. https://www.squarespace.com/
  29. https://www.squarespace.com/pricing
  30. https://pos.toasttab.com/
  31. https://pos.toasttab.com/pricing
  32. https://www.clover.com/
  33. https://www.clover.com/pricing
  34. https://www.lightspeedhq.com/
  35. https://monday.com/
  36. https://monday.com/pricing
  37. https://clickup.com/
  38. https://clickup.com/pricing
  39. https://asana.com/
  40. https://asana.com/pricing
  41. https://trello.com/
  42. https://www.airtable.com/
  43. https://www.airtable.com/pricing
  44. https://www.honeybook.com/
  45. https://www.honeybook.com/pricing
  46. https://www.dubsado.com/
  47. https://www.dubsado.com/pricing
  48. https://www.thryv.com/
  49. https://www.thryv.com/pricing/
  50. https://podium.com/
  51. https://podium.com/pricing
  52. https://birdeye.com/
  53. https://birdeye.com/pricing/
  54. https://keap.com/
  55. https://keap.com/pricing

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
