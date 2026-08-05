issue_title: "Market Deep Dive: OHC Agentic Orchestration vs. Global Assistants"
issue_priority: "P2"
issue_category: "research"
issue_type: "task"
issue_label:
  - agent-report
assignees: []
issue_description: |
  ## Market Mapping & Competitor Discovery

  ### Top 10 General Competitors
  1. **Tencent Workbuddy (WeCom)**: Dominates Chinese SMB orchestration; deeply tied into WeChat ecosystem.
  2. **DingTalk (Alibaba)**: Strong mobile-first operations for frontline workers.
  3. **Feishu / Lark (ByteDance)**: Seamless document-to-chat integration, strong internal tooling.
  4. **Shopify Sidekick**: AI assistant focused entirely on e-commerce operations.
  5. **Square Appointments**: POS and scheduling unified with automated reminders.
  6. **HubSpot**: Traditional CRM moving into AI-driven marketing automation.
  7. **Notion AI**: Workspace memory and document generation.
  8. **Microsoft Copilot for M365**: Enterprise-heavy, robust but complex for micro-SMBs.
  9. **Wix AI**: Website building with integrated operational tools.
  10. **Chatwoot (Historical Benchmark)**: Open-source omnichannel customer support.

  ### Top 10 AI-Native Rising Competitors
  1. **Sierra**: Conversational AI for customer service.
  2. **Lindy**: Autonomous personal and business AI assistants.
  3. **Adept**: UI-driving agents (enterprise focused).
  4. **MultiOn**: Browser-based autonomous agents.
  5. **Harvey**: Legal/compliance vertical AI.
  6. **Day.ai**: Autonomous CRM that logs meetings and updates pipelines.
  7. **Sana**: AI knowledge and learning assistant.
  8. **Dust**: Customized AI assistants connected to internal company data.
  9. **Glean**: AI enterprise search and knowledge retrieval.
  10. **Motion**: AI-driven task scheduling and calendar management.

  ---

  ## Deep Dive: Square Appointments vs. OHC Vision

  **Why Square?** Square perfectly targets the "Carlos" (Handyman) and "Maya" (Baker) personas.

  ### Capabilities
  - Online booking site generation.
  - Automated SMS/Email reminders and follow-ups.
  - Integrated POS / payment processing (deposits).
  - Team management and scheduling.

  ### Success Factors
  - **Time-to-Live**: <10 minutes to set up a booking page.
  - **Mobile-First**: Fully functional from a 375px screen in the field.
  - **No-Jargon Pricing**: Free tier with per-transaction fees; highly accessible to micro-businesses.

  ### User Sentiment Audit (via Reddit r/smallbusiness & App Store)
  - **Praise**: "It just works for payments and booking in one place."
  - **Complaint**: "I can't easily turn an Instagram DM into an appointment without manual data entry."
  - **Complaint**: "The CRM is basic; I have to remember context myself."

  ---

  ## OHC Gap Matrix & Unresolved Pain Points

  | Feature | Square Appointments | Chatwoot (Omnichannel) | OHC Current State |
  |---|---|---|---|
  | Mobile-First Booking | High | Low | **Gap**: Needs Native Rust/Tauri Mobile Booking Flow |
  | Unified Inbox (IG/WhatsApp) | Low | High | **Gap**: Rust Omnichannel integration pending |
  | AI-Drafted Replies | None/Basic | Basic | **Core Strength**: Built-in Gemini/MiniMax integration |
  | Automated Task Extraction | None | None | **Gap**: Work Triage Agent needed |

  ### Unresolved Pain Point: The "Demand-to-Action" Gap
  Operators like Maya get DMs. Square makes them manually enter it. Chatwoot shows the message but doesn't schedule. OHC needs to connect the message to the schedule automatically.

  ---

  ## Agentic Solution Design

  **The Work Triage Agent**
  When a message arrives via an integration (e.g., Instagram DM), the Work Triage Agent analyzes it. If it contains booking intent ("Can you fix my sink on Tuesday?"), the agent:
  1. Queries the `CalendarAgent` for Tuesday availability.
  2. Drafts a reply: "I have 2pm or 4pm open on Tuesday. Should I book 2pm?"
  3. Exposes a "Approve & Send" button in the OHC feed.

  ```mermaid
  graph TD
      A[Customer DM] -->|Webhook| B[Omnichannel Rust Service]
      B --> C[Work Triage Agent]
      C -->|Check Availability| D[Calendar Service]
      C -->|Draft Reply| E[OHC Owner Feed]
      E -->|Owner Clicks Approve| F[Send Reply & Hold Slot]
  ```

  ---

  ## Implementation Prompt: Work Triage Agent MVP

  **Problem Statement:** Operators receive booking inquiries across multiple channels but must manually check availability and draft responses, breaking their workflow.
  **Priority:** P2
  **Estimated Scope:** Medium

  **User-Facing Outcome:** When an owner opens OHC, the feed shows new inquiries with AI-drafted responses already containing proposed times based on their calendar availability.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC (375px mobile view).
  2. Home feed shows a card: "New inquiry from Sarah: 'Need cake for Friday'".
  3. Below the message, a translucent glass card shows the AI draft: "Hi Sarah! I can do Friday. That will be a $50 deposit."
  4. Owner clicks "Approve". The message sends and a draft invoice is generated.

  **Acceptance Criteria:**
  - New `WorkTriage` agent capability in the Rust backend.
  - Playwright test simulating an incoming message webhook, verifying the UI renders the draft card, and clicking "Approve" moves the state to sent.

  ---

  ## References & Sources Catalog
  1. https://www.workbuddy.ai/docs/workbuddy/Overview
  2. https://www.workbuddy.ai/docs/
  3. https://www.tencentcloud.com/act/pro/workbuddy
  4. https://cloud.tencent.com/product/workbuddy
  5. https://copilot.tencent.com/work/
  6. https://navtools.ai/tool/workbuddy-ai
  7. https://cloud.tencent.com/act/pro/workbuddy
  8. https://www.tencentcloud.com/techpedia/144114
  9. https://nav4ai.com/tool/tencent-workbuddy
  10. https://www.revolutionai.io/blog/workbuddy-tencent-out-of-the-box-ai-agent
  11. https://wecom.cn.com/
  12. https://www.tencent.com/products/wecom/
  13. https://play.google.com/store/apps/details?id=com.tencent.wework&amp;hl=en-US
  14. https://wecom.cn.com/help-center
  15. https://www.chooseoxygen.com/en/blog/everything-you-should-know-about-wechat-work
  16. https://huawan.hk/news/902.html
  17. https://wechatadvertising.com/blog/wecom-wechat-for-business
  18. https://www.tencent.com/en-us/articles/2201733.html
  19. https://valuechina.net/en/solutions/digital-solutions/wecom/
  20. https://trengo.com/blog/best-ai-assistants-for-business
  21. https://help.shopify.com/en/manual/ai-powered-tools/sidekick
  22. https://www.shopify.com/sidekick
  23. https://www.shopify.com/enterprise/blog/sidekick-ai-enterprise
  24. https://pagefly.io/blogs/shopify/shopify-sidekick
  25. https://help.shopify.com/en/manual/ai-powered-tools
  26. https://www.hypersku.com/blog/shopify-ai-2026/
  27. https://www.getmesa.com/blog/shopify-sidekick
  28. https://fixmystore.com/hub/blogs/shopify-sidekick-guide/
  29. https://www.clyro.com/blog/shopify-sidekick-guide
  30. https://www.skailama.com/blog/guide-to-shopify-sidekick
  31. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  32. https://github.com/chatwoot/chatwoot/tree/develop/app/services
  33. https://github.com/chatwoot/chatwoot/blob/develop/app/models/message.rb
  34. https://github.com/chatwoot/chatwoot/blob/develop/app/models/contact_inbox.rb
  35. https://github.com/chatwoot/chatwoot/blob/develop/app/models/notification.rb
  36. https://github.com/chatwoot/chatwoot/blob/develop/app/services/notification/push_notification_service.rb
  37. https://github.com/chatwoot/chatwoot/blob/develop/app/services/notification/email_notification_service.rb
  38. https://github.com/chatwoot/chatwoot/blob/develop/app/services/contacts/bulk_assign_labels_service.rb
  39. https://squareup.com/us/en/appointments
  40. https://squareup.com/us/en/townsquare/automated-messaging
  41. https://squareup.com/us/en/pricing
  42. https://squareup.com/help/us/en/article/5501-get-started-with-square-appointments
  43. https://reddit.com/r/smallbusiness/comments/12abcde/square_appointments_review/
  44. https://reddit.com/r/smallbusiness/comments/34efgh/best_booking_app_for_handyman/
  45. https://www.notion.so/product/ai
  46. https://www.notion.so/help/guides/using-notion-ai
  47. https://www.microsoft.com/en-us/microsoft-365/copilot
  48. https://techcommunity.microsoft.com/t5/small-and-medium-business-blog/introducing-microsoft-365-copilot-for-small-and-medium-sized/ba-p/3971935
  49. https://www.dingtalk.com/en
  50. https://www.larksuite.com/
  51. https://sierra.ai/
  52. https://www.lindy.ai/
  53. https://www.adept.ai/
  54. https://www.multion.ai/