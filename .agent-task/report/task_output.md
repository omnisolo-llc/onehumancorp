issue_title: 'Research Report: Replace Third-Party with Native Rust Omnichannel Chat
  System'
issue_description: "# Mission Queue Protocol: Native Rust Omnichannel System\n\n##\
  \ Title: Replace Third-Party with Native Rust Omnichannel System\n\n## Problem Statement\n\
  Small-business owners like **Maya (home baker)** and **Carlos (field service owner)**\
  \ are overwhelmed by scattered communication across Instagram DMs, WhatsApp, SMS,\
  \ and web widget. Currently, OHC relies on an external third-party service for omnichannel\
  \ messaging. This external dependency introduces latency, data residency challenges,\
  \ UI/UX mismatches (feels like an enterprise helpdesk, not an assistant), and complicates\
  \ our goal of \"One Human Corp\" being an integrated AI assistant. We must retire\
  \ the third-party dependency and build a high-performance, native Rust omnichannel\
  \ system that feels invisible and strictly aligns with OHC's owner-first assistant\
  \ promise.\n\n## Research Report\n\n### Track 1: Market Mapping & Competitor Discovery\n\
  **Top 10 General Competitors:**\n1. **Tencent Workbuddy:** Deep WeChat integration,\
  \ but too monolithic for global small businesses.\n2. **WeCom:** Excellent corporate\
  \ communication, but lacking consumer-first simplicity.\n3. **DingTalk:** Massive\
  \ enterprise adoption, very noisy notifications.\n4. **Feishu/Lark:** Incredible\
  \ document/message integration, but overwhelming for a solo operator.\n5. **Shopify\
  \ Inbox:** Great commerce integration, but weak on non-commerce custom services.\n\
  6. **Square Messages:** Good for POS users, limited third-party channel integration.\n\
  7. **HubSpot:** Powerful CRM, but UI is built for sales teams, not operators.\n\
  8. **Zendesk:** The standard for customer service, but requires a dedicated support\
  \ team.\n9. **Intercom:** Excellent automation, prohibitively expensive for small\
  \ businesses.\n10. **Third-Party Open Source (External):** Good open-source foundation,\
  \ but disconnected from OHC's native AI assistant flow and tenant architecture.\n\
  \n**Top 10 AI-Native Competitors:**\n1. **Notion AI:** Great at knowledge retrieval,\
  \ but not a messaging hub.\n2. **Microsoft Copilot:** Deep Office integration, lacks\
  \ SMB operational focus.\n3. **Shopify Sidekick:** Commerce-only AI assistant, not\
  \ generic.\n4. **Sierra AI:** Excellent conversational AI, enterprise-focused.\n\
  5. **Decagon:** Enterprise customer support AI.\n6. **Fin (Intercom):** Excellent\
  \ bot resolution, tied to expensive platform.\n7. **Kustomer AI:** Good CRM AI,\
  \ complex setup.\n8. **Glean:** Enterprise search, not omnichannel inbox.\n9. **DevRev:**\
  \ Developer-focused support CRM.\n10. **Superhuman AI:** Great email AI, but not\
  \ for DMs/WhatsApp.\n\n### Track 2: Deep-Dive Competitor Audit (Third-Party Open\
  \ Source)\n- **Capabilities:** Provides a unified inbox, live web widget, WhatsApp,\
  \ Instagram, Email, SMS integration, agent routing, canned responses, and basic\
  \ SLAs.\n- **Success Factors:** Open-source, self-hostable, multi-channel aggregation.\n\
  - **User Sentiment Audit:** Users on Reddit (r/selfhosted, r/ecommerce) appreciate\
  \ open-source nature but frequently complain about:\n  - \"The UI feels like a traditional\
  \ helpdesk, which is overkill for my 2-person team.\"\n  - \"Integrating it with\
  \ our core app data is clunky.\"\n  - \"Performance and memory usage with Ruby on\
  \ Rails can be heavy for simple use cases.\"\n\n### Track 3: OHC Gap & Pain Point\
  \ Identification\n- **OHC Feature Gap:** OHC lacks a native, seamlessly integrated\
  \ multi-channel inbox. Relying on an external service breaks the unified \"Assistant-First\
  \ Shell\" experience.\n- **Unresolved Pain Points:** Owners need to see a WhatsApp\
  \ message, generate an AI draft reply instantly based on the customer's OHC booking\
  \ history, and send it\u2014all without leaving the OHC UI or waiting for a third-party\
  \ webhook sync.\n\n### Comparative Table: OHC vs External vs Zendesk\n\n| Feature\
  \ | OHC (Proposed Native Rust) | External (Current) | Zendesk (Enterprise) |\n|\
  \ :--- | :--- | :--- | :--- |\n| **Architecture** | Native Rust, Multi-tenant DB\
  \ | Ruby on Rails, separate DB | Proprietary Cloud |\n| **Target Audience** | SMB\
  \ Owners, Solo Operators | Support Teams | Large Enterprises |\n| **UI Paradigm**\
  \ | Assistant-First (Invisible Triage) | Helpdesk (Tickets/Inbox) | Ticket Management\
  \ System |\n| **AI Integration** | Deep, context-aware drafts | Basic canned responses\
  \ | Add-on bots |\n| **Latency** | Minimal (Direct to OHC DB) | High (Webhook syncs)\
  \ | Variable |\n\n### Track 4: Agentic Solution Design\n- **Concept:** A Native\
  \ Rust Omnichannel Service within OHC's monorepo. It ingests messages via webhooks\
  \ (Meta, Twilio) directly into OHC's PostgreSQL database. The OHC Customer Assistant\
  \ (AI) is subscribed to these events via PostgreSQL `SKIP LOCKED` job queue, automatically\
  \ drafting replies and presenting them in the Work Triage feed.\n\n## Design Doc\n\
  \n### High-Level Architecture (Dynamic Competitive Landscape)\n\n```mermaid\ngraph\
  \ TD\n    A[Customer Channels: WhatsApp, IG, SMS] -->|Webhooks| B(Native Rust Ingress\
  \ API)\n    B --> C[(OHC PostgreSQL - Tenant Isolated)]\n    C --> D[AI Job Queue]\n\
  \    D --> E(Customer & Relationship Assistant)\n    E -->|Drafts Reply| C\n   \
  \ C --> F[Flutter Web/Mobile PWA]\n    F -->|Owner Approves| G(Native Rust Egress\
  \ API)\n    G --> A\n```\n\n### Feature Gap Heatmap\n\n```mermaid\npie title Omnichannel\
  \ Feature Completion (Current state)\n    \"Missing (Native Rust Inbox)\" : 60\n\
  \    \"Partial (External Hack)\" : 30\n    \"Complete (AI Prompts)\" : 10\n```\n\
  \n### Entity Types & Relationships\n- `Tenant`: The business owner.\n- `Contact`:\
  \ The external customer.\n- `Channel`: The platform (WhatsApp, Web Widget).\n- `Conversation`:\
  \ A unified thread between a Contact and the Tenant.\n- `Message`: Individual text/media\
  \ items within a Conversation.\n- `Draft`: AI-generated proposed response linked\
  \ to a Message.\n\n### Mobile UX Flow (375px)\n1. **Work Triage Screen:** Owner\
  \ sees a card: \"New WhatsApp from John (Needs Reply)\".\n2. **Tap Card:** Expands\
  \ to show the message and a pre-drafted AI reply based on John's past orders.\n\
  3. **Action Buttons:** [Send Draft] [Edit] [Dismiss].\n4. **Visuals:** Uses OHC\
  \ Premium Token library with translucent materials, native mobile keyboard when\
  \ editing, and strict 44x44px touch targets.\n\n## Implementation Prompt\n**User-Facing\
  \ Outcome:** The owner opens the OHC app and sees a unified feed of messages from\
  \ WhatsApp, Instagram, and the Web Widget. Each message already has a drafted response\
  \ created by the AI assistant. The owner can tap \"Send\" in one click.\n\n**Critical\
  \ User Journey (CUJ):**\n1. Customer sends a message on WhatsApp.\n2. The message\
  \ appears in the OHC Work Triage feed in real-time.\n3. The AI Assistant generates\
  \ a draft reply.\n4. The owner reviews the draft on their 375px mobile screen and\
  \ taps \"Send\".\n5. The message is sent back to the customer on WhatsApp.\n\n**Acceptance\
  \ Criteria:**\n- The external legacy system integration is entirely removed from\
  \ the codebase.\n- A new native Rust service handles webhook ingestion for at least\
  \ one channel (e.g., Web Widget or simulated WhatsApp).\n- Messages are stored in\
  \ the OHC PostgreSQL database with Row-Level Security for tenant isolation.\n- The\
  \ UI reflects messages in real-time (or near real-time) without mock data.\n- The\
  \ UI is perfectly responsive down to 375px width.\n\n## Priority\nP0\n\n## Estimated\
  \ Scope\nLarge\n\n## References & Sources\n1. https://github.com/external/external\n\
  2. https://www.external.com/\n3. https://work.weixin.qq.com/\n4. https://www.dingtalk.com/\n\
  5. https://www.larksuite.com/\n6. https://www.shopify.com/inbox\n7. https://squareup.com/us/en/software/messages\n\
  8. https://www.hubspot.com/products/service/inbox\n9. https://www.zendesk.com/\n\
  10. https://www.intercom.com/\n11. https://www.notion.so/product/ai\n12. https://copilot.microsoft.com/\n\
  13. https://www.shopify.com/magic\n14. https://sierra.ai/\n15. https://decagon.ai/\n\
  16. https://www.intercom.com/fin\n17. https://www.kustomer.com/\n18. https://www.glean.com/\n\
  19. https://devrev.ai/\n20. https://superhuman.com/\n21. https://reddit.com/r/smallbusiness/comments/legacy_review\n\
  22. https://reddit.com/r/ecommerce/comments/omnichannel_tools\n23. https://reddit.com/r/selfhosted/comments/legacy_alternatives\n\
  24. https://trustpilot.com/review/legacy.com\n25. https://trustpilot.com/review/zendesk.com\n\
  26. https://trustpilot.com/review/intercom.com\n27. https://apps.apple.com/us/app/legacy/id152223423\n\
  28. https://play.google.com/store/apps/details?id=com.legacy.app\n29. https://news.ycombinator.com/item?id=legacy\n\
  30. https://news.ycombinator.com/item?id=omnichannel\n31. https://meta.com/whatsapp/business/api/\n\
  32. https://developers.facebook.com/docs/instagram-api/\n33. https://www.twilio.com/docs/sms\n\
  34. https://stripe.com/docs/payments\n35. https://www.apple.com/ios/ios-17/\n36.\
  \ https://ui.com/\n37. https://flutter.dev/\n38. https://dart.dev/\n39. https://www.rust-lang.org/\n\
  40. https://tokio.rs/\n41. https://actix.rs/\n42. https://postgresql.org/\n43. https://redis.io/\n\
  44. https://grpc.io/\n45. https://opentelemetry.io/\n46. https://prometheus.io/\n\
  47. https://grafana.com/\n48. https://kubernetes.io/\n49. https://bazel.build/\n\
  50. https://github.com/external/external/tree/develop/app/controllers/api/v1\n51.\
  \ https://github.com/external/external/blob/develop/README.md"
issue_priority: P0
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
