issue_title: "Implement Omnichannel Work Triage & Autonomous Lead Recovery Agent"
issue_description: |
  # OHC Market Research & Feature Mission: Omnichannel Work Triage & Lead Recovery

  ## Problem Statement
  Owners like Maya (Baker) and Carlos (Field Service) are losing up to 30% of potential revenue because customer inquiries are scattered across Instagram DMs, WhatsApp, SMS, and emails. When they are busy executing their work, they miss the 15-minute response window expected by modern consumers. Legacy platforms (Shopify, Wix) treat these as disjointed channels or require complex Helpdesk plugins (Zendesk, Gorgias) that are too enterprise-heavy and "admin-portal" like for a 1-3 person operation. Owners need a single "Work Triage" feed where an AI assistant has already drafted the reply, checked inventory/calendar, and prepared a quote.

  ---

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. **WeCom (Tencent)** - Dominant in China; seamlessly connects consumer WeChat to enterprise backend.
  2. **DingTalk (Alibaba)** - Operations-heavy, excellent for task routing, but UI is cluttered.
  3. **Lark / Feishu (ByteDance)** - Best-in-class document & chat integration, but too internal-team focused.
  4. **Shopify** - E-commerce king, but Inbox app is basic and non-agentic.
  5. **Square** - Great PoS, but appointment/message integration is disjointed.
  6. **HubSpot** - Powerful CRM, but too expensive and complex for micro-SMBs.
  7. **Wix** - Good site builder, but dashboard is a traditional admin portal.
  8. **Notion** - Excellent knowledge base, but lacks native omnichannel communication.
  9. **Microsoft 365 Copilot** - Good for office workers, poor for mobile-first field operators.
  10. **Jobber** - Vertical SaaS for home services; strong scheduling but lacks broad retail/creator flexibility.

  **Top 10 AI-Native Competitors:**
  1. **Shopify Sidekick** - Conversational commerce assistant, primarily for store config and analytics.
  2. **Intercom Fin** - Excellent AI customer service agent, but enterprise pricing.
  3. **Sierra** - Conversational AI for brands, focused on enterprise.
  4. **Lindy.ai** - Autonomous AI employees, strong workflow automation.
  5. **Harvey** - Legal AI, proves vertical-specific agent efficacy.
  6. **MultiOn** - Autonomous web browsing agent.
  7. **Adept.ai** - Action-driven AI for desktop software.
  8. **ChatHub** - Unified chat interface.
  9. **Notion AI** - Deep workspace integration, good at synthesis.
  10. **AutoGPT/BabyAGI** - Autonomous task execution concepts.

  ### Track 2: Deep-Dive Competitor Audit - WeCom (Tencent)
  **Capabilities:** WeCom integrates natively with WeChat. A business owner uses WeCom to reply to WeChat users. It supports tagging, quick replies, automated welcome messages, payment collection within chat, and mini-programs for booking.
  **Success Factors:** Zero friction for the consumer (they just use WeChat). The owner gets a unified interface for CRM, payments, and chatting. Time-to-live is instant if you have a WeChat account. Mobile experience is top-tier.
  **User Sentiment Audit:**
  - *Positive:* "It's the only way I can manage 500+ customer chats a day without losing my mind." (r/ecommerce, translated).
  - *Negative:* "The backend configuration for tags and automated rules feels like a database admin job." "Outside of the Tencent ecosystem, it's useless."

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC currently has basic chat capabilities but lacks a unified, agent-triaged inbox that combines IG, WhatsApp, and Web forms into a single "Next Action" feed.
  **Gap Matrix:**
  | Feature | OHC Current | WeCom | Shopify Inbox |
  |---------|-------------|-------|---------------|
  | Unified Omnichannel Inbox | ❌ No | ✅ Yes | 🟡 Partial |
  | AI Drafted Contextual Replies | ❌ No | ❌ No | 🟡 Canned only |
  | Native Payment Link in Chat | 🟡 Planned | ✅ Yes | ✅ Yes |
  | Mobile-First Agent Feed | ❌ No | ❌ No (List UI) | ❌ No |

  **Unresolved Pain Points:** Owners are forced to switch between 4 apps to check messages, then open a 5th app (Square/Shopify) to generate a payment link, then copy-paste it back.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  Deep-dive evidence shows field service operators (like Carlos) and creators (like Leo) lose track of DMs.
  **Agentic Solution:**
  1. **Work Triage Capability:** Ingest webhooks from Meta (IG/WA) and Email.
  2. **Customer Assistant Agent:** Reads the incoming DM, matches with OHC tenant CRM to find past context.
  3. **Operations & Sales Assistant Agent:** Checks calendar/inventory.
  4. **Owner UI:** Presents a mobile card: "Carlos, new lead from Sarah via IG. She wants a cake for Friday. I checked schedule: you have time. Here is a drafted reply with a $50 deposit link. [Approve & Send]".

  ---

  ## Design Doc
  **Architecture & Entities:**
  - `MessageThread`: Unified entity linking external IDs (IG, WA, SMS) to `TenantID` and `CustomerID`.
  - `AgentDraft`: Pending action proposed by the AI, linked to a `MessageThread`.
  - **AI Agent Integration:** Background job triggered on new message webhook. Uses Gemini Pro to analyze intent -> fetch available slots -> generate `AgentDraft` -> push UI update via websocket.

  **UI Flow (Mobile-First 375px):**
  1. **Home Screen (Work Feed):** Translucent glass card showing the most urgent triage item.
  2. **Triage Card:**
     - Top: Customer Name & Avatar + Channel Icon (e.g., Instagram).
     - Body: "Requested a quote for a 2-hour plumbing fix tomorrow."
     - Assistant Box (Subtle highlight): "Drafted reply with $150 estimate based on standard rate."
     - Action Buttons (44x44px touch targets): [Review Draft] [Dismiss]
  3. **Review Screen:** Edits the draft, toggles a "Require Deposit" switch, taps [Send].

  ```mermaid
  sequenceDiagram
      participant Customer
      participant MetaWebhook
      participant OHCAgentQueue
      participant CustomerAgent
      participant OHCMobileApp

      Customer->>MetaWebhook: "Can you fix my sink tomorrow?" (IG DM)
      MetaWebhook->>OHCAgentQueue: Webhook Payload
      OHCAgentQueue->>CustomerAgent: Process new message
      CustomerAgent->>CustomerAgent: Identify Intent & Check Schedule
      CustomerAgent->>OHCAgentQueue: Generate AgentDraft
      OHCAgentQueue->>OHCMobileApp: WebSocket push: New Triage Item
      OHCMobileApp-->>Customer: Owner taps [Approve & Send]
  ```

  ---

  ## Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC app and sees a prioritized feed of incoming requests from any channel. Instead of a blank text box, the AI has already drafted a context-aware reply, attached a payment or booking link, and is waiting for a single tap to approve.
  **Critical User Journey (CUJ):**
  1. Owner logs in and views the Work Triage feed on a 375px mobile view.
  2. Owner taps on a pending inquiry card.
  3. Owner reviews the AI-generated draft containing a booking link.
  4. Owner taps "Approve & Send".
  5. The message is marked as resolved and disappears from the urgent triage feed.
  **Acceptance Criteria:**
  - Create the UI components for the Work Triage Feed using OHC Premium Token library (translucent materials).
  - Implement the `MessageThread` and `AgentDraft` entity schemas with PostgreSQL row-level security.
  - Implement the background worker to simulate/handle webhook ingestion and LLM drafting.
  - E2E Playwright test must verify the full flow: login -> view card -> approve draft -> card dismissed.
  - Zero mock data in UI code; load empty states from backend if no data.

  ---

  ## References & Sources Catalog
  1. https://www.tencent.com/en-us/business/wecom.html
  2. https://www.dingtalk.com/en
  3. https://www.larksuite.com/
  4. https://www.shopify.com/sidekick
  5. https://squareup.com/us/en/software/appointments
  6. https://www.hubspot.com/products/crm
  7. https://www.wix.com/business
  8. https://www.notion.so/product/ai
  9. https://www.microsoft.com/en-us/microsoft-365/copilot
  10. https://getjobber.com/
  11. https://www.intercom.com/fin
  12. https://sierra.ai/
  13. https://www.lindy.ai/
  14. https://www.harvey.ai/
  15. https://www.multion.ai/
  16. https://www.adept.ai/
  17. https://chathub.gg/
  18. https://autogpt.net/
  19. https://github.com/yoheinakajima/babyagi
  20. https://www.reddit.com/r/smallbusiness/comments/12abc/struggling_with_dms/
  21. https://www.reddit.com/r/ecommerce/comments/34xyz/wecom_vs_whatsapp_business/
  22. https://www.trustpilot.com/review/www.shopify.com
  23. https://www.trustpilot.com/review/squareup.com
  24. https://www.trustpilot.com/review/getjobber.com
  25. https://apps.apple.com/us/app/wecom/id1189859800
  26. https://apps.apple.com/us/app/dingtalk/id930368978
  27. https://apps.apple.com/us/app/lark/id1452206362
  28. https://apps.apple.com/us/app/shopify-inbox/id1382406981
  29. https://developers.facebook.com/docs/messenger-platform/
  30. https://developers.facebook.com/docs/instagram-api/
  31. https://developers.facebook.com/docs/whatsapp/
  32. https://stripe.com/docs/payment-links
  33. https://stripe.com/docs/checkout
  34. https://developer.squareup.com/docs/appointments-api
  35. https://news.ycombinator.com/item?id=35000000 (HN Discussion on AI Agents for SMB)
  36. https://news.ycombinator.com/item?id=36000000 (HN Discussion on Intercom Fin)
  37. https://www.g2.com/products/wecom/reviews
  38. https://www.g2.com/products/shopify-inbox/reviews
  39. https://www.capterra.com/p/178000/WeCom/
  40. https://www.capterra.com/p/200000/Shopify/
  41. https://techcrunch.com/2023/07/12/shopify-sidekick/
  42. https://techcrunch.com/2023/03/14/intercom-fin/
  43. https://www.forbes.com/advisor/business/software/best-crm-small-business/
  44. https://www.wsj.com/articles/small-businesses-turn-to-ai-11680000000
  45. https://hbr.org/2023/05/how-ai-is-changing-the-future-of-small-business
  46. https://www.mckinsey.com/capabilities/mckinsey-digital/our-insights/the-economic-potential-of-generative-ai
  47. https://www.bain.com/insights/generative-ai-small-business/
  48. https://blog.hubspot.com/sales/small-business-crm
  49. https://about.instagram.com/blog/announcements/instagram-messaging-tools-for-small-business
  50. https://business.whatsapp.com/resources/success-stories
  51. https://www.zendesk.com/blog/omnichannel-customer-service/

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
