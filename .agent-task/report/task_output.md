issue_title: "Implement Work Triage Assistant for Unified Operator View"
issue_description: |
  # OHC Work Triage Assistant: Unifying the Operator's Daily Work

  ## Problem Statement
  Owners and operators like Maya (Home Baker) and Carlos (Field Service Owner) are overwhelmed by scattered tools. They receive inquiries via Instagram DMs, SMS, emails, and website forms. Their day is a chaotic mix of answering messages, remembering follow-ups, and managing bookings across different systems. There is no central, intelligent place that tells them: **"What needs my attention right now, why does it matter, and what should I do next?"** Current tools force the operator to act as the router between a CRM, an inbox, and a calendar.

  ## Research Report & Deep Dive (Track 1 & Track 2)
  ### Competitor Landscape & Selection
  In the current market of work assistants and business tools, several giants dominate:
  1. **DingTalk (Alibaba)**: Extremely powerful operations and HR focus, deeply embedded in the Asian market. Often feels heavy and corporate for a solopreneur.
  2. **WeCom (Tencent)**: Deep WeChat integration, excellent for customer relationship management, but primarily functions as a corporate overlay.
  3. **Lark / Feishu (ByteDance)**: Incredible unified document, chat, and calendar experience. Very strong for teams, but less focused on the solo-operator handling external B2C commerce.
  4. **Shopify Sidekick (Shopify)**: Strong AI commerce copilot, but highly constrained to the Shopify e-commerce ecosystem. Not great for service businesses.
  5. **Square Dashboard**: Good for physical retail and services, but the "assistant" aspect is mostly reporting, not proactive triage.
  6. **HubSpot**: Powerful CRM, but too complex and expensive for a 1-person cart operator or home baker.

  ```mermaid
  quadrantChart
      title Competitor Landscape
      x-axis "Low AI Focus" --> "High AI Focus"
      y-axis "Corporate Focus" --> "Solo-Operator Focus"
      quadrant-1 "Ideal OHC Zone"
      quadrant-2 "Legacy Solo Tools"
      quadrant-3 "Legacy Enterprise"
      quadrant-4 "AI Enterprise"
      "DingTalk": [0.3, 0.2]
      "Lark": [0.6, 0.3]
      "WeCom": [0.4, 0.3]
      "Shopify Sidekick": [0.8, 0.8]
      "HubSpot": [0.5, 0.4]
      "Square AI": [0.7, 0.6]
      "OHC (Target)": [0.9, 0.9]
  ```

  ### Deep Dive Selection: Lark (Feishu)
  - **Capabilities**: Lark unifies chat, docs, calendars, and tasks into a single feed. It excels at breaking down silos between "where we talk" and "where we work."
  - **Success Factors**: Its core success is context preservation. When you chat about a doc, the doc is in the chat. When you create a task, it's linked to the meeting. It minimizes context switching.
  - **Pain Points (from user sentiment & SMB context)**: Lark is built for internal company collaboration, not external customer triage. For Carlos or Maya, they don't need to collaborate on a doc with 5 employees; they need to turn 10 random DMs into 3 booked jobs and 2 paid invoices. Lark is too "office-worker" and not enough "front-line operator."

  ## Gap Analysis & Agentic Solution (Track 3 & 4)
  ### OHC Feature Gap Matrix
  | Feature | Lark / Feishu | Shopify Sidekick | OHC (Current) | OHC (Target Vision) |
  |---------|--------------|------------------|---------------|---------------------|
  | Internal Chat | Excellent | None | None | Basic |
  | External Triage | Poor | E-com Only | Missing | **Best in Class** |
  | AI Action Drafts | Good (Docs) | Good (Store) | Missing | **Automated** |
  | Mobile Experience | High-Friction | Excellent | Good | **Native-Feel PWA** |

  **The OHC Gap**: OHC currently lacks a unified "Work Triage" feed that intelligently surfaces external demand and proposes actions.

  **The Solution: AI Work Triage Feed**
  Instead of distinct tabs for Messages, Invoices, and Bookings, OHC needs a unified "Triage" view (the default home screen). The AI (Gemini) ingests all incoming signals (a new DM, a failed payment, a booking request) and presents them as prioritized, actionable cards.

  ## Design Doc
  ### Mobile-First UI Flow (375px)
  1. **Home Screen (The Triage Feed)**: A vertically scrolling feed of actionable cards. No horizontal scrolling.
  2. **Triage Card Component**:
     - **Header**: Source (e.g., "Instagram DM", "Stripe Alert") and urgency indicator.
     - **Body**: The core issue (e.g., Customer inquiry text, or "Invoice #102 is overdue").
     - **AI Summary/Action**: A highlighted box where the AI explains why this matters.
     - **Action Buttons**: 1-2 primary, high-contrast buttons (e.g., "Review Draft", "Approve Booking"). All targets >= 44x44px.
  3. **Action Modal**: Tapping a button opens a bottom-sheet modal to review the AI's drafted action before confirming.

  ## Implementation Prompt
  **Critical User Journey (CUJ)**
  1. The user (Maya) opens the OHC app on her phone.
  2. The home screen immediately shows a Triage Card for a new inquiry.
  3. The card includes the customer's message and an AI-drafted response with a booking link.
  4. Maya reviews the draft, taps "Send", and the card disappears, moving to a "Completed" state.

  **Acceptance Criteria**
  - Create a new unified Feed UI (mobile-optimized) that displays actionable items.
  - Ensure the feed renders correctly at 375px width.
  - Implement the "Triage Card" component with clear typography and 44x44px minimum touch targets.
  - Demonstrate a flow where tapping an action button reveals a drafted response or proposed state change, and confirming it updates the UI.

    ## Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Medium

  ## References & Sources
  1. https://en.wikipedia.org/wiki/DingTalk
  2. https://en.wikipedia.org/wiki/Lark_(software)
  3. https://en.wikipedia.org/wiki/WeCom
  4. https://www.reddit.com/r/smallbusiness/
  5. https://www.trustpilot.com/review/www.larksuite.com
  6. https://www.trustpilot.com/review/dingtalk.com
  7. https://apps.apple.com/us/app/lark-work-together/id1456277259
  8. https://apps.apple.com/us/app/dingtalk/id930368978
  9. https://en.wikipedia.org/wiki/Shopify
  10. https://www.trustpilot.com/review/www.shopify.com
  11. https://apps.apple.com/us/app/shopify-point-of-sale-pos/id605663731
  12. https://en.wikipedia.org/wiki/Square,_Inc.
  13. https://www.trustpilot.com/review/squareup.com
  14. https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  15. https://en.wikipedia.org/wiki/HubSpot
  16. https://www.trustpilot.com/review/www.hubspot.com
  17. https://apps.apple.com/us/app/hubspot/id1104655618
  18. https://www.reddit.com/r/ecommerce/
  19. https://www.reddit.com/r/freelance/
  20. https://www.reddit.com/r/Entrepreneur/
  21. https://www.reddit.com/r/macapps/
  22. https://www.reddit.com/r/startups/
  23. https://www.reddit.com/r/SideProject/
  24. https://www.trustpilot.com/review/wecom.qq.com
  25. https://apps.apple.com/us/app/wecom/id1382218048
  26. https://en.wikipedia.org/wiki/Microsoft_Copilot
  27. https://www.trustpilot.com/review/microsoft.com
  28. https://apps.apple.com/us/app/microsoft-copilot/id6472538445
  29. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  30. https://www.trustpilot.com/review/notion.so
  31. https://apps.apple.com/us/app/notion-notes-docs-tasks/id1232780281
  32. https://www.capterra.com/p/180630/Lark/
  33. https://www.capterra.com/p/178523/DingTalk/
  34. https://www.capterra.com/p/146033/Shopify/
  35. https://www.capterra.com/p/124747/Square-Point-of-Sale/
  36. https://www.capterra.com/p/101886/HubSpot-CRM/
  37. https://www.capterra.com/p/181467/Notion/
  38. https://www.g2.com/products/lark/reviews
  39. https://www.g2.com/products/dingtalk/reviews
  40. https://www.g2.com/products/shopify/reviews
  41. https://www.g2.com/products/square-point-of-sale/reviews
  42. https://www.g2.com/products/hubspot-sales-hub/reviews
  43. https://www.g2.com/products/notion/reviews
  44. https://www.getapp.com/collaboration-software/a/lark/
  45. https://www.getapp.com/collaboration-software/a/dingtalk/
  46. https://www.getapp.com/ecommerce-software/a/shopify/
  47. https://www.getapp.com/retail-software/a/square-point-of-sale/
  48. https://www.getapp.com/customer-management-software/a/hubspot-crm/
  49. https://www.getapp.com/project-management-planning-software/a/notion/
  50. https://www.softwareadvice.com/crm/hubspot-crm-profile/
  51. https://www.softwareadvice.com/retail/square-profile/
  52. https://www.softwareadvice.com/ecommerce/shopify-profile/
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, ui, feature]
assignees: []
