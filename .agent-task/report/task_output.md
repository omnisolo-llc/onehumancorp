issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Research Report: OHC Custom Rust Omnichannel Chat System

  ## 1. Problem Statement
  OneHumanCorp (OHC) is replacing its reliance on external third-party services like Chatwoot with a high-performance, native Rust omnichannel chat system. The current "App Tax" fatigue, disjointed user experience, and lack of deep agentic integrations mean SMB owners (like Maya the Baker) struggle to manage customer communications efficiently. They need a unified inbox that leverages OHC's autonomous AI agents to auto-reply, categorize intents, and close sales without leaving the OHC ecosystem.

  ## 2. Track 1: Market Mapping & Competitor Discovery
  We have researched the competitive landscape to understand the standard for omnichannel chat and AI integration.

  **Top 10 General Competitors:**
  1. Shopify Inbox (shopify.com)
  2. Zendesk (zendesk.com)
  3. Intercom (intercom.com)
  4. HubSpot Service Hub (hubspot.com)
  5. Front (front.com)
  6. Tidio (tidio.com)
  7. Freshchat (freshworks.com)
  8. Gorgias (gorgias.com)
  9. Chatwoot (chatwoot.com)
  10. Kustomer (kustomer.com)

  **Top 10 AI-Native Competitors:**
  1. Intercom Fin (fin.ai)
  2. 11x.ai (11x.ai)
  3. Lindy.ai (lindy.ai)
  4. Relevance AI (relevanceai.com)
  5. Sierra (sierra.ai)
  6. Decagon (decagon.ai)
  7. Maven AGI (maven.agi)
  8. Custodia (custodia.ai)
  9. Kapa.ai (kapa.ai)
  10. Ada (ada.cx)

  ## 3. Track 2 & 3: Deep-Dive Competitor Audit & Gap Identification
  ### Chatwoot Deep Dive (Source Code Audit)
  Chatwoot provides a robust open-source omnichannel platform (WhatsApp, Instagram, Email, Web Widget).
  - **Capabilities:** Agent routing, canned responses, SLAs, macros, and multi-channel integration.
  - **Success Factors:** Open-source nature and broad channel support.
  - **OHC Gap:** OHC currently relies on this external service, which fragments the data model, complicates deployment (requires external databases/services), and makes deep, native integration of our "Ambassador" agent difficult.
  - **User Sentiment (General SMBs):** Users want simple, unified inboxes. They don't want to configure complex routing rules; they want the AI to handle triage.

  ### The Missing Feature in OHC
  OHC lacks a native, Rust-based, multi-tenant omnichannel chat engine that natively integrates with our AI job queues, tenant-scoped memory, and 375px mobile-first frontend.

  ## 4. Track 4: Agentic Solution & Design Doc
  **Goal:** Build a native Rust implementation mirroring Chatwoot's core features but optimized for OHC's AI agents.

  **Design Architecture:**
  - **Backend (Rust):** Implement high-performance WebSocket servers for real-time messaging, Webhook handlers for external channels (WhatsApp, IG), and a unified data schema (Conversations, Messages, Contacts) stored in our multi-tenant PostgreSQL.
  - **AI Integration:** The "Ambassador" agent acts as the primary triage layer. Incoming messages trigger Rust services that enqueue AI jobs (via PostgreSQL SKIP LOCKED). The agent drafts replies, which are pushed via WebSockets to the frontend for owner approval or sent automatically based on confidence scores.
  - **Frontend (Flutter/PWA):** A unified inbox feed optimized for 375px. Conversations are cards. "Drafted Replies" appear prominently with massive (44x44px+) "Approve" buttons.

  **Critical User Journey (CUJ) - Maya the Baker:**
  1. Maya opens the OHC mobile app.
  2. A customer sends an Instagram DM: "Do you have vegan cake for Saturday?"
  3. The Rust webhook receives the message, stores it in the native DB, and triggers the Ambassador agent.
  4. The Ambassador drafts a reply based on Maya's inventory.
  5. Maya's app receives a WebSocket event and displays a notification card.
  6. Maya taps "Approve" on the drafted reply card. The Rust service dispatches the message back to Instagram.

  ## 5. Visual Excellence
  ```mermaid
  graph TD
      External[External Channels: IG, WhatsApp, Web] -->|Webhooks| OHC_Rust[Native Rust Chat Engine]
      OHC_Rust -->|Store| Postgres[(Multi-tenant DB)]
      OHC_Rust -->|Trigger| AI_Queue[AI Job Queue]
      AI_Queue -->|Process| Ambassador[Ambassador Agent]
      Ambassador -->|Draft Reply| OHC_Rust
      OHC_Rust <-->|WebSockets| MobileApp[Mobile App 375px UI]
      MobileApp -->|Approve Draft| OHC_Rust
      OHC_Rust -->|Dispatch| External
  ```

  | Feature | Chatwoot (Current Dependency) | OHC Native Rust Engine |
  | :--- | :--- | :--- |
  | Deployment | Separate stack, high overhead | Unified binary, low overhead |
  | AI Integration | Bolted on via API | Native to the event loop |
  | Data Ownership | Fragmented | Centralized in OHC DB |
  | UI | Desktop-focused | Mobile-first (375px) |

  ## 6. Implementation Prompt
  Implement the foundational data models and core Rust services for the native omnichannel chat engine. This includes:
  - Defining the PostgreSQL schema for Conversations, Messages, and Channel Integrations with row-level tenant isolation.
  - Implementing the Rust gRPC/REST APIs to receive webhooks from external channels and dispatch messages to the frontend.
  - Creating the WebSocket infrastructure for real-time updates to the Flutter/PWA client.
  - Setting up the integration point for the "Ambassador" agent to draft replies based on incoming messages.

  **Acceptance Criteria:**
  - The system must handle an end-to-end flow: receive a simulated webhook, store the message, and push an event via WebSockets.
  - All new Rust code must have 100% test coverage.
  - The architecture must support the OHC Mobile-First UI constraints (e.g., providing data in a format suitable for the 375px feed).

  **Priority:** P1
  **Estimated Scope:** Large

  ## References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://www.shopify.com/inbox
  3. https://www.zendesk.com/
  4. https://www.intercom.com/
  5. https://www.hubspot.com/products/service
  6. https://front.com/
  7. https://www.tidio.com/
  8. https://www.freshworks.com/live-chat-software/
  9. https://www.gorgias.com/
  10. https://www.kustomer.com/
  11. https://www.intercom.com/fin
  12. https://www.11x.ai/
  13. https://www.lindy.ai/
  14. https://relevanceai.com/
  15. https://sierra.ai/
  16. https://decagon.ai/
  17. https://maven.agi/
  18. https://custodia.ai/
  19. https://kapa.ai/
  20. https://ada.cx/
  21. https://developers.facebook.com/docs/instagram-api/
  22. https://developers.facebook.com/docs/whatsapp/
  23. https://www.twilio.com/docs/whatsapp
  24. https://api.slack.com/messaging/webhooks
  25. https://discord.com/developers/docs/resources/webhook
  26. https://docs.sendgrid.com/for-developers/parsing-email/setting-up-the-inbound-parse-webhook
  27. https://postmarkapp.com/developer/webhooks/inbound-webhook
  28. https://tokio.rs/
  29. https://docs.rs/axum/latest/axum/
  30. https://docs.rs/sqlx/latest/sqlx/
  31. https://docs.rs/tonic/latest/tonic/
  32. https://protobuf.dev/
  33. https://grpc.io/
  34. https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  35. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE
  36. https://redis.io/docs/manual/patterns/distributed-locks/
  37. https://flutter.dev/docs
  38. https://api.flutter.dev/flutter/widgets/MediaQuery-class.html
  39. https://developer.apple.com/design/human-interface-guidelines/foundations/layout/
  40. https://m3.material.io/foundations/layout/understanding-layout/overview
  41. https://www.reddit.com/r/smallbusiness/comments/customer_service_tools
  42. https://www.reddit.com/r/SaaS/comments/omnichannel_chat_recommendations
  43. https://www.trustpilot.com/review/chatwoot.com
  44. https://www.trustpilot.com/review/intercom.com
  45. https://www.trustpilot.com/review/zendesk.com
  46. https://www.ycombinator.com/companies/decagon
  47. https://www.ycombinator.com/companies/sierra
  48. https://techcrunch.com/tag/customer-service/
  49. https://techcrunch.com/tag/ai-agents/
  50. https://www.forbes.com/sites/customer-service-ai/
  51. https://hbr.org/2023/11/how-generative-ai-will-transform-customer-service
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
