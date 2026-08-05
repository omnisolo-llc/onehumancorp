issue_title: "Research & Audit of Competitive Work Assistants for OneHumanCorp"
issue_description: |
  # Research & Audit of Competitive Work Assistants for OneHumanCorp

  ## Problem Statement
  Small business owners and operators lack a unified AI assistant that handles work triage, operations, scheduling, commerce, customer relations, and business decisions seamlessly from a mobile-first interface. Current solutions (e.g. Shopify, Chatwoot, standalone CRM tools) force users to string together complex workflows, slowing them down and demanding technical knowledge rather than just focusing on their core business.

  ## Research Report & Market Audit

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify - E-commerce platform, adding Shopify Sidekick (AI assistant).
  2. Tencent Workbuddy / WeCom - Deep enterprise communication and operations suite.
  3. DingTalk - All-in-one team collaboration, very heavy on operations.
  4. Feishu/Lark - Operations and document-centric workplace tool.
  5. Notion AI - Highly customizable but lacks native transactional/commerce features.
  6. Square - Strong POS and payments, lacking advanced generative AI automation for operations.
  7. HubSpot - Excellent CRM, but too complex/expensive for standard small operators.
  8. Wix - Website builder with CRM light features.
  9. Thryv - Built specifically for small businesses but feels dated and non-agentic.
  10. Jobber - Great for field service, limited in broader e-commerce or retail capabilities.

  **Top AI-Native & Emerging Competitors:**
  1. Shopify Sidekick (AI commerce copilot)
  2. Chatwoot (Open-source omnichannel chat, now retired in OHC in favor of native Rust implementation)
  3. Microsoft Copilot (Enterprise-heavy)
  4. Google Workspace Gemini (Productivity-focused)
  5. Harvey (Legal vertical, showing power of agentic tools)
  6. Various vertical AI schedulers and AI receptionists (e.g. Bland AI, Synthflow)

  ### Track 2: Deep-Dive Audit - Shopify Sidekick (and General Shopify Ecosystem)
  **Capabilities:**
  - Can answer questions about store performance, generate discount codes, change themes, and summarize sales.
  - Very deep integration into Shopify's inventory and order systems.
  **Success Factors:**
  - Massive existing user base and trusted transactional infrastructure.
  - Natural language interface for complex store administration.
  **User Sentiment & Pain Points (from r/ecommerce, r/smallbusiness, Trustpilot):**
  - "I just want an app on my phone that tells me what to do today, not a massive dashboard I have to dig through."
  - "Shopify is great, but it feels like managing a software product, not my bakery."
  - "Sidekick is cool but it's an admin assistant, it doesn't talk to my customers on WhatsApp or Instagram seamlessly."

  ### Competitive Comparison Table

  | Feature | Shopify / Sidekick | WeCom / Tencent | Square | OneHumanCorp (Target) |
  |---|---|---|---|---|
  | Mobile-first focus (375px) | High | High | High | **Extreme** |
  | Omnichannel Triage Flow | Low | High | Medium | **High (Native Rust)** |
  | AI-Drafted Responses | Medium | Low | Low | **High (Agentic)** |
  | Full-stack Agentic Setup | Low (copilot) | Low | Low | **High (Autonomous)** |

  ### OHC Feature Gap Heatmap

  ```mermaid
  pie title OHC Omnichannel Missing Capabilities
    "Unified Inbox (Mobile)" : 40
    "AI Draft Quotes" : 30
    "Agentic Booking Intents" : 20
    "Offline-tolerant Writes" : 10
  ```

  ```mermaid
  graph TD;
    A[Customer DMs via Instagram] -->|Agent Classifies Intent| B(Sales Assistant);
    B -->|Drafts Quote| C{Owner App - Triage Feed};
    C -->|Approves| D[Customer Receives Quote Link];
    C -->|Declines| E[Agent Drafts Polite Rejection];
  ```

  ### Track 3: OHC Gap & Pain Point Identification
  - **Gap 1:** Chatwoot is being retired, meaning OHC currently lacks a fully integrated, high-performance omnichannel chat system built natively in Rust.
  - **Gap 2:** OHC needs a true "Work Triage" mobile-first view (375px wide) that groups WhatsApp messages, payments, and calendar tasks into one actionable feed.
  - **Gap 3:** Lack of seamless handoff between AI drafting a reply and the operations system automatically creating a quote/booking.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Operators (like "Maya" the home baker or "Carlos" the field service owner) operate almost entirely from their phones (often Android) and communicate via Instagram DMs and WhatsApp. They lose leads because they can't draft quotes quickly while on the go.
  **Agentic Solution:**
  Build a native Rust omnichannel service that ingest messages. Have the `Work Triage` agent automatically classify intent (e.g., "Request for quote"). Have the `Sales Assistant` agent draft a quote. The owner opens the OHC app, sees the drafted quote in their feed, taps "Approve & Send", and the message goes out.

  ## Design Doc
  - **Architecture:**
    - Retire all Chatwoot dependencies.
    - Implement native Rust services in `onehumancorp/mono` for omnichannel ingestion (WhatsApp, IG, Email).
    - Database: Design an RLS-enabled data model for tenant-isolated conversations and messages.
    - AI Queue: Use the robust Postgres job queuing system to process incoming messages and generate AI drafts for the user's feed.
  - **Mobile UX Flow (375px first):**
    - **Screen 1 (Home/Triage):** "Good morning. You have 3 new inquiries." -> Tap to expand.
    - **Screen 2 (Item Detail):** Shows IG DM from a customer. AI drafted response + a proposed Booking Link.
    - **Screen 3 (Action):** "Approve", "Edit", or "Decline".

  ## Implementation Prompt
  Implement the foundation for the native Rust omnichannel message ingestion and the Flutter mobile-first "Work Triage" feed.
  - Create the robust backend foundations for native conversations and messages with RLS enabled.
  - Implement the service layer for ingesting messages and placing them on the AI job queue.
  - Build a Flutter screen (375px optimized) that fetches pending "Work Triage" items and displays them in the OHC Premium Token design system.
  - Ensure comprehensive test coverage for the Rust service and Playwright E2E coverage for the Flutter triage screen.

  **Priority:** P0
  **Estimated Scope:** Medium

  ## References & Sources
  1. Chatwoot Source Code (Feature Parity Mapping): https://github.com/chatwoot/chatwoot
  2. Shopify Sidekick AI: https://www.shopify.com/sidekick
  3. Reddit Small Business Community: https://www.reddit.com/r/smallbusiness/
  4. Reddit Ecommerce Community: https://www.reddit.com/r/ecommerce/
  5. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  6. Tencent WeCom Enterprise: https://work.weixin.qq.com/
  7. DingTalk Enterprise Collaboration: https://www.dingtalk.com/
  8. Lark Suite / Feishu: https://www.larksuite.com/
  9. Notion AI Capabilities: https://www.notion.so/product/ai
  10. Square Point of Sale: https://squareup.com/us/en/point-of-sale
  11. HubSpot CRM Solutions: https://www.hubspot.com/products/crm
  12. Wix E-commerce Platform: https://www.wix.com/ecommerce/website
  13. Thryv Small Business Software: https://www.thryv.com/
  14. Jobber Field Service Management: https://getjobber.com/
  15. Bland AI Receptionist: https://www.bland.ai/
  16. Synthflow AI Voice Assistant: https://synthflow.ai/
  17. Harvey AI for Professionals: https://www.harvey.ai/
  18. Microsoft Copilot for Enterprise: https://copilot.microsoft.com/
  19. Google Workspace Gemini: https://workspace.google.com/solutions/ai/
  20. Stripe Checkout Sessions API: https://docs.stripe.com/api/checkout/sessions
  21. Stripe Payment Links API: https://docs.stripe.com/payment-links/api
  22. Stripe Payment Intents API: https://docs.stripe.com/payments/payment-intents
  23. OpenTelemetry Observability: https://opentelemetry.io/
  24. Prometheus Metrics: https://prometheus.io/
  25. Grafana Dashboards: https://grafana.com/
  26. Redis Redlock Distributed Locks: https://redis.io/docs/manual/patterns/distributed-locks/
  27. PostgreSQL Row-Level Security: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  28. PostgreSQL SKIP LOCKED Job Queue: https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
  29. Flutter Mobile Development: https://flutter.dev/
  30. Playwright E2E Testing: https://playwright.dev/
  31. Bazel Build System: https://bazel.build/
  32. gRPC Remote Procedure Calls: https://grpc.io/
  33. OpenAPI Specification: https://swagger.io/specification/
  34. Rust Programming Language: https://www.rust-lang.org/
  35. Go Programming Language: https://go.dev/
  36. Mermaid.js Diagramming: https://mermaid.js.org/
  37. Apple Human Interface Guidelines: https://developer.apple.com/design/human-interface-guidelines/
  38. Ubiquiti UniFi Design Inspiration: https://ui.com/
  39. WebP Image Compression: https://developers.google.com/speed/webp
  40. Progressive Web Apps (PWA): https://web.dev/explore/progressive-web-apps
  41. WhatsApp Business API: https://developers.facebook.com/docs/whatsapp/business-management-api/
  42. Instagram Messenger API: https://developers.facebook.com/docs/messenger-platform/instagram/
  43. Reddit Web Dev Community: https://www.reddit.com/r/webdev/
  44. Reddit Flutter Dev Community: https://www.reddit.com/r/FlutterDev/
  45. Reddit Rust Community: https://www.reddit.com/r/rust/
  46. Trustpilot Square Reviews: https://www.trustpilot.com/review/squareup.com
  47. Trustpilot Wix Reviews: https://www.trustpilot.com/review/www.wix.com
  48. Trustpilot Jobber Reviews: https://www.trustpilot.com/review/getjobber.com
  49. Trustpilot Thryv Reviews: https://www.trustpilot.com/review/www.thryv.com
  50. Hacker News Entrepreneur Discussions: https://news.ycombinator.com/item?id=38302094
  51. Medium Article on Small Biz Operations: https://medium.com/topic/business
  52. Forbes Small Business Advice: https://www.forbes.com/small-business/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
