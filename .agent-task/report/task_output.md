issue_title: "OHC Mission: The Unified Mobile Command Center for Solopreneurs"
issue_description: |
  ## Problem Statement
  Small business owners and solopreneurs like Maya (baker) and Carlos (handyman) are overwhelmed by context-switching between point solutions. They currently have to piece together Shopify for orders, Instagram DMs for leads, Square for in-person payments, and Google Calendar for bookings. This fragmentation leads to missed opportunities, delayed responses, and administrative exhaustion. They need a single, AI-native assistant that proactively triages inbound demand and operational tasks into one unified, mobile-first feed that works flawlessly on a 375px screen.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We explored the general and AI-native competitor landscape.

  **Top 10 General Competitors:**
  1. Shopify
  2. Square
  3. HubSpot
  4. Notion
  5. WeCom
  6. DingTalk
  7. Feishu / Lark
  8. Tencent Workbuddy
  9. Microsoft Copilot
  10. Wix

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick
  2. Notion AI
  3. Harvey AI
  4. Fin (Intercom)
  5. Adept AI
  6. MultiOn
  7. Lindy AI
  8. Sierra
  9. Maven AGI
  10. Devin

  ### Track 2: Deep-Dive Competitor Audit (Tencent Workbuddy)
  *Capabilities:* Tencent Workbuddy acts as an integrated AI assistant inside a chat-like interface. It successfully centralizes operations into a mobile-first feed, handling schedules, approvals, and customer triage.
  *Success Factors:* The "single inbox" approach reduces cognitive load for operators who are constantly on the move. Its deep integration with existing workflows makes it a reliable command center.
  *User Sentiment:* Users of chat-based assistants like Workbuddy love the low barrier to entry on mobile, allowing them to manage their business with just a few taps. However, they strongly dislike when the AI makes confident mistakes or takes autonomous actions without explicit owner approval.

  ### Track 3: OHC Gap & Pain Point Identification
  *OHC Feature Audit:* OHC has agentic capabilities but lacks a unified, mobile-first feed that aggregates cross-channel data (messages, payments, bookings).
  *Gap Matrix:*

  | Feature | Shopify Sidekick | Tencent Workbuddy | OHC (Current) | OHC (Proposed) |
  |---------|------------------|-------------------|---------------|----------------|
  | Mobile-First 375px UI | Limited | Excellent | Moderate | Excellent |
  | Unified Triage Feed | No | Yes | No | Yes |
  | Agentic Task Execution | Yes (Store Only) | Yes (Ops) | Emerging | Comprehensive |
  | Cross-Channel Context | No | Yes | Emerging | Yes |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  *Agentic Solution:* Introduce the **Unified Work Triage Feed**. An AI Agent running asynchronously evaluates incoming webhooks (e-mails, DMs, bookings), categorizes them, drafts responses, and presents them in a prioritized, swipeable 375px-optimized feed. The owner only needs to tap "Approve" or "Edit".

  ```mermaid
  graph TD
      A[Inbound Webhook: DM / Order / Booking] --> B(Work Triage Agent)
      B --> C{Determine Priority & Context}
      C --> D[Draft Response / Action]
      C --> E[Update Operations/Inventory]
      D --> F[Unified Mobile Feed UI]
      F --> G[Owner Approves with 1 Tap]
      G --> H[Execution Agent Performs Action]
  ```

  ## Design Doc
  - **Architecture:**
    - Database: Expand `Tenant` schemas to include `WorkFeedItem` and `AgentDraft`.
    - API: Expose a `/v1/feed` endpoint that streams the top actionable items.
    - Distributed Lock: Use Redis Redlock for ensuring multiple agents don't process the same inbound request simultaneously.
  - **Mobile UX Flow (375px First):**
    - The Home view defaults to the "Needs Attention" feed.
    - Each card displays context (e.g., "Maya, a new cake inquiry from Sarah. She wants a vegan option for Saturday.").
    - A translucent glass bottom sheet allows 1-tap approvals ("Send Quote: $150").
    - No horizontal scrolling; large 44x44px touch targets.

  ## Implementation Prompt
  **User-Facing Outcome:** When the owner opens OHC, they see a single list of prioritized actions (e.g., "3 messages to reply to, 1 invoice to send, 2 bookings to confirm"). The AI has already drafted the replies and prepared the invoices.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on a mobile device (375px viewport).
  2. Owner views the "Needs Attention" feed.
  3. Owner taps an inquiry item.
  4. Owner reviews the drafted response and taps "Approve & Send".
  5. The item is marked completed and animates out of the feed.

  **Acceptance Criteria:**
  - UI strictly adheres to 375px width without horizontal scrolling.
  - All interactive elements meet the 44x44px touch target minimum.
  - The feed successfully loads `WorkFeedItem` records from the API.
  - Playwright E2E test covers the 1-tap approval CUJ.

  **Estimated Scope:** Medium

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://squareup.com/us/en/townsquare/ai-for-small-business
  3. https://www.hubspot.com/artificial-intelligence
  4. https://www.notion.so/product/ai
  5. https://work.weixin.qq.com/
  6. https://www.dingtalk.com/
  7. https://www.larksuite.com/
  8. https://www.tencent.com/en-us/business/
  9. https://copilot.microsoft.com/
  10. https://www.wix.com/studio/ai
  11. https://chatgpt.com/
  12. https://claude.ai/
  13. https://www.intercom.com/fin
  14. https://www.adept.ai/
  15. https://www.multion.ai/
  16. https://www.lindy.ai/
  17. https://sierra.ai/
  18. https://mavenagi.com/
  19. https://www.cognition.ai/introducing-devin
  20. https://zapier.com/ai
  21. https://make.com/
  22. https://www.salesforce.com/einstein/
  23. https://www.zoho.com/zia/
  24. https://monday.com/ai
  25. https://asana.com/product/ai
  26. https://clickup.com/ai
  27. https://www.smartsheet.com/ai
  28. https://trello.com/
  29. https://basecamp.com/
  30. https://www.honeybook.com/
  31. https://www.dubsado.com/
  32. https://www.calendly.com/
  33. https://acuityscheduling.com/
  34. https://www.fresha.com/
  35. https://www.mindbodyonline.com/
  36. https://www.vagaro.com/
  37. https://www.glossgenius.com/
  38. https://www.booksy.com/
  39. https://www.toasttab.com/
  40. https://www.touchbistro.com/
  41. https://www.lightspeedhq.com/
  42. https://www.clover.com/
  43. https://www.revelsystems.com/
  44. https://www.vendhq.com/
  45. https://www.shopkeep.com/
  46. https://www.gocardless.com/
  47. https://stripe.com/
  48. https://www.paypal.com/
  49. https://www.adyen.com/
  50. https://www.braintreepayments.com/
  51. https://www.klarna.com/
  52. https://www.afterpay.com/

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
