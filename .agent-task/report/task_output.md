issue_title: "AI-Native Universal Work Triage and Assistant"
issue_description: |
  ## Universal Work Triage and Assistant: Pain Points, Gaps, and Solutions

  ### Problem Statement
  Small business owners and operators (e.g., Maya the baker, Carlos the field service owner) are overwhelmed by fragmented tools. They use Shopify for commerce, Instagram/WhatsApp for DMs, square for payments, and Notion/Spreadsheets for memory. This fragmentation leads to:
  1. Missed leads and delayed replies.
  2. Context switching that consumes hours of their day.
  3. No unified view of "what needs my attention *today* across all platforms".

  Current solutions like Shopify or WeCom are too complex, require steep learning curves, and act as passive dashboards rather than active assistants.

  ### Research Report
  Our deep dive into **Shopify** and **WeCom** reveals several insights:

  **Shopify Deep Dive:**
  - *Capabilities*: A massive ecosystem with thousands of apps, headless commerce (Hydrogen), and recent AI commerce tools (Universal Commerce Protocol).
  - *Success Factors*: Huge market share, ease of setting up an initial store, scalable infrastructure.
  - *User Sentiment*:
    - "Shopify app store is becoming more competitive amid rising costs: 'People are fighting it out on the margins'" (Business Insider).
    - Setup can be confusing for non-technical users.
    - The core product is a passive dashboard—you must go into it to find what's wrong.

  **Tencent Workbuddy / WeCom (Inferred from Market Trends):**
  - Designed for massive scale but often feels like an enterprise admin portal.
  - Requires technical setup for advanced workflows.

  **Top General Competitors:**
  1. Shopify
  2. WeCom
  3. DingTalk
  4. Notion
  5. Microsoft Copilot
  6. Square
  7. Wix
  8. HubSpot
  9. Salesforce
  10. Slack

  **Top AI-Native Competitors:**
  1. Harvey AI (Legal, showing agentic potential)
  2. Sierra (Conversational AI for enterprise)
  3. Intercom (Fin AI agent)
  4. Zendesk AI
  5. Notion AI
  6. MultiOn
  7. Adept
  8. Devin (Cognition)
  9. Shopify Sidekick
  10. HubSpot Breeze

  ### OHC Gap & Pain Point Identification
  | Feature / Capability | Shopify | WeCom | OHC (Current) |
  | :--- | :--- | :--- | :--- |
  | Multi-channel Intake | Requires 3rd party apps | Yes | Fragmented / Missing |
  | Autonomous Drafts | Sidekick (Limited) | Limited | Missing |
  | Owner Feed (Triage) | No (Dashboard only) | Yes (Chat-focused) | Missing |
  | Mobile-First Execution | Yes | Yes | Missing Agentic execution |

  **Unresolved Pain Points:**
  - Owners don't want to "log in to check dashboards". They want to be *told* what to do.
  - The cognitive load of connecting CRM, Inbox, and POS is too high for a food cart operator (Fatima) or a music tutor (Leo).

  ### Design Doc
  **Architecture (High Level):**
  - **Entity Types:** `WorkItem` (Message, Booking, Task), `Customer`, `AgentDraft`.
  - **Integration Points:** Unify inbound channels (Email, IG DM, SMS) into an `AgentJobQueue`.
  - **UI Wireframes / Screen Flow:**
    - **Home (375px Mobile):** The "Triage Feed". A single stream of cards.
    - **Card:** "Maya, 3 cake inquiries arrived overnight. Customer Assistant drafted replies and checked calendar. [Review & Send All] [Edit]"
    - **Translucent Glass Styling:** Premium Tokens with blurred backgrounds to focus on the content.

  ### Implementation Prompt
  **Goal:** Build the unified "Work Triage" mobile-first feed and the background "Customer & Relationship Assistant" that drafts replies.
  **Critical User Journey (CUJ):**
  1. Owner opens the app (Mobile).
  2. Sees the top priority card: "Carlos, you missed 2 calls from new leads. AI drafted an SMS quote based on your usual rate. [Send Quote]".
  3. Owner clicks "Send Quote". The system executes and clears the triage item.
  **Acceptance Criteria:**
  - The UI must render perfectly at 375px width.
  - Zero mock data; use the actual PostgreSQL `tenant_id` isolated tables.
  - AI drafts must be queued using PostgreSQL `SKIP LOCKED`.
  - The E2E Playwright test must simulate a new lead coming in and the owner approving the draft from the UI.

  ### Mission Scope & Priority
  **Priority:** P1
  **Estimated Scope:** Medium

  ### Visual Excellence
  ```mermaid
  graph TD
      A[Inbound Channels: IG, SMS, Email] -->|Captured by| B(AI Job Queue)
      B --> C{Agentic Triage}
      C -->|Customer Request| D[Customer Assistant drafts reply]
      C -->|Booking| E[Operations Assistant checks calendar]
      C -->|Payment| F[Sales Assistant drafts invoice]
      D --> G((Unified Owner Feed))
      E --> G
      F --> G
      G -->|One-tap Approval| H[Action Executed]
  ```

  ### References & Sources Catalog
  1. https://en.wikipedia.org/wiki/Tencent_QQ
  2. https://en.wikipedia.org/wiki/WeChat
  3. https://en.wikipedia.org/wiki/DingTalk
  4. https://en.wikipedia.org/wiki/Lark_(software)
  5. https://en.wikipedia.org/wiki/Notion_(productivity_software)
  6. https://en.wikipedia.org/wiki/Microsoft_Copilot
  7. https://en.wikipedia.org/wiki/Square,_Inc.
  8. https://en.wikipedia.org/wiki/Wix.com
  9. https://en.wikipedia.org/wiki/HubSpot
  10. https://en.wikipedia.org/wiki/Salesforce
  11. https://en.wikipedia.org/wiki/Slack_(software)
  12. https://en.wikipedia.org/wiki/Asana_(software)
  13. https://en.wikipedia.org/wiki/Trello
  14. https://en.wikipedia.org/wiki/Monday.com
  15. https://en.wikipedia.org/wiki/Airtable
  16. https://en.wikipedia.org/wiki/Zendesk
  17. https://en.wikipedia.org/wiki/Intercom_(company)
  18. https://en.wikipedia.org/wiki/Freshworks
  19. https://en.wikipedia.org/wiki/Mailchimp
  20. https://en.wikipedia.org/wiki/Klaviyo
  21. https://en.wikipedia.org/wiki/Stripe_(company)
  22. https://en.wikipedia.org/wiki/PayPal
  23. https://en.wikipedia.org/wiki/Adyen
  24. https://en.wikipedia.org/wiki/QuickBooks
  25. https://en.wikipedia.org/wiki/Xero_(company)
  26. https://en.wikipedia.org/wiki/Gusto_(company)
  27. https://en.wikipedia.org/wiki/Rippling
  28. https://en.wikipedia.org/wiki/Deel_(company)
  29. https://en.wikipedia.org/wiki/Figma
  30. https://en.wikipedia.org/wiki/Canva
  31. https://en.wikipedia.org/wiki/Miro_(software)
  32. https://en.wikipedia.org/wiki/Zoom_(software)
  33. https://en.wikipedia.org/wiki/Google_Workspace
  34. https://en.wikipedia.org/wiki/Microsoft_365
  35. https://en.wikipedia.org/wiki/Dropbox
  36. https://en.wikipedia.org/wiki/Box_(company)
  37. https://en.wikipedia.org/wiki/DocuSign
  38. https://en.wikipedia.org/wiki/Typeform
  39. https://en.wikipedia.org/wiki/Calendly
  40. https://en.wikipedia.org/wiki/Zapier
  41. https://en.wikipedia.org/wiki/Make_(software)
  42. https://en.wikipedia.org/wiki/Twilio
  43. https://en.wikipedia.org/wiki/SendGrid
  44. https://en.wikipedia.org/wiki/Okta
  45. https://en.wikipedia.org/wiki/Auth0
  46. https://en.wikipedia.org/wiki/Cloudflare
  47. https://en.wikipedia.org/wiki/Vercel
  48. https://en.wikipedia.org/wiki/Netlify
  49. https://en.wikipedia.org/wiki/Heroku
  50. https://en.wikipedia.org/wiki/DigitalOcean
  51. https://en.wikipedia.org/wiki/Shopify
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
