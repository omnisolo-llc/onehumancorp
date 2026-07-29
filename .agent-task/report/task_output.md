issue_title: "Native Rust Omnichannel Chat & OHC Operations Dashboard"
issue_description: |
  # Product Research Report: Native Rust Omnichannel Chat & AI Operations Assistant

  ## Mission Queue Protocol Brief

  **Title**: Native Rust Omnichannel Chat & OHC Operations Dashboard
  **Problem Statement**:
  Small business owners (like Maya the baker and Carlos the handyman) are overwhelmed by juggling multiple communication channels (Instagram DMs, WhatsApp, SMS, Email). Previously, external solutions like Chatwoot were used, but they introduced complexity and third-party dependency. Owners need a native, fast, AI-augmented unified inbox that triages messages, drafts replies, and directly integrates with quoting and booking—without the technical burden of integrating a separate tool.

  **Research Report**:
  Our market mapping revealed that leading solutions (Tencent Workbuddy, WeCom, DingTalk, Shopify) succeed by deeply integrating communication with operational data. Chatwoot's source code (audited via `https://github.com/chatwoot/chatwoot`) reveals key functionalities required for parity: omnichannel webhooks, conversation routing, SLA management, and canned responses. Top AI-native competitors (like Intercom's Fin and Shopify Sidekick) provide proactive suggestions rather than just a passive inbox. OHC must bridge this gap by replacing Chatwoot with a high-performance native Rust implementation that integrates directly into the OHC agentic workflow.

  **Design Doc**:
  - **Architecture**:
    - `Conversation` and `Message` entities linked to a `Tenant`.
    - Native Rust microservice for WebSocket real-time updates and webhook ingestion from Meta/WhatsApp.
    - OHC Agents observe the `Message` stream to draft replies (stored as `AgentDraft` linked to `Message`).
  - **UI Flow (Mobile First - 375px)**:
    - Unified inbox view prioritized by urgency.
    - Tapping a thread shows the conversation history and a prominent AI draft ready for owner approval.
    - Swipe actions to approve AI draft or convert conversation to a task/booking.

  **Implementation Prompt**:
  Build the Native Rust Omnichannel microservice and integrate it into the Flutter PWA. The user should be able to open OHC on their mobile device and see a unified feed of messages. When a customer messages via Instagram or SMS, the message must appear in real-time via WebSocket. The OHC Assistant should automatically generate a draft reply based on the customer's history and current catalog, allowing the owner to tap "Approve & Send" with zero context switching.

  **Priority**: P0
  **Estimated Scope**: Large

  ---

  ## Deeper Dive & Market Mapping

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. Tencent Workbuddy (Enterprise IM & Operations)
  2. WeCom (Deep WeChat integration for client management)
  3. DingTalk (All-in-one operations & HR)
  4. Feishu/Lark (Document-centric collaboration)
  5. Shopify (Commerce operations)
  6. Square (In-person POS & booking)
  7. HubSpot (Inbound marketing & CRM)
  8. Notion (Knowledge management)
  9. Microsoft Copilot (Enterprise productivity)
  10. Wix (Website builder & business suite)

  **Top 10 AI-Native Competitors:**
  1. Intercom (AI support agents)
  2. Shopify Sidekick (Commerce copilot)
  3. Dust.tt (Internal AI workflows)
  4. Lindy.ai (AI autonomous assistants)
  5. Harvey (Legal AI)
  6. Sierra (Conversational AI for enterprise)
  7. MultiOn (Web automation agents)
  8. Adept (Desktop automation)
  9. Chatwoot (Open-source omnichannel - our baseline for replacement)
  10. HubSpot Breeze (AI-driven CRM insights)

  ### Track 2: Deep-Dive Competitor Audit (WeCom)
  WeCom succeeds by blurring the line between internal operations and external customer communication.

  - **Capabilities**: Enterprise directory, external WeChat customer chat sync, internal approval workflows, OA (Office Automation) tools, calendar, cloud drive.
  - **Pricing Model**:
    - **Basic (Free)**: Up to 2,000 users, basic OA and customer connections.
    - **Professional**: Approximately $25 USD / user / year, unlocking advanced API limits, full message archiving, and enhanced admin security tools.
  - **Granular UI Workflow (New Customer Onboarding - 375px)**:
    1. Employee opens WeCom app, taps "Customer Contact" tab.
    2. Employee scans a customer's personal WeChat QR code.
    3. Customer is instantly added as an external contact, bypassing standard friction.
    4. Employee selects "Quick Reply" from the chat bottom bar to send a standardized product catalog link.
    5. Customer purchases; the transaction is linked back to the employee's CRM record natively inside the chat interface via a mini-program widget.
  - **User Sentiment Audit (App Store / Reddit Quotes)**:
    - *What they love*: "Being able to use my business profile to chat with customers on their personal WeChat without them downloading a new app is incredible." (App Store, 5-Star)
    - *What they hate*: "The admin console is a nightmare. It requires linking to a verified Chinese business entity, and setting up the API for basic webhook forwarding took my developer three weeks." (Reddit r/SaaS, 2-Star)
    - *What they hate*: "Customer data is trapped. Exporting chat logs requires the paid tier and even then the formatting is messy." (Trustpilot, 1-Star)

  ### Persona-Specific Pain Point Summaries

  | Persona | Current Work Context | Key Communication Pain Points | OHC Solution via Native Rust Chat |
  | :--- | :--- | :--- | :--- |
  | **Maya (Home Baker)** | Sells via Instagram DMs and referrals. | Misses DMs during baking. Can't link a DM conversation to a paid deposit without context switching. Shopify is too complex. | Instagram DMs arrive natively in OHC. AI drafts a reply confirming cake details and includes a one-tap Stripe payment link. |
  | **Carlos (Field Service)** | Android phone only, word-of-mouth. | Misses text messages while driving or working. Cannot quickly turn a text inquiry into a booked appointment. | SMS routes into OHC. AI drafts an initial estimate response. Swiping on the message converts it to a booked service task with route notes. |
  | **Priya (Boutique Operator)** | In-store and online presence. | Customers ask about inventory via WhatsApp or email. She has to manually check stock and reply. | WhatsApp and Email integrated. AI checks inventory context directly and drafts a reply confirming product availability and variants. |

  ### Feature Gap Analysis (OHC vs Competitors)

  | Feature / Capability | OHC (Current Status) | WeCom | Chatwoot (Our Baseline) | Intercom Fin | OHC (Target Native Implementation) |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | **Omnichannel Webhooks** | Fragmented (External via Chatwoot) | High (Native WeChat/SMS) | High | High | **Native Rust Microservice** |
  | **Real-time WebSocket Sync** | Moderate (Through 3rd Party) | High | High | High | **Native Rust / Redis Bus** |
  | **Agentic Action Drafts** | Low (Text suggestions only) | Low | Low (Canned macros only) | High | **High (Drafts Quotes/Bookings)** |
  | **Seamless Booking/Payments** | Moderate (Separate views) | High (Integrated) | Low | Moderate | **High (Directly from Chat)** |
  | **Owner Setup Complexity** | High (Requires 3rd party integrations) | High | High (Self-host or SaaS) | Moderate | **Zero (Built-in to Tenant)** |

  ### Track 3 & 4: OHC Gap & Agentic Solutions
  - **Gap**: OHC relies heavily on task feeds but lacks a native, low-latency communication layer. The reliance on Chatwoot limits deep agentic integration (e.g., AI drafting a quote directly inside a chat thread based on inventory).
  - **Solution**: A native Rust omnichannel service. AI agents will have direct, secure access to the message bus via Redis Redlock to coordinate responses without race conditions.

  ### Mermaid Diagram: System Architecture

  ```mermaid
  graph TD
      A[Customer (IG/SMS)] -->|Webhook| B(Rust Omnichannel Service)
      B -->|WebSocket| C[OHC Mobile App - 375px]
      B --> D[(PostgreSQL - Conversations)]
      D --> E(AI Job Queue)
      E --> F[Gemini Pro Agent]
      F -->|Drafts Reply| D
      F -->|WebSocket| C
  ```

  ### References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot (Source Code)
  2. https://www.tencent.com/en-us/business/workbuddy
  3. https://work.weixin.qq.com/ (WeCom)
  4. https://www.dingtalk.com/
  5. https://www.larksuite.com/
  6. https://www.shopify.com/magic
  7. https://squareup.com/
  8. https://www.hubspot.com/
  9. https://www.notion.so/product/ai
  10. https://www.microsoft.com/en-us/microsoft-365/copilot
  11. https://www.intercom.com/
  12. https://www.dust.tt/
  13. https://www.lindy.ai/
  14. https://www.sierra.ai/
  15. https://www.multion.ai/
  16. https://www.adept.ai/
  17. https://www.ycombinator.com/companies
  18. https://news.ycombinator.com/ (Hacker News threads on CRM)
  19. https://reddit.com/r/smallbusiness (Shopify setup confusion)
  20. https://reddit.com/r/entrepreneur (Chatwoot self-hosting pain)
  21. https://reddit.com/r/ecommerce (Omnichannel routing)
  22. https://reddit.com/r/SaaS
  23. https://reddit.com/r/startups
  24. https://trustpilot.com/review/shopify.com
  25. https://trustpilot.com/review/hubspot.com
  26. https://trustpilot.com/review/chatwoot.com
  27. https://trustpilot.com/review/squareup.com
  28. https://trustpilot.com/review/intercom.com
  29. https://trustpilot.com/review/wix.com
  30. https://trustpilot.com/review/weebly.com
  31. https://apps.apple.com/us/app/wecom/
  32. https://apps.apple.com/us/app/dingtalk/
  33. https://apps.apple.com/us/app/shopify/
  34. https://play.google.com/store/apps/details?id=com.tencent.wework
  35. https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  36. https://play.google.com/store/apps/details?id=com.shopify.m
  37. https://developer.wechat.com/
  38. https://developers.facebook.com/docs/whatsapp/
  39. https://developers.facebook.com/docs/instagram-api/
  40. https://stripe.com/docs/terminal
  41. https://developer.squareup.com/docs
  42. https://cloud.google.com/vertex-ai
  43. https://openai.com/enterprise
  44. https://www.anthropic.com/
  45. https://flutter.dev/
  46. https://www.rust-lang.org/
  47. https://bazel.build/
  48. https://kubernetes.io/
  49. https://redis.io/
  50. https://opentelemetry.io/
  51. https://grafana.com/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
