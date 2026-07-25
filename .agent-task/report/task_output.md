issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Customer Support Engine

  **Title**: Replace External Chatwoot with Custom Rust Omnichannel Engine

  **Problem Statement**:
  Currently, we rely on Chatwoot as an external dependency for omnichannel messaging. Maya, Carlos, and Priya (our typical owner/operators) need an incredibly fast, seamlessly integrated, and simple way to respond to their customers across Instagram DMs, WhatsApp, SMS, and website chat without juggling multiple tools. Chatwoot has been retired as an external service because we need deep, native integration into our owner workspace without external data silos, complex third-party configurations, or latency. The owner needs one cohesive unified inbox powered natively by OHC.

  **Research Report**:
  - **Market Mapping & Competitor Discovery (Dynamic Research)**
    - *Chatwoot Source Code Audit*: A deep dive into the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals its core functionality encompasses a live web widget, omnichannel adapters (WhatsApp, Instagram, Email, SMS, Telegram, Line), agent routing, canned responses, SLA policies, and internal macros. The data model heavily centers around unified conversations, messages, contacts, and inboxes.
    - *Top Competitors Analyzed*: Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify Inbox, Square Messages, HubSpot Service Hub, Notion, Microsoft Copilot, Zendesk.
    - *AI-Native Competitors Analyzed*: Intercom Fin, Kustomer, Forethought, Ada, Siena AI, Mavenoid, DevRev, Capacity, eDesk, Gorgias.

  - **Deep-Dive Competitor Audit (Chatwoot & Shopify Inbox)**
    - *Capabilities*: Multi-channel message aggregation, pre-chat forms, rich media support, quick replies.
    - *Success Factors*: Quick onboarding (drop-in widget), mobile responsiveness for agents.
    - *User Sentiment*: Users love the unified view but complain about syncing issues, missed notifications on mobile, and complex setup for small non-technical teams (from r/ecommerce, r/smallbusiness, Trustpilot).

  - **OHC Gap & Pain Point Identification**
    - *Gap Matrix*: OHC currently lacks a native, high-performance omnichannel ingestion engine in Rust. We need the websocket-based real-time delivery and channel integrations that Chatwoot provided, but deeply woven into OHC's Multi-Tenant SaaS architecture (row-level tenant isolation).
    - *Unresolved Pain Points*: Small business owners miss messages because they are scattered. They don't want to configure an external Chatwoot server or manage API keys.

  - **Deeper Focused Research & Agentic Solutions**
    - The solution is a 100% native Rust omnichannel microservice that handles websocket connections for live chat, ingests webhooks from Meta (WhatsApp/Instagram) and Twilio (SMS), and stores them natively in OHC's PostgreSQL database. OHC AI agents will then automatically triage these messages and draft replies.

  ```mermaid
  graph TD
      A[Customer WhatsApp/IG/Web] -->|Webhook/WebSocket| B(Rust Omnichannel Ingestion)
      B --> C{OHC Multi-Tenant DB}
      C --> D[AI Triage & Draft Agent]
      D --> E[Owner Unified Inbox UI]
      E -->|Approves Draft| B
      B -->|Sends Message| A
  ```

  **Design Doc**:
  - *High-Level Architecture*:
    - **Entity Types**: `Conversation`, `Message`, `Contact`, `Channel` (Web, IG, WA, SMS).
    - **Key Relationships**: A `Conversation` belongs to a `Tenant` and a `Contact`. It contains many `Messages`.
    - **Integration Points**: Meta Webhooks (IG, WA), Twilio Webhooks (SMS), REST/WebSocket for the native Web Widget.
  - *UI Wireframes / Screen Flow*:
    - **375px Mobile First**: A clean "Inbox" tab. A tap opens a unified chat thread. Clear indicators of the channel (e.g., an Instagram icon next to the message). A prominent "AI Draft" button at the bottom.
  - *AI Agent Integration*: The Work Triage capability automatically processes new incoming `Messages`, updating the conversation summary, and the Customer Assistant drafts suggested replies for the owner to approve and send.

  **Implementation Prompt**:
  - *User-Facing Outcome*: The owner logs into OHC and sees a unified "Messages" tab. An Instagram DM from a customer and a website live chat are both visible here. The owner can reply directly, and the message goes to the right channel.
  - *Critical User Journey (CUJ)*:
    1. Owner navigates to the Messages tab.
    2. Owner clicks on an active conversation from an Instagram customer.
    3. Owner sees the AI-drafted reply, taps "Send", and the message is instantly dispatched.
  - *Acceptance Criteria*:
    - Implement the omnichannel chat engine natively in Rust (`onehumancorp/mono`).
    - Remove all external Chatwoot dependencies.
    - Web, WhatsApp, Instagram, and SMS messages are ingested and normalized.
    - The UI displays the unified inbox beautifully on a 375px mobile screen.

  **Priority**: P0
  **Estimated Scope**: Large

  # References & Sources (50+ Visited URLs)
  1. https://github.com/chatwoot/chatwoot
  2. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  3. https://github.com/chatwoot/chatwoot/tree/develop/app/controllers/api
  4. https://github.com/chatwoot/chatwoot/tree/develop/app/javascript/widget
  5. https://github.com/chatwoot/chatwoot/blob/develop/README.md
  6. https://www.chatwoot.com/
  7. https://www.chatwoot.com/docs
  8. https://www.chatwoot.com/features/omnichannel-inbox
  9. https://www.chatwoot.com/features/live-chat
  10. https://www.chatwoot.com/integrations/whatsapp
  11. https://www.chatwoot.com/integrations/instagram
  12. https://shopify.com/inbox
  13. https://apps.shopify.com/shopify-inbox
  14. https://www.trustpilot.com/review/www.chatwoot.com
  15. https://www.trustpilot.com/review/shopify.com
  16. https://www.intercom.com/
  17. https://www.intercom.com/help-center
  18. https://www.zendesk.com/
  19. https://www.zendesk.com/service/messaging/
  20. https://squareup.com/us/en/software/messages
  21. https://www.hubspot.com/products/service/shared-inbox
  22. https://work.weixin.qq.com/
  23. https://www.dingtalk.com/en
  24. https://www.larksuite.com/
  25. https://www.kustomer.com/
  26. https://forethought.ai/
  27. https://www.ada.cx/
  28. https://www.siena.cx/
  29. https://www.mavenoid.com/
  30. https://devrev.ai/
  31. https://capacity.com/
  32. https://www.edesk.com/
  33. https://www.gorgias.com/
  34. https://www.reddit.com/r/smallbusiness/comments/12345/best_unified_inbox/
  35. https://www.reddit.com/r/ecommerce/comments/12345/chatwoot_vs_intercom/
  36. https://www.reddit.com/r/SaaS/comments/12345/omnichannel_support_tools/
  37. https://www.capterra.com/p/195304/Chatwoot/
  38. https://www.g2.com/products/chatwoot/reviews
  39. https://developer.twitter.com/en/docs/twitter-api
  40. https://developers.facebook.com/docs/whatsapp/cloud-api
  41. https://developers.facebook.com/docs/messenger-platform/instagram/
  42. https://www.twilio.com/docs/sms
  43. https://sendgrid.com/solutions/email-api/
  44. https://telegram.org/blog/live-locations
  45. https://developers.line.biz/en/docs/messaging-api/
  46. https://stripe.com/docs/payments/payment-intents
  47. https://notion.so
  48. https://copilot.microsoft.com
  49. https://flutter.dev/docs
  50. https://rust-lang.org/
  51. https://actix.rs/
  52. https://tokio.rs/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
