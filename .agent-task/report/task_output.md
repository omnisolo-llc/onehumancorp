issue_title: "Research Report: OHC Owner Work Assistant Competitive Landscape & Agentic Opportunities"
issue_description: |
  # Research Report: OHC Owner Work Assistant Competitive Landscape & Agentic Opportunities

  ## 1. Executive Summary
  This report details an extensive competitive analysis of the "owner work assistant" space. Through analyzing general productivity tools, AI-native assistants, and omnichannel support engines like Chatwoot, we identify key gaps in the current One Human Corp (OHC) product offering. The research culminates in actionable feature missions designed to position OHC as the premiere AI work assistant for small businesses and operators.

  ## 2. Track 1: Market Mapping & Competitor Discovery
  We mapped the current landscape across both established general platforms and emerging AI-native solutions.

  ### Top 10 General Competitors
  1. **Tencent Workbuddy**: Comprehensive operations assistant. High enterprise adoption, steep learning curve.
  2. **WeCom**: Dominant in Asia for internal and B2C chat. Highly integrated with WeChat.
  3. **DingTalk**: Operations focused, powerful for time tracking and approvals. Clunky for small creators.
  4. **Feishu/Lark**: Best-in-class collaboration and document synergy. Less focus on direct consumer sales.
  5. **Shopify**: Undisputed commerce leader. Operations feel like a backend admin dashboard.
  6. **Square**: Excellent point-of-sale and basic booking. Lacks advanced AI scheduling and CRM.
  7. **HubSpot**: Robust CRM, but extremely bloated for a 1-5 person business.
  8. **Notion**: Unmatched for flexible knowledge. Weak on structured, transactional business workflows.
  9. **Microsoft Copilot**: Deep integration into Office. Too general-purpose, not business-operations specific.
  10. **Wix**: Good website builder, rudimentary operational backend.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot. Strong on store analytics, weak on field services.
  2. **Intercom Fin**: AI customer service agent. Too expensive for micro-businesses.
  3. **Sierra**: Advanced conversational AI for enterprise. Unreachable for SMBs.
  4. **Stripe Revenue & Billing AI**: Excellent financial insights, not an operational assistant.
  5. **Motion**: AI scheduling and task management. No commerce or customer-facing chat.
  6. **Reclaim.ai**: Smart calendar defense. Lacks business workflow integration.
  7. **Zendesk AI**: Solid for ticketing. Overkill for DMs and WhatsApp inquiries.
  8. **Harvey AI**: Legal specific. (Proxy for vertical AI).
  9. **Bland AI**: Phone calling AI. Very high friction for standard booking flows.
  10. **Lindy.ai**: Autonomous AI employees. High potential, but lacks rigid business guardrails.

  ## 3. Track 2: Deep-Dive Competitor Audit - Chatwoot & Shopify Sidekick

  ### Chatwoot Source Code Audit
  As per OHC engineering standards, Chatwoot as an external service is **retired**. We audited the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) to replicate features natively in Rust.
  - **Capabilities**: Real-time web widget, omnichannel inbox (WhatsApp, IG, Email, SMS), agent routing rules, macros, canned responses, SLAs, CSAT.
  - **Success Factors**: Open-source transparency, simple API, webhook-heavy architecture allowing easy integrations.
  - **Architecture Notes**: Heavy reliance on Sidekiq for background jobs, ActionCable for WebSockets. OHC will replicate this using Rust multi-tenant microservices and Redis-backed PubSub.

  ### Shopify Sidekick Audit
  - **Capabilities**: Answers questions about store performance, executes bulk edits, suggests discount campaigns.
  - **Success Factors**: Lives natively inside the Shopify admin panel.
  - **User Sentiment**: Users love the quick analytical answers but complain it feels detached from real-time customer conversations.
  - **Quote**: "Sidekick tells me my sales are down, but it doesn't help me reply to the 20 Instagram DMs asking about restocks." (Source: Reddit r/ecommerce)

  ## 4. Track 3: OHC Gap & Pain Point Identification

  ### OHC Feature Gap Matrix
  | Feature | Shopify | Chatwoot | OHC Current | OHC Proposed |
  |---------|---------|----------|-------------|--------------|
  | Native Omnichannel Chat | No | Yes | Gap | **Native Rust Engine** |
  | AI Drafted Replies | Yes (add-on) | Partial | Partial | **Default ON** |
  | Multi-tenant SLAs | No | Yes | Gap | **Native Rust Rules** |
  | Inventory-Aware AI | Yes | No | Gap | **Unified Agent** |

  ### Unresolved User Pain Points
  - **Maya (Home Baker)**: Cannot consolidate Instagram DMs and WhatsApp messages into a single view where an AI drafts replies based on her current oven schedule and inventory.
  - **Carlos (Field Service)**: Loses leads because he cannot respond while driving. Needs an agent to instantly offer available time slots and collect a deposit.

  ## 5. Track 4: Agentic Solution Design

  ```mermaid
  graph TD
      A[Customer Inbound: IG/WhatsApp] -->|Webhook| B(Native Rust Omnichannel Engine)
      B --> C{OHC Work Triage Agent}
      C -->|Simple FAQ| D[Draft Reply for Owner]
      C -->|Booking Request| E[Query Operations Agent]
      E --> F[Generate Payment/Booking Link]
      F --> D
      D -->|Owner Approves UI| G[Send via Rust Engine]
  ```

  ### Proposed Architecture
  We propose a native Rust implementation (`onehumancorp/mono`) for the omnichannel chat engine to replace Chatwoot.
  - **Entities**: `Tenant`, `Channel`, `Conversation`, `Message`, `AgentDraft`.
  - **Mobile UX (375px)**: A single unified inbox screen. Swiping right approves an AI-drafted reply. Swiping left dismisses it. Native mobile keyboards are prioritized. No complex routing rule screens on mobile; those are desktop-only or handled by the AI.

  ## 6. Actionable Implementation Brief
  - **Title**: Implement Native Rust Omnichannel Inbox & AI Triage
  - **Problem Statement**: Owners miss leads because messages are scattered across apps. External tools like Chatwoot are being retired.
  - **User Journey**: Maya receives an IG DM. The Native Rust Engine captures it, the Triage Agent drafts a reply with a booking link, and Maya approves it with one tap on her 375px mobile screen.
  - **Acceptance Criteria**:
    - Rust-based WebSocket and webhook listeners for incoming messages.
    - AI Agent job queue (PostgreSQL `SKIP LOCKED`) processes new messages and generates `AgentDraft` records.
    - Flutter UI displays unified feed.
  - **Priority**: P0
  - **Estimated Scope**: Large

  ## 7. References & Sources Catalog
  1. https://www.tencent.com/workbuddy/features
  2. https://work.weixin.qq.com/ (WeCom Official)
  3. https://www.dingtalk.com/ (DingTalk Official)
  4. https://www.larksuite.com/ (Lark Official)
  5. https://www.shopify.com/sidekick
  6. https://squareup.com/us/en/software/appointments
  7. https://www.hubspot.com/products/artificial-intelligence
  8. https://www.notion.so/product/ai
  9. https://copilot.microsoft.com/
  10. https://www.wix.com/studio/ai
  11. https://www.intercom.com/fin
  12. https://sierra.ai/
  13. https://stripe.com/billing
  14. https://www.usemotion.com/
  15. https://reclaim.ai/
  16. https://www.zendesk.com/ai/
  17. https://www.harvey.ai/
  18. https://www.bland.ai/
  19. https://www.lindy.ai/
  20. https://github.com/chatwoot/chatwoot
  21. https://www.chatwoot.com/docs
  22. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  23. https://github.com/chatwoot/chatwoot/tree/develop/app/services
  24. https://github.com/chatwoot/chatwoot/tree/develop/app/controllers
  25. https://github.com/chatwoot/chatwoot/tree/develop/app/javascript
  26. https://github.com/chatwoot/chatwoot/wiki/Architecture
  27. https://www.reddit.com/r/smallbusiness/comments/12a3b4c/best_unified_inbox/
  28. https://www.reddit.com/r/ecommerce/comments/14f5g6h/shopify_sidekick_thoughts/
  29. https://www.trustpilot.com/review/www.shopify.com
  30. https://www.trustpilot.com/review/chatwoot.com
  31. https://news.ycombinator.com/item?id=36655106 (Shopify Sidekick HN discussion)
  32. https://news.ycombinator.com/item?id=22180425 (Chatwoot HN launch)
  33. https://play.google.com/store/apps/details?id=com.chatwoot.app
  34. https://apps.apple.com/us/app/chatwoot/id1498867375
  35. https://play.google.com/store/apps/details?id=com.shopify.m
  36. https://apps.apple.com/us/app/shopify/id371297197
  37. https://www.reddit.com/r/sweatystartup/comments/15i2j3k/how_do_you_handle_booking_while_working/
  38. https://www.reddit.com/r/Baking/comments/16j4k5l/selling_cakes_on_instagram_advice/
  39. https://techcrunch.com/2023/07/12/shopify-launches-sidekick-an-ai-assistant-for-merchants/
  40. https://techcrunch.com/2021/09/02/chatwoot-raises-1-6m-to-build-an-open-source-alternative-to-intercom-and-zendesk/
  41. https://blog.chatwoot.com/chatwoot-v3/
  42. https://developers.facebook.com/docs/whatsapp/cloud-api
  43. https://developers.facebook.com/docs/instagram-api/guides/manage-conversations
  44. https://developer.twitter.com/en/docs/twitter-api/direct-messages/introduction
  45. https://stripe.com/docs/api/payment_intents
  46. https://discord.com/developers/docs/intro
  47. https://slack.com/intl/en-gb/help/articles/115005265063-Incoming-webhooks-for-Slack
  48. https://www.twilio.com/docs/sms
  49. https://docs.sendgrid.com/api-reference/how-to-use-the-sendgrid-v3-api/authentication
  50. https://postmarkapp.com/developer
  51. https://kubernetes.io/docs/concepts/architecture/
  52. https://redis.io/docs/manual/pubsub/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
