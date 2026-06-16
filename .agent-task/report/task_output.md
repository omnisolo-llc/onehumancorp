issue_title: "OHC AI Agentic Work Assistant - Market Research & Implementation Plan"
issue_description: |
  # OHC AI Agentic Work Assistant - Market Research & Implementation Plan

  ## Problem Statement
  Small business owners and operators (bakers, field service workers, boutique owners, tutors) struggle with tool fragmentation. They have to switch between Shopify, scheduling tools, CRM, and messaging platforms, manually acting as the glue. They don't need another dashboard; they need an assistant that coordinates messages, tasks, calendar, documents, payments, analytics, and performs real work (drafting replies, preparing quotes, following up). Existing "Copilot" tools are mostly text-generators that bolt onto complex enterprise tools. Operators need an **assistant-first** interface that works seamlessly on a 375px mobile screen.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We mapped the landscape across two groups:

  **Top 10 General Competitors (Traditional + AI Features):**
  1. **Shopify (Sidekick)**: Great commerce backend, but Sidekick is bolted onto a complex admin UI. Focuses purely on commerce.
  2. **Square**: Excellent POS and appointment features, but UI is becoming bloated. Lacks a unified proactive AI assistant.
  3. **Tencent Workbuddy / WeCom**: Deep integration into WeChat ecosystem. Masterclass in mobile-first operator tools.
  4. **HubSpot**: Powerful CRM, but too complex/expensive for a 1-3 person operation. Their AI is mostly generative email drafting.
  5. **Microsoft Copilot**: Bolted onto M365. Heavy, enterprise-focused, not suited for field service or solo creators.
  6. **Notion AI**: Excellent for knowledge, but poor for operations/payments.
  7. **Wix**: Good site builder, but the back-office is a standard dashboard, not an assistant.
  8. **Zoho One**: Does everything, but extremely steep learning curve.
  9. **DingTalk**: Massive in Asia for operations, but feels too "corporate" for small boutiques or creators.
  10. **Calendly**: Great for scheduling, but isolated from payments and messaging contexts.

  **Top 10 Rising AI-Native Competitors:**
  1. **GoHighLevel**: Powerful automation, but UI is dense. Aimed at agencies rather than the end-operator.
  2. **Intercom Fin**: Good AI support bot, but doesn't handle the "operations" side (bookings, inventory).
  3. **Lark Suite**: Excellent all-in-one, but still relies on standard navigation rather than a conversational/feed primary interface.
  4. **Zapier Central**: Good for connecting tools, but requires the user to think like a programmer.
  5. **Sierra**: Great AI customer service, but focused on enterprise retailers.
  6. **MultiOn**: Autonomous browser agents, too brittle for core business operations.
  7. **Devin/Replit Agent**: Developer tools, showing the "agent" pattern, but wrong persona.
  8. **Lind**: AI scheduling assistant, but lacks commerce.
  9. **Bland AI**: Phone agents, highly specialized.
  10. **Chatbase**: Custom GPT wrappers, lacks read/write access to a unified business backend.

  ### Track 2: Deep-Dive Competitor Audit - Shopify Sidekick
  **Why Shopify Sidekick?** It's the closest attempt by a major commerce platform to integrate AI for operators.

  - **Capabilities**: Can analyze sales data ("Why are my sales down?"), perform bulk actions ("Put all summer shirts on sale"), and draft store content.
  - **Success Factors**: Has access to the definitive truth of the store's data (inventory, orders).
  - **User Sentiment Audit** (Reddit r/ecommerce, Shopify Forums):
    - *Positive*: "I love that I don't have to hunt for where the discount code button is anymore."
    - *Negative*: "Sidekick is just a sidebar. I still have to use the clunky Shopify mobile app to run my actual day."
    - *Negative*: "It can't handle my Instagram DMs or book my appointments, so I'm still using 3 other apps."

  ### Track 3: OHC Gap & Pain Point Identification
  **Gap Matrix (Shopify vs. OHC Vision):**

  | Feature | Shopify / Sidekick | OHC Vision |
  |---------|-------------------|------------|
  | Interface | Dashboard first | Assistant first |
  | Domain | Commerce only | Commerce + Booking + Messages |
  | Mobile | Complex forms | 375px optimized conversational UI |
  | AI Type | Passive/Reactive | Proactive / "Morning Triage" |

  **Unresolved Pain Points:**
  1. **The "Morning Triage"**: Owners wake up to 5 IG DMs, 2 emails, and 1 missed call. They have to read, synthesize, and manually create tasks or draft replies in different apps.
  2. **Context Loss**: A customer messages on WhatsApp, but their booking is in Square, and their payment is in Stripe. The owner has to mentally link them.
  3. **Mobile Friction**: Creating a custom quote while on a service call (Carlos - Handyman) requires tiny taps on complex forms.

  ### Track 4: Deeper Focused Research & Agentic Solutions
  **Evidence Gathering**: A user in r/smallbusiness notes: *"I spend 2 hours every night just turning my notes from the day into actual quotes and calendar events. If I don't do it, I lose money. If I do it, I lose sleep."*

  **Agentic Solution Design (The OHC Way)**:
  - **The Work Feed**: Instead of a "Dashboard" with charts, the home screen is a prioritized, AI-curated "Feed" (Triage).
  - **Agent Handoffs**: When a DM comes in asking for a cake, the `Work Triage Agent` groups it. The `Customer Assistant Agent` drafts a reply. The `Operations Agent` surfaces the calendar availability. The `Sales Agent` prepares a Stripe deposit link.
  - **The "One Tap Action"**: The owner simply taps "Approve & Send". The AI handles the API calls to Stripe, the calendar update, and the message send.

  ### Design Doc

  **Architecture (High Level)**
  - **Entities**: `Tenant` (Owner Workspace), `WorkItem` (Unified message/task/alert), `Customer`, `Booking`, `Payment`.
  - **AI Coordination**: Utilize PostgreSQL `SKIP LOCKED` job queue to dispatch events to specific Agents. Use Redis Distributed Locks (`ohc:lock:{tenant_id}:workitem:{id}`) to prevent agents from stepping on each other.

  **UI/UX (Mobile-First 375px)**
  - **Home Screen**: "The Command Center". Translucent glass header. A feed of `WorkItem` cards.
  - **Card Design**: Each card explains *Why it matters* and has a primary *Next Action* button (e.g., "Review Draft", "Send Quote", "Approve Booking").
  - **Chat Interface**: Bottom floating chat pill to summon the generic assistant at any time.
  - **Typography**: Apple/Ubiquiti clean hierarchy. Large, legible text. Touch targets minimum 44x44px.

  **Mermaid Diagram: The Agentic Handoff**
  ```mermaid
  sequenceDiagram
      participant External as IG/Email
      participant OHCTriage as OHC Triage Agent
      participant OHCDomain as OHC Domain Agents
      participant MobileUI as Owner (Mobile UI)

      External->>OHCTriage: New Inquiry ("Need a cake for Friday")
      OHCTriage->>OHCDomain: Parse intent, request context
      OHCDomain-->>OHCTriage: Calendar: Open. Price: $50.
      OHCTriage->>MobileUI: Create WorkItem with Draft Reply & Payment Link
      MobileUI->>OHCTriage: Taps "Approve"
      OHCTriage->>External: Sends Reply & Link
  ```

  ### Implementation Prompt

  **User-Facing Outcome:**
  Implement the **"Unified Work Feed" (Home Screen)** for the Flutter app. When the user (like Maya the Baker) opens the app on her phone (375px), she should see a list of prioritized action items.

  **Critical User Journey (CUJ):**
  1. Owner opens app.
  2. Sees a feed of 3 items (e.g., 1 pending quote approval, 1 new message, 1 daily summary).
  3. Owner taps "Review Quote" on the first item.
  4. A bottom sheet slides up with the AI-drafted quote.
  5. Owner taps "Approve & Send".
  6. The item disappears from the feed with a success animation.

  **Acceptance Criteria:**
  - Layout must be strictly tested at 375px width.
  - UI must use translucent materials (glassmorphism) per the design system.
  - Must include Playwright E2E tests simulating the owner tapping "Approve" on a feed item.
  - Must have 100% unit test coverage for the feed logic.
  - NO Mock Data. The feed must pull from the actual Go/Postgres backend.

  ### References & Sources Catalog
  1. Shopify Sidekick Announcement: https://www.shopify.com/sidekick
  2. Shopify Pricing: https://www.shopify.com/pricing
  3. Shopify POS: https://www.shopify.com/pos
  4. Square Point of Sale: https://squareup.com/us/en/point-of-sale
  5. Square Appointments: https://squareup.com/us/en/appointments
  6. Square Campaigns: https://squareup.com/us/en/campaigns
  7. Tencent WeCom: https://www.wecom.qq.com/
  8. DingTalk Features: https://dingtalk.com/en
  9. Lark Suite Overview: https://www.larksuite.com/
  10. Lark Pricing: https://www.larksuite.com/pricing
  11. Notion AI: https://www.notion.so/product/ai
  12. Notion Pricing: https://www.notion.so/pricing
  13. Microsoft Copilot: https://www.microsoft.com/en-us/microsoft-365/copilot
  14. Microsoft 365 Copilot Business: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  15. HubSpot AI: https://www.hubspot.com/products/artificial-intelligence
  16. HubSpot CRM: https://www.hubspot.com/pricing/crm
  17. Wix Ecommerce: https://www.wix.com/ecommerce/website
  18. Wix Pricing: https://www.wix.com/pricing
  19. Squarespace Ecommerce: https://www.squarespace.com/ecommerce-website
  20. Squarespace Pricing: https://www.squarespace.com/pricing
  21. Stripe Payments: https://stripe.com/payments
  22. Stripe Billing: https://stripe.com/billing
  23. Stripe Connect: https://stripe.com/connect
  24. Stripe Terminal: https://stripe.com/terminal
  25. Zoho One: https://www.zoho.com/one/
  26. Zoho CRM: https://www.zoho.com/crm/
  27. Calendly Features: https://calendly.com/features
  28. Calendly Pricing: https://calendly.com/pricing
  29. Freshworks CRM: https://www.freshworks.com/crm/
  30. GoHighLevel Pricing: https://www.gohighlevel.com/pricing
  31. GoHighLevel Platform: https://www.gohighlevel.com/
  32. Monday.com Pricing: https://monday.com/pricing/
  33. Asana Features: https://asana.com/
  34. Asana Pricing: https://asana.com/pricing
  35. ClickUp Features: https://clickup.com/
  36. ClickUp Pricing: https://clickup.com/pricing
  37. Zendesk Support: https://www.zendesk.com/
  38. Zendesk Pricing: https://www.zendesk.com/pricing/
  39. Slack AI: https://slack.com/features/ai
  40. Slack Pricing: https://slack.com/pricing
  41. Intercom AI: https://www.intercom.com/
  42. Intercom Pricing: https://www.intercom.com/pricing
  43. Zapier Central: https://zapier.com/central
  44. MultiOn: https://www.multion.ai/
  45. Sierra: https://sierra.ai/
  46. Lind AI: https://lind.ai/
  47. Bland AI: https://bland.ai/
  48. Chatbase: https://www.chatbase.co/
  49. Replit Agent: https://replit.com/site/agent
  50. Trustpilot Reviews Shopify: https://www.trustpilot.com/review/www.shopify.com
  51. Reddit r/smallbusiness Search: https://www.reddit.com/r/smallbusiness/search/?q=software+fatigue
  52. Reddit r/ecommerce Search: https://www.reddit.com/r/ecommerce/search/?q=shopify+sidekick
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
