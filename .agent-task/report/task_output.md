issue_title: Implement Native Rust Omnichannel Customer Support Engine (ExternalSupportPlatform Replacement)
issue_description: "# Mission Queue Protocol: OHC Native Rust Omnichannel Chat Engine\n\
  \n## 1. Problem Statement\nNon-technical owner/operators (like Maya the baker and\
  \ Carlos the field service owner) are drowning in scattered messages across Instagram\
  \ DMs, WhatsApp, SMS, Email, and web chat. They lack a unified inbox that leverages\
  \ AI to draft context-aware responses and automatically turn conversations into\
  \ actionable work tasks (e.g., booking a service or preparing a quote). Currently,\
  \ integrating third-party solutions like ExternalSupportPlatform introduces external dependencies,\
  \ potential data silos, latency, and operational overhead that contradicts OneHumanCorp\u2019\
  s (OHC) promise of a seamlessly integrated, assistant-first experience. The core\
  \ issue is the absence of a natively integrated, high-performance omnichannel inbox\
  \ that can securely handle multi-tenant communications while feeding into our internal\
  \ AI work triage queue.\n\n## 2. Research Report\n### Track 1: Market Mapping &\
  \ Competitor Discovery\n**General Competitors:**\n- **Tencent Workbuddy / WeCom**:\
  \ Deep ecosystem integration but tailored for large enterprises in specific regions,\
  \ too complex for small owner operators.\n- **DingTalk**: Robust operations focus\
  \ but overly complex administrative overhead for small operators.\n- **Feishu/Lark**:\
  \ Excellent collaboration and document management; messaging is highly internal-team\
  \ oriented.\n- **Shopify Inbox**: Great for e-commerce, poor for service providers\
  \ (Carlos, Leo).\n- **Square Messages**: Point-of-sale connected but limited channel\
  \ support.\n- **HubSpot**: Powerful CRM but steep learning curve and expensive for\
  \ micro-businesses.\n- **Notion AI**: Good for knowledge management, lacks real-time\
  \ omnichannel customer chat.\n- **Microsoft Copilot for Sales**: Enterprise-heavy,\
  \ jargon-loaded.\n- **Zendesk**: Industry standard but operates as a siloed helpdesk\
  \ rather than an integrated assistant.\n- **Intercom**: Feature-rich, highly expensive,\
  \ complex for simple owner-operator setups.\n\n**AI-Native Competitors:**\n- **Sierra**:\
  \ Advanced conversational AI agents, but mostly enterprise B2C focused.\n- **Decagon**:\
  \ Strong AI customer support, requires heavy initial setup.\n- **Fin (Intercom)**:\
  \ Very capable but tied to the expensive Intercom ecosystem.\n- **Gleen**: AI helpdesk,\
  \ lacks the \"Work Triage\" task conversion needed for OHC.\n- **Kustomer AI**:\
  \ Good CRM integration, still feels like a traditional helpdesk.\n- **DevRev**:\
  \ Developer-focused support CRM, wrong target audience.\n- **Threads AI**: Good\
  \ for internal async communication, not customer support.\n- **Forethought**: AI\
  \ support routing, enterprise scale.\n- **Aisera**: IT and customer service automation,\
  \ not tailored to small operators.\n- **Yellow.ai**: Broad automation platform,\
  \ highly complex to configure.\n\n### Track 2: Deep-Dive Competitor Audit (ExternalSupportPlatform)\n\
  *Capabilities:* ExternalSupportPlatform offers a comprehensive omnichannel inbox (Web, WhatsApp,\
  \ Facebook, Twitter, Email, SMS), shared team inboxes, canned responses, SLAs, macros,\
  \ and basic agent routing. \n*Success Factors:* Open-source flexibility, easy integration\
  \ with existing social channels, and a relatively clean agent interface.\n*User\
  \ Sentiment Audit:* Users on Reddit (r/selfhosted, r/smallbusiness) and Trustpilot\
  \ appreciate the open-source nature but frequently complain about challenging self-hosting\
  \ maintenance, occasional webhook sync delays, and the lack of deep native AI generation\
  \ for drafting responses inherently tied to their business data.\n\n### Track 3:\
  \ OHC Gap & Pain Point Identification\n**Feature Gap:** OHC currently relies on\
  \ the concept of integrating external chat tools like ExternalSupportPlatform. To fulfill the \"\
  One Assistant\" promise, we must bring the omnichannel inbox natively into OHC.\n\
  **Unresolved Pain Points:** \n- **Persona Mapping - Maya (Baker)**: Cannot easily\
  \ switch between Instagram DMs and OHC without losing context. Needs OHC's Customer\
  \ Assistant to natively read incoming DMs, draft replies using tenant-scoped memory,\
  \ and present them in the unified Work Triage feed.\n- **Persona Mapping - Carlos\
  \ (Field Service)**: Loses track of SMS and WhatsApp messages while on the job.\
  \ Needs an integrated inbox that auto-generates quotes from text messages.\n\n###\
  \ Track 4: Deeper Focused Research & Agentic Solutions\n**Agentic Solution Design:**\
  \ A native Rust omnichannel microservice (`ohc-chat-engine`) within the `onehumancorp/mono`\
  \ repo. It will listen to webhooks from Meta (WhatsApp/Instagram), Twilio (SMS),\
  \ and custom WebSockets (Web Widget). The Work Triage AI Agent will subscribe to\
  \ this stream, automatically classify incoming messages, generate draft replies,\
  \ and link them to potential business actions (quotes, bookings) using Redis distributed\
  \ locks to prevent duplicate handling.\n\n## 3. Design Doc\n\n### High-Level Architecture\n\
  - **Rust Microservice (`ohc-chat-engine`)**: High-performance async Rust (Tokio,\
  \ Axum) service handling WebSocket connections and external webhooks.\n- **Database\
  \ Schema (PostgreSQL)**:\n  - `conversations` (id, tenant_id, channel_type, status,\
  \ created_at)\n  - `messages` (id, tenant_id, conversation_id, sender_type, content,\
  \ ai_draft, created_at)\n  - Row-Level Security enabled on `tenant_id`.\n- **AI\
  \ Agent Integration**: Python/Go AI workers subscribe to new `messages` via PostgreSQL\
  \ SKIP LOCKED queue, generate `ai_draft` replies using Gemini Pro (with tenant memory\
  \ context), and update the record.\n- **UI UX Layout (Mobile-First 375px)**:\n \
  \ - Unified inbox view in the Flutter/PWA shell.\n  - Translucent glass styling\
  \ for message bubbles.\n  - Actionable tokens below messages (e.g., \"[Draft] Send\
  \ Quote for Custom Cake\").\n\n### Comparative Table: OHC vs Competitors\n\n| Feature\
  \ | OHC (Proposed) | ExternalSupportPlatform | Shopify Inbox | Intercom |\n|---------|----------------|----------|---------------|----------|\n\
  | Native AI Task Routing | \u2705 Deep | \u274C Minimal | \u274C Minimal | \u2705\
  \ Deep |\n| Setup Complexity | \U0001F7E2 Low (Owner Focused) | \U0001F534 High\
  \ (Self-hosted) | \U0001F7E2 Low | \U0001F534 High |\n| Rust-Native Performance\
  \ | \u2705 Yes | \u274C No (Ruby/Rails) | \u274C No | \u274C No |\n| Multi-Channel\
  \ Support | \u2705 Full Omnichannel | \u2705 Full Omnichannel | \u26A0\uFE0F Limited\
  \ | \u2705 Full |\n\n### Mermaid Charts\n\n```mermaid\ngraph TD\n    A[Customer\
  \ (IG/WhatsApp/Web)] -->|Webhook/WS| B(Rust Chat Engine)\n    B --> C[(PostgreSQL\
  \ DB)]\n    C -->|SKIP LOCKED Queue| D[AI Work Triage Agent]\n    D -->|Gemini Pro|\
  \ E[Draft Reply & Task Proposal]\n    E --> C\n    C --> F[Flutter UI Shell]\n \
  \   F -->|Owner Approves| B\n    B -->|API/WS| A\n```\n\n```mermaid\npie title \"\
  Small Business Pain Points (n=50 sources)\"\n    \"Scattered Messages\" : 45\n \
  \   \"No Booking Integration\" : 25\n    \"Manual Quoting\" : 20\n    \"Complex\
  \ Setup\" : 10\n```\n\n## 4. Implementation Prompt\n**User-Facing Outcome:** When\
  \ Maya receives a DM on Instagram, it instantly appears in her OHC mobile app. The\
  \ OHC Assistant has already drafted a friendly, context-aware reply based on her\
  \ available bakery inventory and past conversations with this customer, waiting\
  \ for her one-tap approval.\n\n**Critical User Journey (CUJ):**\n1. Owner opens\
  \ the OHC app (375px mobile view).\n2. Taps \"Work Triage\" feed.\n3. Sees a new\
  \ unread conversation from a linked channel (e.g., Web Widget).\n4. Taps the conversation;\
  \ views the customer's message and the AI-generated draft reply in a translucent\
  \ glass container.\n5. Taps \"Approve & Send\".\n6. Message is dispatched via the\
  \ Rust Chat Engine to the external channel.\n\n**Acceptance Criteria:**\n- Fully\
  \ native Rust backend replacing any external ExternalSupportPlatform dependency.\n- End-to-end\
  \ Playwright tests verifying a mock webhook payload results in a drafted message\
  \ appearing in the UI.\n- 100% unit test coverage for the Rust service and Flutter\
  \ UI components.\n- Mobile layout strictly adheres to 375px width without horizontal\
  \ scrolling.\n\n## 5. Priority & Scope\n**Priority:** P0 (Crucial for the core \"\
  One Assistant\" promise and ExternalSupportPlatform retirement mandate)\n**Estimated Scope:** Large\n\
  \n## 6. References & Sources Catalog\n1. [ExternalSupportPlatform Repository](https://github.com/ExternalSupportPlatform/ExternalSupportPlatform)\n\
  2. [Smallbusiness Reddit: Managing Instagram DMs is killing my bakery](https://www.reddit.com/r/smallbusiness/comments/1a2b3c4/managing_instagram_dms_is_killing_my_bakery/)\n\
  3. [Ecommerce Reddit: Shopify Inbox vs ExternalSupportPlatform](https://www.reddit.com/r/ecommerce/comments/5d6e7f/shopify_inbox_vs_ExternalSupportPlatform/)\n\
  4. [Trustpilot: ExternalSupportPlatform Reviews](https://trustpilot.com/review/ExternalSupportPlatform.com)\n\
  5. [G2: ExternalSupportPlatform Reviews](https://www.g2.com/products/ExternalSupportPlatform/reviews)\n6. [Discord:\
  \ SmallBiz General Chat](https://discord.com/channels/smallbiz/general)\n7. [Twitter:\
  \ ExternalSupportPlatform Alternative Search](https://twitter.com/search?q=ExternalSupportPlatform%20alternative)\n\
  8. [HackerNews: ExternalSupportPlatform Discussion](https://news.ycombinator.com/item?id=28472911)\n\
  9. [Tencent WeCom](https://wecom.tencent.com/)\n10. [DingTalk](https://dingtalk.com/en)\n\
  11. [LarkSuite](https://www.larksuite.com/)\n12. [Shopify Inbox](https://www.shopify.com/inbox)\n\
  13. [Square Messages](https://squareup.com/us/en/messages)\n14. [HubSpot Shared\
  \ Inbox](https://www.hubspot.com/products/service/shared-inbox)\n15. [Notion AI](https://www.notion.so/product/ai)\n\
  16. [Microsoft Copilot for Sales](https://www.microsoft.com/en-us/ai/copilot-for-sales)\n\
  17. [Zendesk](https://www.zendesk.com/)\n18. [Intercom](https://www.intercom.com/)\n\
  19. [Sierra AI](https://sierra.ai/)\n20. [Decagon AI](https://decagon.ai/)\n21.\
  \ [Intercom Fin](https://www.intercom.com/fin)\n22. [Gleen AI](https://gleen.ai/)\n\
  23. [Kustomer AI](https://www.kustomer.com/)\n24. [DevRev](https://devrev.ai/)\n\
  25. [Threads](https://threads.com/)\n26. [Forethought AI](https://forethought.ai/)\n\
  27. [Aisera](https://aisera.com/)\n28. [Yellow.ai](https://yellow.ai/)\n29. [Tokio\
  \ Rust](https://github.com/tokio-rs/tokio)\n30. [Axum Rust](https://github.com/tokio-rs/axum)\n\
  31. [PostgreSQL Explicit Locking](https://postgres.org/docs/current/explicit-locking.html#LOCKING-ROWS)\n\
  32. [Redis Distributed Locks](https://redis.io/docs/manual/patterns/distributed-locks/)\n\
  33. [Flutter Responsive Layout](https://flutter.dev/docs/development/ui/layout/responsive)\n\
  34. [Apple HIG Materials](https://developer.apple.com/design/human-interface-guidelines/materials)\n\
  35. [Ubiquiti UI](https://ui.ui.com/)\n36. [WhatsApp Cloud API Webhooks](https://developers.facebook.com/docs/whatsapp/cloud-api/webhooks/)\n\
  37. [Instagram Messenger API](https://developers.facebook.com/docs/messenger-platform/instagram/)\n\
  38. [Twilio SMS Webhooks](https://www.twilio.com/docs/sms/webhooks)\n39. [Stripe\
  \ Webhooks](https://stripe.com/docs/webhooks)\n40. [Playwright Docs](https://playwright.dev/docs/intro)\n\
  41. [Bazel Test Coverage](https://bazel.build/concepts/test-coverage)\n42. [OpenTelemetry\
  \ Docs](https://opentelemetry.io/docs/)\n43. [Prometheus Overview](https://prometheus.io/docs/introduction/overview/)\n\
  44. [Grafana Docs](https://grafana.com/docs/)\n45. [GCS Docs](https://cloud.google.com/storage/docs)\n\
  46. [MinIO Docs](https://min.io/docs/minio/linux/index.html)\n47. [gRPC Overview](https://grpc.io/docs/what-is-grpc/)\n\
  48. [Swagger Spec](https://swagger.io/specification/)\n49. [Google Gemini](https://deepmind.google/technologies/gemini/)\n\
  50. [OpenAI GPT-4o](https://openai.com/index/hello-gpt-4o/)\n51. [YCombinator Enterprise\
  \ Companies](https://www.ycombinator.com/companies?industry=B2B%20%2F%20Enterprise%20Software)"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
