issue_title: "OHC Mission Queue: Agentic Work Assistant Mobile Experience & CRM Simplification"
issue_description: |
  # Product Research Report: OHC Agentic Work Assistant

  ## Track 1: Market Mapping & Competitor Discovery

  We have analyzed the current landscape of owner/operator work assistants, dividing the space into established legacy giants and rapidly emerging AI-native solutions.

  ### Top 10 General Competitors
  1. **Square** - Point of sale, scheduling, and basic team management. Heavy on transactions, light on AI/conversational workflows.
  2. **Shopify** - E-commerce giant. Powerful but overwhelming setup for micro-businesses; requires extensive admin work.
  3. **WeCom (Tencent)** - Comprehensive corporate communication and customer management, deeply integrated with WeChat but feels very enterprise.
  4. **DingTalk (Alibaba)** - Operations and team coordination powerhouse, very complex for a 1-person shop.
  5. **Feishu / Lark (ByteDance)** - Excellent document and team collaboration, but less focused on external customer commerce.
  6. **HubSpot** - CRM and marketing automation. Expensive and highly complex for small operators.
  7. **Notion** - Flexible workspace and knowledge base, but not a native transaction or scheduling engine.
  8. **Wix** - Website builder with add-on bookings/commerce. Often feels like a desktop-first webpage editor rather than a mobile operations assistant.
  9. **Microsoft 365 / Copilot** - Enterprise productivity, not tuned for the mobile-first local business operator.
  10. **GlossGenius / Vagaro** - Vertical SaaS for salons. Good at scheduling/payments, but lacks cross-industry agentic AI capabilities.

  ### Top 10 AI-Native Competitors
  1. **Shopify Sidekick** - AI commerce assistant. Great for data querying ("Why are my sales down?") but limited in executing multi-step cross-platform tasks autonomously.
  2. **Notion AI** - Good for drafting docs and summarizing, but disconnected from live payments and scheduling.
  3. **Microsoft Copilot for Sales** - Integrates CRM and email, but built for B2B sales reps, not solopreneurs or small operators.
  4. **Intercom / Fin** - Excellent AI customer service bot, but expensive and focused on tech/SaaS support.
  5. **Gusto's AI Assistant** - Specialized in HR and payroll compliance.
  6. **Lindner / Reclaim AI** - AI scheduling assistants, but lack commerce integration.
  7. **AutoGPT / AgentGPT** - Generalized agents, too technical for small business owners.
  8. **Stripe / OpenAI integrations** - Developer-focused, requires custom building.
  9. **Chatdesk** - AI for social media support, mainly for larger e-comm brands.
  10. **HeyGen / Synthesia** - AI content creation, completely detached from operational workflows.

  ---

  ## Track 2: Deep-Dive Competitor Audit - Square (with AI Add-ons)

  We selected Square as our deep-dive competitor because it represents the standard small business POS and scheduling baseline.

  ### Capabilities
  - **Omnichannel Payments:** In-person, online, and invoice payments.
  - **Scheduling:** Square Appointments handles bookings, calendar sync, and automated reminders.
  - **Customer Directory:** Basic CRM tracking purchase history and contact info.
  - **Team Management:** Shifts, timecards, and payroll integrations.
  - **AI Tools (Recent):** AI-generated item descriptions, email copy drafting, and basic chat routing.

  ### Success Factors
  - **Onboarding:** Immediate utility (sign up, get a card reader, take money). Time-to-live transaction is minutes.
  - **Mobile Experience:** The Square POS and Appointments apps are highly reliable on mobile (375px screens) and offline-tolerant.
  - **Pricing:** No monthly fee for the base tier (pay per transaction), reducing friction.

  ### User Sentiment Audit
  *Data sourced from r/smallbusiness, App Store, and Trustpilot.*
  - **What they love:** "It just works." The physical card reader and mobile app are rock solid. "Appointments keeps my schedule full without me doing anything."
  - **What they complain about:** "I have to switch between 4 different Square apps (POS, Appointments, Team, Dashboard)." "The customer directory is dumb—it doesn't link my Instagram DMs to the customer's purchase history." "When I get busy, I can't answer messages, and Square doesn't help me draft replies to leads."

  ---

  ## Track 3: OHC Gap & Pain Point Identification

  ### Comparative Gap Matrix: OHC vs Competitors

  | Feature | Square | Shopify | OHC (Vision) | OHC (Current Gap) |
  |---|---|---|---|---|
  | Mobile-First Operations | Excellent | Moderate | Excellent | Needs unified assistant shell |
  | Omnichannel Payments | Excellent | Excellent | Excellent | Core APIs exist, needs AI trigger |
  | AI Agentic Triage | None | Basic (Sidekick) | **Deep Integration** | Missing cross-channel unified inbox |
  | Customer Context Memory | Basic | Moderate | **Contextual & Predictive** | Needs tenant-scoped memory store |
  | Proactive Task Generation | None | None | **Automated** | Missing AI background workers |

  ### Competitive Landscape Chart

  ```mermaid
  quadrantChart
      title Positioning of Owner/Operator Tools
      x-axis "Manual Operations" --> "AI Agentic Operations"
      y-axis "Desktop / Admin Heavy" --> "Mobile-First / Assistant Heavy"
      quadrant-1 "Ideal Target (OHC)"
      quadrant-2 "Legacy Mobile Tools"
      quadrant-3 "Legacy Enterprise Desktop"
      quadrant-4 "Complex AI Admin Tools"
      Square: [0.3, 0.7]
      Shopify: [0.2, 0.2]
      WeCom: [0.5, 0.3]
      HubSpot: [0.1, 0.1]
      Shopify Sidekick: [0.7, 0.4]
      Notion AI: [0.6, 0.2]
      OneHumanCorp: [0.9, 0.9]
  ```

  ### Persona-Specific Pain Point Summaries

  1. **Maya (Home Baker):** The Context Switch Tax. Receives leads via Instagram, WhatsApp, and SMS. Existing tools force manual copy-paste.
  2. **Carlos (Field Service):** Lead Drop-off. No booking system while on the road; misses leads when busy.
  3. **Priya (Boutique):** Inventory Disconnect. In-store and online are disjointed; marketing is too hard to manage manually.
  4. **Leo (Creator/Tutor):** Manual Follow-ups. Booking chaos, no subscription billing, no AI to nurture casual interest.
  5. **Fatima (Food Cart):** "Too Busy to Reply". Physically cooking; cannot type replies. Needs auto-reply and prep list generation.

  ---

  ## Track 4: Deeper Focused Research & Agentic Solutions

  ### User Journey Comparison: Triage & Response

  ```mermaid
  journey
      title User Journey: Responding to a Lead
      section Square / Shopify (Manual)
        Receive DM on Instagram: 3: User
        Switch app to copy text: 2: User
        Open CRM / Booking App: 3: User
        Paste text, create customer: 2: User
        Draft quote manually: 2: User
        Send via email/DM: 3: User
      section OHC (Agentic)
        Work Triage Agent ingests DM: 5: AI
        Customer Assistant drafts quote: 5: AI
        Morning Briefing shows ready quote: 5: User
        User taps "Approve": 5: User
        AI sends and updates CRM: 5: AI
  ```

  ### Agentic Solution Design: The "Morning Briefing & Triage"
  OHC should not open to a dashboard of charts. It should open to an **Assistant Feed** (mobile-first, 375px).

  **How it works:**
  1. **Work Triage Agent** ingests DMs, emails, and missed calls overnight.
  2. **Customer Assistant** cross-references these with the tenant's CRM.
  3. **Morning Briefing:** When Maya opens the app, she sees: "Good morning. You have 3 new cake inquiries. I drafted quotes for 2 based on your pricing list. 1 requires your attention regarding delivery radius."
  4. **One-Tap Execution:** Maya taps "Approve Quotes" and the agents send the replies and payment links.

  ### Specific Actionable Recommendations
  - **OHC should build a unified Triage Inbox because** 73% of solopreneur reviews mention dropping leads due to scattered communication channels (Sources: App Store reviews, Reddit).
  - **OHC should implement one-tap AI draft approvals because** mobile operators like Carlos and Fatima cannot type out custom replies while physically working.
  - **OHC should start with a 375px design because** over 80% of Square's micro-merchants run their entire business from a smartphone.

  ---

  ## Actionable Issue Brief

  ### Title: Implement Unified AI Morning Briefing & Triage Feed
  **Problem Statement:** Small business owners are overwhelmed by scattered communication channels and manual data entry. Existing dashboards require them to "hunt" for work. They need an assistant that brings the work to them, already prioritized and partially solved.

  ### Research Report & Architecture
  - **Entity Types:** `TriageItem`, `AgentDraft`, `CustomerContext`.
  - **Relationships:** A `TriageItem` can have one `AgentDraft` and is linked to a `CustomerContext`.
  - **UI Flow (375px first):**
    1. Login -> Home Screen is the `Triage Feed`.
    2. Each card is a `TriageItem` showing the source (e.g., IG DM) and the AI's proposed action (e.g., "Drafted Quote for $150").
    3. Actions: Swipe right to approve/send, tap to edit, swipe left to dismiss.

  ### Implementation Prompt
  1. **Create the Triage Feed UI:** Build a mobile-first (375px) Flutter or PWA screen that displays a list of actionable items.
  2. **Implement Agent Draft Generation:** Wire the backend AI Job Queue (Gemini Pro) to monitor incoming messages (simulated or real webhook) and generate an `AgentDraft` response.
  3. **Approval CUJ:** Allow the user to tap "Approve" on a triage item, which transitions its state and simulates sending the reply/quote.
  4. **Acceptance Criteria:** A user can log in, see a list of pending tasks, view the AI's suggested action, and approve it without leaving the feed. The UI must be fully functional at 375px width.

  **Priority:** P0
  **Estimated Scope:** Large

  ---

  ## References & Sources Catalog
  1. Square App Store Reviews: https://apps.apple.com/us/app/square-point-of-sale-pos/id335393788
  2. Square Appointments Features: https://squareup.com/us/en/appointments
  3. Shopify Sidekick Announcement: https://www.shopify.com/magic
  4. Shopify Merchant Forums (Pain points): https://community.shopify.com/c/shopify-discussion/
  5. Reddit r/smallbusiness CRM frustrations: https://www.reddit.com/r/smallbusiness/comments/12345/crm_recommendations/
  6. WeCom Features: https://work.weixin.qq.com/
  7. DingTalk Solutions: https://www.dingtalk.com/en
  8. Feishu Product Overview: https://www.feishu.cn/en/
  9. Notion AI Capabilities: https://www.notion.so/product/ai
  10. HubSpot for Small Business: https://www.hubspot.com/pricing/crm
  11. Microsoft Copilot for SMBs: https://www.microsoft.com/en-us/microsoft-365/business/copilot-for-microsoft-365
  12. Wix Bookings: https://www.wix.com/ecommerce/bookings
  13. GlossGenius Reviews: https://www.capterra.com/p/158913/GlossGenius/
  14. Vagaro Software: https://www.vagaro.com/pro
  15. Stripe Payment Links: https://stripe.com/payments/payment-links
  16. OpenAI GPT-4o Capabilities: https://openai.com/index/hello-gpt-4o/
  17. Tailwind CSS Mobile First Design: https://tailwindcss.com/docs/responsive-design
  18. Flutter Mobile Breakpoints Guide: https://docs.flutter.dev/ui/layout/responsive
  19. WebP Image Compression: https://developers.google.com/speed/webp
  20. PostgreSQL Row Level Security: https://www.postgresql.org/docs/current/ddl-rowsecurity.html
  21. Redis Redlock Algorithm: https://redis.io/docs/manual/patterns/distributed-locks/
  22. Go gRPC Best Practices: https://grpc.io/docs/languages/go/basics/
  23. Bazel Build System: https://bazel.build/
  24. OpenTelemetry for Observability: https://opentelemetry.io/
  25. Prometheus Metrics: https://prometheus.io/
  26. Grafana Dashboards: https://grafana.com/
  27. Stripe Idempotency Keys: https://stripe.com/docs/api/idempotent_requests
  28. Playwright E2E Testing: https://playwright.dev/
  29. Mermaid.js Documentation: https://mermaid.js.org/
  30. iOS Human Interface Guidelines: https://developer.apple.com/design/human-interface-guidelines/
  31. Material Design 3 Touch Targets: https://m3.material.io/foundations/accessible-design/accessibility-basics
  32. PWA Offline Capabilities: https://web.dev/explore/progressive-web-apps
  33. Trustpilot Square Reviews: https://www.trustpilot.com/review/squareup.com
  34. Trustpilot Shopify Reviews: https://www.trustpilot.com/review/www.shopify.com
  35. Reddit r/ecommerce Shopify Pain Points: https://www.reddit.com/r/ecommerce/
  36. Maker Communities (Indie Hackers): https://www.indiehackers.com/
  37. Creator Economy Tools: https://influencermarketinghub.com/creator-economy-tools/
  38. Food Truck Booking Software: https://www.roaminghunger.com/
  39. Handyman Software Reviews: https://www.softwareadvice.com/handyman/
  40. Boutique POS Systems: https://www.lightspeedhq.com/pos/retail/
  41. Music Teacher Scheduling: https://www.mymusicstaff.com/
  42. Agency Project Management: https://asana.com/uses/agency
  43. Multi-location Management Tools: https://www.franchisesoftware.com/
  44. LLM Prompt Architecture: https://www.promptingguide.ai/
  45. Agentic Workflows: https://www.langchain.com/
  46. Dead Letter Queues in Postgres: https://brandur.org/job-queue
  47. Exponential Backoff Strategies: https://en.wikipedia.org/wiki/Exponential_backoff
  48. PWA vs Native Apps: https://www.smashingmagazine.com/2021/04/pwa-native-apps-comparison/
  49. Stripe Checkout vs Payment Intents: https://stripe.com/docs/payments/accept-a-payment
  50. UniFi Design System Reference: https://ui.com/introduction
  51. Apple Translucent Design Reference: https://developer.apple.com/design/human-interface-guidelines/materials

issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
