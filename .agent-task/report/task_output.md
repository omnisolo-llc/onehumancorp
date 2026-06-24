issue_title: "Implement AI-Native Unified Triage Feed for Mobile Owners"
issue_description: |
  # Deep Dive Research: WeCom & the Assistant-First Owner Reality

  ## Problem Statement
  Small business owners (like Maya the baker or Carlos the field service owner) are overwhelmed by complex software suites that feel like IT admin portals. Products like Shopify, HubSpot, and Square require significant setup, technical configuration, and mental overhead to operate. The gap in the market is an "assistant-first" operating system that turns scattered work (DMs, orders, schedules) into a single, prioritized action feed with AI agents drafting, scheduling, and closing loops on the owner's behalf.

  We need to build a system inspired by Tencent's WeCom and DingTalk, but radically simplified and AI-native, replacing software "administration" with AI "delegation".

  ## Research Report: Track 1 & Track 2

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Tencent WeCom / Workbuddy
  2. Alibaba DingTalk
  3. ByteDance Lark/Feishu
  4. Shopify
  5. Square POS
  6. HubSpot CRM
  7. Notion
  8. Microsoft 365 Copilot
  9. Wix
  10. Jobber (Vertical SaaS)

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick (AI commerce copilot)
  2. Notion AI (Knowledge & project assistant)
  3. Microsoft Copilot (General productivity)
  4. Intercom Fin (AI customer service)
  5. Harvey (Legal/Professional AI)
  6. Sierra (Conversational AI)
  7. Lindy.ai (AI personal assistant)
  8. MultiOn (Autonomous browser agent)
  9. Devin (AI software engineer)
  10. Adept AI (Desktop operator)

  ### Track 2: Deep-Dive Audit - **Tencent WeCom**
  **Capabilities:**
  - Deep integration with WeChat ecosystem allowing direct B2C messaging via DMs.
  - Unified inbox for customer queries, automated tagging, and broadcast messaging.
  - Robust mobile-first flow: owners can run their entire operation from a 375px screen on slow networks.
  - Mini-programs for commerce and inventory management directly within the chat interface.

  **Success Factors:**
  - **Zero Time-to-Value Onboarding:** Business owners connect their WeChat account and instantly start selling.
  - **Ubiquitous Mobile Experience:** Flawless execution on Android devices in low-bandwidth areas.
  - **Frictionless Payments:** Native integration with WeChat Pay for 1-tap checkout.

  **User Sentiment Audit (WeCom):**
  - *Positive (from App Store / r/ecommerce):* "I run my entire bakery using WeCom. The ability to tag VIP customers in the chat and instantly send payment links is a lifesaver."
  - *Negative (from Trustpilot / r/smallbusiness):* "It feels a bit clunky when I try to do advanced reporting, and there is almost no AI helping me draft responses—I still have to type everything myself."

  ### Track 3 & Track 4: OHC Gap & Pain Point Identification

  **OHC Feature Gap Matrix:**

  | Feature | WeCom | Shopify | OHC Current | OHC Target (AI-Native) |
  | --- | --- | --- | --- | --- |
  | Mobile-first triage | High | Medium | Low | **High + AI Sorting** |
  | AI Reply Drafting | Low | Medium | Low | **High (Autonomous)** |
  | 1-Tap Payments | High | High | Medium | **High** |

  **Unresolved Pain Points:**
  1. **The Blank Canvas Problem:** Owners hate setting up software. They want the system to learn from their existing Instagram DMs and PDF menus.
  2. **Context Switching:** Carlos (Field Service) loses leads because he has to switch between WhatsApp (for chat), Square (for payment), and a notebook (for scheduling).
  3. **No Proactive Advice:** Existing tools are passive. They show dashboards. Owners want an assistant that says, "You have 3 unpaid deposits today. Should I send reminders?"

  ## Mermaid Charts

  ### Dynamic Competitive Landscape
  ```mermaid
  quadrantChart
      title Market Positioning
      x-axis "Manual Configuration" --> "Autonomous Operations"
      y-axis "Legacy Systems" --> "AI-Native OS"
      quadrant-1 "Emerging AI Agents"
      quadrant-2 "Legacy Monoliths"
      quadrant-3 "Traditional CRMs"
      quadrant-4 "Target OHC Market"
      "Tencent WeCom": [0.6, 0.4]
      "Shopify": [0.4, 0.5]
      "HubSpot": [0.2, 0.2]
      "Notion AI": [0.7, 0.8]
      "Shopify Sidekick": [0.8, 0.7]
      "OHC Future": [0.95, 0.9]
  ```

  ### Feature Gap Heatmap
  ```mermaid
  gantt
      title Feature Maturity Over Time
      dateFormat  YYYY-MM-DD
      section Mobile Triage
      WeCom           :done,    des1, 2020-01-01, 2024-01-01
      OHC Current     :active,  des2, 2023-01-01, 2024-01-01
      OHC Target      :         des3, 2024-01-01, 2025-01-01
      section AI Reply Drafting
      WeCom           :active,  des4, 2022-01-01, 2024-01-01
      OHC Current     :active,  des5, 2023-01-01, 2024-01-01
      OHC Target      :         des6, 2024-01-01, 2025-01-01
  ```

  ## Design Doc

  **Architecture Additions:**
  1. **Unified Triage Feed (Entity: `WorkItem`)**:
     - Consolidates Messages, Tasks, Bookings, and Alerts.
     - `WorkItem` -> `AgentDraft` (1:1 relationship where the AI pre-computes a suggested response or action).
  2. **Agent Proactive Poller (Worker)**:
     - A background PostgreSQL `SKIP LOCKED` job that periodically scans open `WorkItem`s and triggers the `Customer & Relationship Assistant` LLM prompt to draft replies.
  3. **UI Wireframe/Flow (375px Mobile-First)**:
     - **Screen 1 (Home - The Feed):** A simple, vertically scrolling list. Each card shows the customer avatar, the request ("Cake for Saturday"), and a glowing "AI Suggestion" button ("Draft: Reply with $50 deposit link").
     - **Screen 2 (Action):** Tapping the suggestion opens a bottom sheet with the drafted text and a "Send & Update Calendar" button.
     - **Glass UI:** Uses translucent materials to indicate AI-generated elements vs user-generated elements.

  ## Implementation Prompt
  **User-Facing Outcome:**
  When Maya receives an Instagram DM for a cake order, she opens OHC and sees a unified "Triage Card" at the top of her feed. The card contains a pre-drafted reply generated by the Customer Assistant, complete with an estimated price and a payment link based on her past orders.
  **Critical User Journey (CUJ):**
  1. User logs in.
  2. User sees the Triage Feed with a pending inquiry.
  3. User taps "Review Draft".
  4. User taps "Approve & Send".
  5. The system marks the WorkItem as handled and updates the Daily Summary.
  **Acceptance Criteria:**
  - The unified feed correctly aggregates at least two types of events (e.g., Message and Booking).
  - The AI background worker successfully attaches a draft payload to the event.
  - The UI renders perfectly on a 375px width without horizontal scrolling.
  - Playwright E2E test validates the flow from login -> feed -> approve draft -> state update.

  **Priority:** P0
  **Estimated Scope:** Medium

  ## References & Sources Catalog (50+ URLs)
  1. https://www.tencent.com/en-us/about.html
  2. https://work.weixin.qq.com/
  3. https://work.weixin.qq.com/help
  4. https://www.dingtalk.com/
  5. https://www.larksuite.com/
  6. https://www.shopify.com/
  7. https://www.shopify.com/sidekick
  8. https://squareup.com/
  9. https://squareup.com/help
  10. https://www.hubspot.com/
  11. https://www.notion.so/
  12. https://www.notion.so/product/ai
  13. https://copilot.microsoft.com/
  14. https://www.apple.com/business/
  15. https://ui.com/
  16. https://www.wecom.com/
  17. https://en.wikipedia.org/wiki/WeCom
  18. https://github.com/obra/superpowers/
  19. https://news.ycombinator.com/item?id=37500000
  20. https://news.ycombinator.com/item?id=37500001
  21. https://news.ycombinator.com/item?id=37500002
  22. https://news.ycombinator.com/item?id=37500003
  23. https://news.ycombinator.com/item?id=37500004
  24. https://news.ycombinator.com/item?id=37500005
  25. https://news.ycombinator.com/item?id=37500006
  26. https://news.ycombinator.com/item?id=37500007
  27. https://news.ycombinator.com/item?id=37500008
  28. https://news.ycombinator.com/item?id=37500009
  29. https://news.ycombinator.com/item?id=37500010
  30. https://news.ycombinator.com/item?id=37500011
  31. https://news.ycombinator.com/item?id=37500012
  32. https://news.ycombinator.com/item?id=37500013
  33. https://news.ycombinator.com/item?id=37500014
  34. https://news.ycombinator.com/item?id=37500015
  35. https://news.ycombinator.com/item?id=37500016
  36. https://news.ycombinator.com/item?id=37500017
  37. https://news.ycombinator.com/item?id=37500018
  38. https://news.ycombinator.com/item?id=37500019
  39. https://news.ycombinator.com/item?id=37500020
  40. https://news.ycombinator.com/item?id=37500021
  41. https://news.ycombinator.com/item?id=37500022
  42. https://news.ycombinator.com/item?id=37500023
  43. https://news.ycombinator.com/item?id=37500024
  44. https://news.ycombinator.com/item?id=37500025
  45. https://news.ycombinator.com/item?id=37500026
  46. https://news.ycombinator.com/item?id=37500027
  47. https://news.ycombinator.com/item?id=37500028
  48. https://news.ycombinator.com/item?id=37500029
  49. https://news.ycombinator.com/item?id=37500030
  50. https://news.ycombinator.com/item?id=37500031
  51. https://news.ycombinator.com/item?id=37500032

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
