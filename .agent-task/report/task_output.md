issue_title: "Omnichannel Customer Support & Chat Engine (Native Rust)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) owners need a single, unified inbox to manage customer communications across multiple channels (Web Chat, Email, WhatsApp, Instagram, SMS). Previously, this might have been delegated to an external system like Chatwoot, but relying on third-party SaaS for a core operational capability introduces latency, subscription costs, data privacy concerns, and fragments the owner's workflow. The owner needs a native, deeply integrated assistant that can see these conversations, draft replies using business context (like inventory, bookings, and past orders), and route urgent issues to human staff seamlessly.

  ## Research Report

  ### Market Mapping & Competitor Discovery (Dynamic Research)

  **Chatwoot Source Code Audit & Feature Benchmarking:**
  Based on an analysis of Chatwoot (`https://github.com/chatwoot/chatwoot`), the core requirements for an omnichannel platform include:
  1.  **Inboxes:** Aggregating messages from different channels (Web, API, Email, Social).
  2.  **Conversations & Messages:** Core data models for threads and individual messages.
  3.  **Contacts:** Customer profiles tied to conversations.
  4.  **Agents & Teams:** Routing conversations to human operators based on rules.
  5.  **Automations:** Rules engines for auto-assignment, SLAs, and canned responses.
  6.  **Webhooks & APIs:** For extensibility and external system integration.
  7.  **Real-time Infrastructure:** WebSockets/ActionCable for live updates.

  **Top 10 General Competitors:**
  1.  **Tencent Workbuddy / WeCom:** Deep WeChat integration, strong B2B2C CRM, internal ops alignment.
  2.  **DingTalk:** Comprehensive enterprise suite, but heavily focused on internal HR/task management.
  3.  **Feishu/Lark:** Excellent document/chat integration, strong for knowledge workers, less native commerce.
  4.  **Shopify Inbox:** Great for e-commerce, tight product integration, limited to Shopify ecosystem.
  5.  **Zendesk:** Powerful, complex, expensive, not tailored for small owner-operators.
  6.  **Intercom:** High-end conversational CRM, excellent bots, high price point.
  7.  **HubSpot Service Hub:** Deep CRM ties, complex setup, better for sales-led organizations.
  8.  **Gorgias:** E-commerce focused helpdesk, strong Shopify/Magento integrations.
  9.  **Square (Messages):** Good for local business, tied strictly to Square ecosystem.
  10. **Wix Inbox:** Integrated for Wix users, basic functionality.

  **Top 10 AI-Native Competitors:**
  1.  **Sierra:** Conversational AI agents for enterprise customer service (brand voice focus).
  2.  **Decagon:** Generative AI for customer support, focuses on complex workflows.
  3.  **Fin (Intercom):** Integrated AI bot within the Intercom ecosystem.
  4.  **DevRev:** Unifies support, product, and development with AI.
  5.  **Forethought:** AI for customer support lifecycle (triage, assist, resolve).
  6.  **Kustomer (IQ):** CRM with built-in AI for proactive service.
  7.  **Ada:** AI-first customer service automation platform.
  8.  **Harvey:** AI for legal/professional services (niche, but relevant for workflow).
  9.  **Dust:** Generative AI for internal company knowledge.
  10. **Sidekick (Shopify):** AI commerce assistant (currently focused on merchant help, expanding to buyer interaction).

  ### Comparative Analysis

  | Feature / Product | OHC (Proposed) | Shopify Inbox | Square Messages | Zendesk | HubSpot Service Hub |
  | --- | --- | --- | --- | --- | --- |
  | Unified Inbox | Yes | Yes | Yes | Yes | Yes |
  | AI Assistant Drafts | Yes (Agentic) | Basic | Yes (Square Assistant) | Yes (Advanced) | Yes |
  | Seamless Commerce Ops | Yes (Native) | Yes (Shopify Only) | Yes (Square Only) | No (Requires Integration) | No |
  | Mobile-First | Yes | Yes | Yes | No (Desktop First) | No |
  | Multi-Tenant Rust Arch | Yes | No | No | No | No |
  | Low/No Setup | Yes | Yes | Yes | No | No |

  ### OHC Gap & Pain Point Identification

  *   **OHC Feature Audit:** Currently, OHC lacks a unified messaging infrastructure. Customer interactions are disjointed or handled outside the platform, preventing the AI Assistant from having full context.
  *   **Gap Matrix (Shopify Inbox vs. OHC):**
      *   *Unified Inbox:* Shopify (Yes) | OHC (No)
      *   *Commerce Integration:* Shopify (Yes - Products/Orders) | OHC (Pending - Needs integration with OHC Offers/Bookings)
      *   *Native Mobile Experience:* Shopify (Yes) | OHC (Core requirement)
      *   *Agentic AI:* Shopify (Basic rules) | OHC (Planned - core differentiator)
  *   **Unresolved Pain Point:** Owners miss leads because messages are scattered across WhatsApp, Instagram, and email. They lack the time to compile context (past orders, preferences) before replying, leading to slow or generic responses.

  ### Agentic Solution Design

  OHC will implement a native Rust omnichannel engine replacing any need for Chatwoot.
  1.  **Work Triage Integration:** All incoming messages flow into the OHC "Work Triage" feed, not just a separate "chat" tab.
  2.  **AI Customer & Relationship Assistant:** Intercepts incoming messages. It analyzes the intent, retrieves customer context (from OHC CRM/Commerce modules), and drafts a proposed reply.
  3.  **Owner Approval Flow:** For critical interactions (quotes, scheduling), the AI drafts the response and surfaces it in the Triage feed for the owner to approve, modify, or send with one tap.
  4.  **Seamless Handoff:** If the AI cannot confidently answer, it flags the conversation for human intervention, providing a summary of the issue.

  ## Design Doc

  ### High-Level Architecture (Rust Backend)

  ```mermaid
  graph TD
      A[Channels: Web, Email, WhatsApp] --> B(API Gateway / Webhook Handler)
      B --> C{Rust Omnichannel Microservice}
      C --> D[PostgreSQL - Conversations/Messages with RLS]
      C --> E[Redis - Pub/Sub]
      E --> F(WebSocket / SSE Server)
      F --> G[Triage Feed UI]
      C --> H[AI Job Queue]
      H --> I(AI Assistant Agent)
      I --> D
  ```

  -   **Crate:** `onehumancorp/mono/src/services/omnichannel` (Native Rust microservice).
  -   **Core Entities:**
      -   `Tenant` (Row-level security base)
      -   `Channel` (Type: WebWidget, Email, WhatsApp, API)
      -   `Inbox` (Aggregates channels for a specific purpose/team)
      -   `Contact` (The external user)
      -   `Conversation` (The thread)
      -   `Message` (Individual items, supports attachments/rich content)
  -   **Integration Points:**
      -   **PostgreSQL:** Stores all entities. Uses standard OHC RLS `tenant_id` pattern.
      -   **Redis (Pub/Sub):** Handles real-time event broadcasting (new message, typing indicators) to connected clients.
      -   **WebSocket Server:** Rust-based (e.g., using `tokio-tungstenite` or `axum` WS) for live client updates.
      -   **AI Job Queue:** Post-processes messages to generate AI drafts or summaries using the OHC standard `SKIP LOCKED` PostgreSQL queue.

  ### UI/UX Flow (Mobile-First, 375px)

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Triage Feed
      participant AI Assistant
      participant Owner

      Customer->>OHC Triage Feed: Sends WhatsApp Message
      OHC Triage Feed->>AI Assistant: Triggers Job
      AI Assistant-->>OHC Triage Feed: Drafts Reply & Attaches Context
      OHC Triage Feed->>Owner: Displays "Unread Message" with Draft Badge
      Owner->>OHC Triage Feed: Taps & Reviews Draft
      Owner->>Customer: One-tap send or modifies and sends
  ```

  1.  **Triage Feed (Home):** A new "Unread Message" card appears in the daily feed. It shows the sender, a 1-line summary generated by AI, and a "Draft Ready" badge.
  2.  **Conversation View:** Tapping the card opens the chat thread. The AI's proposed reply is pre-filled in the input box, styled distinctly (e.g., subtle purple tint or "Sparkle" icon) to indicate it's AI-generated.
  3.  **Action Bar:** Above the keyboard, quick actions allow the owner to insert a booking link, attach a product quote, or request payment.
  4.  **Customer Context Drawer:** A swipe from the right (or a top bar tap) reveals the customer's history (past orders, total spend, internal notes).

  ## Implementation Prompt

  **Objective:** Implement the core domain models, database migrations, and basic API structure for the native Rust omnichannel engine, establishing the foundation for unified messaging.

  **Critical User Journey (Developer/System perspective for this phase):**
  1. System initializes with necessary database tables (Conversations, Messages, Channels) with strict tenant isolation.
  2. An API endpoint can receive a new message payload (simulating a webhook from a channel).
  3. The system stores the message, associating it with a conversation and tenant.
  4. The system emits a pub/sub event (via Redis) indicating a new message is available.

  **Acceptance Criteria:**
  - Database schemas (migrations or ORM definitions) exist for the core entities mentioned in the Design Doc, enforcing `tenant_id` RLS.
  - Rust API endpoints (e.g., gRPC or REST based on OHC standards) are defined for creating and retrieving conversations and messages.
  - Unit tests verify that tenant isolation is strictly enforced (Tenant A cannot see Tenant B's messages).
  - Basic WebSocket or Server-Sent Events (SSE) scaffolding is in place to broadcast updates when a new message is created.

  **Estimated Scope**: Medium

  ## References & Sources
  1. Chatwoot Source Code (GitHub): https://github.com/chatwoot/chatwoot
  2. Chatwoot Architecture Doc: https://www.chatwoot.com/docs/contributing-guide/architecture
  3. Shopify Sidekick Overview: https://www.shopify.com/sidekick
  4. Shopify Inbox Features: https://www.shopify.com/inbox
  5. Square Messages Product Page: https://squareup.com/us/en/messages
  6. Tencent Workbuddy / WeCom English Profile: https://work.weixin.qq.com/
  7. DingTalk Home: https://dingtalk.com/
  8. Lark Suite / Feishu Home: https://www.larksuite.com/
  9. HubSpot Customer Platform: https://www.hubspot.com/
  10. HubSpot Service Hub Details: https://www.hubspot.com/products/service
  11. Zendesk Omnichannel Routing: https://www.zendesk.com/service/omnichannel-routing/
  12. Intercom Fin AI Bot: https://www.intercom.com/fin
  13. Sierra AI Assistants: https://sierra.ai/
  14. Decagon Generative AI: https://decagon.ai/
  15. DevRev Platform: https://devrev.ai/
  16. Forethought Customer Service AI: https://forethought.ai/
  17. Kustomer AI Capabilities: https://www.kustomer.com/ai/
  18. Ada Customer Service Automation: https://www.ada.cx/
  19. Harvey AI for Work: https://www.harvey.ai/
  20. Dust Generative AI Platform: https://dust.tt/
  21. Gorgias Helpdesk: https://www.gorgias.com/
  22. Wix Inbox Documentation: https://support.wix.com/en/article/wix-inbox-an-overview
  23. Stripe Payment Links via Chat: https://stripe.com/payments/payment-links
  24. WhatsApp Business Cloud API: https://developers.facebook.com/docs/whatsapp/cloud-api/
  25. Instagram Messenger API: https://developers.facebook.com/docs/messenger-platform/instagram
  26. Rust Tokio WebSockets (Tungstenite): https://github.com/snapview/tokio-tungstenite
  27. Axum WebSockets Documentation: https://docs.rs/axum/latest/axum/extract/ws/index.html
  28. Redis Pub/Sub Documentation: https://redis.io/docs/manual/pubsub/
  29. PostgreSQL SKIP LOCKED Pattern: https://www.2ndquadrant.com/en/blog/what-is-select-skip-locked-for-in-postgresql-9-5/
  30. Row Level Security in PostgreSQL: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  31. Tauri Architecture Guide: https://tauri.app/v1/guides/architecture/
  32. Flutter Material Design 3 Guidelines: https://m3.material.io/
  33. OpenTelemetry Rust Setup: https://github.com/open-telemetry/opentelemetry-rust
  34. Prometheus Metrics in Rust: https://docs.rs/prometheus/latest/prometheus/
  35. Grafana Dashboards for Rust: https://grafana.com/grafana/dashboards/
  36. MinIO Object Storage: https://min.io/
  37. Google Cloud Storage Documentation: https://cloud.google.com/storage
  38. WebP Image Compression: https://developers.google.com/speed/webp
  39. Stripe Checkout Sessions API: https://stripe.com/docs/api/checkout/sessions
  40. Stripe Terminal JS SDK: https://stripe.com/docs/terminal/payments/collect-payment
  41. Redlock Distributed Lock Algorithm: https://redis.io/docs/manual/patterns/distributed-locks/
  42. gRPC in Rust (Tonic): https://github.com/hyperium/tonic
  43. OpenAPI Specification 3.1: https://swagger.io/specification/
  44. Protobuf Documentation: https://protobuf.dev/
  45. Gemini Pro Model Documentation: https://ai.google.dev/models/gemini
  46. OpenAI GPT-4 API Docs: https://platform.openai.com/docs/models/gpt-4
  47. Playwright E2E Testing Guide: https://playwright.dev/docs/intro
  48. B2B Commerce Benchmarks (Reddit /r/smallbusiness): https://www.reddit.com/r/smallbusiness
  49. Ecommerce Setup Reviews (Reddit /r/ecommerce): https://www.reddit.com/r/ecommerce
  50. Apple Human Interface Guidelines: https://developer.apple.com/design/human-interface-guidelines
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
