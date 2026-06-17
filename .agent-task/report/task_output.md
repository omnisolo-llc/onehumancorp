issue_title: "Implement the 'Triage Action Feed' UI to Centralize OHC Owner Daily Work"
issue_description: |
  # Mission Brief: Triage Action Feed for OHC
  **Title:** Implement the 'Triage Action Feed' UI to Centralize OHC Owner Daily Work
  **Problem Statement:** Owners like Carlos (Field Service) and Maya (Baker) are overwhelmed by scattered tools. They check DMs, emails, booking systems, and payment portals separately. They miss leads and forget follow-ups. Existing CRMs (HubSpot) and Commerce tools (Shopify) are too complex and module-based. OHC needs an AI-curated "Action Feed" where all tasks, messages, and alerts converge into a single scrollable priority list with one-tap agentic actions.

  ## 1. Research Report: Market Mapping & Gap Analysis

  ### Track 1: Market Mapping
  **Top 10 General Competitors:**
  1. Shopify (shopify.com) - Sidekick AI for commerce operations.
  2. Wix (wix.com) - AI site builder, weak operational triage.
  3. Squarespace (squarespace.com) - Blueprint AI for setup.
  4. Square (squareup.com) - Good POS, fragmented inbox.
  5. HubSpot (hubspot.com) - Powerful CRM, too complex for Carlos.
  6. DingTalk (dingtalk.com) - Unified work hub, but very corporate.
  7. Lark/Feishu (larksuite.com) - Great for teams, overkill for solo.
  8. WeCom (work.weixin.qq.com) - Deep WeChat integration, regional lock.
  9. Microsoft Copilot (microsoft.com) - Great for office docs, not physical work.
  10. Notion AI (notion.so) - Great knowledge base, bad at daily transaction triage.

  **Top 10 AI-Native Competitors:**
  1. Durable (durable.co) - 30s site setup.
  2. 10Web (10web.io) - AI WordPress manager.
  3. Lindy.ai (lindy.ai) - AI EA via iMessage/SMS.
  4. Skyvern (skyvern.com) - Browser automation.
  5. Relevance AI (relevanceai.com) - AI workforce builders.
  6. Framer AI (framer.com/ai) - Fast design generation.
  7. Mixo (mixo.io) - Validation tool.
  8. Chatbase (chatbase.co) - AI customer support bots.
  9. Cassidy (cassidyai.com) - AI business assistants.
  10. Artisan (artisan.co) - AI sales development reps (Ava).

  ### Track 2: Deep Dive on DingTalk & Tencent Workbuddy
  **Capabilities:** DingTalk unifies messaging, approvals, tasks, and scheduling into a single feed. It thrives on clear read-receipts and urgent "DING" notifications.
  **Success Factors:** High reliability on poor networks, single pane of glass for all work, strong offline support.
  **User Sentiment:**
  - *Positive:* "Everything I need to run my 15-person crew is in one app."
  - *Negative:* "It feels like a surveillance tool. The UI is cluttered with enterprise features I don't use." (r/smallbusiness).

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Gap Matrix vs DingTalk:**
  | Feature | DingTalk | OHC Current | OHC Target (Action Feed) |
  |---|---|---|---|
  | Unified Inbox | Yes | Fragmented | Yes (AI Triage) |
  | Task Approvals | Yes (Manual) | None | Yes (Agentic) |
  | Mobile First | Yes | Yes | Yes (375px optimized) |
  | Complexity | High (Enterprise) | Medium | Radical Simplicity |

  **Unresolved Pain Points:**
  - Owners have to hunt for what to do next.
  - Notifications are noisy and don't tell the owner *what* action to take.
  - Maya gets an Instagram DM and an email simultaneously; she has to reply in two apps.

  ### Track 4: Agentic Solutions
  **Solution:** The "Triage Action Feed". An AI-curated stream of cards.
  If Maya gets a cake inquiry, the Action Feed shows a card:
  `🎂 New Inquiry: Vegan Chocolate Cake for Saturday.`
  `[Approve Draft Reply & Send Quote] [Ignore]`
  The agent has already drafted the reply and checked inventory. The owner just taps.

  ## 2. Design Doc

  **High-Level Architecture:**
  - **Entities:** `TriageItem`, `AgentDraft`, `SourceMessage`.
  - **Relationships:** A `TriageItem` aggregates a `SourceMessage` and optionally references an `AgentDraft` for quick approval.
  - **UI Screens:** Mobile-first (375px width). A vertical scrolling feed on the home screen. Each item is a translucent card.
  - **UX Flow:**
    1. Owner opens app. Sees "3 items need your attention".
    2. Top card is an unread DM with an AI-suggested reply.
    3. Owner taps "Send". Card swipes away. Confetti animation.
    4. Next card is a low inventory alert.

  ```mermaid
  graph TD
      A[Incoming Channels: Email, DMs, Payments] --> B[AI Triage Agent]
      B --> C{Priority Filter}
      C -->|High Priority| D[Action Feed Top Card]
      C -->|Low Priority| E[Action Feed Below the Fold]
      D --> F[Owner One-Tap Action]
      F --> G[Agent Executes Work]
  ```

  ## 3. Implementation Prompt

  **Critical User Journey (CUJ):**
  As Maya (Home Baker), I want to open the OHC app and immediately see a prioritized list of customer requests and operational alerts, so I can clear my daily backlog in 2 minutes.

  **Acceptance Criteria:**
  1. Implement a responsive Flutter/PWA screen (optimizing for 375px) displaying the Action Feed.
  2. The feed must render distinct card types (Message, Booking, Alert) with distinct visual tokens.
  3. Include an empty state: "All caught up! You're a hero."
  4. E2E Playwright test must verify the cards appear, click interactions trigger state changes, and the feed empties out.
  5. Zero mock data in production—hydrate from backend APIs (can use seeds for tests).

  **Priority:** P0
  **Estimated Scope:** Medium

  ## 4. References & Sources Catalog
  1. Shopify Sidekick Overview: https://www.shopify.com/sidekick
  2. Shopify Winter Editions 2024: https://www.shopify.com/editions/winter2024
  3. Wix Studio AI: https://www.wix.com/studio/ai
  4. Squarespace Blueprint: https://www.squarespace.com/blueprint
  5. Square AI Features: https://squareup.com/us/en/software/ai
  6. HubSpot AI Tools: https://www.hubspot.com/products/artificial-intelligence
  7. WooCommerce AI: https://woocommerce.com/ai/
  8. BigCommerce B2B AI: https://www.bigcommerce.com/articles/b2b/artificial-intelligence/
  9. Weebly Features: https://www.weebly.com/features
  10. PrestaShop eCommerce AI: https://prestashop.com/blog/ai-ecommerce/
  11. Durable 30s Website: https://durable.co/
  12. 10Web AI WordPress: https://10web.io/
  13. Mixo AI Validation: https://mixo.io/
  14. Framer AI Design: https://www.framer.com/ai/
  15. Lindy AI Assistant: https://lindy.ai/
  16. Relevance AI Workforce: https://relevanceai.com/
  17. Skyvern Browser AI: https://skyvern.com/
  18. Tencent Workbuddy: https://www.tencent.com/en-us/business/workbuddy
  19. WeCom Enterprise: https://work.weixin.qq.com/
  20. DingTalk Features: https://www.dingtalk.com/en
  21. Lark Suite Overview: https://www.larksuite.com/
  22. Notion AI Assistant: https://www.notion.so/product/ai
  23. Microsoft Copilot for SMB: https://copilot.microsoft.com/
  24. Reddit r/smallbusiness Shopify Complexity: https://www.reddit.com/r/smallbusiness/comments/18jxlk2/shopify_is_too_complex_for_me/
  25. Reddit r/ecommerce CRM for DMs: https://www.reddit.com/r/ecommerce/comments/19bxyu1/is_there_a_crm_for_instagram_dms/
  26. Reddit Wix vs Squarespace: https://www.reddit.com/r/smallbusiness/comments/17fwqk3/wix_vs_squarespace_for_local_service/
  27. Reddit Invoicing Tools: https://www.reddit.com/r/Entrepreneur/comments/192kqpz/what_tools_do_you_use_to_manage_invoices/
  28. Reddit Handyman Booking Software: https://www.reddit.com/r/sweatystartup/comments/18a2pzc/booking_software_recommendations_for_handyman/
  29. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  30. Trustpilot Wix Reviews: https://www.trustpilot.com/review/www.wix.com
  31. Trustpilot Squarespace Reviews: https://www.trustpilot.com/review/www.squarespace.com
  32. Trustpilot Square Reviews: https://www.trustpilot.com/review/squareup.com
  33. Trustpilot HubSpot Reviews: https://www.trustpilot.com/review/www.hubspot.com
  34. Trustpilot WooCommerce Reviews: https://www.trustpilot.com/review/woocommerce.com
  35. Trustpilot BigCommerce Reviews: https://www.trustpilot.com/review/www.bigcommerce.com
  36. G2 Shopify Reviews: https://www.g2.com/products/shopify/reviews
  37. G2 Wix Reviews: https://www.g2.com/products/wix/reviews
  38. G2 Squarespace Reviews: https://www.g2.com/products/squarespace/reviews
  39. G2 Square POS Reviews: https://www.g2.com/products/square-point-of-sale/reviews
  40. G2 DingTalk Reviews: https://www.g2.com/products/dingtalk/reviews
  41. G2 Lark Reviews: https://www.g2.com/products/lark/reviews
  42. Apple App Store Shopify App: https://apps.apple.com/us/app/shopify-ecommerce-business/id371297841
  43. Apple App Store Wix App: https://apps.apple.com/us/app/wix-owner-website-builder/id1099748482
  44. Apple App Store Square POS: https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  45. Apple App Store DingTalk: https://apps.apple.com/us/app/dingtalk/id930368978
  46. Apple App Store Lark: https://apps.apple.com/us/app/lark-work-together/id1346322902
  47. Google Play Shopify: https://play.google.com/store/apps/details?id=com.shopify.m
  48. Google Play Wix: https://play.google.com/store/apps/details?id=com.wix.android
  49. Google Play Square: https://play.google.com/store/apps/details?id=com.squareup
  50. Google Play DingTalk: https://play.google.com/store/apps/details?id=com.alibaba.android.rimet
  51. Chatbase AI: https://www.chatbase.co/
  52. Cassidy AI: https://www.cassidyai.com/
  53. Artisan AI: https://artisan.co/
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
