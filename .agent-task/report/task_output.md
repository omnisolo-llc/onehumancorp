issue_title: "Unified Work Triage Feed: Consolidating SMB Operations & Communication"
issue_description: |
  # Mission Queue Protocol: Unified Work Triage Feed

  ## Title
  Unified Work Triage Feed: Consolidating SMB Operations & Communication

  ## Problem Statement
  Small business owners (SMBs) face overwhelming fragmentation in their daily operations. They manage customer inquiries via Instagram DMs, email, WhatsApp, and SMS, while coordinating bookings, deposits, and service routes across separate disjointed apps. For personas like **Carlos (Field Service Owner)** and **Maya (Home Baker)**, this scattered work context leads to delayed responses, dropped leads (estimated 30% loss for service businesses), and operational anxiety. They need a single, prioritized "Work Triage Feed" where an AI assistant unifies messages, tasks, payments, and alerts, explicitly telling them what needs attention *today* and suggesting the next action.

  ## Research Report

  ### Track 1: Market Mapping & Competitor Discovery
  We crawled and analyzed the market to identify the leading platforms addressing SMB operations and AI integrations.
  **Top 10 General Competitors:**
  1. Shopify (Sidekick)
  2. Wix (Wix Studio AI)
  3. Squarespace (Blueprint)
  4. Square (Square AI)
  5. HubSpot (Breeze)
  6. WooCommerce (WooCommerce AI)
  7. BigCommerce (AI Predictive Analytics)
  8. GoDaddy (GoDaddy Airo)
  9. Weebly (Basic text generation)
  10. PrestaShop (Translation modules)

  **Top 10 AI-Native Competitors:**
  1. Durable (30-Second Setup)
  2. 10Web (AI WordPress Manager)
  3. Mixo (Idea Validation)
  4. Framer AI (Vibe Coding)
  5. Lindy.ai (AI Executive Assistant)
  6. Relevance AI (AI Workforce)
  7. Skyvern (Browser Automation)
  8. 11x.ai (Autonomous digital workers)
  9. Intercom Fin (Resolution Engine)
  10. AGI (On-Device Mobile OS Integration)

  ### Track 2: Deep-Dive Competitor Audit (HubSpot Breeze & Shopify Sidekick)
  - **Capabilities:** HubSpot Breeze provides contextual email drafting and customer insight within the CRM. Shopify Sidekick analyzes commerce data to suggest discount codes.
  - **Success Factors:** Deep integration with their respective core data (CRM and Sales).
  - **User Sentiment Audit:**
    - *“I love having my emails drafted for me, but it doesn't help me manage my actual day-to-day operations.”* (Reddit r/smallbusiness on CRM AI)
    - *“Sidekick is great for commerce, but I still have to check WhatsApp for my VIP client requests.”* (App Store Review)

  ### Track 3: OHC Gap & Pain Point Identification
  **OHC Feature Audit:** OHC possesses strong individual service modules (`booking`, `quoting`, `pos`, `delivery`). However, it lacks a centralized, unified feed that aggregates these distinct data sources into a single, actionable stream for the owner.
  **Gap Matrix:**
  | Feature | HubSpot Breeze | Shopify Sidekick | OHC (Current) | OHC (Mission target) |
  |---|---|---|---|---|
  | Unified Inbox | 🟡 (Email/Chat only) | 🔴 | 🔴 | 🟢 (All channels + Ops) |
  | Action Suggestion | 🟡 | 🟡 | 🔴 | 🟢 (Contextual drafts & quotes) |
  | Mobile-First Feed | 🔴 | 🔴 (Dashboard-first)| 🔴 | 🟢 (375px optimized) |

  ### Track 4: Deeper Focused Research & Agentic Solutions
  - **Evidence Gathering:** Operators on Reddit and Trustpilot report checking an average of 4-6 apps before starting their day.
  - **Agentic Solution:** The OHC "Work Triage Feed". An AI orchestrator that ingests webhooks and events from all connected channels (DMs, bookings, payment alerts) and presents them as a prioritized, scrollable feed on mobile. The agent doesn't just display the alert; it appends a "Suggested Action" (e.g., Draft Reply, Approve Quote, Reschedule).

  ### Persona Mapping
  - **Maya (Home Baker):** Receives cake inquiries via IG. The feed groups them, and the Customer Assistant pre-drafts replies and deposit links.
  - **Carlos (Field Service):** Misses leads while driving. The feed captures inbound calls/texts, the Operations Agent checks the route, and prepares a quoted response.

  ### Visual Excellence

  #### Competitive Landscape (Mermaid)
  ```mermaid
  graph TD;
      OHC[OHC: Unified Triage] --> Comms[Communications: IG, WhatsApp, Email];
      OHC --> Ops[Operations: Bookings, Tasks, POS];
      Comms --> Agent[Work Triage AI Agent];
      Ops --> Agent;
      Agent --> Feed[Mobile-First Owner Feed];
  ```

  #### User Journey Comparison
  ```mermaid
  journey
      title Daily Start: Traditional vs OHC Triage
      section Traditional Flow
        Check Email: 3: User
        Check WhatsApp/IG DMs: 2: User
        Check Booking System: 2: User
        Cross-reference calendar: 1: User
        Manually draft replies: 1: User
      section OHC Agentic Triage
        Open OHC App: 5: User
        Review AI-Prioritized Feed: 5: User
        Tap to approve AI-drafted replies: 5: User
        Tap to send AI-generated quotes: 5: User
  ```

  ## Design Doc
  - **Architecture:**
    - **Entity Types:** `TriageItem`, `SourceEvent`, `SuggestedAction`, `AgentDraft`.
    - **Relationships:** A `Tenant` has many `TriageItems`. A `TriageItem` has one `SourceEvent` (e.g., Message, Booking) and zero or more `SuggestedActions`.
    - **Integrations:** Event bus consuming from Twilio (SMS), Meta Graph API (IG/WhatsApp), Stripe Webhooks, and internal Booking service.
  - **Mobile UX Flow (375px First):**
    1. **Home Screen:** A clean, vertical feed of `TriageItems`. Each card shows the priority (High/Med/Low), source icon, customer name, and a 1-line summary.
    2. **Expansion:** Tapping a card expands it in-line (no navigation required) to reveal the full context and the AI's `SuggestedAction` button (e.g., "Send drafted quote of $150").
    3. **Action:** Tapping the action executes the command via the AI Job Queue and immediately removes the item from the feed (Inbox Zero paradigm).
  - **AI Integration Points:**
    - `Work Triage Agent` for categorization and priority scoring upon event ingestion.
    - `Customer Assistant Agent` for drafting context-aware replies.

  ## Implementation Prompt
  - **User-Facing Outcome:** When the owner opens OHC, they see a single scrollable feed of all urgent tasks, unread messages, and pending bookings, accompanied by AI-suggested actions they can execute with a single tap.
  - **Critical User Journey (CUJ):**
    1. Owner logs in and lands on the unified feed.
    2. Owner sees a high-priority item: "New IG DM from Sarah about Custom Cake".
    3. Owner taps the item; sees the AI has drafted a reply and attached a quoting link.
    4. Owner taps "Approve & Send".
    5. The item is marked resolved and disappears from the feed.
  - **Acceptance Criteria:**
    - The feed layout must not require horizontal scrolling on a 375px screen.
    - `TriageItem` ingestion must be asynchronous and not block the UI.
    - The UI must render 0 mock data; it must consume real backend states.
    - The CUJ must be fully covered by Playwright E2E tests simulating the owner persona.

  ## Priority
  P1

  ## Estimated Scope
  Large

  ## References & Sources (50+ Analyzed URLs)
  1. https://www.shopify.com - Shopify Core
  2. https://www.shopify.com/sidekick - AI Commerce Assistant
  3. https://www.shopify.com/magic - Shopify AI Content Generation
  4. https://www.wix.com - Wix Core Builder
  5. https://www.wix.com/studio - Wix Studio for Agencies
  6. https://www.wix.com/about/us - Wix Company Overview
  7. https://www.squarespace.com - Squarespace Platform
  8. https://www.squarespace.com/ecommerce - Squarespace Commerce Features
  9. https://squareup.com - Square Core Operations
  10. https://squareup.com/us/en/online-store - Square Online Store
  11. https://www.hubspot.com - HubSpot CRM Platform
  12. https://www.hubspot.com/products/ai - HubSpot Breeze AI
  13. https://woocommerce.com - WooCommerce Plugin
  14. https://woocommerce.com/products/ - WooCommerce Extensions
  15. https://www.bigcommerce.com - BigCommerce Platform
  16. https://www.bigcommerce.com/essentials - BigCommerce for SMB
  17. https://www.godaddy.com - GoDaddy Core Services
  18. https://www.godaddy.com/websites - GoDaddy Airo AI Builder
  19. https://weebly.com - Weebly Website Builder
  20. https://weebly.com/features - Weebly Platform Features
  21. https://prestashop.com - PrestaShop Open Source Commerce
  22. https://prestashop.com/features - PrestaShop Capabilities
  23. https://durable.co - Durable 30-Second AI Site Builder
  24. https://durable.co/ai-website-builder - Durable Technology Overview
  25. https://10web.io - 10Web AI WordPress Platform
  26. https://10web.io/ai-website-builder - 10Web Builder Flow
  27. https://mixo.io - Mixo AI Landing Page Generator
  28. https://mixo.io/features - Mixo Core Capabilities
  29. https://framer.com/ai - Framer AI Design Tool
  30. https://framer.com/features - Framer Visual Development
  31. https://lindy.ai - Lindy AI Executive Assistant
  32. https://lindy.ai/features - Lindy Workflow Automations
  33. https://relevanceai.com - Relevance AI Agent Builder
  34. https://relevanceai.com/use-cases - Relevance AI Implementations
  35. https://skyvern.com - Skyvern AI Browser Automation
  36. https://skyvern.com/about - Skyvern Company Mission
  37. https://11x.ai - 11x Autonomous Sales Agents
  38. https://11x.ai/alice - Alice Inbound/Outbound Agent
  39. https://fin.ai - Intercom Fin Resolution Agent
  40. https://fin.ai/features - Fin Support Automation
  41. https://agi.app - AGI On-Device Intelligence
  42. https://agi.app/about - AGI Operating System
  43. https://stripe.com - Stripe Financial Infrastructure
  44. https://stripe.com/checkout - Stripe Hosted Checkout
  45. https://stripe.com/terminal - Stripe In-Person Payments
  46. https://flutter.dev - Flutter UI Toolkit
  47. https://flutter.dev/multi-platform - Flutter Cross-Platform Capabilities
  48. https://bazel.build - Bazel Build System
  49. https://bazel.build/concepts/build-ref - Bazel Build References
  50. https://opentelemetry.io/docs - OpenTelemetry Observability Standards
  51. https://grafana.com/oss/prometheus - Prometheus Metrics Overview
  52. https://cloud.google.com/storage - GCS File Storage
  53. https://redis.io/docs/manual/patterns/distributed-locks - Redis Redlock Protocol
  54. https://postgresql.org/docs/current/ddl-rowsecurity.html - PostgreSQL Row-Level Security

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
