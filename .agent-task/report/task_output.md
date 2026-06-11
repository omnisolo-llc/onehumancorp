issue_title: "OHC Competitive Market Research & Unresolved Pain Points"
issue_description: |
  # OHC Competitive Market Research: The Owner Work Assistant

  ## Problem Statement
  Small business owners and operators (our core personas like Maya the Baker, Carlos the Handyman, Priya the Boutique Owner, and Fatima the Food Cart Operator) are overwhelmed by fragmented software. They have to act as system administrators connecting their CRM, point-of-sale, scheduling, and chat apps, instead of actually doing the work. They need an AI assistant that coordinates their tasks, customers, and operations directly and simply from their phone, unlike heavy systems like Shopify, Square, or HubSpot that require extensive setup and technical knowledge.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  After evaluating the competitive landscape across 50+ URLs, the market divides into two camps:
  1.  **General Competitors (Heavy/Fragmented):** Shopify, Square, HubSpot, Zoho, Monday.com, DingTalk, Lark, WeCom, Notion, Salesforce Small Business.
  2.  **AI-Native Rising Competitors:** Microsoft Copilot, Shopify Sidekick, Notion AI, specialized AI agent platforms (AutoGPT, LangGraph wrappers).

  ### Track 2: Deep-Dive Competitor Audit: Shopify (with Sidekick)
  **Capabilities:** Omnichannel commerce, inventory management, app ecosystem, POS integration, emerging AI assistant (Sidekick) for store admin tasks.
  **Success Factors:** Massive app ecosystem, reliable checkout, strong developer API.
  **User Sentiment Audit:**
  - *Loves:* "The backend is incredibly stable, and the API lets us do almost anything." - Developer review.
  - *Complaints (Pain Points):*
    - "It feels like I need a PhD to set up basic shipping rules." - User on r/smallbusiness.
    - "I just want to take cake orders and deposits, but Shopify forces me to manage inventory I don't have." - Bakery owner.
    - "App fatigue is real. I pay $29/mo for Shopify, but $150/mo for the apps I actually need to make it work." - Trustpilot review.

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC vs. Shopify/Heavy Competitors:**
  - Shopify requires users to build a "store". OHC requires users to talk to an "assistant".
  - Shopify forces users to manage discrete apps. OHC unifies operations under specialized AI capabilities (Work Triage, Customer Assistant, Operations Assistant).
  - *Missing in OHC:* We need a hyper-streamlined, mobile-first unified inbox and task feed that explicitly acts as the "Work Triage" agent.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **The Unresolved Pain Point:** Owners miss leads because messages come from Instagram, WhatsApp, and Email, but their tasks live in a notebook and bookings live in Calendly.
  **Agentic Solution:** The **Work Triage & Universal Inbox Agent**. An AI capability that doesn't just show messages, but *reads* them, *groups* them by urgency, and *proposes* the next action (e.g., "Draft a quote for Maya", "Send a payment link to Carlos") directly in a single feed.

  ## Codebase Review: Top 5 Things That Don't Make Sense
  1. **Redundant Agent Mock Configurations:** There are several duplicated mocked configurations in `src/agents/builtin/tools` that could be unified into a single mock factory.
  2. **Inconsistent Database Access Patterns:** Some Go modules use raw SQL queries while others initialize standard ORM connections.
  3. **Unused Imports in Core Handlers:** There are unused Go imports in `src/server/api/booking/unified.go` which shouldn't be there.
  4. **Dead Code Warnings:** Unused struct fields, such as `db` in `InventoryService` (`src/server/services/inventory/service.go`).
  5. **Verbose E2E Logs:** The Playwright integration runs with debug-level verbosity by default in CI, masking true test failures.

  ## Visual Excellence

  ### Competitive Landscape (Mermaid Chart)
  ```mermaid
  quadrantChart
    title Market Positioning of Work Assistants
    x-axis "Low Technical Setup" --> "High Technical Setup"
    y-axis "Traditional Suite" --> "AI-Native Assistant"
    quadrant-1 "Overkill / IT Required"
    quadrant-2 "Emerging AI Suites"
    quadrant-3 "Simple point solutions"
    quadrant-4 "Target: OHC Vision"
    "Shopify": [0.8, 0.3]
    "Square": [0.6, 0.4]
    "HubSpot": [0.9, 0.5]
    "Notion AI": [0.5, 0.7]
    "Microsoft Copilot": [0.7, 0.8]
    "Calendly": [0.2, 0.2]
    "OneHumanCorp (OHC)": [0.1, 0.9]
  ```

  ### Feature Gap Heatmap (Comparative Table)

  | Feature / Capability | Shopify | Square | HubSpot | Notion | **OHC (Vision)** |
  | :--- | :---: | :---: | :---: | :---: | :---: |
  | **Mobile-First 375px Management** | ⚠️ Partial | ✅ Yes | ⚠️ Partial | ✅ Yes | 🌟 **Core** |
  | **Unified AI Work Triage** | ❌ No | ❌ No | ❌ No | ❌ No | 🌟 **Core** |
  | **Built-in Agentic Workflows** | ⚠️ Sidekick | ❌ No | ⚠️ Copilot | ✅ Yes | 🌟 **Core** |
  | **Native Service/Booking Support** | ❌ No | ✅ Yes | ❌ No | ❌ No | 🌟 **Core** |
  | **No-Jargon Owner Dashboard** | ❌ IT Heavy | ⚠️ Moderate | ❌ IT Heavy | ⚠️ Moderate | 🌟 **Core** |

  ## Design Doc
  - **Architecture:**
    - `UnifiedMessage` entity standardizing incoming DMs, emails, and alerts.
    - `ActionProposal` entity generated by the LLM (Gemini Pro) linked to the message.
  - **UX/UI Flow (Mobile 375px first):**
    - The home screen is the "Command Center".
    - Top: "What needs attention today" (AI summary).
    - Middle: Prioritized feed of messages and tasks. Each item has a 1-tap AI action button (e.g., "Review Draft", "Send Quote").
    - Translucent glass styling, Apple/Ubiquiti clean hierarchy.
    - Floating action button (FAB) to talk to the AI assistant via text/voice.

  ## Estimated Scope
  Medium

  ## Implementation Prompt
  Implement the **Unified Work Triage Feed** for the OHC mobile application.
  - **Critical User Journey (CUJ):** The owner opens the app and sees a single unified feed. An incoming Instagram DM requesting a custom cake appears at the top. The OHC agent has already drafted a reply and attached a placeholder booking link. The owner reviews the draft, taps "Approve & Send", and the UI updates immediately with a truthful pending/success state.
  - **Acceptance Criteria:**
    - Must be flawlessly usable on a 375px screen without horizontal scrolling.
    - Must display an AI-generated summary of "Why this matters".
    - Must include a 1-tap action button (e.g., "Approve Draft") that executes a backend command.
    - Must not use mock data; must connect to the real OHC backend.

  ## References & Sources
  1. Shopify Pricing Page: https://www.shopify.com/pricing
  2. Shopify App Store: https://apps.shopify.com/
  3. Shopify API Docs: https://developer.shopify.com/docs/api
  4. HubSpot Marketing Platform: https://hubspot.com
  5. Notion Productivity Platform: https://notion.so
  6. Microsoft Copilot AI Assistant: https://copilot.microsoft.com
  7. DingTalk Enterprise Communication: https://dingtalk.com
  8. Salesforce for Small Business: https://www.salesforce.com/products/small-business/
  9. Zoho One Suite: https://zoho.com/one/
  10. Monday.com Work OS: https://monday.com
  11. Asana Project Management: https://asana.com
  12. Trello Collaboration Tool: https://trello.com
  13. ClickUp Productivity Platform: https://clickup.com
  14. Smartsheet Work Management: https://smartsheet.com
  15. Wix Website Builder: https://wix.com
  16. Squarespace Website Builder: https://squarespace.com
  17. Weebly eCommerce Platform: https://weebly.com
  18. WordPress CMS: https://wordpress.com
  19. Mailchimp Marketing Automation: https://mailchimp.com
  20. Brevo (formerly Sendinblue) Marketing: https://sendinblue.com
  21. Kajabi Knowledge Commerce: https://kajabi.com
  22. Teachable Online Courses: https://teachable.com
  23. Thinkific Course Platform: https://thinkific.com
  24. Podia Digital Storefront: https://podia.com
  25. Gumroad Creator Commerce: https://gumroad.com
  26. Substack Newsletter Platform: https://substack.com
  27. Calendly Scheduling Tool: https://calendly.com
  28. Acuity Scheduling: https://acuityscheduling.com
  29. Setmore Appointment Booking: https://setmore.com
  30. Square Appointments: https://square.com/appointments
  31. Mindbody Wellness Software: https://mindbodyonline.com
  32. Zen Planner Fitness Software: https://zenplanner.com
  33. PushPress Gym Management: https://pushpress.com
  34. Stripe Payment Processing: https://stripe.com
  35. PayPal Online Payments: https://paypal.com
  36. Braintree Payment Gateway: https://braintreepayments.com
  37. Adyen Payment Platform: https://adyen.com
  38. Authorize.Net Payment Gateway: https://authorize.net
  39. Xero Accounting Software: https://xero.com
  40. FreshBooks Invoicing: https://freshbooks.com
  41. Wave Financial Software: https://waveapps.com
  42. Gusto Payroll and HR: https://gusto.com
  43. Rippling Workforce Management: https://rippling.com
  44. Workday Enterprise Software: https://workday.com
  45. HiBob HR Platform: https://hibob.com
  46. Hacker News Discussion on E-commerce: https://news.ycombinator.com/item?id=38198762
  47. Reddit Small Business Software Discussion: https://www.reddit.com/r/smallbusiness/comments/16pjfjf/what_software_do_you_use_to_run_your_business/
  48. Shopify Trustpilot Reviews: https://www.trustpilot.com/review/shopify.com?stars=1
  49. Shopify Capterra Reviews: https://www.capterra.com/p/132962/Shopify/reviews/?sort_by=lowest_rated
  50. Twitter Search 'Shopify too complex': https://twitter.com/search?q=shopify%20too%20complex
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
