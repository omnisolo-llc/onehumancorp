issue_title: "Omnichannel AI Inbox: Native Rust Chatwoot Replacement & Agentic Work Triage"
issue_description: |
  # Mission Brief
  The OneHumanCorp (OHC) mission is to build a Tencent Workbuddy-like AI work assistant for owners and operators. The current most pressing gap is the ability to unify customer communications, internal tasks, and agent actions into a single seamless interface. Chatwoot, an external dependency, is being retired, and OHC must replicate its omnichannel capabilities natively in Rust while introducing AI-agentic workflows directly into the inbox experience.

  ## 1. Problem Statement
  Owners like Maya (baker) and Carlos (handyman) are overwhelmed by managing demand across Instagram DMs, WhatsApp, web forms, and emails. They lack a unified interface where they can not only read these messages but also have an AI assistant draft replies, track deposits, create bookings, and surface what needs immediate attention. Traditional CRMs are too complex, and fragmented apps cause missed leads. OHC currently lacks a native, multi-tenant omnichannel communications backend (to replace Chatwoot) and a front-end UI that blends conversation with business operation tasks natively.

  ## 2. Research Report
  ### Track 1: Market Mapping & Competitor Discovery

  **Chatwoot Source Code Audit & Feature Benchmarking**
  Chatwoot's source code reveals a robust Ruby on Rails architecture focused on omnichannel aggregation. Key capabilities we must replicate natively in Rust include:
  - **Channel Adapters**: Web Widget, API channel, WhatsApp, Instagram, Email, SMS.
  - **Core Entities**: Accounts (Tenants), Inboxes, Contacts, Conversations, Messages, Agents, Teams.
  - **Features**: Agent routing (Round Robin/Manual), Canned Responses, Macros, SLA Policies, CSAT surveys.
  - **Real-time**: ActionCable-based WebSockets for typing indicators, presence, and instant message delivery.
  - **Webhooks**: Robust outgoing webhook system for integrations.

  **Top 10 General Competitors**
  1. **Tencent Workbuddy / WeCom**: Deep ecosystem integration, mini-programs, internal/external comms unified.
  2. **DingTalk**: Heavy operations focus, approval workflows, clock-ins.
  3. **Feishu / Lark**: Document-centric, seamless video/chat/docs integration.
  4. **Shopify (Inbox)**: Commerce-focused chat, quick product links, discounts.
  5. **Square (Messages)**: Appointment and payment integrated SMS/chat.
  6. **HubSpot**: Comprehensive but overly complex for micro-operators.
  7. **Intercom**: Premium B2B support, excellent UI, expensive.
  8. **Zendesk**: Enterprise standard, ticketing model (less conversational).
  9. **Freshworks**: Accessible Zendesk alternative.
  10. **Front App**: Shared team inboxes, email-first approach.

  **Top 10 AI-Native Competitors**
  1. **Shopify Sidekick**: AI commerce copilot for store owners.
  2. **Intercom Fin**: High-resolution AI bot for customer resolution.
  3. **Stripe AI**: Billing and data analysis copilot.
  4. **Notion AI**: Workspace knowledge synthesis.
  5. **Zendesk AI**: Intent detection and macro suggestions.
  6. **Kustomer IQ**: AI-driven CRM routing and sentiment analysis.
  7. **Gorgias**: E-commerce focused AI automation.
  8. **PolyAI**: Voice and conversational AI for customer service.
  9. **Forethought.ai**: Generative AI for support ticket resolution.
  10. **Asana AI**: Task summarization and intelligent routing.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Inbox & Sidekick
  **Capabilities**: Shopify Inbox centralizes chat across online store, Instagram, and Facebook Messenger. Sidekick acts as an AI assistant helping the merchant run the store (e.g., "Set up a discount," "Why did sales drop?").
  **Success Factors**: Zero-configuration setup for merchants, seamless integration with store product catalogs, ability to send clickable product cards and discount codes directly in chat.
  **User Sentiment Audit**:
  - *Positive (Reddit r/ecommerce)*: "I love that I can just tap to send a product link without leaving the app."
  - *Negative (Trustpilot)*: "The AI is too basic and doesn't understand custom order workflows. It can't handle split payments or deposits."
  - *Negative (App Store)*: "Notifications fail sometimes, and I miss chats. I wish it would proactively tell me which VIPs need replies."

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit**: OHC has foundational tenant architecture but lacks a native Rust-based omnichannel ingestion pipeline and a unified UI for triaging mixed entity types (messages + tasks + alerts).

  **Gap Matrix (OHC vs Shopify vs Chatwoot)**
  | Feature | Shopify Inbox | Chatwoot | OHC (Current) | OHC (Target) |
  |---------|---------------|----------|---------------|--------------|
  | Omnichannel Ingestion | Yes (Meta focused) | Yes (Broad) | No | **Yes (Rust Native)** |
  | AI Drafts | Basic | No | No | **Yes (Gemini Pro)** |
  | Actionable Tasks in Chat | No | No | No | **Yes (Bookings/Payments)**|
  | Native E2E E-commerce | Yes | No | Partial | **Yes** |
  | SLA / Urgency Alerts | No | Yes | No | **Yes (Agent-driven)** |

  **Unresolved Pain Points for Personas**:
  - **Maya (Baker)**: Needs to send deposit payment links seamlessly within Instagram DMs via the OHC app.
  - **Carlos (Handyman)**: Needs the system to automatically read an SMS request, draft an estimate, and put it in his triage feed.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  Real-world evidence from r/smallbusiness shows operators constantly switching context between WhatsApp (for chat), Square (for payments), and a notebook (for tasks).

  **Agentic Solution Design**: We will build the "Work Triage Feed". Instead of just an "Inbox", it's a prioritization queue. When an Instagram DM arrives:
  1. The Rust ingestion engine saves it.
  2. The AI Customer Assistant agent reads the DM, identifies intent (e.g., "wants a cake for Saturday").
  3. The agent checks Operations calendar for Saturday availability.
  4. The agent drafts a reply and generates a Quote entity.
  5. The owner opens OHC, sees "1 Urgent Action: Approve Quote for Sarah", taps "Approve & Send", and the Rust engine dispatches it via Instagram API.

  ### Mermaid Charts

  ```mermaid
  graph TD
    A[Customer Meta/WhatsApp/Web] -->|Webhook| B(Rust API Gateway)
    B --> C{Rust Omnichannel Router}
    C --> D[Conversation DB - Postgres RLS]
    D --> E[AI Job Queue - SKIP LOCKED]
    E --> F[Gemini Pro Triage Agent]
    F -->|Drafts Reply, Creates Task| D
    D --> G[Flutter PWA UI - Triage Feed]
    G -->|Owner Approves| B
    B -->|Send Message| A
  ```

  ## 3. Design Doc
  **Architecture Overview (Backend - Go/Rust)**:
  - Implement a multi-tenant omnichannel engine in Rust, replacing Chatwoot.
  - **Entities**:
    - `Channel`: configuration for WhatsApp, IG, Web.
    - `Conversation`: maps to a customer thread.
    - `Message`: text, attachments, or actionable cards.
    - `TriageItem`: A wrapper around Conversations, Tasks, and Alerts for the owner feed.
  - **Webhooks**: High-performance HTTP ingest endpoints validating signatures (e.g., Meta Graph API).
  - **AI Integration**: On new `Message`, push to PostgreSQL AI Job Queue. Worker retrieves context (tenant memory), queries Gemini, and creates a pending `Message` (draft status) and updates `TriageItem` priority.

  **Mobile UX Flow (375px first)**:
  1. **Home/Triage Screen**: Clean feed. Top item: "Sarah asked about wedding cake. Draft ready."
  2. **Thread View**: Translucent iOS-style header. Chat bubbles. Below the last message, a distinct "Agent Suggestion" card with the drafted text and a prominent "Send" and "Edit" button.
  3. **Action Menu**: A "+" button near the keyboard allowing the owner to insert a Payment Link, Booking Link, or Policy snippet into the chat natively.

  ## 4. Implementation Prompt
  **User-Facing Outcome**: The owner opens the app and sees a unified "Triage" feed. They tap a customer inquiry from Instagram. The chat opens, and an AI-drafted response is already waiting for approval, complete with a payment link if the customer asked to book. The owner taps "Send", and the message is delivered back to Instagram.

  **Critical User Journey (CUJ)**:
  1. Owner connects Instagram channel (mocked in tests).
  2. System receives simulated incoming webhook from IG.
  3. AI Agent processes the message, drafts reply.
  4. Owner navigates to Triage Feed, sees the new item.
  5. Owner taps item, sees draft, taps "Send".
  6. System records message as sent and clears the triage item.

  **Acceptance Criteria**:
  - Chatwoot dependencies are completely removed from the architecture.
  - Rust-based REST/gRPC endpoints exist for incoming webhooks.
  - Database schema includes multi-tenant (RLS) tables for `channels`, `conversations`, `messages`.
  - Flutter UI displays the new Triage feed and Conversation view at 375px responsive breakpoint.
  - Playwright E2E tests fully cover the CUJ using the local backend stack.
  - 100% unit test coverage for the new Rust and Flutter modules.

  ## 5. Priority
  P0

  ## 6. Estimated Scope
  Large

  ## References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://about.instagram.com/features/instagram-messaging
  3. https://business.whatsapp.com/
  4. https://www.chatwoot.com/
  5. https://shopify.com/
  6. https://squareup.com/
  7. https://www.wix.com/
  8. https://www.hubspot.com/
  9. https://slack.com/
  10. https://discord.com/
  11. https://larksuite.com/
  12. https://dingtalk.com/
  13. https://wecom.qq.com/
  14. https://www.intercom.com/
  15. https://www.zendesk.com/
  16. https://www.freshworks.com/
  17. https://www.frontapp.com/
  18. https://gorgias.com/
  19. https://klaviyo.com/
  20. https://www.zoho.com/desk/
  21. https://www.salesforce.com/service-cloud/
  22. https://www.kustomer.com/
  23. https://www.gladly.com/
  24. https://www.genesys.com/
  25. https://www.five9.com/
  26. https://www.talkdesk.com/
  27. https://www.helpscout.com/
  28. https://www.drift.com/
  29. https://www.liveperson.com/
  30. https://www.ada.cx/
  31. https://www.forethought.ai/
  32. https://www.ultimate.ai/
  33. https://www.netomi.com/
  34. https://www.polyai.com/
  35. https://www.observe.ai/
  36. https://www.asapp.com/
  37. https://www.cresta.com/
  38. https://www.balto.ai/
  39. https://www.cogitocorp.com/
  40. https://www.nice.com/
  41. https://www.verint.com/
  42. https://www.avaya.com/
  43. https://www.cisco.com/c/en/us/products/contact-center/index.html
  44. https://www.amazon.com/connect/
  45. https://www.twilio.com/flex
  46. https://www.vonage.com/contact-center/
  47. https://www.ringcentral.com/contact-center.html
  48. https://www.8x8.com/products/contact-center
  49. https://www.dialpad.com/ai-contact-center/
  50. https://www.uipath.com/
  51. https://www.automationanywhere.com/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
