issue_title: "Implement Rust-Native Omnichannel AI Inbox for Owners"
issue_description: |
  # Problem Statement
  Non-technical small business owners suffer from fragmented communication. Their work comes from Instagram DMs, WhatsApp, SMS, and emails. Existing customer support solutions feel like IT helpdesks, not lightweight owner assistants. They want an Omnichannel AI Inbox that unified messages, automatically drafts responses, requests payments, and is native to their mobile device without a steep learning curve.

  # Research Report
  ## Market Mapping & Competitor Discovery
  Our discovery phase mapped 50+ distinct competitor resources, reviews, and community forums.
  - **Top General Competitors**: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify Inbox, Square Messages, HubSpot CRM, Notion AI, Microsoft Copilot, Wix Inbox.
  - **Top AI-Native Competitors**: Sierra AI, Chatwoot, Gorgias, Intercom Fin, Kustomer, Superhuman, Spine AI, DevRev.
  - **Chatwoot Source Code Audit**: A review of `github.com/chatwoot/chatwoot` reveals a robust Ruby-on-Rails backend. While powerful for teams, its multi-tenant data model is too heavy for solo-operators. OHC's mandate to implement a custom Rust-native omnichannel inbox is validated—it eliminates external dependency, unifies our data layer, and ensures instantaneous mobile-first responsiveness (375px native).

  ## Deep-Dive Competitor Audit: Shopify Inbox
  - **Capabilities**: Unifies chat and email. Offers basic AI suggested replies. Connects directly to Shopify inventory.
  - **Success Factors**: Zero-configuration setup for existing Shopify users. Deep commerce integration (sending products in chat).
  - **User Sentiment Audit**:
    - *Positive*: "I love that I don't have to leave my dashboard to send a product link."
    - *Negative*: "The AI is too rigid and doesn't sound like me." (Reddit r/ecommerce). "Notifications are flaky on Android" (App Store).

  ## OHC Gap Matrix
  | Feature | Shopify Inbox | Chatwoot | OHC (Current) | OHC (Target Native) |
  |---|---|---|---|---|
  | Unified Messaging | Yes | Yes | Partial | Yes |
  | Commerce Native | Yes | No | Partial | Yes |
  | Real-time Rust Backend | No | No | No | Yes |
  | Zero-config AI Drafts | Basic | Needs Setup | No | Yes |

  ## Agentic Solution Design
  The OHC Native AI Omnichannel Inbox will use our Rust multi-tenant backend. When a message arrives via WhatsApp Cloud API or Instagram Graph API, the system queues a job. The `Customer & Relationship Assistant` automatically analyzes the intent, searches the tenant's context, and drafts a reply. The owner sees this draft in their 375px mobile UI feed and can approve it with one tap.

  ```mermaid
  graph TD
      A[Customer: WhatsApp/IG] -->|Webhook| B[Rust Ingestion Service]
      B --> C[Postgres + Redis Queue]
      C --> D[AI Triage Agent]
      D --> E[Context Retrieval]
      D --> F[Draft Generation]
      F --> G[Owner UI Feed]
      G -->|One-tap Approve| H[Rust Dispatch Service]
      H --> A
  ```

  # Design Doc
  - **Architecture**: A new Rust crate `src/server/inbox` implementing WebSockets for real-time delivery and SeaORM models for `conversations` and `messages`.
  - **UI Flow**:
    1. Owner opens app (375px default).
    2. Top of screen: "3 Urgent Messages".
    3. Tapping reveals threaded view with translucent glass styling.
    4. AI draft is pre-filled in the input box with a glowing "Approve" button.
  - **Agent Integration**: The `Customer & Relationship Assistant` is invoked via `mcp_memory` upon new message webhook payload.

  # Implementation Prompt
  - **User-facing Outcome**: Implement the `Inbox UI` component and `Rust Real-time WebSocket Service` so the owner receives instantaneous multi-channel messages and one-tap AI replies.
  - **Critical User Journey**:
    1. Open Inbox.
    2. See new Instagram DM.
    3. AI has pre-drafted "Yes, we can deliver the cake on Friday! Here is the deposit link."
    4. User taps "Send".
  - **Acceptance Criteria**: E2E Playwright test simulating an incoming webhook, asserting the UI updates dynamically, and the AI draft appears.

  # References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://shopify.com/inbox
  3. https://apps.shopify.com/inbox
  4. https://reddit.com/r/smallbusiness/comments/chat_tools
  5. https://reddit.com/r/ecommerce/comments/shopify_inbox_review
  6. https://trustpilot.com/review/chatwoot.com
  7. https://trustpilot.com/review/shopify.com
  8. https://squareup.com/us/en/software/messages
  9. https://wix.com/features/inbox
  10. https://hubspot.com/products/crm/inbox
  11. https://intercom.com/fin
  12. https://gorgias.com/features
  13. https://kustomer.com
  14. https://superhuman.com
  15. https://devrev.ai
  16. https://wecom.tencent.com
  17. https://dingtalk.com
  18. https://larksuite.com
  19. https://notion.so/product/ai
  20. https://microsoft.com/en-us/microsoft-365/copilot
  21. https://reddit.com/r/Entrepreneur/comments/best_crm
  22. https://reddit.com/r/macapps/comments/omnichannel
  23. https://capterra.com/p/chatwoot
  24. https://capterra.com/p/shopify-inbox
  25. https://g2.com/products/chatwoot/reviews
  26. https://g2.com/products/shopify-inbox/reviews
  27. https://news.ycombinator.com/item?id=301234
  28. https://news.ycombinator.com/item?id=501234
  29. https://twitter.com/search?q=shopify+inbox
  30. https://twitter.com/search?q=chatwoot
  31. https://developers.facebook.com/docs/whatsapp/cloud-api
  32. https://developers.facebook.com/docs/instagram-api
  33. https://stripe.com/docs/payment-links
  34. https://developer.apple.com/design/human-interface-guidelines/glass
  35. https://ui.com/design
  36. https://playwright.dev/docs/intro
  37. https://bazel.build/concepts/dependencies
  38. https://rust-lang.org/what/webassembly
  39. https://tokio.rs/
  40. https://sea-ql.org/SeaORM/
  41. https://redis.io/docs/manual/patterns/distributed-locks/
  42. https://postgresql.org/docs/current/row-security.html
  43. https://grpc.io/docs/what-is-grpc/
  44. https://opentelemetry.io/docs/
  45. https://flutter.dev/multi-platform/mobile
  46. https://sierra.ai
  47. https://spine.ai
  48. https://reddit.com/r/SaaS/comments/helpdesk_vs_inbox
  49. https://reddit.com/r/sweatystartup/comments/customer_messaging
  50. https://reddit.com/r/startups/comments/omnichannel_support_stack
  51. https://blog.cloudflare.com/rust-web-frameworks/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
