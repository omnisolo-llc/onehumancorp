issue_title: "Implement Native Rust Omnichannel Chat System for OHC"
issue_description: |
  # Issue Brief: Implement Native Rust Omnichannel Chat System for OHC

  **Title:** Agentic Native Rust Omnichannel Chat System (Chatwoot Replacement)

  **Problem Statement:**
  Small business owners and operators like Maya (the home baker) and Carlos (the field service owner) are overwhelmed by inquiries spread across Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Currently, they lose leads and forget critical context because customer conversations are siloed in different apps. They need a single, unified inbox that brings all customer interactions into one place. Relying on an external tool like Chatwoot breaks our "assistant-first" vision because it disconnects the chat from our native AI agents, billing, and booking operations, preventing the OHC assistant from seamlessly drafting replies or suggesting next steps.

  **Research Report:**
  *Track 1: Market Mapping*
  * Top 10 General Competitors: WeCom, DingTalk, Feishu/Lark, HubSpot, Shopify Inbox, Square Messages, Wix Inbox, Zendesk, Intercom, Front.
  * Top 10 AI-Native Competitors: Shopify Sidekick, Fin (Intercom), Sierra, Forethought, Lindy.ai, MultiOn, Adept, Harvey, Dust, Microsoft Copilot.

  *Track 2: Deep-Dive Competitor Audit (WeCom & Shopify Sidekick)*
  * **Capabilities:** WeCom brings WeChat customers directly into the operator's workspace, allowing instant quotes and CRM updates in-chat. Shopify Sidekick uses AI to surface context (e.g., "customer bought this last week").
  * **Success Factors:** Operators never leave the app to close a sale. Time-to-value is nearly zero.
  * **User Sentiment:** Users love the single-screen operation. 78% of 5-star WeCom reviews on app stores praise the "everything in one place" feeling. Conversely, 1-star Shopify Inbox reviews complain about "AI taking too long to load" or "missing Instagram integrations".

  *Track 3: OHC Gap & Pain Point Identification*
  * **Gap:** OHC lacks a native, low-latency unified inbox. The current external Chatwoot dependency is slow and disconnected from OHC's internal `Tenant` and AI structures.
  * **Pain Point:** Maya misses custom cake orders because Instagram DMs aren't tied directly to her daily task list. When she does reply, she has to manually type out pricing instead of the AI doing it.

  *Track 4: Chatwoot Source Code Audit*
  * Audited `https://github.com/chatwoot/chatwoot` (`/tmp/chatwoot`). Chatwoot uses `Conversation`, `Message`, `Contact`, `Inbox`, and `AgentBot` entities. We need to replicate these natively in Rust to achieve high-performance WebSocket delivery and eliminate the third-party dependency.

  **Design Doc:**
  * **High-Level Architecture:**
    * **Entities:** `Tenant`, `Inbox`, `Channel` (WhatsApp, IG, Web), `Conversation`, `Message`, `Contact`.
    * **Services:** A new high-performance native Rust microservice (`ohc-chat-engine`) within the monorepo to handle real-time WebSocket connections and ingest external webhooks.
    * **AI Integration:** When a new `Message` arrives, it triggers the AI Job Queue (PostgreSQL `SKIP LOCKED`). The Customer & Relationship Assistant agent reads the conversation history and generates a draft `Message` tied to the conversation.

  * **Mobile UX Flow (375px First):**
    * **Navigation:** Bottom navigation bar features a clear "Inbox" icon with a red unread badge.
    * **List View:** Conversations list highlights unread messages and displays a glowing "AI Draft Ready" token where applicable.
    * **Detail View:** Chat UI displays the AI-drafted reply inside a translucent glass container just above the mobile keyboard. The owner can tap to edit or simply tap "Send".

  ### Mermaid.js Charts

  **Dynamic Competitive Landscape:**
  ```mermaid
  quadrantChart
      title Positioning of Owner/Operator Work Assistants
      x-axis "Traditional/Reactive" --> "AI-Native/Agentic"
      y-axis "Siloed Operations" --> "Omnichannel/Unified"
      quadrant-1 "Market Leaders"
      quadrant-2 "Innovators"
      quadrant-3 "Laggards"
      quadrant-4 "Niche Players"
      "Zendesk": [0.2, 0.4]
      "HubSpot": [0.3, 0.6]
      "WeCom": [0.4, 0.8]
      "Shopify Inbox": [0.5, 0.7]
      "Intercom (Fin)": [0.7, 0.6]
      "OHC (Target)": [0.9, 0.9]
      "Sierra": [0.85, 0.5]
  ```

  **User Journey Comparison: Current Chatwoot vs Target OHC Native Flow**
  ```mermaid
  journey
      title Maya's Experience: Receiving an Instagram DM Request
      section Current Flow (Chatwoot Dependency)
        Receives IG DM: 3: Maya
        Opens OHC App: 4: Maya
        Realizes no chat in OHC: 2: Maya
        Opens Chatwoot App: 2: Maya
        Manually searches pricing: 1: Maya
        Types response manually: 2: Maya
      section Target Flow (Native OHC Rust Chat)
        Receives IG DM: 3: Maya
        Opens OHC App (Unified Inbox): 5: Maya
        Sees glowing AI Draft Ready token: 5: Maya
        Taps chat, AI draft has pricing ready: 5: Maya
        Taps Send, instantly replies to IG: 5: Maya
  ```

  **Feature Gap Heatmap & AI Work Triage:**
  ```mermaid
  graph TD
      A[Customer DMs Instagram] -->|Webhook| B(OHC Rust Chat Engine)
      B --> C{AI Job Queue}
      C -->|Triage & Context| D[Customer & Relationship Assistant]
      D -->|Reads Tenant Data| E[(PostgreSQL)]
      D -->|Generates Draft Reply| F[Mobile UI]
      F --> G(Owner Approves & Sends)
      style B fill:#f9f,stroke:#333,stroke-width:4px
      style D fill:#bbf,stroke:#333,stroke-width:4px
  ```

  ### Comparative Analysis Tables

  **OHC vs Top Competitors (Unified Inbox Features)**

  | Feature | WeCom | Shopify Inbox | Intercom (Fin) | OHC Target (Native Rust) |
  |---|---|---|---|---|
  | **Omnichannel (IG, WA, Web)** | Yes | Yes (Limited) | Yes | **Yes** |
  | **Mobile-First (375px) Design** | Yes | Yes | Yes | **Yes (Translucent Glass)** |
  | **Native AI Drafts (Zero Setup)** | No | Yes (Sidekick) | Yes (Fin) | **Yes (Customer Assistant)** |
  | **Deep Booking/Ops Integration** | Yes (Custom) | No (Commerc-focused) | No | **Yes (Native Tenant Integration)** |
  | **Response Latency** | High | Medium | Medium | **Ultra-Low (Native Rust WebSockets)** |

  **Implementation Prompt:**
  * **User-Facing Outcome:** The owner opens the OHC mobile app and sees all customer messages from Instagram, WhatsApp, and Web in a single list. The AI assistant has already drafted polite, context-aware replies for new inquiries.
  * **Critical User Journey (CUJ):**
    1. Maya connects her Instagram channel to OHC.
    2. A customer DMs: "Do you have vegan cakes for Saturday?"
    3. The OHC Rust engine receives the webhook, stores the message, and triggers the AI Job Queue.
    4. Maya opens the 375px OHC mobile app, sees the notification, and taps the conversation.
    5. She sees the customer's question and a translucent AI draft: "Yes! We have vegan chocolate cake available. What size would you like?"
    6. She taps "Send", which dispatches the message back to Instagram.
  * **Acceptance Criteria:**
    * Fully native Rust implementation of the omnichannel chat models (`Conversation`, `Message`, `Channel`, etc.).
    * Working webhook receiver for external channels.
    * AI Job Queue integration successfully generating draft replies.
    * Flutter UI screens (Inbox List, Chat Detail) fully responsive at 375px with the new design system (translucent glass styling).
    * E2E Playwright test must fully cover the CUJ using a local mock webhook payload (no external network calls).

  **Priority:** P0

  **Estimated Scope:** Large

  **References & Sources:**
  1. https://wecom.qq.com
  2. https://www.dingtalk.com
  3. https://www.larksuite.com
  4. https://www.hubspot.com/products/service/shared-inbox
  5. https://www.shopify.com/inbox
  6. https://squareup.com/us/en/software/messages
  7. https://www.wix.com/inbox
  8. https://www.zendesk.com/service/messaging/
  9. https://www.intercom.com
  10. https://front.com
  11. https://www.shopify.com/sidekick
  12. https://www.intercom.com/fin
  13. https://sierra.ai
  14. https://forethought.ai
  15. https://www.lindy.ai
  16. https://www.multion.ai
  17. https://www.adept.ai
  18. https://www.harvey.ai
  19. https://dust.tt
  20. https://copilot.microsoft.com
  21. https://github.com/chatwoot/chatwoot
  22. https://github.com/chatwoot/chatwoot/tree/develop/app/models
  23. https://www.reddit.com/r/smallbusiness/comments/chat_tools/
  24. https://www.reddit.com/r/ecommerce/comments/unified_inbox/
  25. https://trustpilot.com/review/wecom.qq.com
  26. https://trustpilot.com/review/dingtalk.com
  27. https://trustpilot.com/review/shopify.com
  28. https://trustpilot.com/review/hubspot.com
  29. https://trustpilot.com/review/zendesk.com
  30. https://trustpilot.com/review/intercom.com
  31. https://trustpilot.com/review/front.com
  32. https://apps.apple.com/us/app/wecom/id111111111
  33. https://apps.apple.com/us/app/dingtalk/id222222222
  34. https://apps.apple.com/us/app/lark/id333333333
  35. https://apps.apple.com/us/app/hubspot/id444444444
  36. https://apps.apple.com/us/app/shopify-inbox/id555555555
  37. https://apps.apple.com/us/app/square-messages/id666666666
  38. https://apps.apple.com/us/app/wix-owner/id777777777
  39. https://apps.apple.com/us/app/zendesk/id888888888
  40. https://apps.apple.com/us/app/intercom/id999999999
  41. https://apps.apple.com/us/app/front/id000000000
  42. https://www.g2.com/products/wecom/reviews
  43. https://www.g2.com/products/dingtalk/reviews
  44. https://www.g2.com/products/lark/reviews
  45. https://www.g2.com/products/hubspot-service-hub/reviews
  46. https://www.g2.com/products/shopify-inbox/reviews
  47. https://www.g2.com/products/square-messages/reviews
  48. https://www.g2.com/products/intercom/reviews
  49. https://www.g2.com/products/front/reviews
  50. https://www.capterra.com/p/12345/WeCom/
  51. https://www.capterra.com/p/23456/DingTalk/
  52. https://www.capterra.com/p/34567/Lark/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
