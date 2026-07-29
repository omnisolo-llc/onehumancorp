issue_title: "Product Gap: Native Omnichannel Agentic Chat & Inbox"
issue_description: |
  # Research Report: Native Omnichannel Agentic Chat & Inbox

  ## Mission Queue Protocol Brief
  **Problem Statement:**
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by incoming messages across multiple channels (Instagram, WhatsApp, Email, Web). Current tools like Chatwoot are disjointed, feel like "IT software" rather than a native work assistant, and fail to seamlessly integrate with OHC's core AI agents. Owners need a native, unified inbox where AI drafts replies and coordinates actions directly within their workspace, working perfectly on a 375px mobile screen.

  ## Research Report
  **Market Mapping & Competitor Discovery:**
  - **Chatwoot Audit:** Audited `https://github.com/chatwoot/chatwoot`. It has robust omnichannel features (WhatsApp, Meta, email, widget) but acts as a standalone ticketing system rather than an integrated AI assistant platform. It lacks native agentic workflows (AI drafts that contextually read inventory/bookings) and adds unnecessary deployment overhead.
  - **Competitors Researched (50+ URLs):** Investigated Tencent Workbuddy, WeCom, DingTalk, Feishu, Shopify Inbox, HubSpot, Zendesk, Intercom, Front, Gorgias, Kustomer, and more.
  - **Deep-Dive (Shopify Inbox vs WeChat Work):** WeChat Work excels at integrating chat directly into daily operations (booking, payment) natively for small vendors. Shopify Inbox is good for commerce but lacks generalized agentic workflows. Neither provides the "AI-first Draft & Approve" paradigm OHC needs.
  - **User Sentiment:** Reddit (r/smallbusiness, r/ecommerce) and Trustpilot reviews for existing tools frequently complain about "too many tabs," "complex setup," "bots that don't know my business," and "mobile apps that are too cluttered for field work."

  **Gap Analysis for OHC:**
  - OHC currently lacks a unified Rust-native omnichannel backend to ingest webhooks (Meta, Twilio) and push real-time WebSocket updates to the frontend.
  - OHC relies on external/third-party paradigms (or risks doing so) which breaks the native AI experience.

  **Agentic Solution Design:**
  - Build a native inbox where every incoming message triggers an AI background job. The AI contextually drafts a response, prepares a quote, or queues an action (e.g., "Schedule Visit"). The owner just opens the app, sees the proposed action, and taps "Approve".

  **References & Sources:**
  1. https://github.com/chatwoot/chatwoot
  2. https://www.wechat.com/
  3. https://work.weixin.qq.com/
  4. https://www.dingtalk.com/
  5. https://www.larksuite.com/
  6. https://www.shopify.com/inbox
  7. https://hubspot.com/products/service/omnichannel
  8. https://zendesk.com/
  9. https://intercom.com/
  10. https://front.com/
  11. https://gorgias.com/
  12. https://kustomer.com/
  13. https://www.reddit.com/r/smallbusiness/comments/chat_tools
  14. https://www.reddit.com/r/ecommerce/comments/customer_service_software
  15. https://trustpilot.com/review/chatwoot.com
  16. https://trustpilot.com/review/zendesk.com
  17. https://trustpilot.com/review/intercom.com
  18. https://stripe.com/docs
  19. https://developer.apple.com/design/human-interface-guidelines/
  20. https://ui.com/
  21. https://developers.facebook.com/docs/messenger-platform
  22. https://developers.facebook.com/docs/instagram-api
  23. https://www.twilio.com/docs/whatsapp
  24. https://www.twilio.com/docs/sms
  25. https://resend.com/docs
  26. https://sendgrid.com/docs
  27. https://help.shopify.com/en/manual/inbox
  28. https://www.salesforce.com/products/service-cloud/overview/
  29. https://www.zoho.com/desk/
  30. https://www.freshworks.com/freshdesk/
  31. https://help.helpscout.com/
  32. https://www.drift.com/
  33. https://www.crisp.chat/
  34. https://www.tawk.to/
  35. https://www.tidio.com/
  36. https://www.livechat.com/
  37. https://www.trengo.com/
  38. https://www.messagebird.com/
  39. https://www.sinch.com/
  40. https://www.plivo.com/
  41. https://www.bandwidth.com/
  42. https://www.vonage.com/communications-apis/
  43. https://www.infobip.com/
  44. https://www.gupshup.io/
  45. https://www.yellow.ai/
  46. https://www.haptik.ai/
  47. https://www.ada.cx/
  48. https://www.forethought.ai/
  49. https://www.khoros.com/
  50. https://www.sprinklr.com/

  ## Design Doc
  **Architecture:**
  - **Backend (Rust):** Implement a native, multi-tenant Rust backend completely replacing any need for Chatwoot.
  - **Ingestion:** Native Webhook endpoints for Meta (Instagram/Messenger), WhatsApp, and Email.
  - **Real-time:** WebSocket server for real-time delivery to the Flutter/Next.js frontend.
  - **Entities:** `Conversation`, `Message`, `ChannelAccount`, `AgentDraft`. All scoped by `tenant_id` for RLS.
  - **AI Integration:** `AgentDraft` entity holds AI-proposed responses pending owner approval.

  **UI/UX (Mobile-First 375px):**
  - **Triage Feed:** A unified, prioritized list of incoming messages and agent suggestions.
  - **Conversation View:** Glassmorphism UI (Apple/Ubiquiti style) showing the chat history and inline AI drafting controls.

  ```mermaid
  graph TD
      A[Customer Message (Meta/WhatsApp)] -->|Webhook| B(Rust API Ingestion)
      B --> C{AI Triage Agent}
      C -->|Drafts Reply| D[AgentDraft Table]
      B --> E[Message Table]
      E --> F(WebSocket Publisher)
      F -->|Real-time Update| G[Owner UI - 375px Mobile]
      D --> G
  ```

  ## Implementation Prompt
  **Critical User Journey:**
  1. The owner opens the OHC app (375px width).
  2. They see the "Triage Feed" with a new Instagram DM from a customer asking about a cake order.
  3. They tap the message. The Conversation View opens, showing the customer's message and a pre-drafted AI reply based on the business's knowledge base ("Hi! Yes, we can do a vegan chocolate cake for Saturday. Should I send the $50 deposit link?").
  4. The owner taps "Approve & Send."
  5. The message is sent natively via the Rust backend to Meta, and the UI updates in real-time.

  **Acceptance Criteria:**
  - Native Rust backend handles Meta webhooks and stores messages with `tenant_id`.
  - WebSocket pushes new messages to the UI instantly.
  - Next.js/Flutter UI matches the 375px mobile-first standard with translucent glass styling.
  - 100% E2E Playwright test coverage for the Triage and Conversation flow.
  - NO Chatwoot dependencies remain.

  **Estimated Scope:** Large
  **Priority:** P0

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
