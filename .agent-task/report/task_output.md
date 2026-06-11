issue_title: "OHC Mission: Implement Agentic Operations Triage & Customer Follow-up"
issue_description: |
  # OHC Mission Brief: Agentic Operations Triage & Customer Follow-up

  ## Problem Statement
  Small business owners (like Maya the baker, Carlos the handyman, and Fatima the food cart operator) are overwhelmed by fragmented communication channels and manual triage. While general CRMs (like HubSpot) and commerce platforms (like Shopify or Square) manage data and transactions, they fail to act as true **assistants**. Owners have to read dashboards, connect dots manually across DMs, emails, and orders, and manually draft responses or create tasks. The gap is not data storage; the gap is **attention direction and action execution**. They need a system that translates inbound chaos into a prioritized daily plan and drafts contextual responses automatically.

  ## Track 1: Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Shopify (Inbox & Admin)**: High e-commerce utility but dashboard-heavy; requires manual administration.
  2. **Square**: Excellent POS and offline integration but weak cross-channel DM/lead triage.
  3. **HubSpot**: Powerful CRM but complex, technical, and built for B2B sales teams, not operators.
  4. **Wix**: Good all-in-one builder, lacks proactive AI operations.
  5. **Tencent Workbuddy / WeCom**: Excellent chat-to-task pipelines (Asia market), but not adapted for western SMBs.
  6. **DingTalk**: Deeply integrated into operations but heavily structured around corporate hierarchy.
  7. **Notion AI**: Great for knowledge, lacks transactional commerce and scheduling execution.
  8. **Microsoft Copilot for SMB**: Good for Office files, disconnected from point-of-sale and live operations.
  9. **Feishu / Lark**: High operational ceiling, but built for medium/large teams.
  10. **HoneyBook**: Good for independent service providers, but lacks deep physical goods/commerce integration.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: Rising e-commerce copilot, though heavily text-chat oriented rather than proactive UI.
  2. **Auto-GPT / AgentGPT variants for business**: Too abstract, require prompt engineering.
  3. **Harvey (Legal AI)**: Vertical-specific, shows the power of contextual memory.
  4. **Motion**: AI scheduling, but lacks commerce/CRM context.
  5. **Glean**: Excellent knowledge search, but read-only.
  6. **Intercom Fin**: Good for support, but doesn't handle operational task generation.
  7. **Siena AI**: Great AI customer service for commerce, but lacks back-office scheduling.
  8. **Lindsey AI**: Property management AI, great vertical execution.
  9. **Bland AI**: Voice AI agent for phone calls; powerful but a point solution.
  10. **Akkio**: AI analytics for agencies, lacks transactional execution.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick & Inbox

  **Capabilities ("What they can do")**:
  Shopify Sidekick is an AI assistant inside the Shopify admin that can answer questions about the store's data, perform basic tasks (like creating discount codes), and summarize sales. Shopify Inbox centralizes chat.

  **Success Factors**:
  - Integrated directly into the source of truth (orders, products).
  - Natural language interface lowers the barrier to complex queries ("Why are sales down?").
  - Immediate context of customer carts and history.

  **User Sentiment Audit**:
  - *Love*: "It's like having a data analyst on staff." "Creating a discount code by just asking is magic."
  - *Pain Points*: "Inbox still requires me to manually reply to 50 DMs a day." "Sidekick tells me what happened, but doesn't do the work of following up with my delayed suppliers." "I can't run my service/booking business on Shopify easily, so Sidekick doesn't help me schedule Carlos's route."

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit vs Shopify Sidekick / Square Matrix**:

  | Feature | Shopify / Sidekick | Square | OneHumanCorp (Current) | OHC (Vision) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Unified Inbox** | Yes (Shopify Inbox) | Yes (Messages) | Basic | **Proactive Agentic Triage** |
  | **AI Task Generation** | No (Manual via Flow) | No | Missing | **Automatic from DMs** |
  | **Contextual Draft Replies**| Partial (Suggested replies) | No | Missing | **Full AI drafting with tone** |
  | **Service Scheduling** | Weak (Needs apps) | Strong | Missing | **Integrated booking & routing** |
  | **Proactive Summaries** | Yes (Sidekick queries)| No | Missing | **Daily push summaries** |

  **Unresolved Pain Points**:
  Owners (like Maya and Carlos) need an assistant that doesn't just centralize messages, but **reads them, identifies the intent (Lead, Support, Booking), checks availability/inventory, and drafts a complete response or quote for 1-tap approval**. The tool must also render beautifully on a 375px screen, unlike Shopify Admin which is complex on mobile.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  **Deep-Dive Evidence Gathering**:
  Based on extensive community research (r/smallbusiness, r/ecommerce), operators spend 2-3 hours daily just triaging "Do you have this?" or "Can I book for Tuesday?" They don't want a better inbox; they want the inbox processed.

  **Agentic Solution Design**:
  1. **Work Triage Agent**: Monitors inbound (email, DM, form). Classifies intent (e.g., `INTENT_BOOKING`).
  2. **Operations Check**: If `INTENT_BOOKING`, Agent queries Postgres/Redis for available slots.
  3. **Customer Assistant Agent**: Drafts reply ("Hi! I have Tuesday at 2 PM available. Should I lock it in?") and attaches a pending task/quote.
  4. **Owner Review UI (375px first)**: The owner opens the app. Sees "3 Urgent Approvals". Taps "Approve & Send".

  ---

  ## Design Doc: High-Level Architecture & UX

  **Architecture**:
  - **Triggers**: Webhook ingress (Instagram, Email, Forms) -> AI Job Queue (Postgres `SKIP LOCKED`).
  - **Processing**: Worker nodes pick up job, call Gemini Pro / GPT-4o with `tenant_scoped_memory`.
  - **Entities**: `InboundMessage`, `AgentDraft`, `ProposedAction` (e.g., `CreateQuote`, `ScheduleTask`).
  - **Storage**: Store draft responses in `agent_drafts` table linked to `tenant_id` (RLS enforced).

  **UX Flow (Mobile 375px First)**:
  1. **Home Screen**: "Good morning Maya. You have 3 draft replies for custom cakes."
  2. **Triage Card**: Displays customer message + Agent drafted response + Proposed Action (Generate Deposit Link).
  3. **Action Area**: Two massive 44x44px touch targets: [Edit] [Approve & Send].

  ---

  ## Implementation Prompt

  **Target User**: Maya (Home Baker) / Carlos (Handyman)
  **User-Facing Outcome**: When a customer messages them, OHC automatically creates a draft reply and proposes a concrete system action (like creating a calendar booking or a payment link). The owner only has to tap "Approve".
  **Critical User Journey (CUJ)**:
  1. System receives a mock inbound customer message.
  2. Work Triage agent processes it and creates a `DraftResponse` and `ProposedTask`.
  3. Owner logs in, sees the triage dashboard, clicks the drafted item, and clicks "Approve".
  4. The system sends the message and creates the underlying task/quote.

  **Acceptance Criteria**:
  - Must implement the Agent Triage pipeline via AI Job Queue.
  - Must display a "Pending Approvals" feed on the UI.
  - Must be fully responsive down to 375px.
  - Must have 100% test coverage for the triage logic.
  - Must have a Playwright E2E test covering the Owner Approval CUJ.
  - ZERO mock data in the final UI (data must flow from the DB).

  ---

  ## References & Sources Catalog

  ```mermaid
  graph TD;
      Inbound[Inbound Messages/Forms] --> AI_Triage[Work Triage Agent];
      AI_Triage --> Intent[Classify Intent];
      Intent --> |Booking| Ops[Check Availability];
      Intent --> |Sale| Comm[Check Inventory];
      Ops --> Draft[Customer Assistant Drafts Reply];
      Comm --> Draft;
      Draft --> Owner[Owner 1-Tap Approval UI];
      Owner --> Exec[Action Executed];
  ```

  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/inbox
  3. https://squareup.com/us/en/point-of-sale
  4. https://www.hubspot.com/products/crm
  5. https://www.notion.so/product/ai
  6. https://copilot.microsoft.com/
  7. https://www.wecom.qq.com/
  8. https://www.dingtalk.com/en
  9. https://www.larksuite.com/
  10. https://www.honeybook.com/
  11. https://www.intercom.com/fin
  12. https://www.siena.cx/
  13. https://www.glean.com/
  14. https://bland.ai/
  15. https://www.akkio.com/
  16. https://www.usemotion.com/
  17. https://agentgpt.reworkd.ai/
  18. https://www.harvey.ai/
  19. https://lindsey.ai/
  20. https://reddit.com/r/smallbusiness/comments/example1/crm_help
  21. https://reddit.com/r/smallbusiness/comments/example2/too_many_dms
  22. https://reddit.com/r/ecommerce/comments/example3/shopify_inbox_review
  23. https://reddit.com/r/ecommerce/comments/example4/ai_for_store
  24. https://trustpilot.com/review/www.shopify.com
  25. https://trustpilot.com/review/squareup.com
  26. https://trustpilot.com/review/www.hubspot.com
  27. https://trustpilot.com/review/www.wix.com
  28. https://trustpilot.com/review/www.honeybook.com
  29. https://apps.apple.com/us/app/shopify-inbox/id1450682192
  30. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  31. https://play.google.com/store/apps/details?id=com.shopify.inbox
  32. https://play.google.com/store/apps/details?id=com.squareup
  33. https://www.ycombinator.com/companies/industry/b2b-software
  34. https://techcrunch.com/2023/07/12/shopify-sidekick/
  35. https://www.theverge.com/2023/7/12/23792036/shopify-ai-sidekick-ecommerce-assistant
  36. https://www.bloomberg.com/news/articles/2023-07-26/shopify-adds-ai-assistant-to-help-merchants
  37. https://www.cnbc.com/2023/07/12/shopify-announces-new-ai-features-including-sidekick-assistant.html
  38. https://www.wired.com/story/shopify-sidekick-ai/
  39. https://www.forbes.com/sites/forbestechcouncil/2023/08/15/how-ai-is-changing-ecommerce/
  40. https://hbr.org/2023/09/how-generative-ai-is-changing-the-way-we-work
  41. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai
  42. https://www.bain.com/insights/generative-ai/
  43. https://www.bcg.com/capabilities/artificial-intelligence/generative-ai
  44. https://stripe.com/docs/api/checkout/sessions
  45. https://stripe.com/docs/api/payment_intents
  46. https://material.io/design
  47. https://developer.apple.com/design/human-interface-guidelines/
  48. https://flutter.dev/showcase
  49. https://pub.dev/packages/flutter_riverpod
  50. https://bazel.build/concepts/build-ref
  51. https://grpc.io/docs/what-is-grpc/
  52. https://opentelemetry.io/docs/
  53. https://redis.io/docs/manual/patterns/distributed-locks/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
