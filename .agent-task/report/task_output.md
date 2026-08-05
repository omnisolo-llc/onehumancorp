issue_title: "Native Rust Omnichannel Inbox & AI Routing (Chatwoot Replacement)"
issue_description: |
  # Mission Queue Protocol Brief: Native Rust Omnichannel Inbox & AI Routing (Chatwoot Replacement)

  ## Problem Statement
  Small business owners like Maya (Baker) and Carlos (Handyman) are overwhelmed by messages across Instagram, WhatsApp, Web, and Email. Currently, OHC relies on fragmented communication or external dependencies like Chatwoot, which breaks the "OneHumanCorp Promise" of a unified, AI-first work assistant. Owners are forced into technical setups, managing API keys, and handling disjointed customer histories. They need a single, native inbox where AI triages messages, drafts responses, and connects to their operational tasks without needing technical expertise.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  #### Chatwoot Source Code Audit & Feature Benchmarking
  - **Core Engine**: Ruby on Rails backend, Vue.js frontend.
  - **Key Features**: Omnichannel inbox, agent routing, canned responses, macros, SLA policies, webhooks, and multi-tenant architecture.
  - **Shortcomings for OHC**: Chatwoot is built for support teams, not solo owner-operators. It lacks native AI-first triage and operational integration (e.g., turning a chat into a booking or deposit request).

  #### Top 10 General Competitors
  1. **Shopify Inbox**: Great commerce integration, but poor service/appointment support.
  2. **HubSpot**: Powerful but complex and expensive; feels like an enterprise CRM, not an assistant.
  3. **Square Messages**: Good point-of-sale integration, but limited channel support.
  4. **Zendesk**: Enterprise focus; overwhelming for small business operators.
  5. **WeCom (Tencent)**: Deep WeChat integration, but regionally locked and clunky outside China.
  6. **DingTalk**: Massive feature set, but feels like an admin portal.
  7. **Feishu/Lark**: Excellent collaboration, but weak external customer omnichannel features.
  8. **Intercom**: Expensive and heavily geared towards SaaS companies.
  9. **Front**: Great shared inbox, but lacks AI-driven autonomous operational tasks.
  10. **Gorgias**: E-commerce focused, completely ignores service and creator personas.

  #### Top 10 AI-Native Competitors
  1. **Shopify Sidekick**: AI commerce copilot; highly contextual but closed ecosystem.
  2. **Microsoft Copilot for Sales**: Strong enterprise CRM link, useless for micro-businesses.
  3. **Notion AI**: Good for knowledge, but disconnected from live customer chat.
  4. **Sierra**: Conversational AI for enterprise support.
  5. **Decagon**: AI customer support agents.
  6. **Bland AI**: Phone calling agents, weak text/omnichannel support.
  7. **Fin (Intercom)**: AI support bot, expensive addon.
  8. **Kustomer AI**: Good CRM integration, enterprise pricing.
  9. **Forethought**: AI support automation.
  10. **Rippling AI**: Internal HR/IT operations, not customer-facing.

  ### Track 2: Deep-Dive Competitor Audit (Chatwoot)
  - **Capabilities**: Unifies Web, Email, Facebook, Twitter, WhatsApp, Line, SMS into one inbox. Supports teams, automated routing, and CSAT.
  - **Success Factors**: Open-source transparency, simple REST API, extensible webhooks.
  - **User Sentiment Audit**:
    - *Pros*: "Love having all my messages in one place," "Self-hosting is a lifesaver." (Source: Reddit r/selfhosted).
    - *Cons*: "The UI is dated and clunky," "Integrating AI bots requires a lot of custom webhook scripting," "Not really built for solo operators." (Source: Trustpilot, GitHub Issues).

  ### Track 3: OHC Gap & Pain Point Identification
  - **Gap Matrix (Chatwoot vs. OHC)**:
    | Feature | Chatwoot | OHC Current | OHC Target (Native Rust) |
    |---------|----------|-------------|--------------------------|
    | Omnichannel | Yes | Partial | Yes (Unified) |
    | AI Triage | No (Needs add-on) | Weak | Core Capability |
    | Operational Sync | No | Disconnected | Native |
    | Mobile-First | Usable | Broken | Flawless (375px) |
  - **Unresolved Pain Points**: Owners cannot seamlessly turn a WhatsApp message into a booked appointment and payment link in one tap.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering**: A review of Shopify Inbox and Chatwoot user forums reveals that owners spend 2+ hours daily just copying context from chats into their scheduling/billing tools.
  - **Agentic Solution Design**: A native Rust-based omnichannel ingestion engine. When a message arrives (e.g., WhatsApp), the `Work Triage` AI agent intercepts it, summarizes the intent, checks the `Knowledge & Compliance Assistant` for context, and presents the owner with a single actionable card: "Maya, John wants a custom cake for Saturday. I've drafted a reply and a $50 deposit link. [Send & Request Deposit]".

  ## Design Doc

  ### High-Level Architecture
  - **Ingestion Service**: Native Rust microservice (`onehumancorp/mono/chat_engine`) handling Webhooks (WhatsApp, IG) and WebSockets.
  - **Data Model**: `Conversations` (Tenant-scoped), `Messages`, `Contacts`, `AI_Drafts`.
  - **AI Integration**: Messages trigger an asynchronous AI Job Queue event. The `Customer & Relationship Assistant` agent evaluates the `Message` and generates an `AI_Draft` linked to the `Conversation`.

  ### UI Flow & Mobile UX (375px First)
  - **Home Feed**: The assistant shell surfaces urgent unread conversations as actionable cards.
  - **Conversation View**: Clean, translucent glass UI. AI drafts appear in a distinct, glowing container above the native mobile keyboard. The owner can tap "Approve" or edit the draft.
  - **Action Drawer**: Swipe up to instantly attach an Offer, Quote, or Booking link to the chat.

  ```mermaid
  graph TD;
      A[Customer WhatsApp] -->|Webhook| B(Rust Chat Ingestion);
      B --> C{AI Triage Agent};
      C -->|Generate Draft| D[PostgreSQL Message Store];
      C -->|Context| E[Customer History & Operations];
      D --> F[Owner Mobile App 375px];
      F -->|Approve & Send| B;
  ```

  ## Implementation Prompt
  **User-Facing Outcome**: The owner opens the OHC app and sees a unified feed of messages from WhatsApp, Web, and Email. Each message already has a drafted, context-aware reply generated by the AI assistant, along with operational actions (like attaching a payment link).
  **Critical User Journey (CUJ)**:
  1. Receive an incoming WhatsApp message via the unified inbox.
  2. View the AI-generated response draft within the mobile-first (375px) chat UI.
  3. Tap "Approve & Attach Deposit" to send the reply and payment link seamlessly.
  **Acceptance Criteria**:
  - Rust ingestion engine handles concurrent incoming webhooks securely.
  - AI drafts are generated within 3 seconds of message receipt.
  - UI renders flawlessly on a 375px screen without horizontal scrolling.
  - 100% unit test coverage and E2E Playwright verification.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot
  2. https://github.com/chatwoot/chatwoot/issues
  3. https://github.com/chatwoot/chatwoot/pulls
  4. https://www.shopify.com/inbox
  5. https://www.shopify.com/inbox/features
  6. https://www.shopify.com/inbox/pricing
  7. https://apps.shopify.com/chat
  8. https://www.hubspot.com/products/service/shared-inbox
  9. https://www.hubspot.com/pricing/service
  10. https://community.hubspot.com/
  11. https://squareup.com/us/en/software/messages
  12. https://squareup.com/help/us/en/article/7331-square-messages
  13. https://squareup.com/us/en/point-of-sale
  14. https://www.zendesk.com/
  15. https://www.zendesk.com/service/messaging/
  16. https://www.zendesk.com/pricing/
  17. https://work.weixin.qq.com/ (WeCom)
  18. https://work.weixin.qq.com/api/doc/90000/90135/90664
  19. https://www.dingtalk.com/
  20. https://www.dingtalk.com/en
  21. https://www.larksuite.com/
  22. https://www.larksuite.com/product/messenger
  23. https://www.intercom.com/
  24. https://www.intercom.com/fin-ai-copilot
  25. https://www.intercom.com/pricing
  26. https://front.com/
  27. https://front.com/features/shared-inbox
  28. https://front.com/pricing
  29. https://www.gorgias.com/
  30. https://www.gorgias.com/product/omnichannel
  31. https://www.shopify.com/magic/sidekick
  32. https://www.microsoft.com/en-us/ai/copilot-for-sales
  33. https://learn.microsoft.com/en-us/microsoft-sales-copilot/
  34. https://www.notion.so/product/ai
  35. https://www.notion.so/pricing
  36. https://sierra.ai/
  37. https://sierra.ai/product
  38. https://decagon.ai/
  39. https://decagon.ai/platform
  40. https://www.bland.ai/
  41. https://www.bland.ai/docs
  42. https://www.kustomer.com/platform/iq/
  43. https://www.kustomer.com/pricing/
  44. https://forethought.ai/
  45. https://forethought.ai/products/solve
  46. https://www.rippling.com/
  47. https://reddit.com/r/smallbusiness/comments/inbox_recommendations
  48. https://reddit.com/r/smallbusiness/comments/shopify_sidekick_review
  49. https://reddit.com/r/ecommerce/comments/chatwoot_vs_intercom
  50. https://www.trustpilot.com/review/chatwoot.com
  51. https://www.trustpilot.com/review/shopify.com
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
