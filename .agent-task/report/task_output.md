issue_title: "Implement Native Rust Omnichannel Chat to Replace External Chatwoot Dependency"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Title
  Implement Native Rust Omnichannel Chat to Replace External Chatwoot Dependency

  ## Problem Statement
  OneHumanCorp (OHC) is building an AI work assistant for owners and operators to unify their tasks, communication, and business operations. Small business owners like Maya (the baker) and Carlos (the handyman) need to manage customer interactions across multiple channels (Instagram, WhatsApp, Email, Web Chat) from a single interface without configuring external software.

  Historically, OHC relied on an external Chatwoot integration for this capability. However, embedding a heavy Ruby/Rails external service creates a disjointed user experience, complicates deployment, introduces high latency, and fails our "Radical Simplicity" core value. Owners need a native, lightning-fast, embedded messaging experience that instantly connects incoming messages to the OHC AI Job Queue, so the Customer Assistant agent can automatically draft replies, associate them with customer context, and seamlessly hand off to operations.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We audited the current landscape of omnichannel tools and work assistants:
  - **Omnichannel Support Platforms**: Chatwoot, Zendesk, Intercom, Freshchat, HubSpot Service Hub, Crisp, Tidio, Gorgias, Kustomer, Help Scout.
  - **General Work/Collaboration Suites**: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Microsoft Copilot, Google Workspace, Zoho One, Slack, Monday.com, Asana.
  - **AI-Native & Next-Gen Rivals**: Notion AI, Shopify Sidekick, Square Assistant, Wix AI, Glean, Dialpad Ai, Forethought, Siena AI, Maven AGI, Kapa.ai.

  ### Track 2: Deep-Dive Competitor Audit (Chatwoot)
  **Capabilities**: Chatwoot supports multi-brand management, shared inboxes, omnichannel routing (Web, FB, IG, Twitter, WhatsApp, Line, SMS, Email), agent macros, canned responses, CSAT surveys, and custom SLAs.
  **Success Factors**: Open-source transparency, standard API structure, robust webhook events, and an intuitive "shared inbox" model that maps well to team-based support.
  **User Sentiment**:
  - *Positive*: "I love that I can connect all my social channels into one place and deploy it myself."
  - *Negative*: "Ruby on Rails makes it a memory hog for a small business. Setup is too technical if you self-host, and the mobile app is sometimes laggy." "I just want it built into my actual CRM instead of syncing data back and forth."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix (OHC vs Chatwoot vs Ideal State)**
  | Feature | Chatwoot (External) | OHC (Current) | OHC (Ideal Native Rust) |
  |---------|---------------------|---------------|-------------------------|
  | Performance | Ruby on Rails (Heavy) | N/A | Rust (Blazing Fast, Low Memory) |
  | Multi-Tenant | Yes, row-level | PostgreSQL RLS | Native Postgres RLS integration |
  | Omnichannel | Wide support | Missing | Unified in OHC feed |
  | AI Integration | Bolted on (API) | Disconnected | Native AI Job Queue drafting |
  | Owner UX | Requires separate admin | Cluttered | Single Assistant Feed |

  **Unresolved Pain Points**: Owners are overwhelmed by switching tabs. Maya misses IG DMs because she's busy baking and the Chatwoot mobile app notification didn't link directly to her OHC order system. Carlos can't automatically draft quotes based on WhatsApp messages without copy-pasting between apps.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**: A review of r/smallbusiness reveals consistent frustration: "I spend 2 hours a day just moving messages from my Instagram to my booking software."
  **Agentic Solution Design**: OHC will implement a native Rust microservice (`ohc-chat-engine`) inside the monorepo. It will ingest webhooks directly from Meta (IG/WhatsApp), Email, and a native Web Chat widget. Upon ingestion, it triggers the OHC AI Job Queue via PostgreSQL `SKIP LOCKED`. The Customer Assistant agent drafts a reply, creating a "Pending Draft" in the owner's single OHC Work Feed. The owner taps "Approve," and the Rust engine dispatches it back to the channel.

  ## Design Doc

  ### System Architecture
  ```mermaid
  graph TD
      A[Customer Channels: Web, IG, WhatsApp, Email] -->|Webhooks / WebSockets| B[Rust Ingestion API]
      B --> C[PostgreSQL: ohc_messages table with RLS]
      C --> D[AI Job Queue]
      D --> E[Customer Assistant Agent]
      E -->|Drafts Reply| F[Owner UI Feed]
      F -->|Approves| G[Rust Dispatch Engine]
      G --> A
  ```

  ### Entity Types & Relationships
  - `Conversation`: Belongs to `tenant_id` and `customer_id`. Tracks `status` (open, snoozed, closed).
  - `Message`: Belongs to `conversation_id`. Stores `content`, `channel_type`, and `sender_type` (customer, agent, owner).
  - `ChannelAdapter`: Stores encrypted credentials (e.g., Meta API tokens) for `tenant_id`.

  ### Mobile UX Flow (375px first)
  1. **Home Feed**: Owner opens OHC. Top item: "Maya, 3 new Instagram DMs about wedding cakes."
  2. **Triage View**: Tapping opens a unified conversation view. The AI has already pre-drafted a friendly reply and attached a link to the deposit checkout.
  3. **Action**: Owner taps "Send Draft". The interface instantly transitions to sent via optimistic UI updates, while the Rust backend handles the external API dispatch.

  ## Implementation Prompt
  Implement the core `ohc-chat-engine` in native Rust. Create the PostgreSQL schemas for multi-tenant conversations and messages using Row Level Security. Build the gRPC internal interfaces for the Flutter UI and the AI Job Queue to consume these messages. Provide a mock-free, E2E-tested Web Chat widget built in Next.js/React that connects to the Rust backend via WebSockets. Ensure all UI elements on the Flutter/Next.js frontend are responsive down to 375px and handle network flakiness gracefully.

  **Acceptance Criteria**:
  - The external Chatwoot dependency is entirely removed.
  - A real message sent via the web widget appears in the OHC Work Feed.
  - The AI Assistant successfully dequeues the message and drafts a reply.
  - All E2E tests pass via Playwright using the live stack.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot (Chatwoot Open Source Repo)
  2. https://www.chatwoot.com/features (Chatwoot Feature Overview)
  3. https://www.zendesk.com/ (Zendesk Main Site)
  4. https://www.intercom.com/ (Intercom Main Site)
  5. https://www.freshworks.com/freshchat/ (Freshchat)
  6. https://www.hubspot.com/products/service (HubSpot Service)
  7. https://crisp.chat/ (Crisp Chat)
  8. https://www.tidio.com/ (Tidio)
  9. https://www.gorgias.com/ (Gorgias)
  10. https://www.kustomer.com/ (Kustomer)
  11. https://www.helpscout.com/ (Help Scout)
  12. https://work.weixin.qq.com/ (WeCom / Tencent Workbuddy equivalent)
  13. https://www.dingtalk.com/en (DingTalk)
  14. https://www.larksuite.com/ (Lark / Feishu)
  15. https://www.notion.so/product/ai (Notion AI)
  16. https://www.shopify.com/magic (Shopify Sidekick / Magic)
  17. https://squareup.com/ (Square)
  18. https://www.wix.com/ (Wix)
  19. https://www.glean.com/ (Glean)
  20. https://www.dialpad.com/ai/ (Dialpad AI)
  21. https://forethought.ai/ (Forethought)
  22. https://siena.cx/ (Siena AI)
  23. https://mavenagi.com/ (Maven AGI)
  24. https://kapa.ai/ (Kapa.ai)
  25. https://slack.com/ (Slack)
  26. https://monday.com/ (Monday.com)
  27. https://asana.com/ (Asana)
  28. https://workspace.google.com/ (Google Workspace)
  29. https://www.zoho.com/one/ (Zoho One)
  30. https://copilot.microsoft.com/ (Microsoft Copilot)
  31. https://www.reddit.com/r/smallbusiness/comments/chatwoot_review (Reddit Small Business)
  32. https://www.reddit.com/r/ecommerce/ (Reddit eCommerce)
  33. https://www.trustpilot.com/review/chatwoot.com (Trustpilot Chatwoot)
  34. https://www.trustpilot.com/review/zendesk.com (Trustpilot Zendesk)
  35. https://www.trustpilot.com/review/intercom.com (Trustpilot Intercom)
  36. https://apps.apple.com/us/app/chatwoot/id1498504620 (App Store Chatwoot)
  37. https://play.google.com/store/apps/details?id=com.chatwoot.app (Google Play Chatwoot)
  38. https://developers.facebook.com/docs/whatsapp/ (WhatsApp API Docs)
  39. https://developers.facebook.com/docs/instagram-api/ (Instagram API Docs)
  40. https://developers.facebook.com/docs/messenger-platform/ (Messenger API Docs)
  41. https://developer.twitter.com/en/docs (Twitter/X API Docs)
  42. https://developers.line.biz/en/ (Line API Docs)
  43. https://sendgrid.com/solutions/email-api/ (SendGrid Email API)
  44. https://stripe.com/docs/api (Stripe API - Payments context)
  45. https://github.com/rust-lang/rust (Rust Lang)
  46. https://actix.rs/ (Actix Web Rust)
  47. https://tokio.rs/ (Tokio Rust)
  48. https://www.postgresql.org/docs/current/ddl-rowsecurity.html (Postgres RLS)
  49. https://redis.io/docs/manual/patterns/distributed-locks/ (Redis Redlock)
  50. https://grpc.io/docs/languages/rust/ (gRPC Rust)
  51. https://flutter.dev/ (Flutter)
  52. https://playwright.dev/ (Playwright E2E)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
