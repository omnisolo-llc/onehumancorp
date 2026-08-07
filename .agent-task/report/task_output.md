issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  OHC currently relies on external systems like Chatwoot for omnichannel customer support and messaging. The current approach violates OHC's core promise to provide a deeply integrated, assistant-first, owner-centered work environment where all work happens in one place, natively. An external dependency limits our ability to embed AI assistant capabilities deeply into the conversation flow, control the data schema for multi-tenant isolation, and deliver the promised 375px mobile-first performance. Chatwoot has been explicitly retired as a dependency. We need a native Rust implementation of omnichannel chat capabilities inside OHC.

  ## Research Report
  ### Market Mapping & Competitor Discovery
  - **Tencent Workbuddy / WeCom**: Focus on deep integration with WeChat ecosystem, unified communication, and strong organizational management. (Sources: Tencent Workbuddy/WeCom product pages).
  - **Lark/Feishu**: Unified suite combining chat, docs, calendar, and project management. Excellent at breaking down data silos but can feel heavy for small operators. (Source: larksuite.com).
  - **Shopify Inbox/Sidekick**: AI-assisted commerce chat. Good integration with store data, but limited to the Shopify ecosystem. (Source: shopify.com).
  - **Square Messages**: Integrates payments and messaging, but basic omnichannel features compared to dedicated support desks. (Source: squareup.com).
  - **Chatwoot**: Open-source omnichannel customer support platform. Excellent reference for feature set (live chat, WhatsApp, email, social integrations, SLAs, macros, agent routing, CSAT, canned responses). (Sources: chatwoot.com, github.com/chatwoot/chatwoot).
  - **Wix Inbox**: Offers a unified inbox as part of their business management suite, combining site chat, forms, email, and social media. (Source: wix.com).
  - **HubSpot Service Hub**: Comprehensive customer service software with a shared inbox, omnichannel messaging, and AI customer agents. Powerful but complex. (Source: hubspot.com).
  - **Notion AI**: Agentic workflows for internal knowledge, demonstrating the power of deep AI integration. (Source: notion.so).

  ### Deep-Dive Competitor Audit: Chatwoot
  - **Capabilities**: Unified inbox (Live Chat, WhatsApp, Email, FB, IG, Twitter, Telegram, Line, SMS), AI agent (Captain) for drafting/summarizing/translating, smart routing, SLAs, private notes, @mentions, macros, CSAT surveys, canned responses, help center integration.
  - **Success Factors**: Open-source flexibility, strong API/webhook support, clear unified timeline UI, easy widget customization.
  - **User Sentiment**: Users love the omnichannel unification ("indispensable tool for any customer-focused business"). Pain points often revolve around complex self-hosting setups or specific channel integration quirks.

  ### OHC Gap & Pain Point Identification
  - **Gap**: OHC lacks a native omnichannel inbox. We need to replicate Chatwoot's core features (live chat widget, channel adapters for WhatsApp/Email/Social, unified timeline, AI assistant integration) natively in Rust.
  - **Unresolved Pain Point for Owners**: Small business owners (like Maya the Baker or Carlos the Handyman) are overwhelmed by messages across Instagram, WhatsApp, and email. They need a single, simple feed where an AI assistant not only aggregates the messages but also drafts replies and proposes next actions (like sending a quote or scheduling a visit).

  ### Agentic Solution Design
  - **Unified Rust Backend**: Build a multi-tenant omnichannel chat engine in Rust (`onehumancorp/mono`).
  - **Channel Adapters**: Implement adapters for Live Chat (WebSocket), WhatsApp, Email, and Instagram DMs.
  - **AI Triage & Customer Assistant Integration**: Deeply integrate the OHC AI assistant (Gemini/MiniMax) to listen to the unified message stream. The assistant automatically tags messages, drafts replies, and extracts actionable tasks (e.g., "Create Quote for Custom Cake").
  - **Owner UI**: A mobile-first (375px) unified inbox where the owner sees prioritized messages with AI-suggested actions and drafted replies ready for approval.

  ## Design Doc
  - **Architecture**:
    - `ConversationService` (Rust gRPC): Manages conversations, messages, and channel webhooks.
    - `ChannelAdapters`: Modules for handling specific vendor APIs (WhatsApp, Instagram, Email).
    - `AICoordinator`: Listens to new messages, updates tenant-scoped memory, and generates draft replies via the AI Job Queue.
  - **Entity Types**: `Conversation`, `Message` (with polymorphic content/attachments), `Channel` (LiveChat, WhatsApp, etc.), `Participant` (Customer, Owner, AI Agent).
  - **UI Wireframes/Flow (Mobile-First 375px)**:
    - **Unified Inbox Screen**: List of active conversations, sorted by AI-determined priority. Unread indicators and channel icons (WhatsApp, IG).
    - **Conversation Detail Screen**: Chat timeline. At the bottom, an "AI Draft" bubble showing a suggested reply based on customer context. Actions row: [Approve Draft], [Edit], [Create Quote], [Schedule].
    - **Translucent Glass Styling**: Apply OHC Premium Tokens for a clean, modern look.

  ## Implementation Prompt
  - **Objective**: Implement the native Rust omnichannel chat backend and the corresponding Flutter/PWA unified inbox UI to replace Chatwoot.
  - **Critical User Journey (CUJ)**:
    1. A customer sends a message via a simulated external channel (e.g., a mock WhatsApp webhook endpoint for local dev).
    2. The message is ingested by the Rust backend and routed to the correct tenant's unified inbox.
    3. The AI assistant processes the message, updates customer context, and generates a draft reply.
    4. The owner (e.g., Maya) opens the OHC mobile app (375px viewport), sees the new conversation at the top of the feed.
    5. Maya taps the conversation, reviews the AI-drafted reply, taps "Approve", and the message is sent back through the channel adapter.
  - **Acceptance Criteria**:
    - Multi-tenant Rust service handles message CRUD and channel webhooks.
    - UI correctly renders a unified inbox and conversation view without horizontal scrolling at 375px.
    - AI integration successfully drafts replies based on conversation history.
    - ZERO mock data in the UI; all data must flow through the real backend.
    - Full Playwright E2E test covering the CUJ.
    - `bazel test //...` passes 100%.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## Visual Charts

  ```mermaid
  graph TD;
      A[Customer Channels: WhatsApp, IG, Live Chat] -->|Webhooks/WS| B(Rust Omnichannel Gateway);
      B --> C{AI Triage & Context};
      C -->|Drafts & Priorities| D[Unified Inbox DB];
      D --> E[Flutter/PWA Mobile UI];
      E -->|Owner Approves| B;
      B -->|Sends Reply| A;
  ```

  ```mermaid
  pie title Omnichannel Support Value
    "Unified Context" : 40
    "AI Automation" : 35
    "Multi-channel Reach" : 25
  ```

  ## Competitor Comparison Table

  | Feature | Chatwoot | Shopify Inbox | HubSpot Service Hub | OHC (Proposed Native) |
  | :--- | :--- | :--- | :--- | :--- |
  | **Target User** | Support Teams | E-commerce Stores | Enterprise/Mid-Market | Small Business Owners/Operators |
  | **Omnichannel** | Yes (Broad) | Yes (Social/Email) | Yes (Broad) | Yes (Broad, Native) |
  | **AI Integration** | Add-on (Captain) | Commerce-focused | Yes (Agents) | Deep, Assistant-First |
  | **UI Complexity** | High (Admin portal) | Medium | Very High | Radical Simplicity (Mobile First) |
  | **Dependency** | External | Built-in | External Suite | Internal (Rust) |

  ## References & Sources
  1. DingTalk Homepage - https://dingtalk.com/
  2. Lark Homepage - https://www.larksuite.com/en_us/
  3. Shopify Homepage - https://www.shopify.com/
  4. Shopify Pricing - https://www.shopify.com/pricing
  5. Square Homepage - https://squareup.com/us/en
  6. Square Pricing - https://squareup.com/us/en/pricing
  7. Wix Homepage - https://www.wix.com/
  8. Wix Upgrade/Pricing - https://www.wix.com/upgrade/website
  9. HubSpot Homepage - https://www.hubspot.com/
  10. HubSpot Pricing - https://www.hubspot.com/pricing
  11. Notion Homepage - https://www.notion.so/
  12. Notion Pricing - https://www.notion.so/pricing
  13. HackerNews SpaceX - https://news.ycombinator.com/item?id=38318712
  14. Chatwoot Homepage - https://chatwoot.com/
  15. Chatwoot Support Desk - https://www.chatwoot.com/product/support-desk
  16. Chatwoot Live Chat - https://www.chatwoot.com/features/live-chat
  17. Chatwoot Pricing - https://www.chatwoot.com/pricing
  18. Chatwoot Help Center - https://www.chatwoot.com/help-center
  19. Chatwoot Signup - https://app.chatwoot.com/app/auth/signup
  20. Chatwoot Demo Request - https://www.chatwoot.com/request-a-demo
  21. Chatwoot Voice Channel - https://www.chatwoot.com/features/voice-channel
  22. Chatwoot Reviews (G2) - https://g2.com/products/chatwoot/reviews
  23. Chatwoot GitHub Repository - https://github.com/chatwoot/chatwoot
  24. Chatwoot Captain AI - https://www.chatwoot.com/captain
  25. Chatwoot Case Study (Fair Dee) - https://www.chatwoot.com/case-studies/fair-dee
  26. Chatwoot Omnichannel Features - https://www.chatwoot.com/features/omnichannel
  27. Chatwoot Help Center Product - https://www.chatwoot.com/product/help-center
  28. Chatwoot Trust/Security - https://trust.chatwoot.com/
  29. Chatwoot Deployment - https://www.chatwoot.com/deploy
  30. Chatwoot Features Overview - https://www.chatwoot.com/features
  31. Chatwoot Integrations - https://www.chatwoot.com/features/integrations
  32. Chatwoot Mobile Apps - https://www.chatwoot.com/mobile-apps
  33. Chatwoot Changelog - https://www.chatwoot.com/changelog
  34. Chatwoot User Guide - https://www.chatwoot.com/hc/user-guide/en
  35. Chatwoot Team - https://www.chatwoot.com/team
  36. Chatwoot Blog - https://www.chatwoot.com/blog
  37. Chatwoot Case Studies - https://www.chatwoot.com/case-studies
  38. Chatwoot Affiliate Program - https://www.chatwoot.com/affiliate-program
  39. Chatwoot Product Docs - https://www.chatwoot.com/docs/product
  40. Chatwoot Self-Hosted Docs - https://www.chatwoot.com/docs/self-hosted
  41. Chatwoot Developer API - https://www.chatwoot.com/developers/api
  42. Chatwoot Contributing Guide - https://www.chatwoot.com/docs/contributing-guide
  43. Chatwoot Employee Handbook - https://www.chatwoot.com/hc/handbook/en
  44. Chatwoot Tools - https://www.chatwoot.com/tools
  45. Chatwoot Privacy Policy - https://www.chatwoot.com/privacy-policy
  46. Chatwoot Security - https://www.chatwoot.com/security
  47. Chatwoot Terms of Service - https://www.chatwoot.com/terms-of-service
  48. HubSpot Why Choose - https://www.hubspot.com/why-choose-hubspot
  49. HubSpot Marketing Pricing - https://www.hubspot.com/pricing/marketing
  50. Shopify Online Selling - https://www.shopify.com/online
  51. Shopify Agentic Storefronts - https://www.shopify.com/agentic-storefronts
  52. Shopify POS - https://www.shopify.com/pos
  53. Shopify Shop App - https://www.shopify.com/shop
  54. Shopify Channels - https://www.shopify.com/channels
  55. Shopify International - https://www.shopify.com/international

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
