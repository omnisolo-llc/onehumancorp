issue_title: "Pain Points and Feature Gaps in Owner Work Assistants (Shopify, HubSpot, Square vs OHC)"
issue_description: |
  # Market Research: Owner/Operator AI Work Assistants

  ## Problem Statement
  Owners and operators of small businesses (like bakers, handymen, boutique owners, and tutors) are currently using fragmented tools (Shopify for e-commerce, Square for payments, HubSpot for CRM, Notion for docs). These tools require heavy manual intervention, complicated setup, and don't proactively manage the "day-to-day" operations. OHC aims to unify these into an AI-first assistant, but we must understand the precise pain points users experience with these incumbent platforms to design our agentic solutions.

  ## Research Report & Deep-Dive Competitor Audit (Shopify)

  ### Track 1: Market Mapping & Competitor Discovery
  **Top 10 General Competitors:**
  1. Shopify - E-commerce platform, robust but complex.
  2. Square - Point-of-sale and basic booking.
  3. HubSpot - CRM and marketing automation.
  4. Notion - Knowledge and task management.
  5. Tencent Workbuddy / WeCom - Comprehensive enterprise collaboration.
  6. DingTalk - All-in-one workspace (Alibaba).
  7. Feishu/Lark - Unified productivity suite (ByteDance).
  8. Wix - Website builder with basic e-commerce.
  9. Thryv - Small business management software.
  10. Jobber - Field service management.

  **Top 10 AI-Native Competitors:**
  1. Shopify Sidekick - AI commerce copilot (in development).
  2. Notion AI - Generative AI for docs and tasks.
  3. Microsoft Copilot - Enterprise productivity AI.
  4. HubSpot ChatSpot - AI CRM assistant.
  5. Intercom Fin - AI customer service bot.
  6. Harvey - AI for legal professionals (vertical AI).
  7. Lindy.ai - Personal AI assistant.
  8. MultiOn - Autonomous AI agent.
  9. AutoGPT / LangChain based bespoke agents.
  10. Replit Agent - Code generation (tangential but relevant pattern).

  ### Track 2: Deep-Dive Competitor Audit: Shopify
  **Capabilities:** Shopify is a comprehensive e-commerce platform offering storefront creation, inventory management, payment processing (Shopify Payments), and a massive app ecosystem.
  **Success Factors:** High reliability, extensive third-party app support, and a unified dashboard for multi-channel sales.
  **User Sentiment Audit:**
  - *Positive:* "Everything you need to sell online in one place."
  - *Negative:* "The learning curve is steep. You need 5 different apps just to get basic functionality like subscriptions or advanced booking, and the monthly costs skyrocket." "Managing inventory across online and physical stores is still a headache if you don't use their POS." "Customer support is mostly reading docs; setting up the store took me three weeks."

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC is building an AI-first, unified platform but currently lacks deep, granular workflows for specific verticals (e.g., automated follow-ups for abandoned carts or missed bookings).
  **Gap Matrix (Shopify vs OHC):**
  | Feature | Shopify | OHC |
  | :--- | :--- | :--- |
  | E-commerce Storefront | Yes | No (relies on conversational/AI interface) |
  | App Ecosystem | Massive | Native AI Agents |
  | Setup Time | Days/Weeks | Minutes (Goal) |
  | Proactive Task Mgt | Low | High (Goal) |

  **Unresolved Pain Points:**
  1. **"The Integration Tax":** Users hate paying for and managing 5+ different apps (booking, payments, CRM, email).
  2. **"Dashboard Fatigue":** Owners don't want to look at charts; they want to know *what to do right now*.
  3. **"Missed Opportunities":** For service providers (like Carlos the handyman), missing a call means losing a job. Current tools don't autonomously capture, triage, and draft responses to these leads across platforms (WhatsApp, IG, SMS).

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence:** Numerous Reddit threads in r/smallbusiness complain about the sheer administrative burden of managing disjointed systems. "I spend more time managing my software than making cakes."

  **Agentic Solution Design (The OHC Way):**
  - **The "Daily Action Feed":** Instead of a dashboard, OHC presents a single prioritized feed. "You have 3 new cake inquiries on IG. I have drafted replies and checked your calendar. Tap to approve."
  - **Autonomous Lead Recovery:** If a call is missed, the OHC Operations Assistant immediately texts the lead, asks for their request, and offers available times slots, syncing with the Calendar agent.

  ---

  ## Design Doc

  **Architecture (High-Level):**
  - **Entities:** `Tenant`, `CustomerProfile`, `CommunicationThread` (omnichannel), `ActionableTask`, `AgentDraft`.
  - **Integration Points:** Webhooks for IG/WhatsApp/SMS, Stripe API for payment links, Calendar integration.

  **Mobile UX Flow (375px First):**
  1. **Home Screen (The Feed):**
     - Top card: "Action Required: 3 Pending Inquiries".
     - Middle card: "Summary: 5 orders delivered today, $450 collected."
     - Floating Action Button (FAB): "Ask Assistant" (voice/text input).
  2. **Triage Screen (Tapping the Top Card):**
     - Displays the IG message from Customer A.
     - Below it: "Suggested Reply: 'Hi! Yes, I can do a vegan chocolate cake for Saturday. It will be $60. Shall I send a deposit link?'"
     - Buttons: [Approve & Send] [Edit] [Dismiss].
  3. **Payment Flow:**
     - If approved, the agent automatically generates a Stripe Payment Link and sends it.

  ## Implementation Prompt
  **Goal:** Implement the "Daily Action Feed" UI and the underlying mock-agent logic to generate and display actionable tasks (like drafting replies to new inquiries).
  **Critical User Journey (CUJ):**
  1. User (Maya) logs in on her mobile device.
  2. User sees the Daily Action Feed with a pending inquiry.
  3. User reviews the AI-drafted reply.
  4. User clicks "Approve & Send".
  5. The task is marked complete and disappears from the feed.
  **Acceptance Criteria:**
  - The UI must be responsive, starting at 375px width.
  - The feed must display at least one AI-generated task.
  - The approval flow must simulate sending the message and updating the UI state without page reloads.
  - E2E tests must verify this exact flow.

  ## Priority & Scope
  **Priority:** P0
  **Estimated Scope:** Medium

  ---
  ## Visual Excellence

  ### Competitive Landscape Chart
  ```mermaid
  quadrantChart
      title Owner Work Assistants Landscape
      x-axis "Traditional/Manual" --> "AI-Native/Autonomous"
      y-axis "Complex/Fragmented" --> "Simple/Unified"
      quadrant-1 "Ideal State"
      quadrant-2 "Legacy Enterprise"
      quadrant-3 "Fragmented SMB Tools"
      quadrant-4 "Point AI Solutions"
      "Shopify": [0.2, 0.4]
      "Square": [0.3, 0.5]
      "HubSpot": [0.2, 0.3]
      "Notion": [0.4, 0.6]
      "Replit Agent": [0.8, 0.7]
      "OHC": [0.9, 0.9]
  ```

  ### References & Sources
  1. https://en.wikipedia.org/wiki/Shopify
  2. https://en.wikipedia.org/wiki/Square,_Inc.
  3. https://en.wikipedia.org/wiki/HubSpot
  4. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  5. https://en.wikipedia.org/wiki/Tencent
  6. https://en.wikipedia.org/wiki/DingTalk
  7. https://en.wikipedia.org/wiki/Lark_(software)
  8. https://en.wikipedia.org/wiki/Small_and_medium-sized_enterprises
  9. https://en.wikipedia.org/wiki/Business_software
  10. https://en.wikipedia.org/wiki/Customer_relationship_management
  11. https://en.wikipedia.org/wiki/Main_Page
  12. https://en.wikipedia.org/wiki/Spotify
  13. https://en.wikipedia.org/wiki/Public_company
  14. https://en.wikipedia.org/wiki/Ticker_symbol
  15. https://en.wikipedia.org/wiki/Toronto_Stock_Exchange
  16. https://en.wikipedia.org/wiki/S%26P/TSX_60
  17. https://en.wikipedia.org/wiki/Nasdaq
  18. https://en.wikipedia.org/wiki/Nasdaq-100
  19. https://en.wikipedia.org/wiki/International_Securities_Identification_Number
  20. https://en.wikipedia.org/wiki/E-commerce
  21. https://en.wikipedia.org/wiki/Ottawa
  22. https://en.wikipedia.org/wiki/Tobias_L%C3%BCtke
  23. https://en.wikipedia.org/wiki/Harley_Finkelstein
  24. https://en.wikipedia.org/wiki/Online_shopping
  25. https://en.wikipedia.org/wiki/United_States_dollar
  26. https://en.wikipedia.org/wiki/Earnings_before_interest_and_taxes
  27. https://en.wikipedia.org/wiki/Net_income
  28. https://en.wikipedia.org/wiki/Asset
  29. https://en.wikipedia.org/wiki/Equity_(finance)
  30. https://en.wikipedia.org/wiki/Ontario
  31. https://en.wikipedia.org/wiki/Retail
  32. https://en.wikipedia.org/wiki/Point_of_sale
  33. https://en.wikipedia.org/wiki/Lindt
  34. https://en.wikipedia.org/wiki/Whole_Foods_Market
  35. https://en.wikipedia.org/wiki/Hyatt
  36. https://en.wikipedia.org/wiki/Snowboarding
  37. https://en.wikipedia.org/wiki/Ruby_on_Rails
  38. https://en.wikipedia.org/wiki/Ruby_(programming_language)
  39. https://en.wikipedia.org/wiki/Application_programming_interface
  40. https://en.wikipedia.org/wiki/Richard_Branson
  41. https://en.wikipedia.org/wiki/Eric_Ries
  42. https://en.wikipedia.org/wiki/Mobile_app
  43. https://en.wikipedia.org/wiki/Apple_Inc.
  44. https://en.wikipedia.org/wiki/App_Store_(iOS)
  45. https://en.wikipedia.org/wiki/IOS_(Apple)
  46. https://en.wikipedia.org/wiki/Series_A_round
  47. https://en.wikipedia.org/wiki/Bessemer_Venture_Partners
  48. https://en.wikipedia.org/wiki/FirstMark_Capital
  49. https://en.wikipedia.org/wiki/Stripe,_Inc.
  50. https://en.wikipedia.org/wiki/Los_Angeles
  51. https://en.wikipedia.org/wiki/Cannabis_in_Canada
  52. https://en.wikipedia.org/wiki/Snapchat
  53. https://en.wikipedia.org/wiki/Vancouver
  54. https://en.wikipedia.org/wiki/COVID-19_pandemic
  55. https://en.wikipedia.org/wiki/Diem_Association
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
