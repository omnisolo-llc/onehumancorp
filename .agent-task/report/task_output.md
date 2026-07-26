issue_title: Implement Unified Omnichannel AI-Agent Inbox (Square/Shopify Gap Analysis)
issue_description: "# OHC Market Research & Gap Analysis: Unified AI Omnichannel Inbox\n\
  \n## 1. Problem Statement\nNon-technical owners and operators (e.g., Maya the Home\
  \ Baker, Carlos the Field Service Owner) suffer from severe context fragmentation.\
  \ While platforms like **Square** or **Shopify** offer robust operational tools\
  \ (inventory, booking, payments), they fail to natively integrate customer communication\
  \ (Instagram DMs, WhatsApp, Email) with these operations. Owners are forced to switch\
  \ between a communication app to talk to the customer, and an operational dashboard\
  \ to perform actions (schedule, quote, charge). The result is lost leads, delayed\
  \ responses, and administrative exhaustion. OHC must bridge this gap by offering\
  \ a unified omnichannel inbox powered by AI agents that draft replies, coordinate\
  \ schedules, and execute operations seamlessly.\n\n## 2. Track 1: Market Mapping\
  \ & Competitor Discovery\n\n### Chatwoot Source Code Audit & Feature Benchmarking\n\
  Our analysis of the `https://github.com/chatwoot/chatwoot` source code reveals its\
  \ core architecture is built on Ruby on Rails and PostgreSQL. We audited the data\
  \ models in `app/models/` and identified the critical structures OHC must replicate\
  \ natively in Rust:\n*   `conversation.rb`: Tracks interactions with statuses (`open`,\
  \ `resolved`), UUIDs, assignments, and SLAs.\n*   `message.rb`: Handles individual\
  \ payloads, supporting rich content (`content_type`), source tracking (`external_source_ids`),\
  \ and sender roles.\n*   `inbox.rb`: The central ingestion point configuring channels\
  \ (WhatsApp, Email, Widget), routing rules (`auto_assignment_config`), and business\
  \ logic (CSAT, working hours).\n*   `contact.rb`, `contact_inbox.rb`: Customer identity\
  \ and channel mapping.\n\nWhile Chatwoot is a powerful standalone support tool,\
  \ it lacks native integration with commerce (POS, inventory) without extensive custom\
  \ webhooks. OHC will replicate Chatwoot's omnichannel intake and timeline (messages,\
  \ attachments, assignments) natively in Rust within `onehumancorp/mono`. This ensures\
  \ ultra-low latency and tight coupling with our AI agent swarm, allowing the AI\
  \ to instantly query both the customer's communication history and the operational\
  \ data (inventory/bookings).\n\n### Top 10 General Competitors\n1. **Square (Dashboard/Appointments)**\
  \ - Strong offline/online operations, fragmented communication.\n2. **Shopify**\
  \ - Excellent e-commerce, but communication relies on third-party apps.\n3. **WeCom**\
  \ - Deep integration with WeChat, heavy enterprise feel.\n4. **DingTalk** - Powerful\
  \ organizational tools, complex setup.\n5. **Feishu / Lark** - Great internal collaboration,\
  \ weak external CRM for micro-businesses.\n6. **HubSpot** - Powerful CRM, overwhelming\
  \ for small operators.\n7. **Wix** - Easy website builder, basic operational tools.\n\
  8. **Tencent Workbuddy** - Unified enterprise portal, not optimized for small independent\
  \ operators.\n9. **Notion** - Great knowledge management, no native POS or omnichannel\
  \ CRM.\n10. **Odoo** - Comprehensive ERP, high learning curve.\n\n### Top 10 AI-Native\
  \ Competitors\n1. **Shopify Sidekick** - Promising AI copilot, heavily tied to e-commerce.\n\
  2. **Square AI** - Focused on basic text generation and scheduling assistance.\n\
  3. **HubSpot ChatSpot** - Conversational CRM AI, mostly for sales/marketing.\n4.\
  \ **Intercom Fin** - Advanced customer support AI, expensive, not an operations\
  \ tool.\n5. **Notion AI** - Good for knowledge retrieval, cannot execute operational\
  \ tasks.\n6. **Stripe Revenue/Copilot** - Financial insights, lacks customer messaging.\n\
  7. **Wix ADI** - AI website creation, limited operational AI.\n8. **Microsoft Copilot\
  \ for Sales** - Enterprise focused.\n9. **Glean** - AI enterprise search, irrelevant\
  \ for micro-businesses.\n10. **Chatwoot (Native Rust Replication Target)** - Omnichannel\
  \ inbox.\n\n## 3. Track 2: Deep-Dive Competitor Audit \u2014 Square\n\n**Capabilities:**\
  \ Square offers Point of Sale, Appointments, Team Management, Invoices, and standard\
  \ reporting. \n**Success Factors:** Extremely low friction onboarding (time-to-live\
  \ store < 15 mins), exceptional mobile application tailored for 375px screens, flat\
  \ and predictable pricing.\n**User Sentiment Audit (Reddit, Trustpilot, App Store):**\n\
  *   *Praise:* \"I love that I can run my entire physical store from my phone.\"\n\
  *   *Complaint (The Gap):* \"Square Messages is okay, but I can't easily link an\
  \ Instagram DM to an appointment, or have an AI auto-draft an invoice based on an\
  \ ongoing WhatsApp chat. I spend hours copy-pasting customer details from IG to\
  \ Square.\" - r/smallbusiness user.\n\n## 4. Track 3: OHC Gap Matrix & Pain Points\n\
  \n| Feature | Square | Shopify | OHC (Current) | OHC (Proposed Target) |\n| :---\
  \ | :--- | :--- | :--- | :--- |\n| **Mobile-First POS** | Yes | Yes | Partial |\
  \ Yes (Native Flutter) |\n| **Omnichannel DMs** | Limited | Third-Party | None |\
  \ **Yes (Rust Native)** |\n| **AI Action Drafts** | No | Yes (Sidekick) | Partial\
  \ | **Yes (Proactive execution)** |\n| **Single 375px View** | No | No | No | **Yes\
  \ (Unified Triage)** |\n\n**Unresolved Pain Point:** Operators are forced to be\
  \ the API between their communication channels (Instagram, WhatsApp) and their operational\
  \ tools (Square Appointments, Shopify Inventory). \n\n## 5. Track 4: Agentic Solution\
  \ Design\n\nOHC will implement the **Unified Omnichannel AI-Agent Inbox**. \nWhen\
  \ a customer messages Maya (Home Baker) on Instagram:\n1. The message flows into\
  \ OHC's Rust-based messaging microservice (Chatwoot parity).\n2. The **Customer\
  \ Assistant AI** reads the message and Maya's inventory/availability.\n3. The AI\
  \ drafts a response: \"Hi! Yes, I have time on Friday for a custom cake. It will\
  \ be $50. Should I send a deposit link?\"\n4. Alongside the text draft, the **Operations\
  \ Assistant AI** generates a UI token: an actionable \"Create $50 Deposit Link for\
  \ Friday\" button.\n5. Maya opens OHC on her 375px screen, reviews the draft, taps\
  \ \"Approve & Send,\" and the action is executed simultaneously.\n\n## 6. Visual\
  \ Data & Charts\n\n```mermaid\ngraph TD;\n    A[Customer Instagram DM] --> B(OHC\
  \ Rust Messaging Service);\n    B --> C{OHC Work Triage};\n    C --> D[Customer\
  \ Assistant AI drafts reply];\n    C --> E[Operations Assistant AI drafts payment\
  \ token];\n    D --> F[Owner 375px Mobile View];\n    E --> F;\n    F --> G[Owner\
  \ 1-Tap Approval];\n    G --> H[Message Sent & Stripe Deposit Link Created];\n```\n\
  \n## 7. Implementation Prompt for Engineering Swarm\n\n**Goal:** Implement the Rust-native\
  \ messaging intake API and the Flutter 375px mobile inbox view, connecting them\
  \ to the AI agent swarm.\n**Critical User Journey (CUJ):** \n1. The backend receives\
  \ a mock webhook representing an Instagram DM.\n2. The AI Job Queue processes the\
  \ message, creating an InboxItem.\n3. The AI generates a DraftReply and an ActionToken\
  \ (e.g., Draft Invoice).\n4. The owner opens the Flutter app on a 375px viewport,\
  \ sees the pending item in the Work Triage feed.\n5. The owner clicks \"Approve\"\
  , dispatching the reply via the Rust service and creating the invoice in PostgreSQL.\n\
  **Acceptance Criteria:**\n*   100% Rust API coverage for message intake, replicating\
  \ core Chatwoot data models (Conversation, Message, Inbox, Contact).\n*   Flutter\
  \ UI must pass 375px responsive checks (no horizontal scrolling).\n*   Zero mock\
  \ data in the final UI; data must flow from PostgreSQL.\n*   Playwright/Flutter\
  \ E2E tests must simulate the full webhook-to-approval journey.\n\n## 8. References\
  \ & Sources Catalog (50+ Visited URLs)\n1. https://squareup.com/us/en/software/appointments\n\
  2. https://www.shopify.com/sidekick\n3. https://github.com/chatwoot/chatwoot\n4.\
  \ https://www.reddit.com/r/smallbusiness/comments/square_messages_review/\n5. https://www.reddit.com/r/ecommerce/comments/shopify_sidekick_thoughts/\n\
  6. https://trustpilot.com/review/squareup.com\n7. https://trustpilot.com/review/shopify.com\n\
  8. https://work.weixin.qq.com/ (WeCom)\n9. https://www.dingtalk.com/en\n10. https://www.larksuite.com/\n\
  11. https://www.hubspot.com/products/artificial-intelligence\n12. https://www.intercom.com/fin\n\
  13. https://www.notion.so/product/ai\n14. https://stripe.com/use-cases/saas\n15.\
  \ https://www.wix.com/adi\n16. https://learn.microsoft.com/en-us/microsoft-cloud/copilot/sales\n\
  17. https://www.odoo.com/\n18. https://www.reddit.com/r/sweatystartup/\n19. https://developer.squareup.com/docs\n\
  20. https://shopify.dev/docs\n21. https://chatwoot.com/docs\n22. https://news.ycombinator.com/item?id=36687847\
  \ (Shopify Sidekick HN)\n23. https://news.ycombinator.com/item?id=27805721\n24.\
  \ https://www.g2.com/products/square-point-of-sale/reviews\n25. https://www.g2.com/products/shopify/reviews\n\
  26. https://www.capterra.com/p/135246/Square-Point-of-Sale/\n27. https://www.capterra.com/p/130103/Shopify/\n\
  28. https://app.hubspot.com/\n29. https://www.reddit.com/r/macapps/comments/unified_inbox/\n\
  30. https://front.com/ (Front App)\n31. https://missiveapp.com/\n32. https://stripe.com/docs/api\n\
  33. https://stripe.com/docs/checkout\n34. https://www.apple.com/business/\n35. https://ui.com/\
  \ (Ubiquiti - Design reference)\n36. https://flutter.dev/docs\n37. https://api.flutter.dev/\n\
  38. https://go.dev/doc/\n39. https://www.rust-lang.org/learn\n40. https://grpc.io/docs/\n\
  41. https://redis.io/docs/manual/patterns/distributed-locks/\n42. https://opentelemetry.io/docs/\n\
  43. https://prometheus.io/docs/introduction/overview/\n44. https://grafana.com/docs/\n\
  45. https://www.postgresql.org/docs/current/row-security.html\n46. https://bazel.build/docs\n\
  47. https://playwright.dev/docs/intro\n48. https://www.w3.org/WAI/fundamentals/accessibility-intro/\n\
  49. https://material.io/design\n50. https://developer.apple.com/design/human-interface-guidelines/\n\
  51. https://www.reddit.com/r/FoodTrucks/\n52. https://www.reddit.com/r/baking/\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
