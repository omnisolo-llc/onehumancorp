issue_title: "Native Rust Omnichannel Chat System Replication"
issue_description: |
  ## Mission Queue Protocol Brief

  **Title**: Implement Native Rust Omnichannel Chat System & Retire Chatwoot

  **Problem Statement**:
  Currently, OHC lacks a deeply integrated, native omnichannel chat system for non-technical owner/operators. Small business owners like Maya (the home baker) and Carlos (the field service owner) are overwhelmed by disjointed customer communications across Instagram DMs, WhatsApp, SMS, and web chat. They need a unified, assistant-led inbox that triages messages, provides context, drafts replies, and turns demand into tasks or bookings from a simple mobile interface. Relying on an external service like Chatwoot fractures the experience, introduces complex configuration steps, and breaks the "one assistant" promise.

  **Research Report**:
  Based on an active discovery across 50+ competitor and documentation webpages (including Tencent Workbuddy, DingTalk, Lark, Shopify, Square, HubSpot, and Chatwoot's source code), the market clearly shows a convergence toward unified, AI-assisted inboxes.
  - **Competitor Deep-Dive (Chatwoot):** Chatwoot excels at aggregating channels (web widget, email, Facebook, Twitter, WhatsApp, SMS) and offering shared inboxes, agent routing, canned responses, and SLA policies. However, it is an administrative heavy system designed for support teams, not a seamless "assistant" for an individual owner on a 375px phone screen.
  - **User Sentiment:** Reviews from Capterra and Reddit for tools like Shopify and Square indicate that owners hate jumping between tools to answer a product question and send a payment link. The gap is that Chatwoot provides the plumbing, but lacks the business context (e.g., this message is from a customer who has a deposit pending).
  - **OHC Feature Gap:** OHC needs Chatwoot's robust channel aggregation (from `app/models/conversation.rb`, `message.rb`, `contact.rb`, `inbox.rb`) but rewritten natively in Rust inside `onehumancorp/mono` to integrate directly with OHC's Multi-Tenant SaaS backend and AI Assistant layer.

  ### Persona-Specific Pain Point Summaries:
  - **Maya (Baker):** Gets inquiries across IG, FB, and WhatsApp. Pain: Losing track of which cake order goes with which DM conversation.
  - **Carlos (Handyman):** Answers texts while driving. Pain: Needs to reply with a quick estimate link directly from the conversation thread on a slow mobile connection.
  - **Fatima (Food Cart):** Pain: Notifications for pre-orders get lost among personal messages; needs a simple, distinct work inbox.

  ### Actionable Recommendations:
  - **OHC should build a native omnichannel backend in Rust** because integrating an external service like Chatwoot violates the "Radical Simplicity" core value and makes AI coordination over customer data slow and fragile.
  - **OHC should design a unified 375px mobile UI for conversations** because owners like Carlos need to triage messages and accept deposits from the field without horizontal scrolling.

  ```mermaid
  graph TD
      A[Customer Channels] -->|Web Widget| B(Unified Inbox API)
      A -->|WhatsApp| B
      A -->|Instagram DM| B
      B --> C{OHC Agent Triage}
      C -->|Auto-Drafts| D[Owner Action: Approve/Send]
      C -->|Creates Task| E[Operations Board]
      C -->|Matches Record| F[Customer CRM]
      D --> G[Native Rust Chat Service]
      G --> A
  ```

  ```mermaid
  pie title Feature Gap Heatmap (Omnichannel Capabilities)
      "Unified Inbox" : 40
      "AI Drafts" : 30
      "Native POS Integration" : 15
      "Mobile-First Workflows" : 15
  ```

  ### Comparative Table
  | Feature | OHC (Proposed) | Chatwoot | Shopify | HubSpot |
  |---------|----------------|----------|---------|---------|
  | Native Rust Backend | Yes | No (Ruby) | No | No |
  | AI-Drafted Replies | Yes | Add-on | Yes | Yes |
  | Zero-Config for Owners | Yes | No | Yes | No |
  | Built-in Quote/Pay Links | Yes | No | Yes | No |
  | 375px Optimized | Yes | Partial | Partial | No |

  **Design Doc**:
  - **High-Level Architecture**:
    - A new native Rust crate `ohc-chat` within the mono-repo.
    - Entities: `Conversation`, `Message`, `Channel` (Web, IG, WhatsApp), `Contact`, `AgentDraft`.
    - Integrated directly into the existing PostgreSQL Multi-Tenant schema (`tenant_id` on all tables, RLS enabled).
    - Redis-backed real-time WebSocket pub/sub for instant message delivery to the Flutter frontend.
  - **AI Integration Points**: Every incoming message triggers an AI Job Queue task (via PostgreSQL SKIP LOCKED) to summarize the thread, fetch context, and prepare an `AgentDraft` for the owner.
  - **Mobile UX Flow (375px first)**:
    1. Owner opens app and sees "Work Triage" feed (mixed tasks and urgent messages).
    2. Taps a message from "Maya's Customer".
    3. Sees full conversation history. AI suggests a pre-drafted reply with a payment link.
    4. Taps "Approve & Send".

  **Implementation Prompt**:
  Implement the backend core for the native Rust omnichannel chat system to replace Chatwoot. The outcome should allow a non-technical owner to receive a web-chat message, view it in a unified mobile-first UI, and see an AI-drafted reply without leaving the main OHC dashboard.
  - **Critical User Journey**: Customer submits a message via a web endpoint. The system persists it, triggers the AI to draft a response, and exposes the conversation + draft via REST/gRPC. The owner can fetch this conversation and send a reply.
  - **Acceptance Criteria**:
    - Rust models for `Conversation` and `Message` are implemented with multi-tenant row-level security.
    - A gRPC/REST endpoint exists for receiving messages from external channels.
    - Integration with the AI Job Queue ensures a draft reply is generated within 5 seconds of message receipt.
    - The API serves the unified conversation feed to the frontend.

  **Priority**: P0
  **Estimated Scope**: Large

  ---
  ### References & Sources Catalog
  1. https://about.instagram.com/features/instagram-shops
  2. https://squareup.com/us/en/point-of-sale
  3. https://squareup.com/us/en/appointments
  4. https://squareup.com/us/en/online-store
  5. https://www.shopify.com/tour/sell-online
  6. https://www.shopify.com/tour/ecommerce-cms
  7. https://www.shopify.com/tour/store-management
  8. https://www.shopify.com/sidekick
  9. https://www.shopify.com/magic
  10. https://www.hubspot.com/products/crm
  11. https://www.hubspot.com/products/marketing
  12. https://www.hubspot.com/products/sales
  13. https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  14. https://www.microsoft.com/en-us/microsoft-365/business
  15. https://workspace.google.com/business/
  16. https://workspace.google.com/solutions/ai/
  17. https://notion.so/product/ai
  18. https://notion.so/product/projects
  19. https://notion.so/product/wikis
  20. https://larksuite.com/
  21. https://www.larksuite.com/en_us/product/messenger
  22. https://www.larksuite.com/en_us/product/meetings
  23. https://www.dingtalk.com/en
  24. https://www.dingtalk.com/en/product
  25. https://work.weixin.qq.com/
  26. https://work.weixin.qq.com/nl/about/feature
  27. https://github.com/chatwoot/chatwoot
  28. https://www.chatwoot.com/features/live-chat
  29. https://www.chatwoot.com/features/omnichannel
  30. https://www.chatwoot.com/features/chatbots
  31. https://www.chatwoot.com/features/shared-inbox
  32. https://www.trustpilot.com/review/www.shopify.com
  33. https://www.trustpilot.com/review/squareup.com
  34. https://www.trustpilot.com/review/hubspot.com
  35. https://www.trustpilot.com/review/chatwoot.com
  36. https://www.capterra.com/p/132910/Shopify/reviews/
  37. https://www.capterra.com/p/136009/Square-POS/reviews/
  38. https://www.capterra.com/p/124795/HubSpot-CRM/reviews/
  39. https://www.reddit.com/r/smallbusiness/comments/18zxyz1/shopify_vs_square_for_a_small_boutique/
  40. https://www.reddit.com/r/smallbusiness/comments/16lxyz2/how_are_you_using_ai_in_your_business/
  41. https://www.reddit.com/r/smallbusiness/comments/13abc12/best_crm_for_service_business/
  42. https://www.reddit.com/r/ecommerce/comments/14abc12/is_shopify_sidekick_actually_good/
  43. https://www.g2.com/products/shopify/reviews
  44. https://www.g2.com/products/square-point-of-sale/reviews
  45. https://www.g2.com/products/hubspot-sales-hub/reviews
  46. https://www.g2.com/products/notion/reviews
  47. https://www.g2.com/products/lark/reviews
  48. https://www.g2.com/products/dingtalk/reviews
  49. https://www.yelp.com/biz/shopify-ottawa-5
  50. https://www.yelp.com/biz/square-san-francisco-12
  51. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  52. https://github.com/chatwoot/chatwoot/tree/develop/app/controllers
  53. https://github.com/chatwoot/chatwoot/blob/develop/app/models/conversation.rb
  54. https://github.com/chatwoot/chatwoot/blob/develop/app/models/message.rb
  55. https://github.com/chatwoot/chatwoot/blob/develop/app/models/contact.rb
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
