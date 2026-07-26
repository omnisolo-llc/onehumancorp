issue_title: "Native Rust Omnichannel Inbox & AI Triage Engine"
issue_description: |
  # Native Rust Omnichannel Inbox & AI Triage Engine

  **Priority:** P0
  **Estimated Scope:** Large

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by fragmented communication channels. Customers reach out via Instagram DMs, WhatsApp, SMS, email, and website widgets. Currently, these messages exist in silos, leading to missed opportunities, delayed responses, and lost revenue. Existing solutions like Chatwoot are being retired as an external dependency, necessitating a native, high-performance replacement built directly into OneHumanCorp (OHC). Furthermore, owners need an assistant that doesn't just centralize messages, but actively triages them, drafts responses, and connects them to business actions (booking, quoting, payment).

  ## Research Report & Deep Dive Audit: Chatwoot & Shopify

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the current landscape of owner/operator work assistants and omnichannel tools, visiting over 50 unique sources (see References & Sources Catalog below).
  - **Traditional Giants:** Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark dominate enterprise and SMB communications in Asia with deeply integrated workflows. Shopify dominates SMB e-commerce but often relies on third-party apps for robust omnichannel chat (e.g., Gorgias). Square excels in POS but its messaging is often transactional.
  - **Omnichannel Specialists:** Chatwoot (open source), Intercom, Zendesk provide robust centralized inboxes but are often overly complex or expensive for micro-SMBs, and lack deep native integration into a specific operating system like OHC.
  - **AI-Native Rising Stars:** Shopify Sidekick (AI assistant for merchants), Notion AI, and various specialized AI customer support agents (like Gorgias AI) are showing the value of context-aware AI that can take actions, not just suggest text.

  ### Dynamic Competitive Landscape

  ```mermaid
  quadrantChart
      title Dynamic Competitive Landscape - SMB Omnichannel vs AI Actionability
      x-axis Low Actionability --> High Actionability
      y-axis Siloed Channels --> Unified Omnichannel
      quadrant-1 AI-Powered Omnichannel
      quadrant-2 Traditional Helpdesks
      quadrant-3 Basic Chat Widgets
      quadrant-4 Specialized Vertical SaaS
      "Chatwoot": [0.3, 0.8]
      "Shopify Sidekick": [0.8, 0.5]
      "Intercom": [0.6, 0.85]
      "WeCom": [0.7, 0.7]
      "Zendesk AI": [0.65, 0.9]
      "Square POS": [0.6, 0.3]
      "OHC (Proposed)": [0.9, 0.9]
  ```

  ### Track 2: Deep-Dive Competitor Audit - Chatwoot & Shopify Sidekick
  **Chatwoot (External Service to be Retired):**
  - *Capabilities:* Centralized inbox for Web, Email, Facebook, Twitter, WhatsApp, SMS, Line. Features include agent routing, canned responses, macros, CSAT surveys, SLA policies.
  - *Success Factors:* Open-source nature, comprehensive channel integrations, webhook support.
  - *User Sentiment (Trustpilot/Reddit):* Users love the unified view but complain about complex setup, resource-heavy self-hosting, and the need for significant configuration to integrate with core business data (orders, bookings).

  **Shopify (Sidekick/Inbox):**
  - *Capabilities:* Shopify Inbox centralizes chat. Sidekick (AI) helps merchants manage store tasks, analyze data, and draft replies.
  - *Success Factors:* Deep integration with product catalog, inventory, and customer order history.
  - *User Sentiment:* Merchants appreciate seeing a customer's cart during a chat. Pain points include the disjointed experience when jumping between standard chat and AI tools, and the lack of support for service-based businesses (appointments).

  ### Track 3: OHC Gap & Pain Point Identification
  - **The Chatwoot Gap:** OHC is explicitly retiring Chatwoot as an external dependency. We must build a native Rust multi-tenant omnichannel chat engine.
  - **The Actionability Gap:** Competitor inboxes (like Chatwoot) are passive. They show messages. Owners need an active assistant (like Sidekick, but broader) that reads a message and says, "This is a catering inquiry for next Tuesday. Should I send the $500 quote draft?"
  - **The Mobile-First Gap:** Many robust inboxes are painful to use on a 375px screen. OHC must provide a fluid, Apple-quality translucent mobile experience for triaging work on the go.

  #### Feature Gap Heatmap (OHC vs Competitors)

  | Feature | Chatwoot (Current) | Shopify Sidekick | Square Messages | OHC (Proposed Native) |
  | :--- | :---: | :---: | :---: | :---: |
  | Unified Omnichannel Inbox | ✅ | ⚠️ (Apps needed) | ❌ | ✅ |
  | Native Rust Backend Performance | ❌ (Ruby) | ❌ | ❌ | ✅ |
  | Multi-Tenant Row Level Security | ⚠️ (Logical) | ✅ | ✅ | ✅ (PostgreSQL RLS) |
  | Proactive AI Triage & Drafting | ❌ | ✅ (Limited) | ❌ | ✅ |
  | 1-Tap Action Execution (Booking/Quote) | ❌ | ⚠️ (Commerce only) | ⚠️ (Payments only)| ✅ |
  | 375px Mobile-First Translucent UX | ❌ | ⚠️ | ⚠️ | ✅ (Flutter) |

  ### Track 4: Agentic Solution Design
  OHC needs a Native Rust Omnichannel Inbox powered by an AI Triage Engine.
  1.  **Native Rust Backend:** Implement high-performance WebSocket handling, webhook ingests (Stripe, WhatsApp, IG), and message persistence using Rust and PostgreSQL (RLS for tenant isolation).
  2.  **Unified Work Feed UI:** A Flutter-based mobile-first (375px) feed that merges messages, system alerts, and actionable tasks.
  3.  **AI Work Triage Agent:** When a message arrives (e.g., "Do you have vegan cakes?"), the AI Triage Agent intercepts it, checks OHC Knowledge/Inventory, drafts a reply, and presents it to the owner in the feed for 1-tap approval.

  ## Design Doc
  ### High-Level Architecture
  - **Backend (Rust):**
    - `ohc_chat_engine`: Crate handling WebSocket connections, message routing, and channel adapters (Email, Web Widget, simulated WhatsApp/IG).
    - Database Models: `Conversations`, `Messages`, `ChannelInboxes`, `Participants`, all keyed by `tenant_id` for PostgreSQL RLS.
    - AI Integration: Background job queue (using PostgreSQL SKIP LOCKED) processes new messages through the AI Triage Agent (Gemini/GPT) before they appear in the owner's feed.
  - **Frontend (Flutter):**
    - `WorkTriageScreen`: The main entry point. A unified list blending unread messages and suggested actions.
    - `ConversationDetailScreen`: Chat interface showing message history and AI-drafted suggested replies at the bottom (1-tap send).
    - **Visuals:** Use OHC Premium Tokens. Translucent glass effects for sticky headers, strong typography, clear status indicators (Unread, Draft Ready, Action Required).

  ### Mobile UX Flow (375px First)

  ```mermaid
  sequenceDiagram
      participant C as Customer (Maya's Client)
      participant O as OHC Omnichannel Gateway (Rust)
      participant A as AI Triage Agent
      participant F as OHC Mobile Feed (375px)

      C->>O: "Can I order 20 vegan cupcakes for Friday?" (IG DM)
      O->>A: New inbound message (Context: Maya's Bakery)
      A->>A: Check inventory & calendar
      A->>A: Draft reply & Prepare Quote Link
      A->>O: Return Draft Reply + Action Payload
      O->>F: Push notification: "New Inquiry + Draft Ready"
      F-->>Maya: Taps notification, views conversation
      Maya->>F: Taps "Approve & Send Quote"
      F->>O: Execute Action
      O->>C: Send reply with Stripe Payment Link
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens OHC and sees a unified "Work Triage" feed containing messages from all channels. Instead of just reading messages, the owner sees AI-generated draft responses and suggested actions (like sending a quote) ready for 1-tap approval.

  **Critical User Journey (CUJ):**
  1.  The owner navigates to the Work Triage section.
  2.  They select a new customer inquiry that arrived via a simulated external channel (e.g., a web widget or API ingest).
  3.  The chat view displays the customer's message and a pre-drafted AI response based on the business's context.
  4.  The owner taps "Approve & Send" on the AI draft.
  5.  The message is sent, and the conversation state updates.

  **Acceptance Criteria:**
  - Implement the core database schemas for native omnichannel chat (Conversations, Messages).
  - Implement a basic Rust service layer to handle message ingestion and retrieval.
  - Implement the Flutter UI for the Work Triage feed and Conversation view.
  - Integrate a mock or basic AI prompt flow that generates a suggested reply for incoming messages.
  - Ensure the UI is fully responsive and optimized for a 375px mobile screen.
  - All E2E Playwright/Flutter tests must pass, verifying the entire flow from message ingest to owner approval.

  ## References & Sources Catalog
  - [Chatwoot GitHub Repository](https://github.com/chatwoot/chatwoot)
  - [Chatwoot Official Website](https://www.chatwoot.com/)
  - [Chatwoot Features](https://www.chatwoot.com/features)
  - [Chatwoot Documentation](https://www.chatwoot.com/docs)
  - [Tencent Official Website](https://www.tencent.com/en-us/)
  - [WeCom Official Website](https://work.weixin.qq.com/)
  - [DingTalk Official Website](https://www.dingtalk.com/en)
  - [Feishu / Lark Official Website](https://www.larksuite.com/)
  - [Shopify Official Website](https://www.shopify.com/)
  - [Shopify Magic & Sidekick](https://www.shopify.com/magic)
  - [Square Official Website](https://squareup.com/us/en)
  - [Square Point of Sale](https://squareup.com/us/en/point-of-sale)
  - [HubSpot Official Website](https://www.hubspot.com/)
  - [HubSpot AI](https://www.hubspot.com/artificial-intelligence)
  - [Notion Official Website](https://www.notion.so/)
  - [Notion AI](https://www.notion.so/product/ai)
  - [Microsoft Copilot](https://copilot.microsoft.com/)
  - [Microsoft 365 Copilot](https://www.microsoft.com/en-us/microsoft-365/copilot)
  - [Intercom Official Website](https://www.intercom.com/)
  - [Intercom AI Bot](https://www.intercom.com/ai-bot)
  - [Zendesk Official Website](https://www.zendesk.com/)
  - [Zendesk AI](https://www.zendesk.com/service/ai/)
  - [Gorgias Ecommerce Helpdesk](https://www.gorgias.com/)
  - [Klaviyo Marketing Automation](https://www.klaviyo.com/)
  - [Wix Official Website](https://www.wix.com/)
  - [Wix Studio](https://www.wix.com/studio)
  - [Salesforce Einstein AI](https://www.salesforce.com/einstein/)
  - [Zoho CRM Zia AI](https://www.zoho.com/crm/zia/)
  - [Asana Intelligence](https://asana.com/product/ai)
  - [monday AI](https://monday.com/ai)
  - [Reddit - r/smallbusiness](https://www.reddit.com/r/smallbusiness/)
  - [Reddit - r/Entrepreneur](https://www.reddit.com/r/Entrepreneur/)
  - [Reddit - r/ecommerce](https://www.reddit.com/r/ecommerce/)
  - [Shopify Reviews - Trustpilot](https://www.trustpilot.com/review/www.shopify.com)
  - [Square Reviews - Trustpilot](https://www.trustpilot.com/review/squareup.com)
  - [Chatwoot Reviews - Trustpilot](https://www.trustpilot.com/review/chatwoot.com)
  - [Shopify - App Store](https://apps.apple.com/us/app/shopify/id371297800)
  - [Square POS - App Store](https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788)
  - [WeCom - App Store](https://apps.apple.com/us/app/wecom/id1189621106)
  - [Shopify - Google Play](https://play.google.com/store/apps/details?id=com.shopify.m)
  - [Square - Google Play](https://play.google.com/store/apps/details?id=com.squareup)
  - [Shopify Reviews - G2](https://www.g2.com/products/shopify/reviews)
  - [Square POS Reviews - G2](https://www.g2.com/products/square-point-of-sale/reviews)
  - [Shopify Reviews - Capterra](https://www.capterra.com/p/133541/Shopify/)
  - [Square POS Reviews - Capterra](https://www.capterra.com/p/146059/Square-Point-of-Sale/)
  - [TechCrunch - Small Business](https://techcrunch.com/tag/small-business/)
  - [Forbes - Small Business](https://www.forbes.com/small-business/)
  - [Bloomberg Technology](https://www.bloomberg.com/technology)
  - [WSJ - Entrepreneurship](https://www.wsj.com/business/entrepreneurship)
  - [Y Combinator B2B Companies](https://www.ycombinator.com/companies?industry=B2B)
  - [Hacker News](https://news.ycombinator.com/)
  - [DEV Community - AI](https://dev.to/t/ai)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
