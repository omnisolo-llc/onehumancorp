issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners need to manage inquiries across multiple channels (Instagram DMs, WhatsApp, SMS, Email). Previously, OHC relied on an external third-party service, Chatwoot. As per OHC Engineering Standards, Chatwoot as an external service is 100% RETIRED. Small businesses cannot rely on disconnected external tools that add complexity and break the "unified assistant" experience. We need a native, high-performance omnichannel chat system built in Rust within the `onehumancorp/mono` repository to replace Chatwoot, fully integrating with OHC's AI agents.

  # Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Code Audit:** I reviewed Chatwoot's architecture (`https://github.com/chatwoot/chatwoot`). It relies on Ruby on Rails, PostgreSQL, Redis, and WebSockets for real-time communication. Key features include omnichannel adapters, agent routing, canned responses, and SLAs.
  - **Shopify Inbox / Wix Inbox:** These platforms aggregate messages but fail to provide true autonomous agentic drafting out of the box. They act as passive tools rather than active assistants.
  - **Intercom / Zendesk:** Too complex and expensive for SMBs, requiring dedicated customer support teams.
  - **OHC Native Rust Solution:** By building natively in Rust, OHC can leverage high concurrency and memory safety, essential for managing thousands of WebSocket connections per tenant. This native system will directly hook into OHC's Event Mesh and "The Ambassador" AI agent, enabling proactive draft generation (Read-Approve vs Read-Reply) without leaving the OHC ecosystem.

  **Top 10 General Competitors:**
  1. Shopify Inbox
  2. Wix Inbox
  3. Squarespace (Basic Email integration)
  4. HubSpot Service Hub
  5. Intercom
  6. Zendesk
  7. Freshchat
  8. Salesforce Service Cloud
  9. GoDaddy (Unified Inbox)
  10. WeCom (Tencent)

  **Top 10 AI-Native Competitors:**
  1. Intercom Fin
  2. 11x.ai (Alice/Julian)
  3. Lindy.ai
  4. Sierra
  5. Decagon
  6. Maven AGI
  7. Kustomer (AI features)
  8. Forethought
  9. Rasa
  10. Ada

  **Deep-Dive Competitor Audit (Chatwoot & Intercom Fin):**
  - **Capabilities:** Chatwoot excels at channel aggregation (WhatsApp, IG, Email, Web Widget) and human-agent routing. Intercom Fin excels at autonomous resolution.
  - **Success Factors:** Chatwoot's open-source nature allowed deep customization. Intercom Fin's success comes from its RAG capabilities, resolving 50%+ of queries.
  - **User Sentiment (Chatwoot):** Users love the omnichannel inbox but complain about the heavy resource usage of Rails/Sidekiq and complex self-hosting setup.
  - **OHC Opportunity:** Combine Chatwoot's omnichannel aggregation with Intercom Fin's agentic resolution, built natively in Rust for performance and simplicity, fully managed within the OHC platform.

  ### Competitive Comparison Table
  | Feature | Chatwoot | Shopify Inbox | Intercom Fin | **OHC (Target)** |
  | :--- | :--- | :--- | :--- | :--- |
  | **Omnichannel Aggregation** | Yes | Yes (Basic) | Yes | **Yes (Native Rust)** |
  | **Autonomous Resolution** | No | No | Yes (High Cost) | **Yes (Agentic)** |
  | **Setup Complexity** | High (Rails/Redis) | Low | Medium | **Zero-Click Onboarding** |
  | **SMB Focus** | Medium | High | Low (Enterprise) | **High (Owner-first)** |
  | **Agent Proactive Drafts** | No | No | Yes | **Yes (Read-Approve)** |

  ### Persona-Specific Pain Point Summary
  1. **Maya (Home Baker):** Receives cake inquiries via IG DMs. *Pain Point:* Currently manually tracking DMs and missing orders while baking. *Solution:* The Agentic Inbox drafts replies recognizing the customer's previous orders and provides an "Approve" button, keeping her hands free.
  2. **Carlos (Field Service Owner):** Gets quotes via SMS and WhatsApp. *Pain Point:* Loses 30% of leads while driving or on a job site. *Solution:* The Ambassador agent negotiates quotes and secures deposits autonomously while he works.
  3. **Priya (Boutique Operator):** Needs to answer product availability questions. *Pain Point:* Inbox is disconnected from POS inventory. *Solution:* The agent checks live POS stock before drafting a reply about size availability.
  4. **Leo (Creator and Tutor):** *Pain Point:* Cannot easily convert casual DM chats into recurring lesson subscriptions. *Solution:* The agent can recognize intent and share direct Stripe checkout links within the same chat.
  5. **Fatima (Food Cart Operator):** *Pain Point:* App interfaces are in English and not designed for slow mobile networks. *Solution:* The Rust backend will offer highly responsive, low-latency WebSocket connections ensuring the mobile UI updates instantly.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Channels: IG, WA, Email, Web] -->|Webhooks/WS| B(Rust Omnichannel Gateway)
      B --> C{Rust WebSocket Server - Tokio/Tungstenite}
      C <--> D[Frontend Flutter Shell 375px]
      B --> E[Identity Resolution Engine]
      E --> F[Unified Customer Graph DB]
      B --> G[OHC Event Mesh]
      G --> H[The Ambassador AI Agent]
      H -->|Drafts Reply| F
      F --> C
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Feed View:** An "Unresolved Conversations" section showing incoming messages.
  - **Conversation View:** A chat interface. If "The Ambassador" has drafted a reply, a translucent glass-styled card floats above the input box showing the draft with "Approve" and "Edit" buttons.
  - **Web Widget:** A lightweight, customizable script that tenants can embed on external sites, communicating directly with the Rust WebSocket server.

  ### Key Entities
  - `Conversation`: Links messages to a Customer and Tenant.
  - `Message`: Represents a single chat message (incoming or outgoing).
  - `Channel`: The source/destination (e.g., `Instagram`, `WebWidget`).
  - `AgentDraft`: Links an AI-generated draft to a specific Conversation.

  # Implementation Prompt
  **User-Facing Outcome:** A tenant owner can view and respond to messages from their website widget, Instagram, and WhatsApp directly within the OHC mobile or web app. AI automatically drafts responses for them to approve.
  **Critical User Journey & Acceptance Criteria:**
  1. Implement a Rust-based WebSocket server (using `tokio` and `tungstenite` or similar) in the backend to handle real-time chat connections.
  2. Create backend models and endpoints for `Conversation`, `Message`, and `Channel`.
  3. Implement a simple web chat widget (HTML/JS) that connects to this Rust WebSocket server.
  4. Integrate the incoming message event with the existing "Ambassador" agent logic to generate `AgentDraft`s.
  5. The Flutter frontend must display incoming messages in real-time via WebSockets and show AI drafts for 1-tap approval.
  6. Provide comprehensive Playwright E2E tests simulating a customer message from the widget, the agent drafting a reply, and the owner approving it.

  **Priority:** P0
  **Estimated Scope:** Large

  # References & Sources
  1. https://github.com/chatwoot/chatwoot
  2. https://www.shopify.com/inbox
  3. https://www.wix.com/inbox
  4. https://www.hubspot.com/products/service/shared-inbox
  5. https://www.intercom.com/fin
  6. https://www.zendesk.com/
  7. https://www.11x.ai/
  8. https://www.lindy.ai/
  9. https://sierra.ai/
  10. https://decagon.ai/
  11. https://mavenagi.com/
  12. https://www.kustomer.com/
  13. https://forethought.ai/
  14. https://rasa.com/
  15. https://www.ada.cx/
  16. https://www.salesforce.com/products/service-cloud/overview/
  17. https://www.freshworks.com/freshchat/
  18. https://www.godaddy.com/help/what-is-the-unified-inbox-28198
  19. https://work.weixin.qq.com/
  20. https://www.ycombinator.com/companies/decagon
  21. https://techcrunch.com/2024/02/14/sierra-ai-customer-service-agent/
  22. https://www.bloomberg.com/news/articles/2024-06-18/ai-startup-maven-agi-raises-20-million-to-automate-customer-support
  23. https://www.reddit.com/r/smallbusiness/comments/16kzwm1/best_shared_inbox_for_small_business/
  24. https://www.reddit.com/r/SaaS/comments/18wq55v/chatwoot_vs_intercom_for_startup/
  25. https://www.trustpilot.com/review/www.intercom.com
  26. https://www.trustpilot.com/review/chatwoot.com
  27. https://github.com/chatwoot/chatwoot/issues
  28. https://docs.rs/tokio/latest/tokio/
  29. https://docs.rs/tungstenite/latest/tungstenite/
  30. https://tokio.rs/tokio/tutorial/channels
  31. https://www.shopify.com/partners/blog/shopify-inbox
  32. https://support.wix.com/en/article/wix-inbox-an-overview
  33. https://www.intercom.com/blog/ai-customer-service/
  34. https://www.11x.ai/blog/digital-workers
  35. https://lindy.ai/features
  36. https://sierra.ai/product
  37. https://decagon.ai/platform
  38. https://mavenagi.com/product
  39. https://www.kustomer.com/platform/ai/
  40. https://forethought.ai/platform/
  41. https://rasa.com/docs/
  42. https://www.ada.cx/platform/ai-agent
  43. https://www.salesforce.com/products/einstein/overview/
  44. https://www.freshworks.com/freshchat/ai-chatbot/
  45. https://github.com/tokio-rs/tokio
  46. https://github.com/snapview/tungstenite-rs
  47. https://doc.rust-lang.org/book/ch16-00-concurrency.html
  48. https://www.youtube.com/watch?v=J5aKMB6nN1I (Chatwoot overview)
  49. https://www.youtube.com/watch?v=7h2Y9rW1Y1c (Intercom Fin overview)
  50. https://news.ycombinator.com/item?id=38133596 (Discussion on AI customer support)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
