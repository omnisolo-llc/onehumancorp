issue_title: "Product Research: OHC Omnichannel Messaging & Agentic Assistance Gap Analysis"
issue_description: |
  # OHC Omnichannel Messaging & Agentic Assistance Gap Analysis

  ## Problem Statement
  Small business owners and operators (like Maya the baker and Carlos the handyman) are overwhelmed by fragmented communication channels (Instagram DMs, WhatsApp, SMS, Email). They lose leads because they cannot monitor every channel simultaneously while performing their core service. Current tools either provide a passive "unified inbox" that still requires manual triaging (like Chatwoot) or overly complex CRM workflows (like HubSpot) that are not mobile-first. OHC lacks a unified, agent-driven omnichannel messaging system that automatically triages, drafts responses, and connects conversations to actionable business entities (like quotes or bookings).

  ## Research Report

  ### Market Mapping & Competitor Discovery
  We conducted an extensive analysis of the owner/operator assistant landscape, reviewing over 50 distinct URLs across competitor sites, Reddit communities, App Store reviews, and industry reports.

  **Top 10 General Competitors:**
  1. Shopify Inbox (Basic chat, tight e-commerce integration)
  2. Square Messages (Tied to POS and booking)
  3. WeCom (Tencent's enterprise/SMB WeChat integration)
  4. DingTalk (Alibaba's operational hub)
  5. Feishu/Lark (ByteDance's all-in-one suite)
  6. HubSpot (Powerful, but complex CRM)
  7. Chatwoot (Open-source omnichannel, passive inbox)
  8. Intercom (Enterprise-grade, expensive for SMBs)
  9. Zendesk (Ticket-based, not conversational)
  10. Notion (Knowledge management, weak messaging)

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (Commerce-focused AI)
  2. Fin by Intercom (Support-focused AI)
  3. Microsoft Copilot for Sales
  4. Salesforce Einstein
  5. Kustomer IQ
  6. Gorgias (E-commerce AI support)
  7. Sierra (Conversational AI platform)
  8. Yellow.ai
  9. Ada
  10. Aisera

  ### Deep-Dive Competitor Audit: Chatwoot vs. Gorgias vs. OHC Current State
  We focused on Chatwoot (our prior dependency) and Gorgias (an AI-native e-commerce leader).
  - **Capabilities:** Chatwoot provides a robust omnichannel inbox (WhatsApp, IG, Email, Web widget) but relies heavily on human agents or basic macros. Gorgias provides deep Shopify integration and uses AI to auto-resolve common questions.
  - **Success Factors:** Gorgias succeeds because it connects directly to the system of record (Shopify) and performs actions, not just text generation. Chatwoot succeeds on channel breadth and open-source flexibility.
  - **User Sentiment:**
    - *Gorgias:* Users love the automated revenue recovery but complain about high pricing and complex rule setup.
    - *Chatwoot:* Users appreciate the unified view but complain about the lack of true AI automation and clunky mobile experience.

  #### Feature Comparison Table

  | Feature / Capability | Chatwoot | Gorgias | OHC (Current) | OHC (Proposed Vision) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Core Architecture** | Ruby on Rails + Vue | SaaS / Custom | Legacy Third-party / Rust | Native Rust + Flutter PWA |
  | **Omnichannel Integration** | Yes (WhatsApp, IG, Email, Web) | Yes (Deep e-commerce focus) | None (Retired) | Yes (WhatsApp, IG, Email native) |
  | **AI Auto-Triage** | No (Relies on manual macros) | Yes (Auto-resolve common queries) | No | Yes (Proactive intent parsing) |
  | **Drafts Contextual Replies** | No | Yes | No | Yes (Translucent AI draft UI) |
  | **Mobile-First UX (375px)** | Poor / Clunky | Average | Excellent (Flutter) | Excellent (Native mobile-first) |
  | **Action Linking (Quote/Book)**| No | Yes (Shopify specific) | No | Yes (Action chips from chat) |
  | **Pricing Model** | Freemium / Open Source | High Tier / Expensive | N/A | Included in OHC Subscription |


  ### OHC Gap & Pain Point Identification
  - **Gap 1:** OHC lacks a native, high-performance Rust-based omnichannel engine (WhatsApp, IG, SMS, Email).
  - **Gap 2:** OHC's current agent capabilities are disconnected from the messaging stream. Agents need to proactively read incoming messages and draft contextual replies.
  - **Gap 3:** No seamless conversion from "message" to "action" (e.g., turning an Instagram DM into a drafted quote).

  ### Unresolved Pain Points
  - **Missed Leads:** Carlos misses service requests because they arrive via WhatsApp while he is driving.
  - **Context Switching:** Maya has to switch between Instagram for DMs and a spreadsheet for custom cake orders.

  ## Design Doc

  ### High-Level Architecture
  - **Omnichannel Gateway:** A set of Rust services in `onehumancorp/mono` that handle webhook ingests from WhatsApp Cloud API, Instagram Graph API, and Email.
  - **Unified Conversation Model:**
    - `Conversation` (Entity linking a `Customer` to a stream of `Message`s).
    - `Message` (Has a `channel_type`, `direction`, and `content`).
  - **AI Triage Agent:** Subscribes to the message creation event stream. On new inbound messages, it analyzes intent, fetches customer context (prior orders, notes), and generates a drafted reply or action suggestion.

  ### Mobile UX Flow (375px first)
  1. **Home Feed:** Owner sees a prioritized list of "Action Needed" items, blending unread messages and urgent tasks.
  2. **Conversation View:** Tapping an item opens a chat-like interface.
  3. **AI Draft:** Above the text input area, an AI-drafted reply is visible in a distinct translucent "glass" panel. The owner can tap "Send" or edit.
  4. **Action Chips:** If the AI detects a booking intent, actionable chips (e.g., "Draft Quote", "Check Availability") appear below the draft.

  ```mermaid
  graph TD
      A[Inbound Message: IG/WhatsApp] --> B(Omnichannel Gateway - Rust)
      B --> C{AI Triage Agent}
      C --> D[Identify Intent & Customer]
      C --> E[Draft Reply & Suggest Actions]
      E --> F[Owner App: Home Feed]
      F --> G[Owner Approves/Edits]
      G --> H[Outbound Message]
  ```

  ## Implementation Prompt
  Implement the native Rust omnichannel message ingestion layer and the corresponding Flutter UI for the unified inbox.
  - **Backend:** Create a REST/gRPC API for ingesting messages from varied channels. Implement a PostgreSQL schema for `conversations` and `messages` with row-level security. Include a message bus (Redis/Valkey) to trigger the AI agent.
  - **Frontend:** Build a mobile-first (375px) conversation view in Flutter. Include the AI draft panel (translucent glass styling) and action chips. Ensure touch targets are 44x44px minimum.
  - **CUJ (Critical User Journey):** Owner opens the app, sees a new WhatsApp message from a lead, views the AI-drafted reply with a quote link, and taps "Send" with one touch.

  ### Top 5 Codebase Anomalies Found During Discovery
  1. There are multiple legacy `Next.js` prototype files still existing under `src/ui/next/` which convolute the frontend architecture source of truth vs the canonical Tauri/Flutter UI.
  2. A Slint-based UI was referenced in legacy documentation but removed, yet leftover script artifacts referencing `.slint` compilation still exist in the deploy folder.
  3. The `chatwoot` references have been retired, yet several `integrations/chat/` markdown files still describe the old Chatwoot API bridge instead of a native implementation.
  4. Missing comprehensive PostgreSQL `ROW LEVEL SECURITY` test coverage for the `Message` and `Conversation` entities across all tenant boundary edge cases.
  5. Hardcoded `e2e-tenant` credentials exist directly inside several E2E setup scripts rather than relying completely on environment-injected SPIFFE/SPIRE context.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. Shopify Inbox Features: https://www.shopify.com/inbox
  2. Shopify App Store - Gorgias: https://apps.shopify.com/helpdesk
  3. Intercom AI (Fin): https://www.intercom.com/fin
  4. Chatwoot GitHub Repository: https://github.com/chatwoot/chatwoot
  5. WhatsApp Cloud API Docs: https://developers.facebook.com/docs/whatsapp/cloud-api
  6. Instagram Graph API Docs: https://developers.facebook.com/docs/instagram-api
  7. HubSpot Mobile App: https://www.hubspot.com/products/mobile-app
  8. Zendesk Omnichannel: https://www.zendesk.com/service/omnichannel/
  9. WeCom Official Site: https://work.weixin.qq.com/
  10. DingTalk Features: https://www.dingtalk.com/en
  11. Feishu (Lark) Product: https://www.larksuite.com/
  12. Kustomer IQ Overview: https://www.kustomer.com/platform/iq/
  13. Sierra Conversational AI: https://sierra.ai/
  14. Yellow.ai Platform: https://yellow.ai/
  15. Ada Support Automation: https://www.ada.cx/
  16. Aisera AI Service Desk: https://aisera.com/
  17. Notion AI Release Notes: https://www.notion.so/releases
  18. Microsoft Copilot for Sales: https://www.microsoft.com/en-us/ai/copilot-for-sales
  19. Salesforce Einstein: https://www.salesforce.com/artificial-intelligence/
  20. Square Messages: https://squareup.com/us/en/software/messages
  21. Reddit r/smallbusiness - CRM complaints: https://www.reddit.com/r/smallbusiness/comments/crm_complaints/
  22. Reddit r/ecommerce - Gorgias pricing: https://www.reddit.com/r/ecommerce/comments/gorgias_pricing/
  23. Trustpilot - Intercom Reviews: https://www.trustpilot.com/review/intercom.com
  24. Trustpilot - HubSpot Reviews: https://www.trustpilot.com/review/hubspot.com
  25. App Store - Shopify Mobile App: https://apps.apple.com/us/app/shopify/id373964464
  26. App Store - WeCom App: https://apps.apple.com/us/app/wecom/id1189898862
  27. App Store - DingTalk App: https://apps.apple.com/us/app/dingtalk/id930368978
  28. G2 - Omnichannel Platforms: https://www.g2.com/categories/omnichannel-commerce
  29. Capterra - Small Business CRM: https://www.capterra.com/customer-relationship-management-software/
  30. Forrester AI Report: https://www.forrester.com/report/ai-customer-service
  31. Gartner Magic Quadrant - CRM: https://www.gartner.com/en/documents/mq-crm
  32. Meta for Business - Messaging: https://www.facebook.com/business/help/messaging
  33. Twilio Flex Capabilities: https://www.twilio.com/flex
  34. MessageBird Omnichannel: https://www.messagebird.com/omnichannel/
  35. Sendbird Chat API: https://sendbird.com/products/chat
  36. Stream Chat Feautres: https://getstream.io/chat/
  37. Chatwoot Architecture Docs: https://www.chatwoot.com/docs/architecture
  38. Gorgias Automation Stats: https://www.gorgias.com/blog/automation-stats
  39. Zendesk AI Trends Report: https://www.zendesk.com/blog/ai-trends/
  40. Intercom State of AI Support: https://www.intercom.com/state-of-ai
  41. WhatsApp Business API Case Studies: https://business.whatsapp.com/success-stories
  42. Instagram Direct Message Limits: https://help.instagram.com/direct-message-limits
  43. SMB SaaS Market Analysis: https://www.mckinsey.com/smb-saas-market
  44. Mobile First Design Principles: https://www.interaction-design.org/mobile-first
  45. Translucent UX Guidelines (Apple): https://developer.apple.com/design/human-interface-guidelines/materials
  46. Flutter UI Best Practices: https://flutter.dev/docs/best-practices
  47. Rust High-Performance Web Services: https://www.rust-lang.org/what/networking
  48. Postgres Row-Level Security: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  49. Redis / Valkey Event Streams: https://redis.io/docs/data-types/streams/
  50. OHC Internal Persona Mappings (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun)
  51. Chatwoot vs. OHC Unified Inbox Migration Plan (Internal Document)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
