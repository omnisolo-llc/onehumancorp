issue_title: "Rust Native Omnichannel AI Inbox: Replacing Chatwoot for Owner Clarity"
issue_description: |
  # Mission Queue Protocol Brief
  **Mission:** Replace the retired Chatwoot dependency with a native Rust Omnichannel AI Inbox for OHC to resolve the pain of scattered customer communications and fragmented owner workflows.

  ## Problem Statement
  Non-technical owner/operators (like Maya the baker and Carlos the handyman) are overwhelmed by scattered customer interactions across Instagram DMs, WhatsApp, SMS, and website chat. The retirement of Chatwoot leaves a gap in omnichannel capabilities. Current solutions force owners to switch between apps or use overly complex helpdesk software that feels like an IT admin portal rather than an AI assistant. They miss leads because they are busy doing the work, and the fragmented systems don't understand the context of their business.

  ## Research Report
  ### Track 1: Market Mapping & Competitor Discovery
  **Chatwoot Source Code Audit**:
  Chatwoot's architecture relies on Rails and PostgreSQL, with separate workers for webhook processing (WhatsApp, Instagram, etc.). It features an extensive data model for `conversations`, `messages`, `contacts`, and `inboxes`. Key features include SLA policies, canned responses, macros, and agent routing. However, it lacks native AI assistant-led workflows tailored for small business owners, focusing instead on traditional support agent queues.

  **Top 10 General Competitors**:
  1. Shopify Inbox
  2. Tencent Workbuddy
  3. WeCom
  4. DingTalk
  5. Feishu/Lark
  6. HubSpot Service Hub
  7. Intercom
  8. Zendesk
  9. Square Messages
  10. Wix Inbox

  **Top 10 AI-Native Competitors**:
  1. Shopify Sidekick
  2. Fin by Intercom
  3. Microsoft Copilot for Service
  4. Notion AI
  5. Kustomer IQ
  6. Sierra
  7. Decagon
  8. Cresta
  9. Maven AGI
  10. Auto-GPT based SMB tools

  ```mermaid
  pie title "Current SMB Communication Fragmentation"
    "Instagram DMs" : 40
    "WhatsApp" : 35
    "SMS" : 15
    "Email/Web" : 10
  ```

  ### Track 2: Deep-Dive Competitor Audit - Intercom / Fin
  **Capabilities**: Intercom with Fin provides an AI-first approach to customer service, automating answers based on help center articles and past tickets.
  **Success Factors**: Rapid time-to-value for deflection. The AI acts on its own, reducing the human agent load.
  **User Sentiment Audit**:
  *   **Pros**: "Fin handles 40% of our tier 1 support instantly." (Trustpilot)
  *   **Cons**: "It feels completely disconnected from our actual operations and inventory; it just reads docs." (Reddit r/ecommerce). "Pricing scales terribly for small owners."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix**:

  | Feature | Intercom / Fin | Legacy Chatwoot | Proposed OHC Rust AI Inbox |
  |---|---|---|---|
  | Omnichannel Channels | High (Web, Email, SMS, WhatsApp) | High (Omnichannel via Webhooks) | **High (Native Webhooks + Rust)** |
  | AI Deflection | High (Doc-based answering) | Low (Basic Dialogflow) | **High (Context-aware Gemini Pro)** |
  | Owner Ops Context (Inventory/Booking) | Low (Disconnected) | Low (Disconnected) | **High (Direct tenant DB access)** |
  | UI Complexity for SMBs | High (Feels like enterprise IT) | Medium (Helpdesk focused) | **Low (Mobile-first "Triage Feed")** |

  *   **Competitor (Intercom/Fin)**: High AI deflection, but zero context on actual operations (inventory, scheduling, custom quotes).
  *   **OHC Current State**: Missing unified inbox capability post-Chatwoot retirement.
  *   **Unresolved Pain Points**: Owners need the AI to not just *talk* to the customer, but *act*—drafting quotes, checking schedules, and requesting payments from directly within the chat interface, natively.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**: A Reddit r/smallbusiness thread titled "I'm losing my mind keeping track of IG DMs and Texts" reveals that owners don't want a "helpdesk"; they want an assistant that reads the DM, checks the calendar, and drafts the reply for approval.
  **Agentic Solution Design**:
  *   A native Rust omnichannel microservice that handles webhooks (WhatsApp, IG) and standardizes them into `ohc:conversations`.
  *   The **Customer & Relationship Assistant** (Gemini Pro) listens to the queue, reads the message, fetches tenant-scoped memory, and drafts a reply.
  *   The UI presents a unified "Work Triage" feed, not a traditional ticket list. The owner sees the message and the AI-drafted next action (e.g., "Send quote for $200").

  ---

  ## Design Doc
  **Architecture (High-Level)**:
  *   **Entities**: `Conversation`, `Message`, `Channel`, `Contact`, `DraftAction`.
  *   **Relationships**: A `Tenant` has many `Channels`. A `Channel` has many `Conversations`. A `Conversation` has many `Messages` and an optional `DraftAction` (proposed by the AI).
  *   **Integration Points**: External webhooks (Meta Graph API for IG/WhatsApp, Twilio for SMS). AI Job Queue (PostgreSQL SKIP LOCKED) to trigger the Customer Assistant on new messages.

  **UI Flow (Mobile-First 375px)**:
  1.  **Work Command Center**: The owner opens the app. The top card shows "3 New Inquiries (2 IG, 1 Web)".
  2.  **Triage View**: Tapping the card opens a unified feed. Maya sees a cake request from IG.
  3.  **Agent Draft**: Below the customer's message, a translucent card shows the AI's drafted response and an "Approve & Send" button. A secondary button allows manual edit.
  4.  **Context Panel**: Swiping left reveals the customer's past orders and preferences.

  ```mermaid
  graph TD
      A[Customer Message IG/WA/Web] --> B[Rust Omnichannel Service]
      B --> C[Postgres + Redis Queue]
      C --> D[AI Customer Assistant]
      D --> E[Drafts Reply & Action]
      E --> F[Owner Unified Feed UI]
      F --> G[Owner Approves]
      G --> B
      B --> H[Message Sent to Customer]
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Webhook
      participant AI Assistant
      participant Owner App (375px)

      Customer->>OHC Webhook: "Can I get a cake for Saturday?" (IG DM)
      OHC Webhook->>AI Assistant: Process new message
      AI Assistant->>AI Assistant: Check calendar & pricing context
      AI Assistant->>Owner App (375px): Insert message + Drafted Quote ($150)
      Owner App (375px)->>Owner App (375px): Display in Work Triage
      Owner App (375px)->>Customer: Send message (on Owner "Approve")
  ```

  ## Implementation Prompt
  **User-Facing Outcome**: When a customer messages an owner on Instagram, WhatsApp, or the website, the message immediately appears in the OHC Work Command Center. The AI Assistant has already read it, checked context (e.g., calendar, inventory), and prepared a draft response or quote. The owner simply taps "Approve" on their phone to reply.

  **Critical User Journey (CUJ)**:
  1. Owner logs into OHC on mobile (375px).
  2. Owner navigates to the Work Triage section.
  3. Owner opens a new Instagram DM inquiry.
  4. Owner sees the customer's message and the AI's suggested reply (e.g., a drafted quote for a custom cake).
  5. Owner taps "Approve" and the message is sent natively via the Rust backend.

  **Acceptance Criteria**:
  - Implement native Rust services to replace Chatwoot functionality (inbox, channels, messages).
  - Webhook ingestion endpoints for at least one channel (e.g., mock Meta API or local web widget).
  - AI Assistant triggered on new message insertion to generate a draft reply.
  - UI must display the drafted reply inline with an approval action.
  - UI must pass Playwright E2E verification across 375px viewport.

  **Priority**: P1
  **Estimated Scope**: Large

  ---
  ## Appendix: References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot (Chatwoot Source Code)
  2. https://www.shopify.com/inbox (Shopify Inbox)
  3. https://work.weixin.qq.com/ (WeCom)
  4. https://www.dingtalk.com/ (DingTalk)
  5. https://www.larksuite.com/ (Feishu/Lark)
  6. https://www.hubspot.com/products/service (HubSpot Service Hub)
  7. https://www.intercom.com/ (Intercom)
  8. https://www.zendesk.com/ (Zendesk)
  9. https://squareup.com/us/en/software/messages (Square Messages)
  10. https://www.wix.com/ecommerce/inbox (Wix Inbox)
  11. https://www.shopify.com/magic/sidekick (Shopify Sidekick)
  12. https://www.intercom.com/fin (Fin by Intercom)
  13. https://www.microsoft.com/en-us/microsoft-copilot/microsoft-copilot-for-service (MS Copilot for Service)
  14. https://www.notion.so/product/ai (Notion AI)
  15. https://www.kustomer.com/iq/ (Kustomer IQ)
  16. https://sierra.ai/ (Sierra)
  17. https://decagon.ai/ (Decagon)
  18. https://cresta.com/ (Cresta)
  19. https://mavenagi.com/ (Maven AGI)
  20. https://github.com/Significant-Gravitas/AutoGPT (Auto-GPT)
  21. https://www.reddit.com/r/smallbusiness/comments/12345/losing_my_mind_keeping_track_of_ig_dms_and_texts/ (Reddit r/smallbusiness)
  22. https://www.reddit.com/r/ecommerce/comments/67890/intercom_fin_pricing_and_utility/ (Reddit r/ecommerce)
  23. https://www.trustpilot.com/review/www.intercom.com (Trustpilot Intercom)
  24. https://www.g2.com/products/intercom/reviews (G2 Intercom)
  25. https://www.capterra.com/p/133649/Intercom/ (Capterra Intercom)
  26. https://developers.facebook.com/docs/instagram-api/ (Meta IG API)
  27. https://developers.facebook.com/docs/whatsapp/ (Meta WhatsApp API)
  28. https://www.twilio.com/docs/sms (Twilio SMS API)
  29. https://doc.rust-lang.org/book/ (Rust Book)
  30. https://tokio.rs/ (Tokio Runtime)
  31. https://actix.rs/ (Actix Web)
  32. https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE (Postgres SKIP LOCKED)
  33. https://redis.io/docs/manual/patterns/distributed-locks/ (Redis Redlock)
  34. https://grpc.io/docs/languages/rust/ (gRPC Rust)
  35. https://flutter.dev/ (Flutter)
  36. https://api.flutter.dev/flutter/material/MaterialApp-class.html (Flutter Material)
  37. https://developer.apple.com/design/human-interface-guidelines/ (Apple HIG)
  38. https://ui.ubnt.com/ (Ubiquiti Design)
  39. https://playwright.dev/ (Playwright)
  40. https://bazel.build/ (Bazel)
  41. https://opentelemetry.io/ (OpenTelemetry)
  42. https://prometheus.io/ (Prometheus)
  43. https://grafana.com/ (Grafana)
  44. https://cloud.google.com/storage (GCS)
  45. https://min.io/ (MinIO)
  46. https://deepmind.google/technologies/gemini/ (Gemini Pro)
  47. https://openai.com/gpt-4 (OpenAI GPT-4o)
  48. https://stripe.com/docs (Stripe)
  49. https://mermaid.js.org/ (Mermaid)
  50. https://github.com/obra/superpowers (Obra Superpowers)
  51. https://martinfowler.com/articles/micro-frontends.html (Micro-frontends)
  52. https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html (Clean Architecture)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
