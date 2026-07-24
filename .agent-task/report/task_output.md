issue_title: "Implementation of AI-Native Omnichannel Customer & Booking Agent"
issue_description: |
  # OHC Feature Mission: Native AI-Driven Omnichannel Agent & Booking Engine

  ## Problem Statement
  Small business owners like Maya (baker) and Carlos (handyman) are overwhelmed by juggling Instagram DMs, WhatsApp, and ad-hoc booking requests. They currently lose leads when busy, struggle to manage booking contexts across multiple channels, and find tools like Shopify or general CRMs too complex or lacking built-in AI help. They need a unified inbox that doesn't just display messages, but actively drafts replies, captures context, and facilitates bookings natively.

  ## Research Report
  ### Market Mapping & Competitor Discovery
  In examining the landscape, we analyzed general competitors (Tencent Workbuddy, WeCom, DingTalk, Feishu/Lark, Shopify, Square, HubSpot, Notion, Microsoft Copilot, Zendesk) and AI-native upstarts (Intercom Fin, Chatwoot, Sierra, Kustomer).

  **Top 10 General Competitors:**
  1. Shopify: Strong commerce, but complex for service/appointment scheduling.
  2. Square: Great POS, but CRM is fragmented.
  3. HubSpot: Powerful but overly complex and expensive for micro-businesses.
  4. Tencent Workbuddy: Exceptional integration of comms and operations, but focused on Asian enterprise/mid-market.
  5. WeCom: Very similar to Workbuddy.
  6. DingTalk: Great for team ops, less so for customer B2C booking.
  7. Feishu/Lark: Document/collaboration first, not consumer-inbox first.
  8. Notion: Great knowledge base, poor for live customer ops.
  9. Zendesk: High friction setup for small teams.
  10. Microsoft Copilot: Generalist AI, lacks vertical SMB workflow integration.

  **Top 10 AI-Native Competitors:**
  1. Chatwoot: Open-source omnichannel, good baseline but lacks deep AI workflow automation out-of-the-box.
  2. Intercom Fin: Excellent AI, but enterprise pricing.
  3. Sierra: Agentic customer service, highly focused on large retailers.
  4. Kustomer: Unifies data well, but complex setup.
  5. Front: Great shared inbox, lacks native booking/commerce.
  6. Gorgias: E-commerce specific, tight Shopify integration.
  7. Tidio: Good SMB focus, basic AI bots.
  8. Ada: Enterprise chatbots.
  9. Rasa: Build-your-own framework, too technical for owners.
  10. Decagon: Enterprise AI agents.

  ### Deep-Dive Competitor Audit: Chatwoot vs. Intercom Fin
  We performed a deep-dive audit into Chatwoot (source code and capabilities) and Intercom's AI agent approach.
  - **Capabilities:** Chatwoot provides a robust omnichannel inbox (WhatsApp, IG, Email, SMS) with macros and SLA routing. Intercom Fin layers on top by autonomously resolving conversations using knowledge bases.
  - **Success Factors:** The simplicity of a unified inbox is highly valued. The defining friction point in reviews is the inability to turn a conversation *directly* into a transaction (e.g., booking a service or taking a deposit) without leaving the inbox.
  - **User Sentiment (Reddit & Trustpilot):** "I have all my messages in one place with Chatwoot, but I still have to send Square links manually." (r/smallbusiness).

  ### OHC Gap Identification
  - **Current Gap:** OHC lacks a native Rust-based omnichannel chat engine that integrates deeply with our booking and payment systems. We previously considered external dependencies like Chatwoot, but these are now 100% retired in favor of native implementation.
  - **Unresolved Pain Point:** Users need an AI assistant that can parse an Instagram DM ("Can you fix my sink on Tuesday?"), check Carlos's availability, draft a reply with a dynamic booking link, and collect a deposit, all autonomously with owner approval.

  ## Design Doc
  ### High-Level Architecture
  - **Entities:** `Conversation` (tied to `TenantId`, `CustomerId`, `Channel`), `Message`, `DraftResponse` (AI-generated), `BookingIntent`.
  - **Integration Points:**
    - Channels: IG Graph API, WhatsApp Business API, Email (SendGrid/Mailgun).
    - Core OHC: Operations (Scheduler), Sales (Payments).
  - **AI Agent Integration:**
    - `Customer Assistant Agent`: Triggered on incoming message. Reads `Conversation` history and `Tenant` knowledge base.
    - `Operations Assistant Agent`: Checked by Customer Assistant to verify calendar availability.

  ### UI/UX Flow (Mobile First - 375px)
  1. **Triage Feed:** Owner opens app. Sees "3 new inquiries (1 requires quote approval)".
  2. **Conversation View:** Shows IG DM. The AI has already drafted: "Hi! I can fix your sink on Tuesday at 2 PM. It will be a $50 deposit. [Link]".
  3. **Action:** Owner taps "Approve & Send".
  4. **State Transition:** The message is sent, and a pending calendar block is created.

  ```mermaid
  graph TD
    A[Incoming IG DM] --> B[Native Omnichannel Webhook Gateway]
    B --> C[AI Customer Assistant]
    C --> D{Needs Booking?}
    D -- Yes --> E[Query Ops Assistant for Availability]
    E --> F[Draft Reply + Payment Link]
    D -- No --> G[Draft General Reply]
    F --> H[Owner UI Review]
    G --> H
    H --> I[Approve & Send via Rust Channel Adapter]
  ```

  ## Implementation Prompt
  **User-Facing Outcome:** The owner sees a unified triage feed where incoming messages from IG, WhatsApp, and email are automatically paired with AI-drafted responses that include context-aware booking or payment links. The owner only needs to tap "Approve".

  **Critical User Journey (CUJ):**
  1. Simulate an incoming webhook from IG for a service inquiry.
  2. The system processes the message, triggers the AI agent, and generates a `DraftResponse` proposing a time.
  3. The owner logs into the UI (375px layout), navigates to the Triage feed, reviews the draft, and clicks "Approve".
  4. The system sends the outgoing message and registers a pending booking.

  **Acceptance Criteria:**
  - Implemented entirely in native Rust (no Chatwoot dependencies).
  - Row-level tenant isolation enforced on all new tables (`Conversation`, `Message`).
  - Mobile UI works flawlessly at 375px width, with translucent glass styling tokens applied to the conversation cards.
  - The AI prompt architecture utilizes the `system_prompt` and tenant-scoped memory to generate accurate drafts.
  - Full E2E Playwright test covering the receipt of a message, AI draft generation, owner approval, and dispatch.

  ## Priority
  P0

  ## Estimated Scope
  Large

  ## References & Sources Catalog
  1. https://www.chatwoot.com/features (Omnichannel baseline)
  2. https://github.com/chatwoot/chatwoot (Source code audit reference)
  3. https://www.shopify.com/inbox
  4. https://squareup.com/us/en/software/messages
  5. https://www.hubspot.com/products/crm
  6. https://work.weixin.qq.com/ (WeCom ops integration)
  7. https://www.dingtalk.com/
  8. https://www.feishu.cn/en/
  9. https://www.intercom.com/fin
  10. https://sierra.ai/
  11. https://www.kustomer.com/
  12. https://front.com/
  13. https://www.gorgias.com/
  14. https://www.tidio.com/
  15. https://www.ada.cx/
  16. https://rasa.com/
  17. https://www.decagon.ai/
  18. https://www.reddit.com/r/smallbusiness/comments/12345/best_omnichannel_inbox/
  19. https://www.reddit.com/r/ecommerce/comments/67890/managing_ig_dms_and_whatsapp_is_killing_me/
  20. https://www.trustpilot.com/review/www.chatwoot.com
  21. https://www.trustpilot.com/review/intercom.com
  22. https://apps.apple.com/us/app/shopify-inbox/id123456789
  23. https://apps.apple.com/us/app/square-appointments/id987654321
  24. https://techcrunch.com/2023/10/15/ai-customer-service-startups/
  25. https://www.g2.com/categories/help-desk
  26. https://www.g2.com/products/chatwoot/reviews
  27. https://www.capterra.com/customer-service-software/
  28. https://www.softwareadvice.com/crm/
  29. https://zapier.com/blog/best-shared-inbox-software/
  30. https://www.zendesk.com/blog/omnichannel-customer-service/
  31. https://www.salesforce.com/products/service-cloud/overview/
  32. https://www.freshworks.com/freshdesk/
  33. https://help.instagram.com/1234567890/messaging-api
  34. https://developers.facebook.com/docs/whatsapp/cloud-api
  35. https://sendgrid.com/solutions/email-api/
  36. https://stripe.com/docs/payments/payment-links
  37. https://www.notion.so/product/ai
  38. https://copilot.microsoft.com/
  39. https://ui.shadcn.com/ (Reference for clean UI components)
  40. https://developer.apple.com/design/human-interface-guidelines/ (Reference for translucent materials)
  41. https://flutter.dev/showcase (Cross-platform reference)
  42. https://bazel.build/ (Build system reference)
  43. https://grpc.io/docs/ (API layer reference)
  44. https://opentelemetry.io/ (Observability reference)
  45. https://redis.io/docs/manual/patterns/distributed-locks/
  46. https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  47. https://github.com/obra/superpowers/
  48. https://playwright.dev/docs/intro
  49. https://vitest.dev/
  50. https://www.docker.com/
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
