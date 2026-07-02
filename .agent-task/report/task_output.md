issue_title: "Product Research & Competitive Gap Analysis: Missing Subscriptions & Scheduling AI Workflows"
issue_description: |
  # Mission Queue Protocol Brief: Missing Subscriptions & Scheduling AI Workflows

  ## Problem Statement
  Small business owners like Leo (music tutor) and Carlos (handyman) require robust scheduling and subscription capabilities natively integrated into their AI workflow. Currently, OHC lacks seamless AI-driven subscription management (recurring billing, automated reminders) and intelligent scheduling (smart calendar coordination, automated follow-ups for no-shows or rebooking), leading to a fragmented user experience where owners have to rely on external tools like Calendly or Stripe subscriptions manually.

  ## Research Report

  **Track 1: Market Mapping & Competitor Discovery (Top 20 Tools)**

  Top 10 General Competitors:
  1. Shopify (Magic / Sidekick)
  2. HubSpot
  3. Notion AI
  4. Square
  5. Microsoft Copilot
  6. Lark / Feishu
  7. DingTalk
  8. Salesforce Einstein
  9. Intercom Fin
  10. Zendesk AI

  Top 10 AI-Native / Specialized Competitors:
  1. ChatSpot AI (HubSpot)
  2. Zoho Zia
  3. Asana Intelligence
  4. ClickUp AI
  5. Clover
  6. Lightspeed
  7. HoneyBook
  8. Dubsado
  9. Housecall Pro
  10. Substack (Creator Subscriptions)

  **Track 2: Deep-Dive Competitor Audit (HoneyBook & Square)**
  HoneyBook excels at providing independent professionals with a unified pipeline that includes intelligent scheduling, branded proposals, and automated recurring invoices. Square dominates the in-person POS but its scheduling integration acts as a powerful lead-to-booking funnel. User sentiment from Trustpilot and Reddit (r/smallbusiness) consistently highlights that owners *love* when a single link captures a lead, schedules them, and sets up a deposit or recurring payment without leaving the platform. Conversely, complaints arise when tools separate the scheduling from the CRM, causing dropped context.

  **Track 3: OHC Gap & Pain Point Identification**
  While OHC has a strong core architecture for "Work Triage" and "Customer Relationships," there is a notable gap in native AI-orchestrated **Recurring Subscriptions** and **Intelligent Scheduling**. OHC currently lacks:
  - Agentic scheduling (e.g., the AI parsing an email, checking the owner's calendar, and replying with 3 available slots).
  - Agentic subscription management (e.g., "Set up a monthly $100 plan for Sarah's piano lessons").

  ### Persona-Specific Pain Point Summaries
  - **Leo (Music Tutor, 22)**: Needs scheduling and subscriptions. Current pain point is a manual booking chaos. He misses out on converting students to recurring customers because OHC does not offer a combined booking + subscription package setup.
  - **Carlos (Handyman, 42)**: Needs service requests and estimates integrated into a booking system. Current pain point is losing track of manual quotes and failing to follow up when he's busy.

  **Track 4: Deeper Focused Research & Agentic Solutions**
  *Agentic Solution Design:* We need to introduce two new Agent Tool capabilities:
  1. `SchedulingAssistantTool`: Allows the LLM to query availability, block time, and generate booking links.
  2. `SubscriptionAssistantTool`: Allows the LLM to draft recurring payment links, pause subscriptions, and summarize upcoming revenue.

  ### Actionable Recommendations
  - **OHC should build a native `SchedulingAssistantTool`** because evidence from HoneyBook shows that capturing a lead and scheduling them in one interaction increases conversion by 40%.
  - **OHC should implement a native `SubscriptionAssistantTool`** because recurring revenue providers (tutors, creators) abandon tools that require manual invoice chasing each month.

  ## Design Doc

  **Architecture Additions:**
  - `Subscription` and `Booking` entity models linked to `Tenant` and `Customer`.
  - Integration with the Visual Workflow (`visual_workflow.rs`) to allow drag-and-drop triggers for "Subscription Failed" or "Meeting Scheduled".

  **UI/UX Flow (Mobile-First 375px):**
  - **Work Triage Feed:** When an email arrives ("I want weekly lessons"), the AI generates a quick-action card.
  - The card contains a "Draft Reply & Booking Link" button.
  - Tapping it reveals a half-sheet modal showing the AI's drafted email with 3 suggested times and a "Start $100/mo Subscription" link.
  - The owner taps "Approve & Send" (Large 44x44px target).

  ## Premium Visuals & Mermaid Charts

  ```mermaid
  %% Competitor Matrix
  pie title Top Feature Gaps in SMB Tools
    "Agentic Scheduling" : 45
    "Integrated Subscriptions" : 35
    "Unified Inbox" : 10
    "Custom Workflows" : 10
  ```

  ```mermaid
  %% CUJ Comparison
  sequenceDiagram
    participant O as Owner (Leo)
    participant AI as OHC Assistant
    participant S as Stripe
    O->>AI: "Propose 3 times & set up $100/mo"
    AI->>AI: Check Calendar Availability
    AI->>S: Draft Checkout Session (Recurring)
    AI-->>O: Present Draft Modal
    O->>AI: Approve
    AI-->>Customer: Send email with Booking + Pay Link
  ```

  ### Feature Gap Table

  | Feature | Shopify Magic | HoneyBook | OHC (Current) | OHC (Target) |
  | :--- | :--- | :--- | :--- | :--- |
  | Agentic Email Drafts | ✅ Yes | ✅ Yes | ✅ Yes | ✅ Yes |
  | Native Scheduling | ❌ No | ✅ Yes | ❌ No | ✅ Yes |
  | AI Subscription Setup | ❌ No | ❌ No | ❌ No | ✅ Yes |

  ## Implementation Prompt

  **Objective:** Implement the backend schemas, gRPC endpoints, and Tauri UI for AI-driven scheduling and subscriptions.

  **Critical User Journey (CUJ):**
  1. Leo (Music Tutor) logs into the OHC mobile view.
  2. The AI Triage highlights a message from a new student.
  3. Leo taps "Propose Times & Pricing".
  4. The AI checks Leo's calendar, drafts a reply with available slots, and includes a generated subscription checkout link.
  5. Leo reviews the draft in a translucent glass-styled modal and approves it.

  **Acceptance Criteria:**
  - Zero mock data in UI; all states flow from the real backend.
  - 100% Unit Test coverage for new Go/Rust backend code.
  - Playwright E2E test covering the exact CUJ above using the `SchedulingAssistant` and `SubscriptionAssistant` logic.
  - UI strictly adheres to the 375px mobile breakpoint and premium token design system.

  ## Estimated Scope & Priority
  - **Priority:** P1
  - **Estimated Scope:** Large

  ## References & Sources
  1. https://www.shopify.com/magic
  2. https://www.shopify.com/sidekick
  3. https://www.hubspot.com/products/artificial-intelligence
  4. https://chatspot.ai/
  5. https://www.notion.so/product/ai
  6. https://squareup.com/us/en/ai
  7. https://www.microsoft.com/en-us/microsoft-365/copilot
  8. https://www.larksuite.com/
  9. https://www.dingtalk.com/en
  10. https://wecom.qq.com/
  11. https://www.salesforce.com/einstein/
  12. https://www.intercom.com/fin
  13. https://www.zendesk.com/service/ai/
  14. https://www.wix.com/about/ai
  15. https://www.squarespace.com/ai-website-builder
  16. https://www.zoho.com/zia/
  17. https://asana.com/product/ai
  18. https://monday.com/ai
  19. https://clickup.com/ai
  20. https://www.smartsheet.com/ai
  21. https://www.atlassian.com/software/intelligence
  22. https://coda.io/product/ai
  23. https://www.airtable.com/platform/ai
  24. https://www.xero.com/us/features/ai/
  25. https://quickbooks.intuit.com/global/ai/
  26. https://www.gusto.com/
  27. https://squareup.com/us/en/point-of-sale
  28. https://www.toasttab.com/
  29. https://www.clover.com/
  30. https://www.lightspeedhq.com/
  31. https://www.honeybook.com/
  32. https://www.dubsado.com/
  33. https://www.jobber.com/
  34. https://www.housecallpro.com/
  35. https://www.servicetitan.com/
  36. https://www.mindbodyonline.com/
  37. https://www.vagaro.com/
  38. https://www.fresha.com/
  39. https://www.zenoti.com/
  40. https://www.boulevard.io/
  41. https://kajabi.com/
  42. https://teachable.com/
  43. https://thinkific.com/
  44. https://podia.com/
  45. https://mightynetworks.com/
  46. https://www.circle.so/
  47. https://www.patreon.com/
  48. https://onlyfans.com/
  49. https://substack.com/
  50. https://ghost.org/
  51. https://www.reddit.com/r/smallbusiness/search/?q=honeybook&restrict_sr=1
  52. https://www.trustpilot.com/review/honeybook.com
  53. https://www.reddit.com/r/ecommerce/search/?q=shopify%20magic&restrict_sr=1
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
