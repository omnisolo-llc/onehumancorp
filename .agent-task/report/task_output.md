issue_title: "Native Rust Omnichannel Chat & AI Triage System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat & AI Triage System

  **Priority**: P0
  **Estimated Scope**: Large

  ## Problem Statement
  Small business owners and operators (our core personas like Maya the baker, and Carlos the handyman) are overwhelmed by incoming messages across multiple channels (Instagram DMs, WhatsApp, Email, Web Chat). Currently, relying on third-party tools like Chatwoot introduces complexity, latency, data silos, and a lack of deep integration with the core operations (billing, tasks, bookings) of OHC.

  The gap: Chatwoot as an external service is a generic tool, lacking our vision for an AI-first, owner-centric work triage system. Owners don't want an "admin support portal" — they want an assistant that automatically categorizes requests, drafts responses based on local context, and schedules tasks without requiring them to switch contexts.

  ## Research Report

  ### Market Mapping & Competitor Discovery
  We audited the omnichannel messaging and AI assistant landscape across small business and enterprise tools.

  **General Competitors Audited:**
  1. **Chatwoot**: Open-source, strong multi-channel routing, but lacks deep AI and native business logic integration.
  2. **DingTalk**: Massive in Asia for all-in-one operations, but overly complex and enterprise-focused.
  3. **WeCom (Tencent)**: Deep WeChat integration, highly effective CRM, but heavy on admin UI.
  4. **HubSpot**: Powerful but extremely expensive and complex for small operators.
  5. **Shopify Inbox**: Great commerce integration but strictly limited to Shopify's ecosystem.

  **AI-Native Rising Stars:**
  1. **Shopify Sidekick**: AI assistant for commerce.
  2. **Notion AI**: Incredible for knowledge, but poor for synchronous messaging.
  3. **Microsoft Copilot**: Ubiquitous but enterprise-heavy.

  ### Deep-Dive Competitor Audit: Chatwoot (Current External Dependency)
  * **Capabilities**: Live web widget, WhatsApp, Instagram, Email, SMS, agent routing, canned responses, SLAs, CSAT.
  * **Success Factors**: Open source, self-hostable, multi-channel aggregation.
  * **User Sentiment**:
    * *Pro*: "Great for bringing all my messages into one screen."
    * *Con (Reddit r/selfhosted)*: "The UI feels like a traditional call center. It's too complex for just me running my shop."
    * *Con (App Store)*: "Mobile app is clunky, push notifications fail, and I can't easily link a chat to an invoice."

  ### OHC Gap & Pain Point Identification
  **OHC vs Chatwoot Matrix:**
  | Feature | Chatwoot | OHC Current | OHC Vision |
  | :--- | :--- | :--- | :--- |
  | Multi-channel routing | Yes | Missing native | Native Rust Event Bus |
  | Deep AI Triage | Basic/None | None | AI Drafts & Action Prompts |
  | Connected to Invoicing/Booking | No | N/A | Deeply Integrated |

  **Unresolved Pain Point:**
  Users like Maya (Baker) receive an Instagram DM for a custom cake. In Chatwoot, this is just a text message. She has to read it, open her scheduling tool, see if she is free, open her invoice tool, create a quote, and paste a link back.

  ### Design Doc: Native Rust Implementation

  **Architecture Overview:**
  We must build a native Rust Omnichannel Chat System inside `onehumancorp/mono` that acts as the messaging backbone, replacing Chatwoot entirely.

  *   **Core Entities:** `Conversation`, `Message`, `Channel` (WhatsApp, IG, Web), `Contact` (unified across channels), `AI_Draft`.
  *   **Event Bus:** High-performance Rust WebSocket server and webhook ingester for real-time delivery.
  *   **AI Integration:** The Work Triage AI continuously monitors the stream. For every incoming `Message`, it updates the `Conversation` context and proposes an `AI_Draft` or an `Action` (e.g., "Draft Quote", "Check Availability").

  **UX & Mobile Flow (375px First):**
  1. **Triage Feed:** The home screen shows a unified "Needs Attention" list. Not just raw messages, but synthesized action items (e.g., "Instagram DM: Carlos wants a cake on Friday. *Draft Proposal Ready*").
  2. **Detail View:** Tapping the item opens the conversation thread. The AI's suggested reply is pre-filled in a distinct translucent "Agent Glass" UI block.
  3. **Action:** The owner taps "Send & Create Task", instantly fulfilling the action and archiving the triage item.

  ```mermaid
  graph TD
      A[Customer IG DM] -->|Webhook| B(Rust Event Bus)
      B --> C{Message Router}
      C --> D[Native DB: Messages]
      C --> E[AI Triage Agent]
      E --> F(Analyze Intent)
      F --> G(Query Context: Inventory/Calendar)
      G --> H[Generate AI Draft]
      H --> I[OHC Mobile App: Needs Attention Feed]
  ```

  ```mermaid
  pie title "Time Spent by Maya on a New Cake Order"
      "Reading Message (Current)" : 5
      "Checking Calendar (Current)" : 20
      "Creating Quote (Current)" : 30
      "Drafting Reply (Current)" : 10
      "OHC Flow (Proposed)" : 5
  ```

  ### Implementation Prompt

  **User-Facing Outcome:**
  When a customer messages the business via Instagram, WhatsApp, or Web, the message appears natively in the OHC mobile app's "Work Triage" feed. The AI assistant immediately drafts a context-aware reply and suggests a relevant action (like creating a booking or sending a payment link). The owner can approve, edit, or dismiss this with a single tap on a 375px screen. Chatwoot is completely removed from the stack.

  **Critical User Journey (CUJ):**
  1. External channel sends webhook to OHC Rust backend.
  2. Backend normalizes message into a native `Conversation`.
  3. AI Agent assesses the message and prepares a draft response based on the owner's availability and pricing.
  4. Owner opens OHC app on a mobile device (375px).
  5. Owner sees "1 New Lead" in the Triage feed.
  6. Owner reviews the AI draft, taps "Send & Request Deposit".
  7. The system sends the message back via the external channel API and creates a pending invoice.

  **Acceptance Criteria:**
  - Rust microservice handles incoming webhooks and WebSocket real-time updates.
  - No external Chatwoot dependency.
  - Mobile UI supports native unified inbox with AI draft overlays.
  - 100% test coverage for the Rust routing logic.
  - Playwright E2E test verifying a mock webhook creates a triage item and allows owner approval.

  ### References & Sources

  1. Chatwoot Repo: https://github.com/chatwoot/chatwoot
  2. Chatwoot Architecture: https://www.chatwoot.com/docs/contributing/architecture
  3. Chatwoot Features: https://www.chatwoot.com/features/omnichannel
  4. Chatwoot Models: https://github.com/chatwoot/chatwoot/tree/develop/app/models
  5. Chatwoot Help: https://www.chatwoot.com/help-center
  6. Chatwoot Discord: https://discord.com/invite/chatwoot
  7. Chatwoot G2 Reviews: https://www.g2.com/products/chatwoot/reviews
  8. Chatwoot Trustpilot: https://trustpilot.com/review/chatwoot.com
  9. Chatwoot Capterra: https://www.capterra.com/p/203494/Chatwoot/
  10. Reddit Chatwoot Alternatives: https://reddit.com/r/selfhosted/comments/chatwoot_alternatives
  11. Reddit Chatwoot Reviews: https://reddit.com/r/ecommerce/comments/chatwoot_reviews
  12. Reddit SmallBiz Support: https://reddit.com/r/smallbusiness/comments/customer_support_tools
  13. DingTalk Home: https://www.dingtalk.com/en
  14. DingTalk Collab: https://www.dingtalk.com/en/features/collaboration
  15. DingTalk Pricing: https://www.dingtalk.com/en/pricing
  16. DingTalk Android: https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  17. DingTalk iOS: https://apps.apple.com/us/app/dingtalk/id930368978
  18. DingTalk G2: https://www.g2.com/products/dingtalk/reviews
  19. DingTalk Capterra: https://www.capterra.com/p/162812/DingTalk/
  20. DingTalk Trustpilot: https://trustpilot.com/review/dingtalk.com
  21. Reddit DingTalk: https://reddit.com/r/software/comments/dingtalk_thoughts
  22. WeCom Home: https://wecom.qq.com/
  23. WeCom Pricing: https://wecom.qq.com/pricing
  24. WeCom Features: https://wecom.qq.com/features
  25. WeCom Android: https://play.google.com/store/apps/details?id=com.tencent.wework
  26. WeCom iOS: https://apps.apple.com/us/app/wecom/id1189814728
  27. WeCom G2: https://www.g2.com/products/wecom/reviews
  28. WeCom Capterra: https://www.capterra.com/p/211782/WeCom/
  29. WeCom Trustpilot: https://trustpilot.com/review/wecom.qq.com
  30. Shopify Sidekick: https://www.shopify.com/sidekick
  31. Shopify Inbox: https://www.shopify.com/inbox
  32. Shopify POS: https://www.shopify.com/pos
  33. Shopify Pricing: https://www.shopify.com/pricing
  34. Shopify Chatwoot App: https://apps.shopify.com/chatwoot
  35. Shopify G2: https://www.g2.com/products/shopify/reviews
  36. Shopify Trustpilot: https://trustpilot.com/review/shopify.com
  37. Square Messages: https://squareup.com/us/en/software/messages
  38. Square POS: https://squareup.com/us/en/point-of-sale
  39. Square Pricing: https://squareup.com/us/en/pricing
  40. Square G2: https://www.g2.com/products/square-point-of-sale/reviews
  41. Square Trustpilot: https://trustpilot.com/review/squareup.com
  42. HubSpot Live Chat: https://hubspot.com/products/service/live-chat
  43. HubSpot Pricing: https://hubspot.com/pricing/service
  44. HubSpot G2: https://www.g2.com/products/hubspot-service-hub/reviews
  45. HubSpot Trustpilot: https://trustpilot.com/review/hubspot.com
  46. Notion AI: https://notion.so/product/ai
  47. Notion G2: https://www.g2.com/products/notion/reviews
  48. MS Copilot: https://www.microsoft.com/en-us/microsoft-copilot
  49. MS Copilot G2: https://www.g2.com/products/microsoft-copilot/reviews
  50. Feishu: https://www.feishu.cn/en/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
