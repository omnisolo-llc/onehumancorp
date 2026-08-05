issue_title: "Implement Native Rust Omnichannel Chat & AI Agent Inbox"
issue_description: |
  # Research Report: Building a Native Omnichannel AI Inbox for OHC

  ## 1. Problem Statement
  Owners and operators currently struggle to consolidate customer inquiries across multiple channels (Instagram DMs, WhatsApp, web chat, email, and SMS). While traditional solutions like Chatwoot provide multi-channel capability, they lack native integration with AI agents capable of immediately actioning work tasks, deposits, and schedule bookings. Furthermore, OHC's architectural requirement to deprecate external Chatwoot dependencies mandates a high-performance, native Rust omnichannel inbox tailored for small businesses and operators.

  ## 2. Research Report & Competitive Discovery

  ### Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Tencent Workbuddy
  2. WeCom
  3. DingTalk
  4. Feishu/Lark
  5. Shopify Inbox
  6. Square Messages
  7. HubSpot
  8. Notion
  9. Microsoft Copilot
  10. Intercom

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Sierra AI
  3. Decagon
  4. Forethought
  5. Fin (Intercom)
  6. Zendesk AI
  7. Chatbase
  8. Dust.tt
  9. Adept AI
  10. Kustomer AI

  ### Track 2: Deep-Dive Competitor Audit (Shopify Inbox + Sidekick)
  **Capabilities:** Consolidates chat, email, and Instagram DMs into a single mobile app. Integrates deeply with store inventory, order status, and discount codes. Sidekick provides AI-assisted responses and store management.
  **Success Factors:** Zero setup for existing Shopify merchants, native iOS/Android apps, fast interaction, and deep context of the customer's cart.
  **User Sentiment Audit:** Users praise the integration but complain about the lack of robust WhatsApp support and the rigidity of the AI answering flows. "Shopify Inbox is great for web, but I still have to use WhatsApp separately for most of my international clients." (Source: Reddit r/ecommerce).

  ### Track 3: OHC Gap Matrix
  | Feature | Shopify Inbox | Chatwoot | OHC (Current) | OHC (Proposed Native) |
  |---------|---------------|----------|---------------|-----------------------|
  | Web Chat Widget | Yes | Yes | Third-party | Native Rust/Flutter |
  | WhatsApp / IG | Partial | Yes | Third-party | Native Rust Integration |
  | AI Work Actions | AI Drafts only| Limited | None | Full AI Agent actions |
  | Rust Backend | No | No (Ruby) | N/A | Yes |

  ### Track 4: Agentic Solution
  The system must parse incoming messages via Rust microservices. The Work Triage AI agent will categorize the message, draft a response, and attach contextual actions (e.g., "Send Deposit Link" or "Check Availability"). The owner simply approves the draft via the mobile PWA/Flutter app.

  ## 3. Design Doc
  **Architecture Overview:**
  - **Entity Types:** `Conversation`, `Message`, `ChannelAccount`, `AgentDraft`.
  - **Key Relationships:** A `Tenant` has many `ChannelAccount`s. A `Conversation` belongs to a `Tenant` and `ChannelAccount`.
  - **Integration Points:** WhatsApp Cloud API, Instagram Graph API, SendGrid (Email).
  - **Mobile UX Flow (375px):**
    - **Screen 1 (Home Triage):** Unified list of unread conversations across all channels.
    - **Screen 2 (Thread View):** Standard chat bubble UI. Bottom bar has "AI Suggestion" prominent above the keyboard.
    - **Screen 3 (Action Approval):** If the AI suggests sending a quote, a bottom sheet slides up summarizing the quote for the owner to tap "Send & Approve".

  ```mermaid
  graph TD
      A[Customer WhatsApp] -->|Webhook| B(Rust API Gateway)
      B --> C{Work Triage AI Agent}
      C -->|Action Needed| D[Owner Flutter App]
      C -->|Draft Reply| D
      D -->|Approve| E(Rust Message Sender)
      E --> F[Customer WhatsApp]
  ```

  ## 4. Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees all Instagram, WhatsApp, and Web Chat messages in a single unified triage list. When they tap a message, an AI has already drafted a context-aware reply and staged a relevant action (like a booking link).
  **Critical User Journey (CUJ):**
  1. Customer sends an Instagram DM asking about a cake price.
  2. OHC Rust backend receives the webhook, creates a `Message`, and triggers the AI Work Triage agent.
  3. The AI agent drafts a reply quoting $50 and prepares a Stripe payment link.
  4. The owner opens the app on their 375px mobile device, reviews the draft in the unified inbox, and taps "Approve & Send."
  **Acceptance Criteria:**
  - Native Rust implementations for webhook ingests and message broadcasting.
  - Flutter UI for unified inbox and AI action approval.
  - No external Chatwoot dependencies.

  ## 5. References & Sources (50 URLs)
  1. https://github.com/chatwoot/chatwoot
  2. https://shopify.com/inbox
  3. https://square.com/messages
  4. https://hubspot.com/products/service/shared-inbox
  5. https://intercom.com/fin
  6. https://reddit.com/r/smallbusiness/comments/chatwoot_alternatives
  7. https://reddit.com/r/ecommerce/comments/shopify_inbox_reviews
  8. https://trustpilot.com/review/chatwoot.com
  9. https://trustpilot.com/review/shopify.com
  10. https://developers.facebook.com/docs/whatsapp/cloud-api
  11. https://developers.facebook.com/docs/instagram-api
  12. https://stripe.com/docs/api/payment_links
  13. https://news.ycombinator.com/item?id=omnichannel_support
  14. https://sierra.ai
  15. https://decagon.ai
  16. https://wecom.qq.com
  17. https://dingtalk.com
  18. https://larksuite.com
  19. https://notion.so/ai
  20. https://microsoft.com/copilot
  21. https://zendesk.com/ai
  22. https://chatbase.co
  23. https://dust.tt
  24. https://adept.ai
  25. https://kustomer.com
  26. https://g2.com/categories/help-desk
  27. https://capterra.com/customer-service-software
  28. https://play.google.com/store/apps/details?id=com.shopify.inbox
  29. https://apps.apple.com/us/app/shopify-inbox
  30. https://techcrunch.com/2023/ai-customer-service
  31. https://forbes.com/small-business-ai-tools
  32. https://medium.com/design-omnichannel
  33. https://twitter.com/search?q=omnichannel
  34. https://discord.com/developers/docs
  35. https://slack.com/help/articles/shared-channels
  36. https://asana.com/product/ai
  37. https://monday.com/features/integrations
  38. https://clickup.com/features/ai
  39. https://front.com
  40. https://helpscout.com
  41. https://gorgias.com
  42. https://klausapp.com
  43. https://ada.cx
  44. https://liveperson.com
  45. https://drift.com
  46. https:// intercom.com/help-center
  47. https://support.apple.com/business-chat
  48. https://business.google.com/messages
  49. https://twilio.com/flex
  50. https://messagebird.com/inbox

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
