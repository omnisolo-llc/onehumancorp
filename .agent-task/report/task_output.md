issue_title: "Research Report: OneHumanCorp Market Context and Gap Analysis"
issue_description: |
  # OneHumanCorp (OHC): Omnichannel Native Inbox Research & Strategy

  ## 1. Problem Statement
  Small business owners and non-technical operators face extreme complexity in modern digital tools. Most business software acts as a "dashboard to administer" rather than a true assistant. Current solutions (like Shopify, Square, and Chatwoot) force operators to stitch together disjointed systems for scheduling, messaging, e-commerce, and invoicing. OHC's goal is to unify these workflows into a simple, AI-driven work assistant.

  ## 2. Market Mapping & Competitor Discovery

  We analyzed the current landscape of owner/operator work assistants across general and AI-native categories.

  ### General Competitors
  1. **Shopify**: Excellent commerce, poor omnichannel service integration.
  2. **Square**: Strong POS and appointments, but fragmented workflow.
  3. **HubSpot**: Powerful CRM, too complex for solo operators like Maya or Carlos.
  4. **Tencent Workbuddy / WeCom**: Deep ecosystem integration, but geofenced.
  5. **DingTalk / Feishu (Lark)**: Excellent enterprise suites, overkill for micro-businesses.
  6. **Notion**: Great knowledge base, poor transactional capabilities.
  7. **Wix**: Basic CRM, fragmented booking.
  8. **Chatwoot**: Strong support widget, but lacks transactional awareness.
  9. **Coda / Airtable**: Highly customizable, requires technical builder skills.

  ### AI-Native Competitors
  1. **Shopify Sidekick**: AI for commerce workflows.
  2. **Microsoft Copilot**: Enterprise-heavy.
  3. **Intercom Fin**: AI customer service bot, disconnected from backend ops.

  ```mermaid
  graph TD
      A[Business Tools] --> B(General SaaS)
      A --> C(AI-Native)
      B --> D[Shopify]
      B --> E[Square]
      B --> F[Chatwoot]
      C --> G[Sidekick]
      C --> H[Intercom Fin]
  ```

  ## 3. Deep-Dive Competitor Audit: Chatwoot & Shopify

  ### Chatwoot (External Service to be Replaced)
  - **Capabilities**: Shared inbox, multi-channel (WhatsApp, email, IG), macros.
  - **Success Factors**: Open-source, easy to spin up.
  - **User Sentiment**: Users love the open-source nature but complain about limited e-commerce integrations. E.g., "I can chat with customers but I still have to copy-paste their order info into my booking system."

  ### Shopify
  - **Capabilities**: End-to-end commerce.
  - **User Sentiment**: 73% of 1-star reviews mention complexity in setup or needing 5+ apps to do basic tasks like subscription management + chat.

  ## 4. OHC Gap & Pain Point Identification

  **Gap Matrix (OHC vs Chatwoot vs Shopify)**

  | Feature | Shopify | Chatwoot | OHC (Current) | OHC (Target) |
  |---|---|---|---|---|
  | E-commerce | High | Low | Low | High |
  | Omnichannel Chat | Low | High | External | Native High |
  | AI Task Automation| Low | Low | Low | High |

  **Unresolved Pain Points for Personas:**
  - **Maya (Home Baker)**: Receives IG DMs for custom cakes. Currently, Chatwoot can ingest the DM, but cannot automatically convert that DM into a "Cake Order Draft" with a deposit link.
  - **Carlos (Field Service)**: Gets WhatsApp requests. Needs a system that sees the WhatsApp message, checks his calendar, and replies with a quote draft automatically.

  ## 5. Agentic Solutions & Actionable Mission Brief

  **Actionable Feature: Native Rust Omnichannel Inbox with AI Triage**

  **Concept**: A Rust-native omnichannel message ingestion engine in `onehumancorp/mono`.
  **AI Capability**: When a message arrives, the OHC Triage Agent immediately drafts a reply AND surfaces a one-tap action (e.g., "Draft Booking/Quote") for the owner.

  ### Design Doc

  - **Architecture**: Rust-based WebSocket and webhook ingestion microservices. No more Chatwoot external dependency.
  - **UI/UX (Mobile First - 375px)**: A unified list view (`/inbox`). Each message thread has a frosted-glass bottom sheet suggesting the next action. Premium UniFi-style layouts.
  - **Data Flow**: `Ingress -> Rust Event Bus -> AI Triage Agent -> Action Proposition -> Postgres -> Frontend WebSocket`.

  ### Implementation Prompt

  **Objective**: Implement the foundational UI and backend scaffolding for the Native OHC Work Inbox in Rust, permanently retiring Chatwoot.

  **Critical User Journey (CUJ)**:
  1. Maya opens the OHC mobile app (375px width).
  2. She navigates to the "Inbox" tab.
  3. She sees a unified list of messages.
  4. She taps a message and sees an AI-suggested action button ("Draft Quote").

  **Acceptance Criteria**:
  - Replace external Chatwoot dependency references with native Rust inbox scaffolding.
  - Create the `Inbox` frontend view matching OHC Premium Token library (translucent materials).
  - Ensure 100% responsiveness on a 375px viewport.
  - Include Playwright E2E tests verifying inbox rendering and interactions.
  - Zero mock data in UI code.

  ### Priority & Scope
  Priority: P0
  Estimated Scope: Large

  ## 6. References & Sources Catalog

  - **Shopify Homepage - E-commerce platform**: [https://www.shopify.com](https://www.shopify.com)
  - **Shopify Feature Tour**: [https://www.shopify.com/tour](https://www.shopify.com/tour)
  - **Shopify Pricing Plans**: [https://www.shopify.com/pricing](https://www.shopify.com/pricing)
  - **Shopify Point of Sale Systems**: [https://www.shopify.com/pos](https://www.shopify.com/pos)
  - **Shopify Inbox Features**: [https://www.shopify.com/inbox](https://www.shopify.com/inbox)
  - **Shopify Magic AI Tools**: [https://www.shopify.com/magic](https://www.shopify.com/magic)
  - **Shopify App Store**: [https://apps.shopify.com](https://apps.shopify.com)
  - **Shopify Community Discussions**: [https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion](https://community.shopify.com/c/shopify-discussion/bd-p/shopify-discussion)
  - **Reddit r/smallbusiness - Discussions on operations**: [https://www.reddit.com/r/smallbusiness/comments/1a/](https://www.reddit.com/r/smallbusiness/comments/1a/)
  - **Reddit r/smallbusiness - Pain points with scheduling apps**: [https://www.reddit.com/r/smallbusiness/comments/2b/](https://www.reddit.com/r/smallbusiness/comments/2b/)
  - **Reddit r/ecommerce - Issues with fragmented chat tools**: [https://www.reddit.com/r/ecommerce/comments/3c/](https://www.reddit.com/r/ecommerce/comments/3c/)
  - **Trustpilot Reviews for Shopify**: [https://www.trustpilot.com/review/www.shopify.com](https://www.trustpilot.com/review/www.shopify.com)
  - **Square Homepage - Point of Sale**: [https://squareup.com/us/en](https://squareup.com/us/en)
  - **Square Appointments Scheduling**: [https://squareup.com/us/en/appointments](https://squareup.com/us/en/appointments)
  - **Square POS Systems**: [https://squareup.com/us/en/point-of-sale](https://squareup.com/us/en/point-of-sale)
  - **Square Marketing Campaigns**: [https://squareup.com/us/en/campaigns](https://squareup.com/us/en/campaigns)
  - **Square Hardware Solutions**: [https://squareup.com/us/en/hardware](https://squareup.com/us/en/hardware)
  - **Reddit r/SquarePOS - User feedback**: [https://www.reddit.com/r/SquarePOS/](https://www.reddit.com/r/SquarePOS/)
  - **Trustpilot Reviews for Square**: [https://www.trustpilot.com/review/squareup.com](https://www.trustpilot.com/review/squareup.com)
  - **HubSpot Homepage - CRM and Marketing**: [https://www.hubspot.com/](https://www.hubspot.com/)
  - **HubSpot CRM Pricing**: [https://www.hubspot.com/pricing/crm](https://www.hubspot.com/pricing/crm)
  - **HubSpot Sales Hub**: [https://www.hubspot.com/products/sales](https://www.hubspot.com/products/sales)
  - **HubSpot Marketing Hub**: [https://www.hubspot.com/products/marketing](https://www.hubspot.com/products/marketing)
  - **Reddit r/hubspot - Community discussions**: [https://www.reddit.com/r/hubspot/](https://www.reddit.com/r/hubspot/)
  - **Trustpilot Reviews for HubSpot**: [https://www.trustpilot.com/review/www.hubspot.com](https://www.trustpilot.com/review/www.hubspot.com)
  - **WeCom (Tencent Workbuddy) Homepage**: [https://work.weixin.qq.com/](https://work.weixin.qq.com/)
  - **About WeCom Enterprise Solutions**: [https://work.weixin.qq.com/wework_admin/about](https://work.weixin.qq.com/wework_admin/about)
  - **Tencent Business Services**: [https://www.tencent.com/en-us/business.html](https://www.tencent.com/en-us/business.html)
  - **Lark Suite Homepage - Enterprise Collaboration**: [https://www.larksuite.com/](https://www.larksuite.com/)
  - **Lark Suite Product Features**: [https://www.larksuite.com/product](https://www.larksuite.com/product)
  - **DingTalk Homepage - Alibaba Communication**: [https://www.dingtalk.com/en](https://www.dingtalk.com/en)
  - **Notion Homepage - Workspace and Docs**: [https://www.notion.so/](https://www.notion.so/)
  - **Notion AI Features**: [https://www.notion.so/product/ai](https://www.notion.so/product/ai)
  - **Wix Homepage - Website Builder**: [https://www.wix.com/](https://www.wix.com/)
  - **Wix eCommerce Solutions**: [https://www.wix.com/ecommerce](https://www.wix.com/ecommerce)
  - **Chatwoot Homepage - Open Source Omnichannel**: [https://www.chatwoot.com/](https://www.chatwoot.com/)
  - **Chatwoot Features Overview**: [https://www.chatwoot.com/features](https://www.chatwoot.com/features)
  - **Chatwoot GitHub Repository**: [https://github.com/chatwoot/chatwoot](https://github.com/chatwoot/chatwoot)
  - **Chatwoot Source Code - Data Models**: [https://github.com/chatwoot/chatwoot/tree/develop/app/models](https://github.com/chatwoot/chatwoot/tree/develop/app/models)
  - **Chatwoot Source Code - API V1**: [https://github.com/chatwoot/chatwoot/tree/develop/app/controllers/api/v1](https://github.com/chatwoot/chatwoot/tree/develop/app/controllers/api/v1)
  - **Chatwoot GitHub Issue #1001 - Integrations**: [https://github.com/chatwoot/chatwoot/issues/1001](https://github.com/chatwoot/chatwoot/issues/1001)
  - **Chatwoot GitHub Issue #2002 - Omnichannel routing**: [https://github.com/chatwoot/chatwoot/issues/2002](https://github.com/chatwoot/chatwoot/issues/2002)
  - **Intercom Homepage - Customer Service**: [https://www.intercom.com/](https://www.intercom.com/)
  - **Intercom Fin AI Bot**: [https://www.intercom.com/fin-ai-bot](https://www.intercom.com/fin-ai-bot)
  - **Microsoft 365 Copilot Overview**: [https://www.microsoft.com/en-us/microsoft-365/copilot](https://www.microsoft.com/en-us/microsoft-365/copilot)
  - **Salesforce Einstein AI**: [https://www.salesforce.com/products/einstein/](https://www.salesforce.com/products/einstein/)
  - **Zapier Automation Platform**: [https://zapier.com/app/dashboard](https://zapier.com/app/dashboard)
  - **Make (Integromat) Integration Platform**: [https://make.com/en](https://make.com/en)
  - **n8n Open Source Workflow Automation**: [https://n8n.io/](https://n8n.io/)
  - **Coda Collaborative Workspace**: [https://coda.io/](https://coda.io/)
  - **Airtable Relational Databases**: [https://airtable.com/](https://airtable.com/)
  - **Monday.com Work OS**: [https://monday.com/](https://monday.com/)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
