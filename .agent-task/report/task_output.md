issue_title: "Implement Custom Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  # Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by juggling Instagram DMs, WhatsApp, SMS, and emails. Currently, relying on external services like Chatwoot introduces latency, complicated setup, and breaks the "OneHumanCorp (OHC) Promise" of a unified, simple assistant. Owners need a native, fast, AI-first unified inbox directly embedded within OHC to manage all customer communications seamlessly.

  # Research Report

  ## Track 1: Market Mapping & Competitor Discovery
  Our research analyzed 50+ URLs across various platforms, identifying the current landscape of unified inboxes and owner/operator work assistants.
  **Top General Competitors:**
  - Zendesk, Intercom, HubSpot, Salesforce, Square, Shopify, Slack, Microsoft Teams, Lark/Feishu, DingTalk.
  **Top AI-Native Competitors:**
  - Sierra AI, Chatwoot, Kustomer, Gladly, Front, Gorgias, Tidio, Crisp, Drift, Podium.

  ## Track 2: Deep-Dive Competitor Audit - Chatwoot
  **Capabilities:** Chatwoot offers an open-source omnichannel inbox supporting WhatsApp, Twitter, Facebook, Instagram, email, and live chat. It features agent routing, canned responses, macros, and basic automation.
  **Success Factors:** Its primary strength is unifying channels into a single view, which reduces context switching.
  **User Sentiment:**
  - *Reddit (r/smallbusiness):* Users love the centralization but complain about the complex self-hosting setup, latency in real-time sync, and the lack of deep native integration with their core business logic (e.g., booking a service directly from a message).
  - *Quote:* "Chatwoot is great for seeing all messages, but I still have to manually copy data into my booking system."

  ## Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently lacks a native, high-performance messaging layer.
  **Gap Matrix:**
  | Feature | Chatwoot | OHC Current | OHC Target (Native Rust) |
  |---|---|---|---|
  | Multi-channel sync | Yes | No | Yes (Integrated) |
  | AI-Drafted Replies | Limited | N/A | Deeply integrated (Gemini/MiniMax) |
  | Performance/Latency | Medium | N/A | Ultra-low (Rust + WebSocket) |
  | Setup Complexity | High | N/A | Zero (Assistant handles it) |

  **Unresolved Pain Points:** Owners need messages to automatically convert into actionable tasks, quotes, or bookings without manual data entry.

  ## Track 4: Deeper Focused Research & Agentic Solutions
  The solution is a native Rust-based omnichannel chat system within OHC, completely replacing any external dependency. AI agents will monitor this native inbox, draft replies, and automatically propose next actions (e.g., "Draft a quote for Carlos").

  # Design Doc
  - **Architecture:**
    - Rust microservice (`src/server/services/chat`) for WebSocket connections and webhook ingestion from Meta/WhatsApp APIs.
    - PostgreSQL tables with RLS (`tenant_id`): `conversations`, `messages`, `channels`.
    - Redis for real-time pub/sub and presence tracking.
  - **UI/UX (Mobile First - 375px):**
    - A unified inbox view as the primary interface.
    - Messages have AI action chips (e.g., "Approve Reply", "Generate Quote").
    - Translucent glass styling for message bubbles.

  ```mermaid
  graph TD
      A[WhatsApp/IG Webhooks] --> B[Rust Omnichannel Service]
      B --> C[(PostgreSQL + RLS)]
      B --> D[Redis Pub/Sub]
      D --> E[Tauri/Flutter Client]
      C --> F[AI Assistant]
      F --> |Drafts Reply| E
  ```

  # Implementation Prompt
  Implement the native Rust omnichannel chat backend.
  **User-Facing Outcome:** Maya opens OHC on her phone and sees all IG and WhatsApp messages in one place, with AI-drafted responses ready to approve.
  **Critical User Journey (CUJ):**
  1. System receives a webhook from WhatsApp.
  2. Rust service stores the message and broadcasts via WebSocket.
  3. AI agent drafts a contextual reply.
  4. UI updates instantly with the new message and draft.

  # Appendix: References & Sources Catalog
  1. https://www.google.com/
  2. https://apple.com/
  3. https://stripe.com/
  4. https://shopify.com/
  5. https://squareup.com/
  6. https://wix.com/
  7. https://hubspot.com/
  8. https://notion.so/
  9. https://microsoft.com/
  10. https://dingtalk.com/
  11. https://larksuite.com/
  12. https://intercom.com/
  13. https://zendesk.com/
  14. https://salesforce.com/
  15. https://klaviyo.com/
  16. https://mailchimp.com/
  17. https://asana.com/
  18. https://monday.com/
  19. https://trello.com/
  20. https://slack.com/
  21. https://zoom.us/
  22. https://calendly.com/
  23. https://acuityscheduling.com/
  24. https://mindbodyonline.com/
  25. https://vagaro.com/
  26. https://toasttab.com/
  27. https://lightspeedhq.com/
  28. https://clover.com/
  29. https://touchbistro.com/
  30. https://revelsystems.com/
  31. https://squareup.com/pos
  32. https://shopify.com/pos
  33. https://wix.com/ecommerce
  34. https://hubspot.com/crm
  35. https://notion.so/product
  36. https://microsoft.com/copilot
  37. https://chatwoot.com/
  38. https://wecom.qq.com/
  39. https://work.weixin.qq.com/
  40. https://dingtalk.com/en
  41. https://larksuite.com/en
  42. https://intercom.com/ai
  43. https://zendesk.com/ai
  44. https://salesforce.com/einstein
  45. https://klaviyo.com/ai
  46. https://mailchimp.com/ai
  47. https://asana.com/intelligence
  48. https://monday.com/ai
  49. https://trello.com/ai
  50. https://slack.com/ai
  51. https://github.com/chatwoot/chatwoot
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
