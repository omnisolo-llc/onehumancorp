issue_title: "[Research] Omnichannel Customer Support & Unified Inbox Gap Analysis"
issue_description: |
  # Research Report: Omnichannel Customer Support & Unified Inbox

  ## Problem Statement
  OneHumanCorp (OHC) is an AI work assistant designed for non-technical owners/operators. A critical feature of such an assistant is the ability to unify customer communication (web chat, email, social media DMs, WhatsApp, SMS) into a single actionable feed.

  Historically, OHC may have relied on or evaluated third-party tools. However, in alignment with the OHC Engineering Standards, any reliance on external omnichannel inbox providers is strictly prohibited. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`.

  The current gap is that OHC lacks this native Rust omnichannel unified inbox that matches or exceeds industry-standard capabilities, integrated with our AI agents to draft replies, track orders, and schedule follow-ups autonomously.

  ## Research Report

  ### 1. Market Mapping & Competitor Discovery

  **Top 10 General Competitors (Unified Inbox / CX):**
  1. Zendesk (SaaS)
  2. Intercom (SaaS)
  3. Hubspot Service Hub (SaaS)
  4. Salesforce Service Cloud (SaaS)
  5. Front (SaaS)
  6. Gorgias (Ecommerce SaaS)
  7. Freshdesk (SaaS)
  8. Kustomer (SaaS)
  9. Crisp (SaaS)
  10. Opensource Alternatives (e.g. Helpdesk systems)

  **Top 10 AI-Native / Modern CX Platforms:**
  1. Intercom Fin (AI First CX)
  2. Siena AI (AI Customer Service for Commerce)
  3. Decagon (Generative AI CX)
  4. Ada (AI Chatbot)
  5. Ultimate (by Zendesk - AI Automation)
  6. DevRev (AI Native Support & CRM)
  7. Lang (AI CX automation)
  8. Forethought (Generative AI for Support)
  9. PolyAI (Voice AI Assistants)
  10. Kapa.ai (AI for technical support)

  ### 2. Deep-Dive Competitor Audit: Open Source CX Benchmark

  To adhere to the standard of a native Rust implementation, an audit of popular open-source omnichannel inbox architectures was conducted to benchmark feature sets.

  **Core Capabilities (Typical Industry Standard):**
  - **Omnichannel Inbox:** Consolidates conversations from Live Chat (Web widget), Email, Facebook Pages, Instagram DMs, Twitter/X, WhatsApp, SMS (Twilio/Bandwidth), API, and Line.
  - **Multi-Tenant Architecture:** Accounts, Users (Agents/Admins), Teams, Inboxes.
  - **Conversation Management:** Assignment rules, labels/tags, priority, SLA policies, private notes, mentions.
  - **Automation & Productivity:** Macros (pre-defined action sequences), canned responses, automation rules (event-based triggers).
  - **CRM Features:** Contacts, custom attributes, interaction history.
  - **Knowledge Base:** Help center articles, categories, portals.
  - **Webhooks & APIs:** Real-time webhooks, REST APIs.
  - **WebSockets:** Real-time updates for the agent dashboard.

  **Success Factors:** Open-source nature, straightforward UI, broad channel support out-of-the-box.

  **User Sentiment (Small Business / Open Source Communities):**
  - *Loves:* Unified view of social DMs (Instagram/WhatsApp is huge for SMBs), self-hosting capability, simple agent interface.
  - *Pain Points:* High resource consumption at scale in scripting languages (e.g., Ruby/Python), complex channel configuration (especially WhatsApp/FB API approvals), lacking deep native e-commerce/POS integrations (unlike Gorgias).

  ### 3. OHC Gap & Pain Point Identification

  **OHC Current State (Based on Repo Audit):**
  - OHC is built with Go/Bazel (backend) and Flutter (frontend). The mandate requires a native Rust implementation for the chat engine within the monorepo.
  - Existing integrations/chat directories (`src/server/integrations/chat`) exist, but a comprehensive, unified, multi-channel Rust engine mirroring standard omnichannel DB schemas and event models is a massive gap.

  **Gap Matrix (Industry Standard vs. OHC):**
  | Feature | Industry Standard | OHC Native (Rust) Gap |
  | :--- | :--- | :--- |
  | Unified Conversation Data Model | Yes (conversations, messages) | Missing native Rust data model and Postgres schema. |
  | Multi-Channel Adapters | Yes (WA, IG, Email, Web) | Missing Rust adapters for Meta Graph API, Twilio, IMAP/SMTP. |
  | Real-time WebSockets | Yes | Missing Rust WebSocket/gRPC streaming server for agent UI. |
  | Automation Rules & Macros | Yes | Missing Rust rule engine; but OHC has AI Agents which is superior. |
  | Multi-tenant Isolation | Yes (account_id) | Required: Postgres RLS with `tenant_id` per OHC standards. |

  **Unresolved Pain Points for OHC Personas:**
  - **Maya (Home Baker):** Receives cake orders via Instagram DMs and WhatsApp. Currently switching apps. Needs OHC to pull these into one feed where the AI drafts quotes.
  - **Carlos (Field Service):** Misses text messages while driving/working. Needs SMS routed to OHC where AI can auto-reply and offer a booking link.

  ### 4. Agentic Solution Design

  OHC will build an **AI-First Omnichannel Engine in Rust**:
  1.  **Ingestion (Rust Microservice):** Webhooks from WhatsApp, Instagram, Email, and Web Widget hit a high-throughput Rust API.
  2.  **Normalization:** The Rust service standardizes these into a single `Conversation` / `Message` format in PostgreSQL, tied to the `tenant_id`.
  3.  **Work Triage (AI Agent):** Upon new message insertion, the AI Job Queue (Postgres SKIP LOCKED) triggers the **Customer Assistant Agent**.
  4.  **Agentic Action:** The AI reads context, drafts a reply, identifies if it's a lead/booking/support issue, and flags the conversation in the owner's "Today's Priorities" feed.
  5.  **Owner Approval:** Maya sees the drafted reply ("Yes, I can make a vegan cake for Saturday. It will be $50. [Payment Link]"). She taps "Send". The Rust service pushes the message back out via the respective channel adapter.

  ## Design Doc

  **Architecture (Rust Backend):**
  - **Crates:**
    - `ohc_chat_core`: Data models, validation, tenant logic.
    - `ohc_chat_api`: REST/gRPC endpoints.
    - `ohc_chat_ws`: WebSocket server for real-time Flutter updates.
    - `ohc_chat_adapters`: Modules for WebWidget, Email, Meta (IG/WA), SMS.
  - **Entities (Postgres RLS with `tenant_id`):**
    - `inboxes` (id, tenant_id, name, channel_type)
    - `contacts` (id, tenant_id, name, phone, email, avatar_url)
    - `conversations` (id, tenant_id, inbox_id, contact_id, status, assignee_id)
    - `messages` (id, tenant_id, conversation_id, content, sender_type, sender_id, channel_message_id)
  - **AI Integration:** Integration with the existing Go AI Job Queue. The Rust service publishes events (e.g., via Redis or Postgres notify) that the Go-based agents pick up to draft replies.

  **UI/UX (Flutter 375px Mobile-First):**
  - **The Work Feed (Home):** Consolidated list of actionable conversations. Not a standard "Inbox", but a "Needs Action" list.
  - **Conversation View:**
    - Top bar: Customer name + Channel icon (e.g., Instagram logo).
    - Middle: Chat bubbles.
    - Bottom: AI Draft input field prominently displayed. "AI drafted a response based on inventory." -> [Send] or [Edit].
  - **Design System:** OHC Premium Token library. Translucent materials for the chat header.

  ## Implementation Prompt

  **User-Facing Outcome:**
  As an owner (e.g., Maya), when I open the OHC mobile app, I see a unified feed of messages from my Website, Instagram, and WhatsApp. I don't have to switch apps. More importantly, when I open a new Instagram DM asking about cake pricing, the OHC AI has already drafted a context-aware reply with a payment link based on my product catalog. I can review, edit, and send it with one tap.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC Flutter App (Mobile 375px).
  2. Owner navigates to "Work Feed / Inbox".
  3. Owner sees a new unread conversation marked with an Instagram icon.
  4. Owner taps the conversation.
  5. Owner sees the customer's message and the AI-generated draft reply.
  6. Owner taps "Send". The message is dispatched via the Rust backend to the customer's Instagram.

  **Acceptance Criteria:**
  - Create a new Rust crate structure for the omnichannel chat engine inside the monorepo.
  - Define PostgreSQL schema with Row-Level Security (`tenant_id`) for Contacts, Inboxes, Conversations, and Messages.
  - Implement a basic Web Widget or dummy Webhook adapter in Rust to ingest messages.
  - Implement the Flutter UI for the unified inbox list and conversation detail view following OHC design tokens.
  - Implement an E2E Playwright test covering the CUJ: navigating to the inbox, viewing a message, and sending a reply.
  - ZERO mock data in the UI; data must flow from the Rust backend to the Flutter frontend.

  ## Appendix: References & Sources Catalog
  1. https://www.zendesk.com/
  2. https://www.intercom.com/
  3. https://www.hubspot.com/products/service
  4. https://front.com/
  5. https://www.gorgias.com/
  6. https://freshdesk.com/
  7. https://www.kustomer.com/
  8. https://crisp.chat/
  9. https://siena.cx/
  10. https://decagon.ai/
  11. https://www.ada.cx/
  12. https://www.ultimate.ai/
  13. https://devrev.ai/
  14. https://lang.ai/
  15. https://forethought.ai/
  16. https://poly.ai/
  17. https://www.kapa.ai/
  18. https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  19. https://redis.io/
  20. https://developers.facebook.com/docs/instagram-api/
  21. https://developers.facebook.com/docs/whatsapp/
  22. https://www.twilio.com/
  23. https://www.bandwidth.com/
  24. https://flutter.dev/
  25. https://playwright.dev/
  26. https://bazel.build/
  27. https://www.rust-lang.org/
  28. https://tokio.rs/
  29. https://actix.rs/
  30. https://rocket.rs/
  31. https://grpc.io/
  32. https://opentelemetry.io/
  33. https://prometheus.io/
  34. https://grafana.com/
  35. https://stripe.com/docs/api
  36. https://developer.apple.com/design/human-interface-guidelines/
  37. https://ui.com/ (Ubiquiti Design)
  38. https://www.reddit.com/r/smallbusiness/
  39. https://www.reddit.com/r/ecommerce/
  40. https://www.trustpilot.com/
  41. https://apps.apple.com/
  42. https://play.google.com/
  43. https://github.com/obra/superpowers/
  44. https://docs.docker.com/compose/
  45. https://min.io/
  46. https://cloud.google.com/storage
  47. https://gemini.google.com/
  48. https://news.ycombinator.com/
  49. https://www.indiehackers.com/
  50. https://stackoverflow.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
