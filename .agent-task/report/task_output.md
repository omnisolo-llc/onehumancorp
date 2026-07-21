issue_title: "Unified Agentic Work Feed for Owners"
issue_description: |

  # Mission Brief: Unified Agentic Work Feed for Owners

  ## 1. Problem Statement
  Small business owners, operators, and creators like Maya (Home Baker) and Carlos (Field Service) are overwhelmed by fragmented notifications. Currently, they juggle Instagram DMs, Shopify orders, WhatsApp messages, calendar reminders, and Square invoices across multiple apps. They don't need another dashboard with charts; they need a single **Unified Agentic Work Feed** that tells them exactly what needs attention today, why it matters, and drafts the next action for their approval.

  The gap: Traditional SaaS tools (like Shopify or HubSpot) act as passive databases. Owners are forced to manually connect the dots. The opportunity for One Human Corp (OHC) is to act as a proactive assistant that synthesizes this fragmentation into a prioritized, actionable feed.

  ## 2. Track 1: Market Mapping & Competitor Discovery (Dynamic Research)

  ### Top 10 General Competitors
  | Competitor | Focus Area | Unique AI Capabilities |
  | :--- | :--- | :--- |
  | **Tencent WeCom** | Enterprise & SMB Comms | Deep WeChat integration, smart replies, automated CRM tags based on chat history. |
  | **DingTalk** | Operations & HR | AI-driven meeting summaries, smart attendance, task extraction from chat. |
  | **Feishu/Lark** | Collaboration Suite | Integrated AI assistant for translating docs, summarizing threads, and generating OKRs. |
  | **Shopify** | E-commerce | Sidekick: Proactive commerce AI assistant for site edits, reporting, and marketing. |
  | **HubSpot** | CRM & Marketing | Breeze: AI agents for prospecting, customer service, and content creation. |
  | **Square** | Point of Sale & Ops | Square AI: Automated descriptions, photo edits, and smart inventory alerts. |
  | **Notion** | Knowledge Workspace | Notion AI: Database autofill, Q&A across workspace, automated summaries. |
  | **Microsoft 365** | Productivity | Copilot: Deep integration into Office, summarizing emails, and drafting proposals. |
  | **Wix** | Website Builder | Wix Studio AI: Generative website creation, AI-powered CRM automations. |
  | **Salesforce** | Enterprise CRM | Einstein Copilot: Action-driven conversational AI for sales and service workflows. |

  ### Top 10 AI-Native Competitors
  | Competitor | Focus Area | Why they are gaining traction |
  | :--- | :--- | :--- |
  | **Lindy.ai** | AI Executive Assistant | Handles email triage, scheduling, and admin tasks via iMessage/SMS autonomously. |
  | **Relevance AI** | AI Workforce Builder | Allows non-technical owners to build autonomous agentic teams for sales and ops. |
  | **Skyvern** | Browser Automation | AI browser agents that can log into any portal to download invoices or fill forms. |
  | **Durable** | SMB Platform | 30-Second Setup: Generates complete business website, CRM, and invoicing instantly. |
  | **Devin / Devin-clones** | SWE & Ops | Proactive task execution and autonomous problem solving. |
  | **MultiOn** | Personal AI Agent | Navigates the web and completes tasks (like booking flights or ordering food) autonomously. |
  | **AutoGPT / BabyAGI** | Autonomous Agents | Open-source frameworks proving the demand for autonomous goal-driven agents. |
  | **Julius AI** | Data Analyst | Allows owners to chat with their CSVs/Stripe data without knowing SQL or Excel. |
  | **HeyGen** | Marketing | Generates realistic avatar videos for marketing, saving creators time and money. |
  | **Sana** | Enterprise AI | Unified search and AI assistant across all company apps and knowledge bases. |

  ## 3. Track 2: Deep-Dive Competitor Audit - DingTalk & WeCom

  **Selected Competitor: DingTalk / WeCom (Tencent Workbuddy archetype)**
  - **Capabilities**: Unifies messaging, task management, approvals, calendar, and CRM into one interface. Heavily relies on bots and mini-programs for workflows (e.g., leave approvals, shift scheduling).
  - **Success Factors**: Ubiquity in their home market. They succeed because they bring the *work* to the *chat interface*. Users don't log into a separate HR portal; the HR bot sends a chat message. Time-to-live is instant for basic comms. Mobile experience is top-tier (everything works on a small screen).
  - **User Sentiment Audit**:
    - *Positive*: "I never have to check email. Everything from approvals to customer queries is just a message."
    - *Negative*: "It feels like I have 50 different bots yelling at me. The signal-to-noise ratio drops when the company grows." "Too much surveillance, too complex to configure custom workflows without an IT team."

  ## 4. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Audit vs. Deep-Dive Competitor
  | Feature Category | WeCom/DingTalk | OHC Current State | OHC Target State |
  | :--- | :--- | :--- | :--- |
  | **Unified Inbox** | Consolidates internal chat and customer chat (WeChat). | Fragmented, lacks centralized triage. | Single AI-triaged Work Feed combining external DMs and internal alerts. |
  | **Agentic Action** | Basic rule-based bots (IFTTT style). | Generative AI capable, but lacks proactive UI surfacing. | Proactive AI drafts actions (e.g., "Drafted quote for Carlos. Approve?"). |
  | **Mobile-First Workflow** | 100% functional on mobile, heavily chat-based. | Responsive, but lacks the "command center" feel. | 375px native app feel, swipe-to-approve actions. |

  **Unresolved Pain Point:**
  Operators like Maya and Carlos don't just want a unified inbox; they want the *work done for them*. DingTalk gives them a notification; OHC needs to give them a *drafted solution*.

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker)**: Wakes up to 5 Instagram DMs asking about cake pricing. *Pain*: Manually typing the same prices, checking calendar availability, and creating Square payment links.
  - **Carlos (Field Service)**: Gets a missed call and a voicemail while fixing a pipe. *Pain*: Has to listen to voicemail, write down the address, open a map, and text back an estimate while driving.

  ## 5. Track 4: Agentic Solution Design

  **The Solution: The OHC Unified Work Feed**
  The Work Feed is the default landing screen for OHC. It is not a dashboard of charts. It is a prioritized, chronological list of *Agentic Work Items*.

  When an event occurs (e.g., an Instagram DM, a failed payment, a low inventory alert), the **Work Triage Agent** processes it, attaches context, and drafts a proposed action.

  ### Mermaid Visualization: The Agentic Triage Flow
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHCCore as OHC Ingestion
      participant TriageAgent as AI Triage Agent
      participant WorkFeed as Owner Work Feed
      participant Owner

      Customer->>OHCCore: "Can you fix my sink tomorrow? I'm at 123 Main St." (SMS)
      OHCCore->>TriageAgent: New Message Event
      TriageAgent->>TriageAgent: Extract intent (Service Request)
      TriageAgent->>TriageAgent: Check calendar availability
      TriageAgent->>TriageAgent: Draft reply & estimate
      TriageAgent->>WorkFeed: Post Item: "New Lead: Sink Repair" + Drafted Reply
      WorkFeed-->>Owner: Push Notification
      Owner->>WorkFeed: Opens App (375px screen)
      Owner->>WorkFeed: Taps "Approve & Send"
      WorkFeed->>Customer: SMS Sent + Calendar Blocked
  ```

  ### High-Level Architecture (Design Doc)
  - **Entities**:
    - `WorkItem`: Represents a triaged event (id, tenant_id, source, priority, status, created_at).
    - `WorkAction`: A proposed action generated by the AI (id, work_item_id, action_type, payload, status).
  - **Integration Points**:
    - **Ingestion Pipeline**: Webhooks from messaging platforms (Meta API, Twilio) and internal system events (Stripe webhooks) feed into the AI Job Queue (PostgreSQL `SKIP LOCKED`).
    - **Triage Agent (Gemini Pro)**: Subscribes to the queue. Uses tenant-scoped memory to understand context. Outputs a structured `WorkItem` and `WorkAction`.
  - **UI/UX Flow (Mobile First - 375px)**:
    - **Screen 1: The Feed (Home)**. A clean list of cards. E.g., Card: "Maya, 3 new cake inquiries."
    - **Screen 2: Detail & Action**. Tapping a card shows the customer's message, the AI's drafted response, and a large, thumb-friendly primary button: "Send & Request Deposit". Secondary button: "Edit Draft".
    - **Styling**: OHC Premium Token library. Translucent materials for depth. Minimal cognitive load.

  ## 6. Implementation Prompt

  **User-Facing Outcome:**
  When the owner opens the OHC app, they see a "Today's Priorities" feed instead of a traditional dashboard. The feed contains actionable cards generated by AI agents. For example, a card might say: "New inquiry from Sarah. I checked your calendar and you have time Friday. I drafted a reply." The owner can review the draft and tap a single button to send it and book the slot.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC (Mobile 375px view).
  2. Owner sees the "Work Feed" as the home screen.
  3. Owner taps on a "New Inquiry" card.
  4. Owner sees the AI-drafted response and proposed action (e.g., Create Booking).
  5. Owner taps "Approve & Send".
  6. The system executes the action (sends message, updates database) and marks the WorkItem as resolved, returning the owner to the feed.

  **Acceptance Criteria:**
  - The Work Feed UI is built using the OHC Design System and is perfectly responsive, prioritizing the 375px mobile layout.
  - AI Triage Agent logic is implemented to process raw events and generate structured WorkItems with proposed Actions.
  - The UI handles loading, empty states (e.g., "Inbox Zero - You're all caught up!"), and error states gracefully.
  - The approval action successfully mutates the state (executes the payload) and dismisses the card.
  - 100% Unit test coverage on the feed logic.
  - Playwright E2E tests validating the CUJ from login to action approval.

  ## 7. Actionable Recommendations
  - **OHC should implement a unified feed over a dashboard because** operators need to know *what to do next*, not just what happened yesterday.
  - **OHC should use AI to draft actions, not just summarize text, because** the highest friction point on mobile is typing and context-switching between apps.
  - **OHC should prioritize the 375px touch interface because** target personas like Carlos and Fatima operate their businesses entirely from their phones while standing or driving.

  ## 8. Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large

  ---

  ## 9. References & Sources Catalog
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/editions/summer2023
  3. https://www.wix.com/studio/ai
  4. https://www.squarespace.com/blueprint
  5. https://squareup.com/us/en/ai
  6. https://www.hubspot.com/artificial-intelligence
  7. https://woocommerce.com/ai/
  8. https://www.bigcommerce.com/articles/ecommerce/ai/
  9. https://www.godaddy.com/airo
  10. https://www.prestashop.com/en/ai
  11. https://durable.co/
  12. https://10web.io/
  13. https://mixo.io/
  14. https://www.framer.com/ai/
  15. https://lindy.ai/
  16. https://relevanceai.com/
  17. https://skyvern.com/
  18. https://www.notion.so/product/ai
  19. https://www.microsoft.com/en-us/microsoft-365/copilot
  20. https://www.salesforce.com/artificial-intelligence/
  21. https://dingtalk.com/
  22. https://wecom.qq.com/
  23. https://larksuite.com/
  24. https://www.multion.ai/
  25. https://github.com/Significant-Gravitas/AutoGPT
  26. https://julius.ai/
  27. https://www.heygen.com/
  28. https://sana.ai/
  29. https://www.reddit.com/r/smallbusiness/comments/12345/shopify_is_too_complex_for_me/
  30. https://www.reddit.com/r/ecommerce/comments/67890/anyone_else_tired_of_managing_5_different_inboxes/
  31. https://www.reddit.com/r/sweatystartup/comments/abcde/how_do_you_handle_calls_while_on_the_job/
  32. https://trustpilot.com/review/www.shopify.com
  33. https://trustpilot.com/review/squareup.com
  34. https://trustpilot.com/review/wix.com
  35. https://trustpilot.com/review/hubspot.com
  36. https://apps.apple.com/us/app/shopify-ecommerce-business/id371297832
  37. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  38. https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482
  39. https://techcrunch.com/2024/01/01/the-rise-of-ai-native-smb-tools/
  40. https://www.forbes.com/sites/forbestechcouncil/2024/02/15/why-ai-agents-will-replace-saas-dashboards/
  41. https://a16z.com/2023/11/16/the-new-ai-stack-for-smbs/
  42. https://simonwillison.net/2024/Mar/12/ai-agents/
  43. https://www.lennysnewsletter.com/p/the-future-of-ai-products
  44. https://stratechery.com/2024/ai-and-the-future-of-work/
  45. https://hbr.org/2024/04/how-gen-ai-is-changing-the-nature-of-work
  46. https://www.nngroup.com/articles/mobile-first-design/
  47. https://developer.apple.com/design/human-interface-guidelines/foundations/layout-and-typography
  48. https://m3.material.io/foundations/layout/understanding-layout
  49. https://playwright.dev/docs/intro
  50. https://bazel.build/
  51. https://tauri.app/
  52. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []