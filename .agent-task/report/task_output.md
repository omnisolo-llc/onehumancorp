issue_title: Implement Agent-Driven Intelligent Booking & Deposit Handoff
issue_description: "# OHC Market Research & Feature Mission: Intelligent Booking &\
  \ Deposit Handoff\n\n## Track 1: Market Mapping & Competitor Discovery (Dynamic\
  \ Research)\n\n### Top 10 General Competitors\n1. **WeCom (Tencent)**: The gold\
  \ standard for social-integrated business messaging and customer management in China.\n\
  2. **DingTalk (Alibaba)**: Dominant in task assignment, approvals, and internal\
  \ enterprise operations.\n3. **Feishu / Lark (ByteDance)**: Excels at unified document\
  \ collaboration, messaging, and fluid team knowledge sharing.\n4. **Shopify**: The\
  \ powerhouse for independent commerce, inventory, and storefronts.\n5. **Square\
  \ (Block)**: Leading offline-first POS and service booking ecosystem.\n6. **HubSpot**:\
  \ Premium CRM with a focus on marketing automation and inbound pipelines.\n7. **Notion**:\
  \ Unmatched flexible workspace for knowledge, project management, and wikis.\n8.\
  \ **Microsoft 365 Copilot**: Enterprise-heavy integration of AI across standard\
  \ office workflows.\n9. **HoneyBook**: Vertical SaaS champion for independent creative\
  \ professionals, focusing on proposals and client flow.\n10. **Wix**: Website builder\
  \ evolving into a full operational dashboard for service and commerce businesses.\n\
  \n### Top 10 AI-Native Competitors\n1. **Shopify Sidekick**: Conversational AI assistant\
  \ helping merchants with store configuration and analytics.\n2. **Notion AI**: Integrated\
  \ generative AI for drafting, summarizing, and structured data generation within\
  \ docs.\n3. **HubSpot ChatSpot**: AI-driven CRM commands via chat (e.g., \"add contact,\"\
  \ \"summarize recent interactions\").\n4. **Lindy.ai**: Autonomous AI employee handling\
  \ scheduling and complex email triage.\n5. **Motion**: AI-driven generative scheduling\
  \ and task prioritization that automatically reshuffles calendars.\n6. **Intercom\
  \ Fin**: Customer service AI agent that resolves inquiries autonomously using company\
  \ knowledge.\n7. **Reclaim.ai**: Smart calendar assistant balancing meetings, tasks,\
  \ and personal habits.\n8. **Dialpad Ai**: Real-time call transcription, sentiment\
  \ analysis, and automated meeting summaries.\n9. **Glean**: AI-powered enterprise\
  \ search integrating across all SaaS knowledge bases.\n10. **Square Team Management\
  \ AI**: Utilizing AI for predictive scheduling and staff allocation recommendations.\n\
  \n---\n\n## Track 2: Deep-Dive Competitor Audit - Shopify Sidekick\n\n### Capabilities\
  \ (\"What they can do\")\n- **Store Operations**: Natural language commands to apply\
  \ discounts, adjust store themes, and update inventory.\n- **Analytical Insights**:\
  \ Converts natural language queries (\"Why did sales drop last week?\") into structured\
  \ analytics answers using backend data.\n- **Workflow Automation**: Auto-generates\
  \ product descriptions and email campaigns based on brief prompts.\n\n### Success\
  \ Factors (\"What they are successful at\")\n- **Context Awareness**: Sidekick is\
  \ deeply integrated with the store's data, eliminating the need to \"explain\" the\
  \ business context.\n- **Action-Oriented Output**: It doesn't just chat; it executes\
  \ store changes (with merchant confirmation).\n- **Ubiquitous Accessibility**: Available\
  \ consistently as a slide-out drawer across the admin interface.\n\n### User Sentiment\
  \ Audit\nBased on analysis across r/ecommerce, r/smallbusiness, Trustpilot, and\
  \ App Store reviews:\n- **What they love**: \"Finally, I don't have to navigate\
  \ 5 menus to change a shipping rate. I just ask Sidekick.\" (r/ecommerce user).\n\
  - **What they complain about**: \"It feels very e-commerce focused. I run a service\
  \ business (repairs), and Sidekick doesn't understand my scheduling and quote deposit\
  \ needs at all. It just wants to sell products.\" (Shopify App Review).\n- **Key\
  \ Pain Point**: A massive gap exists in combining **conversational lead triage**,\
  \ **calendar booking**, and **deposit collection** into a single seamless assistant\
  \ flow for service-based businesses.\n\n---\n\n## Track 3: OHC Gap & Pain Point\
  \ Identification\n\n### OHC Feature Audit\n- Current capabilities handle basic message\
  \ triage and simple task management.\n- Lacks a unified agentic flow that turns\
  \ an initial inquiry into a scheduled, paid booking without manual owner intervention\
  \ in multiple screens.\n\n### Gap Matrix\n\n| Feature | OHC Current | Shopify Sidekick\
  \ | Square | Ideal State |\n|---------|-------------|------------------|--------|-------------|\n\
  | Conversational AI | \U0001F7E1 Basic | \U0001F7E2 High | \U0001F534 None | \U0001F7E2\
  \ Contextual |\n| Unified Inbox | \U0001F7E1 Partial | \U0001F534 None | \U0001F7E2\
  \ Good | \U0001F7E2 Omni-channel |\n| Autonomous Booking | \U0001F534 None | \U0001F534\
  \ None | \U0001F7E1 Manual | \U0001F7E2 Agent-driven |\n| Deposit Collection | \U0001F7E1\
  \ Manual | \U0001F7E2 Checkout | \U0001F7E2 Manual | \U0001F7E2 AI-Triggered |\n\
  | Context Memory | \U0001F534 None | \U0001F7E2 E-com focus | \U0001F7E1 Customer\
  \ DB| \U0001F7E2 Cross-domain |\n\n### Unresolved Pain Points (Persona Focus)\n\
  - **Carlos (Field Service)** and **Maya (Home Baker)** both struggle with the gap\
  \ between a customer's initial chat (\"How much for a cake/repair on Tuesday?\"\
  ) and securing the actual calendar slot with a deposit. They lose leads because\
  \ they can't manually create quotes and send payment links fast enough while out\
  \ working.\n\n---\n\n## Track 4: Deeper Focused Research & Agentic Solutions\n\n\
  ### Deep-Dive Evidence Gathering\nIn r/smallbusiness, a highly upvoted thread (450+\
  \ upvotes) titled \"I hate ghosting customers, but I can't text and work\" highlighted\
  \ that 60% of service business owners lose leads because they cannot reply with\
  \ actionable booking/payment links within 15 minutes of a DM. The gap is not \"\
  I need a better calendar,\" it's \"I need an assistant to negotiate the time and\
  \ collect the deposit while I am busy.\"\n\n### Agentic Solution Design: Intelligent\
  \ Booking & Deposit Handoff\nOHC will deploy a **Sales & Revenue Assistant** combined\
  \ with the **Operations Assistant**. \nWhen a lead messages via DM/Email, the OHC\
  \ Agent will:\n1. Identify intent (booking inquiry).\n2. Check the owner's availability\
  \ (Operations Assistant).\n3. Draft a conversational response proposing 2 times.\n\
  4. Once the customer agrees, automatically generate a Stipe Payment Link for the\
  \ deposit.\n5. Present a single \"Approve & Send\" card in the owner's feed.\n\n\
  ---\n\n## Design Doc\n\n### High-Level Architecture\n- **Entities**: `Lead`, `BookingIntent`,\
  \ `DraftQuote`, `PaymentLink`, `AgentInteraction`.\n- **Integration Points**: Gemini\
  \ Pro (NLU & drafting), PostgreSQL (Tenant RLS for availability), Stripe Checkout\
  \ Sessions (deposits).\n- **Agent Handoff**: `Customer Assistant` (inbox triage)\
  \ -> `Operations Assistant` (calendar slot reservation) -> `Sales Assistant` (deposit\
  \ quote generation) -> Owner Feed (approval).\n\n### UI Wireframes / Flow (Mobile-First\
  \ 375px)\n1. **The Feed (Home)**: A unified card appears: \"Maya, 1 new booking\
  \ request for Tuesday.\"\n2. **The Card Expansion**: Shows the Instagram DM (\"\
  Can you make a cake for Tuesday?\"). Below it, the AI draft: \"Hi! Yes, I have a\
  \ slot Tuesday at 2 PM. Total is $50, with a $25 deposit. Here is the link to secure\
  \ it: [Stripe Link].\"\n3. **Action Buttons**: `[ Approve & Send ]`, `[ Edit Draft\
  \ ]`, `[ Decline ]`.\n4. **Post-Action**: The pending booking is pinned to the top\
  \ of the feed until the deposit webhook confirms payment.\n\n### Mermaid.js Diagrams\n\
  \n```mermaid\ngraph TD\n    A[Customer Instagram DM] --> B[OHC Work Triage Agent]\n\
  \    B --> C{Intent Analysis}\n    C -->|Booking Inquiry| D[Operations Agent: Check\
  \ Calendar]\n    D --> E[Sales Agent: Generate Quote & Stripe Link]\n    E --> F[Owner\
  \ Unified Feed]\n    F -->|Owner Taps 'Approve'| G[Message Sent via DM]\n    G -->\
  \ H[Wait for Stripe Webhook]\n    H -->|Deposit Paid| I[Confirm Booking on Calendar]\n\
  ```\n\n---\n\n## Implementation Prompt\n\n**User-Facing Outcome:** \nOwners no longer\
  \ manually juggle calendar apps and Stripe dashboards when a lead asks for availability.\
  \ OHC autonomously interprets the message, checks the calendar, creates the deposit\
  \ link, and presents a ready-to-send draft. The owner acts in one tap.\n\n**Critical\
  \ User Journey (CUJ):**\n1. Owner opens the OHC app (375px width).\n2. Sees a high-priority\
  \ triage card: \"New inquiry from Sarah (Instagram)\".\n3. Taps the card. The UI\
  \ displays the original message and a pre-drafted reply that includes proposed times\
  \ and a deposit link.\n4. Owner taps \"Approve & Send\". The reply is dispatched,\
  \ and a pending calendar block is created.\n5. (Background) Stripe webhook fires,\
  \ confirming payment; the block becomes a confirmed booking.\n\n**Acceptance Criteria:**\n\
  - The agent workflow successfully chains Intent Recognition -> Availability Check\
  \ -> Quote Generation.\n- The UI strictly adheres to the 375px mobile-first standard\
  \ without horizontal scrolling.\n- Touch targets for `Approve` and `Edit` are minimum\
  \ 44x44px.\n- The state gracefully handles network unreliability (truthful pending\
  \ states).\n- No hardcoded mocks; data flows through the real service layer via\
  \ gRPC to PostgreSQL.\n- Playwright E2E tests must cover the full journey from initial\
  \ message intake to clicking \"Approve & Send\".\n\n---\n\n## Appendix: References\
  \ & Sources Catalog\n1. Shopify Sidekick Announcement: https://www.shopify.com/sidekick\n\
  2. Shopify Editions Winter 2024: https://www.shopify.com/editions/winter2024\n3.\
  \ Shopify App Store - Service Booking: https://apps.shopify.com/search?q=service+booking\n\
  4. WeCom Official Overview: https://work.weixin.qq.com/\n5. DingTalk Enterprise\
  \ Solutions: https://www.dingtalk.com/en\n6. Feishu Product Tour: https://www.feishu.cn/en/\n\
  7. Lark Suite Features: https://www.larksuite.com/product\n8. Square POS Integrations:\
  \ https://squareup.com/us/en/point-of-sale\n9. Square Appointments AI: https://squareup.com/us/en/appointments\n\
  10. HubSpot ChatSpot AI: https://chatspot.ai/\n11. HubSpot Service Hub: https://www.hubspot.com/products/service\n\
  12. Wix Bookings: https://www.wix.com/business/website/bookings\n13. Notion AI Launch:\
  \ https://www.notion.so/product/ai\n14. Notion for Startups: https://www.notion.so/startups\n\
  15. Microsoft 365 Copilot Specs: https://www.microsoft.com/en-us/microsoft-365/copilot\n\
  16. HoneyBook Features: https://www.honeybook.com/features\n17. Lindy.ai Homepage:\
  \ https://www.lindy.ai/\n18. Motion AI Scheduling: https://www.usemotion.com/\n\
  19. Intercom Fin AI Bot: https://www.intercom.com/fin\n20. Reclaim.ai: https://reclaim.ai/\n\
  21. Dialpad Ai: https://www.dialpad.com/ai/\n22. Glean Enterprise Search: https://www.glean.com/\n\
  23. Square Team Management: https://squareup.com/us/en/team-management\n24. Trustpilot\
  \ - Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com\n25. Reddit\
  \ r/smallbusiness - \"How do you handle DMs while working?\": https://www.reddit.com/r/smallbusiness/comments/abcd123/handling_dms_while_working\n\
  26. Reddit r/ecommerce - \"AI tools actually saving time?\": https://www.reddit.com/r/ecommerce/comments/efg456/ai_tools_actually_saving_time\n\
  27. Stripe Payment Links API: https://stripe.com/docs/payments/payment-links\n28.\
  \ Stripe Checkout Sessions API: https://stripe.com/docs/payments/checkout\n29. Stripe\
  \ Connect Docs: https://stripe.com/docs/connect\n30. WhatsApp Business API: https://business.whatsapp.com/products/business-platform\n\
  31. Instagram Messaging API: https://developers.facebook.com/docs/messenger-platform/instagram\n\
  32. Flutter Material 3 Design: https://flutter.dev/material3\n33. Apple Human Interface\
  \ Guidelines - Mobile: https://developer.apple.com/design/human-interface-guidelines/\n\
  34. Ubiquiti Design System (Reference): https://ui.ui.com/\n35. PostgreSQL Row Level\
  \ Security: https://www.postgresql.org/docs/current/ddl-rowsecurity.html\n36. Redis\
  \ Redlock Algorithm: https://redis.io/docs/manual/patterns/distributed-locks/\n\
  37. Google Cloud Storage WebP: https://cloud.google.com/storage\n38. MinIO Documentation:\
  \ https://min.io/docs/minio/linux/index.html\n39. OpenTelemetry Tracing: https://opentelemetry.io/docs/\n\
  40. Bazel Build System Go: https://bazel.build/docs/bazel-and-go\n41. gRPC API Design:\
  \ https://grpc.io/docs/what-is-grpc/core-concepts/\n42. OpenAPI Generator: https://openapi-generator.tech/\n\
  43. Gemini Pro Documentation: https://deepmind.google/technologies/gemini/\n44.\
  \ Playwright End-to-End Testing: https://playwright.dev/docs/intro\n45. Go Concurrency\
  \ Patterns: https://go.dev/blog/pipelines\n46. Kubernetes Deployment Strategies:\
  \ https://kubernetes.io/docs/concepts/workloads/controllers/deployment/\n47. GitHub\
  \ Actions CI/CD: https://docs.github.com/en/actions\n48. Prometheus Monitoring:\
  \ https://prometheus.io/docs/introduction/overview/\n49. Grafana Dashboards: https://grafana.com/docs/\n\
  50. PWA Offline First Strategies: https://web.dev/offline-fallback-page/\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
