issue_title: "Research: AI Assistants & Core Workflow Gaps for Owner/Operators"
issue_description: |
  # Research: Market Mapping & OHC Feature Gaps for Owner/Operators

  **Role**: Principal Product Researcher & Oracle (L7)
  **Mission**: Drive OHC's market leadership as a Tencent Workbuddy-like owner work assistant. This report identifies user pain points, uncovers emerging trends, and generates actionable feature missions.

  ---

  ## Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  ### Top 10 General Competitors
  | Competitor | Core Strength | Relevance to OHC |
  | :--- | :--- | :--- |
  | **Shopify** | E-commerce dominance | High - Maya (Baker), Priya (Boutique) need inventory and sales. |
  | **Square** | In-person payments & POS | High - Carlos (Handyman), Fatima (Food Cart) need quick payments. |
  | **HubSpot** | CRM & Inbound Marketing | Medium - Good for leads, but often too complex for solo operators. |
  | **LarkSuite (Feishu)** | Unified collaboration | High - Strong all-in-one approach (chat, docs, tasks). |
  | **DingTalk** | Mobile-first operations | High - Deep integration with enterprise and local business workflows. |
  | **WeCom (Tencent)** | Customer connection | High - Seamless connection between business operations and consumer apps (WeChat). |
  | **Notion** | Flexible knowledge/workspaces | Medium - Highly customizable but requires significant manual setup. |
  | **Wix** | Website builder & basic CRM | Medium - Easy entry but limited deep operational tools. |
  | **HoneyBook** | Client flow for independents | High - Excellent for service providers (Nora, Leo) managing proposals and invoices. |
  | **Trello/Asana** | Task Management | Medium - Good for tasks, poor for unified customer/revenue context. |

  ### Top 10 AI-Native Competitors
  | Competitor | AI Focus | URL |
  | :--- | :--- | :--- |
  | **Shopify Sidekick** | Commerce Copilot | https://www.shopify.com/sidekick |
  | **Lindy.ai** | Autonomous AI Employees | https://lindy.ai/ |
  | **Artisan.co** | AI B2B Sales Agents | https://www.artisan.co/ |
  | **ChatSpot.ai (HubSpot)** | Conversational CRM | https://chatspot.ai/ |
  | **Notion AI** | Knowledge & Writing Assistant | https://www.notion.so/product/ai |
  | **Microsoft Copilot** | Enterprise Productivity | https://www.microsoft.com/en-us/microsoft-365/copilot |
  | **Salesforce Einstein** | Predictive & Generative CRM | https://www.salesforce.com/einstein/ |
  | **Gorgias (AI Features)** | E-commerce Support AI | https://www.gorgias.com/ |
  | **ClickUp Brain** | AI Knowledge & Tasks | https://clickup.com/ai |
  | **Airtable AI** | AI-powered Workflows | https://www.airtable.com/platform/ai |

  ---

  ## Track 2: Deep-Dive Competitor Audit - **Shopify Sidekick**

  **Why Shopify Sidekick?** Shopify is the 800lb gorilla in e-commerce, but they are transitioning from a "platform" to an "assistant" model with Sidekick, attempting to solve the exact complexity problems OHC targets.

  ### Capabilities ("What they can do")
  *   **Conversational Commerce Data**: "Why are my sales down this week?" Sidekick analyzes store data and provides plain-text answers.
  *   **Task Execution**: "Put all my summer t-shirts on sale for 20% off." Sidekick executes bulk actions.
  *   **Store Design/Edits**: Modifying themes or content via natural language.
  *   **Content Generation**: Drafting product descriptions, blog posts, and marketing emails.

  ### Success Factors ("What they are successful at")
  *   **Contextual Awareness**: Sidekick has deep access to the merchant's exact data (inventory, sales, customers).
  *   **Action-Oriented**: It doesn't just give advice; it executes actions within the Shopify ecosystem.
  *   **Trust Building**: It explains *what* it is going to do before doing it, maintaining operator control.

  ### User Sentiment Audit (Aggregated themes from Reddit/Communities)
  *   **What they love**: Time saved on tedious tasks (bulk editing), simplified reporting (not having to build custom reports).
  *   **What they complain about**: It's still tightly bound to the Shopify ecosystem. If you sell off-platform (Instagram DMs, in-person without Square), Sidekick is blind. Small service-based businesses (like Leo the tutor or Carlos the handyman) find Shopify too product-heavy and Sidekick unhelpful for service scheduling.
  *   **Quote**: "I just want something that tells me who I forgot to reply to on IG and sends them a payment link, Shopify is overkill." - (Paraphrased from r/smallbusiness discussions on e-commerce platforms).

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Gap Matrix
  | Feature Category | Shopify Sidekick | HoneyBook | **OHC Target State** | **Current OHC Reality (Codebase Audit)** |
  | :--- | :--- | :--- | :--- | :--- |
  | Unified Inbox (IG, Email, SMS) | Partial (Apps needed) | Email mostly | **Yes, Native** | Needs robust multi-channel triage |
  | Conversational Analytics | Yes | Basic | **Yes, Proactive** | Proactive "Daily Summary" is missing |
  | Service Scheduling | No | Yes | **Yes, Integrated** | Needs deep booking+payment integration |
  | Autonomous Proposal Gen | No | Yes | **Yes, AI-Drafted** | Missing AI workflow for proposals |
  | Mobile-First Execution | Medium | Medium | **Extreme (375px)** | Needs continuous 375px optimization |

  ### Unresolved Pain Points (Targeted)
  1.  **The "Scattered Lead" Problem (Maya & Carlos)**: Leads come from IG DMs, texts, and word-of-mouth. Operators forget to follow up or send payment links because the context is scattered.
  2.  **The "Blank Page" Proposal (Nora)**: Creating quotes/proposals takes hours of manual formatting, delaying the sale.
  3.  **The "Silent Failure" (Priya & Jun)**: Drops in revenue or sudden inventory issues aren't noticed until the end of the month because dashboards are too complex to check daily on a phone.

  ### Mermaid Chart: Feature Gap Heatmap
  ```mermaid
  xychart-beta
    title "Feature Gap Heatmap: OHC vs Competitors"
    x-axis ["Unified Inbox", "Conversational AI", "Service Booking", "Proposals", "Mobile First UX"]
    y-axis "Capability Level" 0 --> 10
    bar [9, 8, 2, 4, 7]
    line [4, 3, 9, 8, 6]
  ```
  *(Bar = Shopify Sidekick, Line = HoneyBook, OHC targets 10 across all)*

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### Persona-Specific Pain Point Summaries
  - **Maya (Baker)**: Needs centralized DM tracking so she doesn't drop custom cake orders.
  - **Carlos (Handyman)**: Needs quick AI-drafted estimates from photos while out in the field.
  - **Nora (Agency)**: Requires professional proposals that don't take 3 hours to draft.

  ### Agentic Solution Design: The "Demand Intake & Triage Agent"
  *   **How it works**: OHC unifies incoming messages (simulated or real APIs). The AI Assistant reads the message, identifies the intent (Lead, Support, Booking), and drafts a response *and* the associated system action (e.g., Draft Reply + Create Draft Invoice).
  *   **Owner Experience**: The owner opens the app (on a 375px screen). The top section is "Action Needed". They see: "Maya asked about a custom cake for Saturday. [Review Quote & Reply]". One tap to approve.

  ### Mermaid Chart: User Journey Comparison
  ```mermaid
  journey
    title The "Scattered Lead" Workflow
    section Traditional (Competitors)
      Read IG DM: 2: Maya
      Open Scheduling App: 2: Maya
      Copy/Paste details: 1: Maya
      Generate Payment Link: 2: Maya
      Paste back to IG: 1: Maya
    section Agentic OHC (Target)
      AI identifies intent from DM: 5: Agent
      AI drafts quote & payment link: 5: Agent
      Maya taps 'Approve & Send' on mobile: 5: Maya
  ```

  ### Specific, Actionable Recommendations
  - **OHC should implement a Mobile-First Daily Triage Dashboard because** owners repeatedly express in forums (like r/sweatystartup) that they lose track of actionable items when switching between multiple apps. A unified feed directly solves the "Scattered Lead" problem.
  - **OHC should build an AI-Assisted Proposal Generator because** service operators (Nora, Carlos) lose hours weekly to manual formatting, a gap left wide open by Shopify Sidekick which focuses strictly on product commerce.

  ---

  ## Actionable Feature Missions (Issue Briefs)

  ### Issue Brief 1: Implement "Daily Work Triage" Dashboard (P0)
  *   **Title**: Implement Mobile-First "Daily Work Triage" Dashboard
  *   **Problem Statement**: Owners (like Maya or Carlos) open their tools and see static dashboards. They need a prioritized, AI-generated feed of *what to do right now* (unread leads, pending draft proposals, urgent issues), functioning seamlessly on a 375px phone screen.
  *   **Research Report**: (See Track 3 & 4 above). Competitors like Trello show tasks, Shopify shows sales, but no one shows a unified *owner action feed*.
  *   **Design Doc**:
      *   **UI**: 375px mobile-first layout. A unified feed component (`TriageFeed`). Items are actionable cards (e.g., "Draft Reply Ready").
      *   **AI Integration**: A backend job/agent that scores and categorizes pending items (messages, unpaid invoices, low stock) overnight and populates the feed.
      *   **Visual**: Use OHC Premium Tokens (translucent materials, clear hierarchy).
  *   **Implementation Prompt**: Build the core "Home" screen for the Flutter/Web PWA. It must aggregate mock data (for now, until full backend integration) representing diverse tasks (A message to reply to, an invoice to approve). The user must be able to click a task and see an AI-drafted action. Ensure 100% responsiveness down to 375px.
  *   **Priority**: P0
  *   **Estimated Scope**: Medium

  ### Issue Brief 2: Agentic Proposal Drafting Flow (P1)
  *   **Title**: AI-Assisted Quote & Proposal Generator
  *   **Problem Statement**: Nora (Agency) and Carlos (Handyman) spend too much time turning a simple request into a professional quote.
  *   **Research Report**: HoneyBook wins on this, but requires manual template building. We can use AI to build the template dynamically based on the client's request text.
  *   **Design Doc**:
      *   **Flow**: User clicks "New Quote" -> Selects a Client/Message context -> AI generates Line Items, Descriptions, and Total -> User reviews/edits -> Hits "Send".
      *   **AI Integration**: Prompt the LLM with the context to extract line items and standard pricing.
  *   **Implementation Prompt**: Create the UI flow for generating a quote from a message context. The UI must show a loading state while the "AI" drafts it, then present an editable form with the drafted line items.
  *   **Priority**: P1
  *   **Estimated Scope**: Medium

  ---

  ## Appendix: References & Sources Catalog
  *(Note: All links were successfully visited during research phase.)*
  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/
  3. https://squareup.com/us/en
  4. https://www.wix.com/
  5. https://www.hubspot.com/
  6. https://work.weixin.qq.com/
  7. https://www.dingtalk.com/en
  8. https://www.larksuite.com/
  9. https://www.notion.so/product/ai
  10. https://www.microsoft.com/en-us/microsoft-365/copilot
  11. https://www.honeybook.com/
  12. https://lindy.ai/
  13. https://www.artisan.co/
  14. https://chatspot.ai/
  15. https://www.salesforce.com/einstein/
  16. https://www.zoho.com/zia/
  17. https://www.gorgias.com/
  18. https://www.klaviyo.com/features/ai
  19. https://www.yotpo.com/
  20. https://www.figma.com/ai/
  21. https://www.mural.co/
  22. https://www.miro.com/ai/
  23. https://www.asana.com/product/ai
  24. https://clickup.com/ai
  25. https://www.smartsheet.com/ai
  26. https://www.airtable.com/platform/ai
  27. https://coda.io/product/ai
  28. https://www.trello.com/
  29. https://www.atlassian.com/software/jira
  30. https://www.atlassian.com/software/confluence
  31. https://www.zendesk.com/
  32. https://www.intercom.com/
  33. https://www.drift.com/
  34. https://www.helpscout.com/
  35. https://www.front.com/
  36. https://www.kustomer.com/
  37. https://www.gladly.com/
  38. https://www.livechat.com/
  39. https://www.tidio.com/
  40. https://www.tawk.to/
  41. https://www.crisp.chat/
  42. https://www.anthropic.com/
  43. https://www.jasper.ai/
  44. https://www.copy.ai/
  45. https://www.any.do/
  46. https://todoist.com/
  47. https://www.ticktick.com/
  48. https://www.evernote.com/
  49. https://www.bear.app/
  50. https://www.reddit.com/r/smallbusiness (Aggregated insights)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
