issue_title: "OHC Mission Research: Deep Dive into Square POS & AI Competitors"
issue_description: |
  ## OneHumanCorp (OHC) Market Research & Gap Analysis

  ### 1. Market Mapping & Competitor Discovery

  #### Top 10 General Competitors:
  1. **Square POS:** Dominant in small business payments and all-in-one hardware ecosystems.
  2. **Shopify:** The standard for e-commerce, increasingly adding AI features.
  3. **WeCom (Tencent):** Ubiquitous in China, highly integrated with WeChat for B2B/B2C communications.
  4. **DingTalk (Alibaba):** Comprehensive communication and collaboration platform.
  5. **Lark (ByteDance):** All-in-one suite combining chat, calendar, docs, and project management.
  6. **HubSpot:** Powerful CRM and marketing platform moving downmarket with starter tiers.
  7. **Notion:** Workspace collaboration with aggressive AI rollouts.
  8. **Microsoft Copilot / M365:** Heavy enterprise presence, bringing AI directly into existing workflows.
  9. **Wix:** Website builder expanding into business management and studio tools.
  10. **Zoho One:** The "operating system for business" offering a massive suite of integrated apps.

  #### Top 10 AI-Native Competitors & Assistants:
  1. **Shopify Sidekick:** AI commerce copilot directly in the Shopify admin.
  2. **Salesforce Einstein:** AI embedded into CRM for sales prediction and automated outreach.
  3. **Slack AI:** Native summarization, search, and workflow automation.
  4. **Jasper:** AI marketing copilot for campaign generation.
  5. **Motion:** AI-driven calendar and task manager.
  6. **Reclaim.ai:** Smart calendar scheduling.
  7. **Lindy.ai:** Autonomous AI agents for personal workflows.
  8. **Otter.ai:** Meeting transcription and summarization assistant.
  9. **Adept.ai:** Building AI models that can act on software UIs.
  10. **Claude (Anthropic):** Highly capable conversational agent often used as a backend for complex business reasoning.

  ---

  ### 2. Deep-Dive Competitor Audit: Square POS & Shopify Sidekick

  We conducted a deep dive into Square POS (a leader in SMB physical operations) and Shopify Sidekick (the vanguard of AI commerce assistants).

  #### Square POS
  - **Capabilities:** Omnichannel payment processing, offline mode (24h retention), unified inventory, staff scheduling, customer loyalty, advanced reporting. Distinct modes for Retail, Restaurant, Services, and Beauty.
  - **Success Factors:** The magic of Square is the seamless integration of elegant hardware (Terminal, Register, Stand) with zero-training-required software. It thrives on "time to live"—merchants can sign up and take payments within minutes.
  - **User Sentiment:**
    - *Love:* "The ecosystem just works. It's so easy to train new staff."
    - *Pain Points:* Customer support is often cited as a weakness at scale; fees can become burdensome; inventory syncing can sometimes lag for complex multi-location setups.

  #### Shopify Sidekick
  - **Capabilities:** Store design (theme picking), photo editing, SEO copywriting, tech support/setup, social media post generation, weekly performance summaries, discount code generation, low stock alerts.
  - **Success Factors:** Embedded directly in the Shopify admin panel (the purple glasses icon). It has strict domain knowledge of commerce and can take actions *on behalf* of the user (e.g., "create a discount code").
  - **User Sentiment:**
    - *Love:* "It wrote 50 product descriptions for me in 10 minutes."
    - *Pain Points:* Sometimes feels like a chatbot rather than a proactive assistant. Users want it to "just do" more complex workflow automations without needing perfect prompting.

  ---

  ### 3. OHC Gap & Pain Point Identification

  Based on our repository scan (`src/agents/builtin/tools/`), OHC currently has foundational tools for:
  - `booking`
  - `create_service_request`
  - `finance`
  - `marketing`
  - `marketplace`

  **The Gaps vs. Competitors:**
  1. **Lack of Proactive AI Work Triage:** Square and Shopify require the user to *find* the problem (e.g., check low inventory manually or ask Sidekick). OHC is supposed to be an "Owner Work Assistant", but currently relies on reactive tools.
  2. **Missing Seamless Hardware/Offline Resilience:** Square dominates because of its offline payment mode (24h buffer). OHC lacks defined protocols for offline-tolerant local operations on mobile (a critical requirement for our "Fatima - Food Cart" persona).
  3. **Fragmented UI Intent:** Sidekick opens a sidebar. OHC's current visual workflow tools exist, but we lack a unified "Assistant-First Shell" where the AI aggregates Finance, Booking, and Service Requests into a single "What to do today" feed.

  **Unresolved Pain Point:** Small business owners (like Carlos the Handyman or Maya the Baker) do not want to prompt an AI to ask "What is my revenue today?" They want opening the app to immediately present: *"You have 3 unpaid invoices, 1 new cake booking for Saturday, and your revenue is up 5%. Tap here to send invoice reminders."*

  ---

  ### 4. Agentic Solution Design: The "Morning Briefing" Agent

  **Problem Statement:** Owners are overwhelmed. They don't have time to review dashboards or write perfect prompts.

  **The Agentic Solution:**
  Implement a "Morning Briefing" proactive agent flow.
  1. **Background Aggregation:** A scheduled cron-agent (using `ohc-builtin-agent`) runs every morning. It invokes `finance`, `booking`, and `create_service_request` tools to gather the state of the tenant's business.
  2. **LLM Synthesis:** The agent synthesizes this data into a 3-bullet "Today's Priorities" summary using the LLM.
  3. **Actionable Drafts:** The agent proactively drafts the next logical actions (e.g., drafting a follow-up SMS to an unpaid invoice, or drafting a confirmation for a new booking).
  4. **Owner Approval:** The owner opens the OHC mobile PWA. They are not greeted with a dashboard of charts, but with the Morning Briefing. They simply swipe or tap "Approve" on the drafted actions.

  ```mermaid
  sequenceDiagram
      participant Cron as Daily Scheduler
      participant Triage as Work Triage Agent
      participant Tools as Booking/Finance DB
      participant LLM as Gemini/GPT
      participant Owner as OHC Mobile App

      Cron->>Triage: Trigger Daily Briefing
      Triage->>Tools: Fetch new bookings, unpaid invoices
      Tools-->>Triage: Return raw business state
      Triage->>LLM: Synthesize state & propose actions
      LLM-->>Triage: 3-bullet summary + 2 draft SMS actions
      Triage->>Owner: Push "Morning Briefing" to Home Screen
      Owner->>Owner: Reads summary
      Owner->>Triage: Taps "Approve SMS Actions"
      Triage->>Tools: Execute actions (send SMS)
  ```

  #### Implementation Prompt for Engineering Swarm
  **Critical User Journey (CUJ):**
  As an owner (e.g., Maya), when I log into OHC for the first time today, I should see an AI-generated "Morning Briefing" card on the home screen. It should list 2-3 specific, synthesized priorities (e.g., pending bookings) and offer a 1-tap "Action" button (e.g., "Confirm Bookings") that the agent has pre-drafted.
  **Acceptance Criteria:**
  - Create a new backend agent routine that aggregates data from existing `booking` and `finance` tools.
  - Expose a new API endpoint for the frontend to fetch the pre-computed "Briefing".
  - Frontend (Flutter/PWA) must display this briefing beautifully on a 375px wide screen using OHC translucent glass styling.
  - Include 100% unit test coverage for the aggregation logic and at least 5 Playwright E2E tests validating the UI click flow for approving the agent's draft.

  ---

  ### 5. References & Sources Catalog

  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/pricing
  3. https://www.shopify.com/online
  4. https://www.shopify.com/agentic-storefronts
  5. https://www.shopify.com/pos
  6. https://www.shopify.com/shop
  7. https://www.shopify.com/marketing
  8. https://www.shopify.com/analytics
  9. https://www.shopify.com/orders
  10. https://www.shopify.com/shipping
  11. https://www.shopify.com/finance
  12. https://www.shopify.com/flow
  13. https://www.shopify.com/mobile
  14. https://www.shopify.com/checkout
  15. https://www.shopify.com/payments
  16. https://squareup.com/us/en/point-of-sale
  17. https://squareup.com/us/en/restaurants
  18. https://squareup.com/us/en/restaurants/coffee-shop
  19. https://squareup.com/us/en/retail
  20. https://squareup.com/us/en/beauty
  21. https://squareup.com/us/en/services
  22. https://squareup.com/us/en/hardware/handheld
  23. https://squareup.com/us/en/hardware/terminal
  24. https://squareup.com/us/en/hardware/register
  25. https://squareup.com/us/en/hardware/stand
  26. https://squareup.com/us/en/hardware/kiosk
  27. https://squareup.com/us/en/hardware/contactless-chip-reader
  28. https://squareup.com/us/en/hardware/reader
  29. https://squareup.com/us/en/marketing
  30. https://squareup.com/us/en/messages
  31. https://squareup.com/us/en/ai
  32. https://squareup.com/us/en/software/loyalty
  33. https://squareup.com/us/en/gift-cards
  34. https://squareup.com/us/en/staff/shifts
  35. https://squareup.com/us/en/payroll
  36. https://squareup.com/us/en/banking
  37. https://squareup.com/us/en/banking/checking
  38. https://squareup.com/us/en/banking/savings
  39. https://squareup.com/us/en/banking/loans
  40. https://developer.squareup.com/us/en
  41. https://www.larksuite.com/en_sg/
  42. https://www.notion.so/product/ai
  43. https://www.wix.com/studio
  44. https://slack.com/features/ai
  45. https://www.salesforce.com/products/einstein/overview/
  46. https://asana.com
  47. https://trello.com
  48. https://www.zoho.com/one/
  49. https://work.weixin.qq.com/
  50. https://dingtalk.com/en

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
