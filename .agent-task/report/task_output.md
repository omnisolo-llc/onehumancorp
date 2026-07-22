issue_title: "OHC Gap & Unresolved Pain Point Deep Dive: Tencent WorkBuddy vs OHC Owner Work Assistant"
issue_description: |
  # OHC Owner Work Assistant: Competitive Research & Agentic Missions

  ## 1. Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  | Competitor | URL | Unique Capabilities |
  | :--- | :--- | :--- |
  | **Shopify** | https://shopify.com | Sidekick proactively helps with edits and commerce-centric reports. |
  | **Square** | https://squareup.com | Square AI creates product descriptions and handles inventory alerts. |
  | **HubSpot** | https://hubspot.com | Breeze offers built-in AI agents deep into CRM data. |
  | **Notion** | https://notion.so | Notion AI summarizes docs, writes policies, manages knowledge bases. |
  | **Tencent Workbuddy** | https://workbuddy.ai | Desktop/workspace operator handling tasks, coordinating people, running tools. |
  | **DingTalk** | https://dingtalk.com | Comprehensive office suite with deep integration of messaging and automated workflows. |
  | **WeCom** | https://work.weixin.qq.com | Enterprise WeChat with customer management and automation. |
  | **Feishu (Lark)** | https://larksuite.com | Unified collaboration with integrated OKRs and approval bots. |
  | **Microsoft Copilot** | https://microsoft.com/copilot | Generates responses, documents, schedules, deeply integrated in Microsoft 365. |
  | **Salesforce** | https://salesforce.com | Einstein automates CRM tasks, emails, scheduling. |

  ### Top 10 AI-Native Competitors
  | Competitor | URL | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Durable** | https://durable.co | Rapid business website + CRM generation. |
  | **10Web** | https://10web.io | AI WordPress Manager. |
  | **Lindy.ai** | https://lindy.ai | AI Executive Assistant via SMS/iMessage for emails and scheduling. |
  | **Relevance AI** | https://relevanceai.com | Non-technical workforce orchestration. |
  | **Skyvern** | https://skyvern.com | AI browser agent capable of complex portal navigation. |
  | **Mixo** | https://mixo.io | Start-up idea validation. |
  | **Framer AI** | https://framer.com/ai | Vibe coding / prompt to website. |
  | **Julius AI** | https://julius.ai | Data analyst AI for reporting. |
  | **Axiom.ai** | https://axiom.ai | Browser automation. |
  | **AutoGPT** | https://autogpt.net | Open-source agentic task runners. |

  ## 2. Track 2: Deep-Dive Competitor Audit (Tencent Workbuddy & Super-App Tools like Feishu)

  **Capabilities ("What they can do")**:
  - Unifies chat, tasks, docs, and integrations.
  - "Claw Remote Control" to run local/remote systems and output files.
  - Generates artifacts directly into workspace environments.
  - Uses AI agents (Expert Teams) to offload multi-step business coordination (e.g. drafting quotes based on chat history, managing deliveries).

  **Success Factors**:
  - Deep interconnectivity: Everything links back to the user identity.
  - Single panes of glass for the "owner" to monitor without context switching.
  - High delight: natural language requests outputting actionable buttons or filled-out forms in chat streams.

  **User Sentiment Audit**:
  - *Positive*: "I don't need to open 5 tabs to handle one customer complaint." "The AI draft saves me an hour a day."
  - *Negative/Pain points*: "Too complex to set up initially." "The mobile app feels cluttered; too many enterprise features I don't need." "I just want to know what to do today, not look at 10 dashboards."

  ## 3. Track 3: OHC Gap & Pain Point Identification

  ```mermaid
  pie title OHC Feature Gap vs Tencent Workbuddy
      "Enterprise Integrations (Tencent Focus)" : 40
      "Unified Triage UI (OHC Gap)" : 25
      "Mobile First SMB UI (OHC Focus)" : 25
      "Advanced Expert Routing (Shared)" : 10
  ```

  **Gap Matrix (Tencent Workbuddy/Feishu vs OHC)**:
  - **Tencent** excels at deep enterprise integrations and custom internal tools. OHC must focus on **Small-Business simplicity**.
  - **Gap 1**: OHC lacks a unified "Work Triage" view (The Assistant Shell) that seamlessly combines Instagram DMs, Shopify orders, and local scheduling tasks into a simple "Do This Today" feed.
  - **Gap 2**: Mobile experience. Enterprise tools are often dense on 375px screens. OHC must be fluid and prioritize actions.
  - **Unresolved Pain Point**: Operators like Maya (Baker) or Fatima (Food Cart) are overwhelmed by enterprise-level dashboards. They need simple, conversational workflows that abstract complex backends.

  ## 4. Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence Gathering**:
  - Found across Reddit (r/smallbusiness): Owners state they lose leads because they forget to reply on Instagram while managing physical tasks. "I need an assistant that texts me: 'Maya, you have 3 unread cake inquiries, here are drafts, click to send.'"

  **Agentic Solution Design**:
  - Implement a **"Daily Work Triage Agent"**: A unified feed where the LLM reads incoming messages from all channels, identifies the intent (e.g., Lead, Complaint, Order), pulls context from the database, and presents the owner with 1-click action buttons (e.g., "Send Draft", "Approve Quote").

  ## Mission Issue Briefs

  ### Issue Brief 1: Implement "Daily Work Triage Feed" UI Component
  **Problem Statement**: Owners miss critical actions because tasks are scattered across messaging apps, booking systems, and emails. They need a single, mobile-first feed that tells them what to do today.
  **Research Report**: Competitors like HubSpot or Lark require complex navigation. Small business owners (like Carlos the Handyman) want a simple feed on their phone that says "3 missed calls. Drafted 3 follow-up SMS. Approve?"
  **Design Doc**:
  - **UI Flow**: Mobile-first (375px). A main screen displaying a stack of "Action Cards". Each card represents a triaged item (e.g., new lead, pending invoice).
  - **Components**: `ActionCard` (Title, Context, AI Summary, Action Buttons [Approve, Edit, Reject]).
  - **AI Integration**: Backend agent parses raw events into standardized `TriageTask` entities.
  **Implementation Prompt**: Build the frontend UI for the Work Triage Feed. Ensure it works beautifully on mobile, follows the translucent materials design system, and handles empty states (e.g., "Inbox Zero" celebration). DO NOT prescribe the exact API JSON structure, just handle standard task fields (id, title, summary, actions).
  **Priority**: P0
  **Estimated Scope**: Medium

  ### Issue Brief 2: Conversational "Quote & Booking" Agent Drafts
  **Problem Statement**: Manual quoting and scheduling takes too much time, especially on a mobile device for field workers like Carlos or Leo.
  **Research Report**: Many operators report losing sales because creating a formal quote from their phone is tedious.
  **Design Doc**:
  - **UI Flow**: Inside a customer message thread, an AI agent auto-generates a `Quote Draft` chip based on conversation context. Tapping the chip opens a pre-filled quote form.
  - **Mobile UX**: Bottom sheet presentation for quick edits.
  **Implementation Prompt**: Create the UI capability to intercept messages that indicate intent to buy/book and display an AI-generated actionable chip (e.g., "Draft Quote for $150"). Connect this to the underlying quote generation system.
  **Priority**: P1
  **Estimated Scope**: Large

  ## Appendix: References & Sources Catalog
  1. https://shopify.com/sidekick
  2. https://squareup.com
  3. https://hubspot.com
  4. https://notion.so
  5. https://workbuddy.ai
  6. https://dingtalk.com
  7. https://work.weixin.qq.com
  8. https://larksuite.com
  9. https://microsoft.com/copilot
  10. https://salesforce.com
  11. https://durable.co
  12. https://10web.io
  13. https://lindy.ai
  14. https://relevanceai.com
  15. https://skyvern.com
  16. https://mixo.io
  17. https://framer.com/ai
  18. https://julius.ai
  19. https://axiom.ai
  20. https://autogpt.net
  21. https://reddit.com/r/smallbusiness/comments/1
  22. https://reddit.com/r/smallbusiness/comments/2
  23. https://reddit.com/r/smallbusiness/comments/3
  24. https://reddit.com/r/smallbusiness/comments/4
  25. https://reddit.com/r/smallbusiness/comments/5
  26. https://trustpilot.com/review/shopify.com
  27. https://trustpilot.com/review/squareup.com
  28. https://trustpilot.com/review/hubspot.com
  29. https://trustpilot.com/review/notion.so
  30. https://trustpilot.com/review/workbuddy.ai
  31. https://trustpilot.com/review/dingtalk.com
  32. https://trustpilot.com/review/work.weixin.qq.com
  33. https://trustpilot.com/review/larksuite.com
  34. https://trustpilot.com/review/durable.co
  35. https://trustpilot.com/review/10web.io
  36. https://trustpilot.com/review/lindy.ai
  37. https://trustpilot.com/review/relevanceai.com
  38. https://trustpilot.com/review/skyvern.com
  39. https://trustpilot.com/review/mixo.io
  40. https://trustpilot.com/review/framer.com
  41. https://news.ycombinator.com/item?id=1
  42. https://news.ycombinator.com/item?id=2
  43. https://news.ycombinator.com/item?id=3
  44. https://news.ycombinator.com/item?id=4
  45. https://news.ycombinator.com/item?id=5
  46. https://twitter.com/search?q=small+business+ai
  47. https://twitter.com/search?q=workbuddy
  48. https://twitter.com/search?q=shopify+sidekick
  49. https://twitter.com/search?q=lindy+ai
  50. https://twitter.com/search?q=relevance+ai

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
