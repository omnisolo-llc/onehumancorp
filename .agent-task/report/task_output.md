issue_title: "Implement High-Performance Native Rust Omnichannel Customer Chat Engine (Retire Chatwoot)"
issue_description: |
  # Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Priya (boutique operator) are overwhelmed by incoming requests across multiple distinct channels (Instagram DMs, WhatsApp, SMS, Web Chat, Email).
  Currently, OmniSolo/OHC is mandating the **RETIREMENT** of the external third-party Chatwoot service dependency.
  These owners currently lack a unified, native, high-performance omnichannel inbox that highlights which customer interactions need immediate attention and why. They need a system natively integrated into OHC (written in Rust) to triage these interactions, coordinate routing, enforce SLAs, and maintain a unified customer context without logging into multiple disparate tools or relying on an external Chatwoot deployment.

  # Research Report

  ## Market Context
  Platforms like Shopify Sidekick, HubSpot's Service Hub, and Chatwoot are built to help businesses streamline customer communication. However, third-party integrations (like relying on a standalone Chatwoot deployment) introduce latency, architectural complexity, data sovereignty issues, and fragmented UX.

  ## Competitor Analysis: HubSpot vs WeCom vs Chatwoot

  | Feature / Capability | Chatwoot (External) | HubSpot AI | WeCom | Proposed OHC Native (Rust) |
  |---|---|---|---|---|
  | **Core Value** | Omnichannel inbox, live chat, agent routing, macros. | Deep CRM integration, automated response drafting based on past sales context. | Strong enterprise chat, seamless WeChat bridging, deep local integrations. | Unified native omnichannel inbox built in high-performance Rust. |
  | **Success Factors** | Open-source, self-hostable. | Unified Customer View (see Shopify orders next to emails), Low Friction AI approval. | Real-Time Synchronicity, massive ecosystem integration. | No external dependency, instant real-time sync via WebSockets, tight OHC UI integration. |
  | **User Sentiment** | Users love open-source but complain about operational overhead (Ruby/Rails + Postgres + Redis). | Praised for CRM integration, complained about pricing and complexity. | Highly adopted in specific markets, heavy for small solo operators. | Owners need a simple, fast "Work Feed" without administrating separate chat servers. |

  ## OHC Gap Analysis & Unresolved Pain Points
  - **Gap 1**: OHC relies on Chatwoot, which is now deprecated. We lack a native omnichannel pipeline.
  - **Gap 2**: The UI lacks a "Work Feed" that unifies messages with business tasks (e.g., "Send Invoice").
  - **Gap 3**: AI drafting is disconnected from the native messaging context because messages live in a 3rd party system.

  ## Proposed Agentic Solution
  A native Rust-based omnichannel engine (`ohc-omni`) that ingests webhooks from external providers, standardizes them into `Message` and `Conversation` entities, and streams them via WebSockets to a Flutter-based unified Work Feed UI. An AI agent (`TriageAgent`) listens to new messages, queries the business's policy database, and drafts replies instantly.

  # Design Doc

  ## Architecture / Entities
  - `Contact`: The unified customer profile.
  - `Conversation`: A grouped thread of messages linked to a `Contact` and an `Inbox`.
  - `Message`: Individual communication units (inbound/outbound) with robust attachment handling.
  - `Inbox` / `Channel`: Natively implemented adapters for Web Widget, Email, SMS, WhatsApp, and Instagram.
  - `AutomationRule` / `SLA`: Rules engine for automated routing, tagging, and escalation.

  ## Visual & UX Flow
  ```mermaid
  graph TD
      A[Customer on IG] -->|Webhook| B(OHC Rust Gateway)
      B --> C{Triage Agent}
      C -->|Drafts Reply| D[OHC Database]
      B --> D
      D -->|WebSocket| E[Flutter Mobile App - Work Feed]
      E -->|Owner Taps Approve| F(OHC Rust Gateway)
      F -->|API Call| A
  ```

  ### Mobile First UX (375px)
  1. **The Feed**: 375px wide. Translucent glass effect header. Cards for each unread conversation prioritized by SLA.
  2. **Conversation View**: Shows the AI drafted reply in a slightly raised, highlighted container with a primary "Approve & Send" button. Clear indicators of which channel a message came from.

  # Implementation Prompt
  - **Retire Chatwoot**: Remove all external Chatwoot dependencies, API clients, and configuration from the OHC codebase.
  - **Backend (Rust)**: Implement the core omnichannel conversational models (`Contact`, `Conversation`, `Message`, `Inbox`, `Channel`) in native Rust. Ensure strict multi-tenant data isolation.
  - **Channel Adapters**: Build the foundational Rust channel adapters, starting with a native Web Chat Widget and Email integration.
  - **Real-time Engine**: Implement a robust Rust-based WebSocket server to broadcast real-time `message.created`, `conversation.updated` events to connected clients.
  - **Frontend (Flutter)**: Build the mobile-first (375px) Omnichannel Inbox UI to seamlessly connect to the new Rust backend, ensuring it feels like a native assistant command center. Ensure all loading states and error handling for the AI generation are smooth and transparent to the user.

  # Estimated Scope
  Large

  # References & Sources Catalog
  1. https://github.com/chatwoot/chatwoot - Chatwoot Source Code Repository
  2. https://www.hubspot.com/products/service - HubSpot Service Hub Features
  3. https://work.weixin.qq.com/ - WeCom Official Site
  4. https://www.shopify.com/magic - Shopify Magic and Sidekick
  5. https://www.zendesk.com/service/messaging/ - Zendesk Messaging
  6. https://intercom.com/ - Intercom Omnichannel
  7. https://front.com/ - Front App Collaborative Inbox
  8. https://gorgias.com/ - Gorgias E-commerce Helpdesk
  9. https://www.salesforce.com/products/service-cloud/overview/ - Salesforce Service Cloud
  10. https://kustomer.com/ - Kustomer CRM
  11. https://www.freshworks.com/freshchat/ - Freshchat
  12. https://www.zoho.com/desk/ - Zoho Desk
  13. https://www.twilio.com/en-us/flex - Twilio Flex
  14. https://messagebird.com/ - MessageBird Inbox
  15. https://www.gladly.com/ - Gladly Customer Service Platform
  16. https://www.trengo.com/ - Trengo Omnichannel Inbox
  17. https://crisp.chat/ - Crisp Multichannel Customer Support
  18. https://www.tawk.to/ - tawk.to Free Live Chat
  19. https://www.tidio.com/ - Tidio Live Chat & AI Chatbots
  20. https://www.livechat.com/ - LiveChat Software
  21. https://user.com/ - User.com Marketing Automation
  22. https://www.helpscout.com/ - Help Scout
  23. https://www.kayako.com/ - Kayako Help Desk
  24. https://www.groovehq.com/ - Groove Shared Inbox
  25. https://missiveapp.com/ - Missive Team Inbox
  26. https://www.dixa.com/ - Dixa Customer Service Platform
  27. https://www.re-amaze.com/ - Reamaze Helpdesk
  28. https://www.richpanel.com/ - Richpanel
  29. https://suported.com/ - Supported Customer Service
  30. https://www.reply.io/ - Reply Sales Engagement
  31. https://respond.io/ - Respond.io Customer Conversation Platform
  32. https://www.getcontrol.co/ - Control Customer Management
  33. https://www.omniloop.io/ - Omniloop Support
  34. https://trustpilot.com/review/www.hubspot.com - HubSpot Reviews on Trustpilot
  35. https://trustpilot.com/review/www.shopify.com - Shopify Reviews on Trustpilot
  36. https://reddit.com/r/smallbusiness - Reddit Small Business Community
  37. https://reddit.com/r/Entrepreneur - Reddit Entrepreneur Community
  38. https://reddit.com/r/ecommerce - Reddit E-commerce Community
  39. https://reddit.com/r/SaaS - Reddit SaaS Community
  40. https://news.ycombinator.com/ - Hacker News (Omnichannel discussions)
  41. https://appstore.com/ - Apple App Store (Competitor App Reviews)
  42. https://play.google.com/store/apps - Google Play Store (Competitor App Reviews)
  43. https://www.g2.com/categories/help-desk - G2 Help Desk Software Reviews
  44. https://www.capterra.com/help-desk-software/ - Capterra Help Desk Reviews
  45. https://www.softwareadvice.com/crm/help-desk-software-comparison/ - Software Advice
  46. https://www.gartner.com/en/customer-service-support - Gartner Customer Service Research
  47. https://www.forrester.com/bold - Forrester Customer Experience Reports
  48. https://techcrunch.com/category/enterprise/ - TechCrunch Enterprise SaaS News
  49. https://www.protocol.com/enterprise - Protocol Enterprise Coverage
  50. https://www.saastr.com/ - SaaStr SaaS Business Trends
  51. https://stripe.com/docs - Stripe API for Payment Integrations (Reference)
  52. https://discord.com/developers/docs - Discord API for Webhook Architecture Reference
  53. https://developers.facebook.com/docs/whatsapp - WhatsApp Business API Docs

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
