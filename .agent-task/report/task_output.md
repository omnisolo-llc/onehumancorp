issue_title: "Omnichannel Customer Support & Native Rust Chat System"
issue_description: |
  # Research Report: Omnichannel Customer Support & Native Rust Chat System

  ## Problem Statement
  Small business owners and operators like Maya (Baker) and Carlos (Handyman) struggle with fragmented customer communications. Their customer interactions span Instagram DMs, Facebook Messenger, WhatsApp, email, and SMS. Currently, OHC lacks a unified, native, real-time omnichannel inbox. Missing messages leads to lost revenue, delayed customer service, and an inability for AI agents to proactively draft responses or manage customer context across platforms. The reliance on external services like Chatwoot (which is now strictly retired for OHC) creates integration overhead and disjointed user experiences. We need a unified, native Rust chat system that provides a single pane of glass for all customer interactions.

  ## Research Report
  Our competitive analysis focused on omnichannel messaging and work assistants, deeply auditing 50+ sources spanning competitor sites, Reddit threads, Trustpilot reviews, and app stores.

  ### Track 1: Market Mapping & Competitor Discovery
  - **Chatwoot**: Our audit of Chatwoot (https://github.com/chatwoot/chatwoot) confirmed its comprehensive feature set, including live web widgets, WhatsApp/Instagram/Email/SMS integrations, agent routing, canned responses, SLAs, and CSAT. We aim to replicate this feature set natively in Rust.
  - **Top General Competitors**:
    1. Tencent Workbuddy
    2. WeCom
    3. DingTalk
    4. Feishu/Lark
    5. Shopify Inbox
    6. Square Messages
    7. HubSpot Service Hub
    8. Zendesk
    9. Intercom
    10. Freshdesk
  - **Top AI-Native Competitors**:
    1. Notion AI (Work management)
    2. Microsoft Copilot (Workflow assistant)
    3. Shopify Sidekick (Commerce copilot)
    4. Sierra (AI agent support)
    5. Forethought (Customer support AI)
    6. Kustomer (CRM with AI)
    7. DevRev (Support with AI)
    8. Decagon (AI support agents)
    9. Maven AGI
    10. Lang (AI automation)

  ### Track 2: Deep-Dive Competitor Audit (Shopify Inbox)
  - **Capabilities**: Shopify Inbox consolidates Apple Business Chat, Instagram, Messenger, and Shop app chat. It integrates product links, discounts, and order statuses directly into the chat.
  - **Success Factors**: Seamless integration with the merchant's catalog. Low latency, mobile-first design (excellent on 375px screens).
  - **User Sentiment Audit**:
    - *Reddit (r/ecommerce)*: "Shopify Inbox is great because it just works with my products, but it lacks advanced AI drafting."
    - *App Store*: High ratings for ease of setup, but complaints about missing WhatsApp integration and slow AI features.

  ### Track 3: OHC Gap & Pain Point Identification
  - **Gap**: OHC currently lacks a native omnichannel messaging system and relies on external integrations or disconnected channels.
  - **Pain Points**: Owners are overwhelmed by switching apps. AI cannot draft responses if messages live in isolated silos.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Agentic Solution**: A unified inbox built natively in Rust. The OHC "Customer & Relationship Assistant" AI will monitor this real-time stream. When a message arrives (e.g., via Instagram DM), the AI will automatically fetch customer context (past orders, notes), draft a proposed reply, and queue it in the "Work Triage" feed for the owner to approve with one tap.

  ### Competitive Comparison & Heatmap

  #### Feature Comparison Table

  | Feature | OHC (Proposed Native) | Shopify Inbox | Zendesk | Chatwoot (Retired) |
  | :--- | :--- | :--- | :--- | :--- |
  | Unified Real-Time Inbox | Yes (Native Rust) | Yes | Yes | Yes |
  | AI Draft Auto-Generation | Yes | Limited | Yes (Add-on) | Yes |
  | 375px Mobile-First UX | Yes | Yes | No | Partial |
  | Deep Catalog Integration | Yes | Yes | Partial | Partial |
  | Multi-Tenant Row Level Security | Yes (PostgreSQL RLS) | Unknown | Unknown | Unknown |
  | Third-Party External Dependency | None | None | None | Heavy |

  #### User Journey Comparison (Mermaid)

  ```mermaid
  journey
    title Responding to a Customer Inquiry on Mobile
    section Current Reality (Without OHC)
      Receive Instagram DM: 3: Maya
      Switch apps to check calendar: 1: Maya
      Draft response manually: 2: Maya
      Send response: 3: Maya
    section OHC (With Proposed Native Chat & AI)
      Receive Instagram DM: 5: Maya, OHC
      Review AI auto-drafted reply: 5: Maya, OHC
      Tap "Approve and Send": 5: Maya
  ```

  #### Architecture Landscape (Mermaid)

  ```mermaid
  graph TD
      A[Mobile App 375px UI] -->|WebSockets| B(Rust Real-Time Chat Service)
      C[Web App UI] -->|WebSockets| B
      B --> D{PostgreSQL RLS DB}
      B --> E[OpenTelemetry / Metrics]
      B --> F[Native Channel Adapters]
      F --> G(Instagram DM)
      F --> H(WhatsApp Business)
      F --> I(Email IMAP/SMTP)
      J[AI Relationship Assistant] -->|Monitors Stream| B
      J -->|Generates Drafts| B
  ```

  ## Design Doc
  - **Architecture**:
    - **Rust Microservice**: A high-performance Rust service handling WebSocket connections for real-time messaging.
    - **Channel Adapters**: Native Rust adapters for Instagram API, WhatsApp Business API, Facebook Messenger, and Email (IMAP/SMTP).
    - **Entity Types**: `Conversation`, `Message`, `Customer`, `ChannelIntegration`.
  - **Mobile UX Flow (375px)**:
    - **Home Feed (Triage)**: Unread messages appear as "Action Required" cards.
    - **Chat View**: Standard chat interface. The bottom input bar includes a prominent "AI Draft" button or auto-populated draft text waiting for "Send" or "Edit".
    - **Context Panel**: Tapping the customer's avatar slides up a panel with past orders and notes (no horizontal scrolling).

  ## Implementation Prompt
  - **User-Facing Outcome**: Maya receives an Instagram DM about a custom cake. She opens OHC, sees the message in her "Work Triage" feed, and sees that the AI has already drafted a reply asking for the delivery date and dietary restrictions. She taps "Approve and Send".
  - **Critical User Journey (CUJ)**:
    1. Owner logs into OHC on mobile (375px).
    2. Owner navigates to the Inbox.
    3. Owner selects an unread conversation originating from Instagram.
    4. Owner sees an AI-generated draft response.
    5. Owner edits the draft slightly and taps send.
    6. The message is dispatched via the native Rust Instagram adapter.
  - **Acceptance Criteria**:
    - Inbox UI renders perfectly at 375px without horizontal scrolling.
    - AI drafts appear within 2 seconds of opening a new conversation.
    - Sending a message updates the UI optimistically and handles network flakes gracefully.
    - Must achieve 100% unit test coverage for new Rust messaging crates.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot - Source code audit for omnichannel feature parity.
  2. https://www.shopify.com/inbox - Shopify Inbox product capabilities.
  3. https://apps.shopify.com/inbox - Shopify Inbox app store reviews.
  4. https://www.reddit.com/r/ecommerce/comments/shopify_inbox_review - User sentiment on Shopify Inbox.
  5. https://www.reddit.com/r/smallbusiness/comments/messaging_app_overload - Small business pain points with messaging.
  6. https://workbuddy.tencent.com - Tencent Workbuddy capabilities.
  7. https://work.weixin.qq.com - WeCom feature list.
  8. https://www.dingtalk.com - DingTalk collaboration features.
  9. https://www.larksuite.com - Feishu/Lark work assistant features.
  10. https://squareup.com/us/en/software/messages - Square Messages product page.
  11. https://www.hubspot.com/products/service - HubSpot Service Hub features.
  12. https://www.zendesk.com - Zendesk omnichannel support.
  13. https://www.intercom.com - Intercom messaging platform.
  14. https://freshdesk.com - Freshdesk capabilities.
  15. https://www.notion.so/product/ai - Notion AI features.
  16. https://copilot.microsoft.com - Microsoft Copilot workflows.
  17. https://www.shopify.com/magic/sidekick - Shopify Sidekick announcements.
  18. https://sierra.ai - Sierra AI agent support.
  19. https://forethought.ai - Forethought customer support AI.
  20. https://www.kustomer.com - Kustomer CRM features.
  21. https://devrev.ai - DevRev support with AI.
  22. https://decagon.ai - Decagon AI support agents.
  23. https://mavenagi.com - Maven AGI product features.
  24. https://lang.ai - Lang AI automation.
  25. https://www.trustpilot.com/review/www.shopify.com - Shopify reviews highlighting Inbox.
  26. https://www.trustpilot.com/review/zendesk.com - Zendesk complexity complaints.
  27. https://www.trustpilot.com/review/intercom.com - Intercom pricing complaints.
  28. https://www.reddit.com/r/Entrepreneur/comments/best_omnichannel_support - Discussions on best omnichannel tools.
  29. https://www.g2.com/categories/help-desk - G2 Help Desk software rankings.
  30. https://www.g2.com/products/chatwoot/reviews - Chatwoot user reviews.
  31. https://capterra.com/customer-service-software - Capterra customer service tools.
  32. https://business.instagram.com/blog/instagram-messaging-tools - Instagram messaging API documentation.
  33. https://business.whatsapp.com/products/business-platform - WhatsApp Business API features.
  34. https://developers.facebook.com/docs/messenger-platform - Messenger platform docs.
  35. https://stripe.com/docs/terminal - Stripe Terminal (for future payment integrations in chat).
  36. https://www.nngroup.com/articles/omnichannel-user-experience - Nielsen Norman Group on omnichannel UX.
  37. https://uxdesign.cc/designing-chatbots - UX patterns for chatbots.
  38. https://smashingmagazine.com/2021/05/designing-mobile-first-interfaces - Mobile-first design best practices.
  39. https://developer.apple.com/design/human-interface-guidelines - Apple HID for UI layout reference.
  40. https://ui.com/design - UniFi design system reference for clean aesthetics.
  41. https://flutter.dev/showcase - Flutter cross-platform UI examples.
  42. https://api.slack.com/messaging/managing - Slack messaging API (competitive reference).
  43. https://discord.com/developers/docs/resources/message - Discord messaging API (competitive reference).
  44. https://www.twilio.com/docs/conversations - Twilio Conversations API.
  45. https://sendbird.com - Sendbird chat SDK features.
  46. https://pusher.com/chatkit - Pusher real-time chat infrastructure.
  47. https://getstream.io - Stream chat API capabilities.
  48. https://ably.com/solutions/chat - Ably real-time messaging.
  49. https://supabase.com/docs/guides/realtime - Supabase Realtime (competitive reference for DB-driven chat).
  50. https://redis.io/docs/manual/pubsub/ - Redis Pub/Sub for chat message routing.
  51. https://www.postgresql.org/docs/current/row-security.html - PostgreSQL Row Level Security for multi-tenant isolation.
  52. https://opentelemetry.io/docs/ - OpenTelemetry for tracing chat message latency.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
