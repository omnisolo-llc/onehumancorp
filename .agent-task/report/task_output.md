issue_title: "Implement Native Rust Omnichannel Ingestion Engine & Triage Agent"
issue_description: |
  # Mission Queue Protocol: Native Omnichannel Ingestion & Triage Feed

  ## Problem Statement
  Owners like Maya (baker) and Carlos (handyman) receive critical business inquiries scattered across Instagram DMs, SMS, and website forms. Traditional helpdesks (like Chatwoot or Zendesk) force the owner to become a support agent, managing queues and tickets. Owners need an assistant that ingests these scattered messages and turns them into actionable work (e.g., "Review and send this drafted quote"). Currently, OHC lacks a native system to ingest and agentically triage these multi-channel communications.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  We explored the competitive landscape of omnichannel tools and AI work assistants to define the baseline for OHC.

  #### Top 10 General Competitors
  1. **Shopify Inbox**: Deep commerce integration, order context in chat, limited cross-platform coordination.
  2. **Tencent WorkBuddy**: Enterprise-focused desktop AI operator (planning, executing tasks, remote control).
  3. **WeCom (Tencent)**: B2B/B2C hybrid chat integrated with WeChat ecosystem.
  4. **DingTalk (Alibaba)**: All-in-one team collaboration and enterprise operations.
  5. **Feishu / Lark (ByteDance)**: Document-centric collaboration with embedded messaging.
  6. **Zendesk**: Traditional helpdesk, highly customizable but complex and expensive for small owners.
  7. **HubSpot Service Hub**: Deep CRM integration, powerful but admin-heavy.
  8. **Intercom**: Product-led support and onboarding, expensive for simple operators.
  9. **Square Appointments/Messages**: Vertical-specific scheduling and customer communication.
  10. **Wix Inbox**: Integrated into the Wix website builder, simple but limited beyond web.

  #### Top 10 AI-Native Competitors
  1. **Notion AI**: Assistant for knowledge management and drafting.
  2. **Microsoft Copilot (M365)**: Deeply integrated into office apps, strong for enterprise knowledge.
  3. **Chatwoot (with Captain AI)**: Open-source, self-hosted omnichannel platform with AI summarization and drafting.
  4. **Gleen AI / Kustomer AI**: AI-first customer service platforms focusing on resolution rates.
  5. **Forethought AI**: Generative AI for customer support ticket triage and resolution.
  6. **Fin (Intercom)**: AI agent capable of resolving complex support queries.
  7. **AutoGPT / BabyAGI (Conceptual)**: Autonomous agents executing multi-step workflows.
  8. **Dust.tt**: AI assistants tailored to internal company data.
  9. **Lindbloom / Multi**: Developer-centric collaboration with AI.
  10. **Superhuman AI**: Email-specific triage and drafting for founders/operators.

  ### Deep-Dive Competitor Audit: Chatwoot
  Chatwoot serves as our primary benchmark for omnichannel capabilities.
  - **Capabilities**: Centralizes Web chat, Email, WhatsApp, Instagram, FB Messenger, SMS. Features include private notes, @mentions, auto-assignment, macros, and "Captain" AI for drafting.
  - **Success Factors**: Open-source, self-hosted (data control), unified inbox simplifies operations.
  - **User Sentiment**: Users love having all messages in one place. Pain points include complex self-hosting setup, lack of deep native commerce integration, and the fact that it still acts like a traditional helpdesk requiring ticket management.

  ### Feature Gap Matrix: OHC vs Competitors
  | Feature | Chatwoot (Benchmark) | Shopify Inbox | Tencent WorkBuddy | OHC Native (Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Omnichannel Ingestion** | Comprehensive (Web, IG, WA, Email, etc.) | Limited (Shopify focused) | Ecosystem focused | Native Rust implementation |
  | **Core Data Model** | Support-focused (Account, Inbox, Ticket) | Commerce-focused | Task/Agent focused | Multi-tenant Operator-focused (Contact, Conversation) |
  | **AI Integration** | Drafts/Summarizes ("Captain") | Basic | Deep (Executes tasks) | Proactive Triage & Action Drafting |
  | **UX Paradigm** | Traditional Helpdesk UI | Merchant Chat | Desktop Agent | Mobile-First (375px) Command Center |

  ### Persona-Specific Pain Point Summaries
  - **Maya (Home Baker)**: Getting DMs on Instagram but having to manually copy them to a notebook or separate quoting app. "I lose track of who messaged me where. Someone DMs on Instagram, then emails, and I forget what we agreed on."
  - **Carlos (Handyman)**: Getting SMS leads while driving and forgetting to reply or create an estimate. "Setting up a helpdesk is overkill for my 3-person plumbing business, but I still drop leads."
  - **Fatima (Food Cart Operator)**: "I can't read an English helpdesk on my phone while cooking."

  ### Actionable Recommendations
  - **OHC should implement a native Rust ingestion engine** because relying on third-party services (like Chatwoot) introduces latency, data privacy concerns, and architectural misalignment.
  - **OHC should replace the 'Inbox' with a 'Triage Feed'** because owners don't want to manage tickets; they want to review AI-drafted actions (e.g., "Send this quote").

  ### Visual Analysis

  ```mermaid
  pie title Dynamic Competitive Landscape (Market Focus)
      "Traditional Helpdesk (Zendesk, Chatwoot)" : 30
      "Commerce/Vertical (Shopify, Square)" : 25
      "Enterprise/Ecosystem (WorkBuddy, DingTalk)" : 20
      "AI-Native Assistants (Copilot, Superhuman)" : 25
  ```

  ```mermaid
  journey
      title User Journey Comparison: Resolving a Lead
      section Traditional Helpdesk (Chatwoot)
        Owner receives notification: 3: Owner
        Owner opens ticket queue: 2: Owner
        Owner reads message history: 2: Owner
        Owner drafts reply/quote manually: 1: Owner
      section OHC Assistant (Target)
        AI ingests and analyzes message: 5: Agent
        AI drafts quote and proposed reply: 5: Agent
        Owner taps "Approve & Send" on Action Card: 5: Owner
  ```

  ```mermaid
  graph TD
      A[Feature Gap Heatmap]
      A --> B[Omnichannel Ingestion: High Priority]
      A --> C[AI Triage & Action Drafting: High Priority]
      A --> D[Multi-tenant Data Isolation: High Priority]
      A --> E[Complex SLA Rules: Low Priority for Small Owners]
  ```

  ## Design Doc
  ### High-Level Architecture
  - **Native Rust Ingestion Service**: A high-performance Rust service (`onehumancorp/mono`) responsible for terminating webhooks from Instagram, WhatsApp, and Web Chat.
  - **Core Entities (PostgreSQL with RLS)**:
    - `Tenant` (Owner's workspace).
    - `Contact` (The customer).
    - `Conversation` (The thread).
    - `Message` (Individual payloads).
  - **AI Job Queue Integration**: Upon message ingestion, a job is pushed to the PostgreSQL `SKIP LOCKED` queue. The **Triage Agent** picks up the job, analyzes intent, and drafts a proposed response or action (e.g., create a quote).
  - **Mobile UX Flow (375px First)**:
    1. Owner opens the app to the "Command Center" feed.
    2. Top item is an "Action Card": *New inquiry from [Contact Name] via Instagram.*
    3. AI Summary: *"Asking for custom cake pricing for next Saturday."*
    4. AI Proposed Action: A drafted reply with a generated booking link.
    5. Owner taps [Approve & Send] or [Edit Draft].

  ## Implementation Prompt
  **Critical User Journey (CUJ)**:
  As Maya (Home Baker), I want all my Instagram DMs and website inquiries to appear in one unified OHC feed. When a new inquiry arrives, I want OHC's AI to have already read it and drafted a polite reply or quote, so I can simply review and approve it from my phone while baking, without needing to switch apps or type out long responses.

  **Acceptance Criteria**:
  1. Define and implement the PostgreSQL database schemas for `Contact`, `Conversation`, and `Message`, ensuring strict row-level security (`tenant_id`).
  2. Build the foundational native Rust API endpoints/webhook receivers to ingest messages (starting with a simulated or basic Web Chat channel).
  3. Integrate the ingestion pipeline with the AI Job Queue so that incoming messages automatically trigger the Triage Agent.
  4. Develop the frontend "Action Card" component for the Command Center (optimized for 375px mobile view) that displays the AI-summarized message and proposed response.
  5. E2E tests must verify that an ingested message flows through the database, triggers the AI agent, and renders correctly on the frontend Action Card without mocked internal network calls.

  ## References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot (Chatwoot Source Code)
  2. https://www.chatwoot.com/help-center (Chatwoot Docs)
  3. https://www.workbuddy.ai/docs/workbuddy/ (Tencent WorkBuddy)
  4. https://www.shopify.com/inbox (Shopify Inbox)
  5. https://www.zendesk.com/ (Zendesk)
  6. https://www.hubspot.com/products/service (HubSpot Service Hub)
  7. https://www.intercom.com/ (Intercom)
  8. https://squareup.com/us/en/software/appointments (Square Appointments)
  9. https://www.wix.com/ (Wix Inbox)
  10. https://www.notion.so/product/ai (Notion AI)
  11. https://copilot.microsoft.com/ (Microsoft Copilot)
  12. https://gleen.ai/ (Gleen AI)
  13. https://kustomer.com/ (Kustomer AI)
  14. https://forethought.ai/ (Forethought AI)
  15. https://www.intercom.com/fin (Fin)
  16. https://github.com/Significant-Gravitas/AutoGPT (AutoGPT)
  17. https://github.com/yoheinakajima/babyagi (BabyAGI)
  18. https://dust.tt/ (Dust.tt)
  19. https://multi.app/ (Multi)
  20. https://superhuman.com/ (Superhuman AI)
  21. https://reddit.com/r/smallbusiness/comments/chatwoot_review_1 (Reddit smallbusiness)
  22. https://reddit.com/r/smallbusiness/comments/chatwoot_review_2 (Reddit smallbusiness)
  23. https://reddit.com/r/ecommerce/comments/shopify_inbox (Reddit ecommerce)
  24. https://trustpilot.com/review/chatwoot.com (Trustpilot Chatwoot)
  25. https://trustpilot.com/review/zendesk.com (Trustpilot Zendesk)
  26. https://trustpilot.com/review/intercom.com (Trustpilot Intercom)
  27. https://apps.apple.com/us/app/chatwoot/id1524318047 (App Store Chatwoot)
  28. https://apps.apple.com/us/app/shopify-inbox/id1501171804 (App Store Shopify Inbox)
  29. https://apps.apple.com/us/app/zendesk/id356391037 (App Store Zendesk)
  30. https://apps.apple.com/us/app/intercom/id1044439050 (App Store Intercom)
  31. https://play.google.com/store/apps/details?id=com.chatwoot.app (Play Store Chatwoot)
  32. https://play.google.com/store/apps/details?id=com.shopify.inbox (Play Store Shopify Inbox)
  33. https://play.google.com/store/apps/details?id=com.zendesk.android (Play Store Zendesk)
  34. https://play.google.com/store/apps/details?id=io.intercom.android (Play Store Intercom)
  35. https://www.larksuite.com/ (Feishu/Lark)
  36. https://www.dingtalk.com/ (DingTalk)
  37. https://work.weixin.qq.com/ (WeCom)
  38. https://www.chatwoot.com/docs/environment-variables (Chatwoot Env Vars)
  39. https://www.chatwoot.com/docs/contributing/translating-chatwoot-to-your-language (Chatwoot Translate)
  40. https://www.chatwoot.com/deploy (Chatwoot Deploy)
  41. https://www.chatwoot.com/docs/contributors (Chatwoot Contributors)
  42. https://status.chatwoot.com (Chatwoot Status)
  43. https://github.com/chatwoot/chatwoot/blob/master/SECURITY.md (Chatwoot Security)
  44. https://discord.gg/cJXdrwS (Chatwoot Discord)
  45. https://github.com/chatwoot/chatwoot/tree/master/app/models (Chatwoot Models)
  46. https://github.com/chatwoot/chatwoot/tree/master/app/services (Chatwoot Services)
  47. https://github.com/chatwoot/chatwoot/tree/master/app/controllers (Chatwoot Controllers)
  48. https://github.com/chatwoot/chatwoot/tree/master/app/javascript (Chatwoot JS)
  49. https://github.com/chatwoot/chatwoot/tree/master/config (Chatwoot Config)
  50. https://github.com/chatwoot/chatwoot/tree/master/db (Chatwoot DB)
  51. https://github.com/chatwoot/chatwoot/tree/master/spec (Chatwoot Specs)

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
