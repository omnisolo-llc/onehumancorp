issue_title: "Proactive Daily Briefing & One-Tap Action Feed"
issue_description: |
  # Mission Queue Protocol: Owner Work Assistant Gap Analysis & Solution

  ## Problem Statement
  Owners and operators (like Maya the home baker, Carlos the field service owner, and Fatima the food cart operator) are overwhelmed by complex administrative dashboards in existing software (Shopify, Wix, Square, HubSpot, Notion). They log in and see raw data (charts, metrics, raw task lists) instead of clear, prioritized next actions. The fundamental gap is that existing tools present "what happened" instead of "what to do now." This leaves the non-technical owner to manually piece together context across disconnected tools and agents to figure out their daily priorities.

  ## Research Report: Competitor Deep Dive & Market Mapping

  ### Track 1: Market Mapping & Competitor Discovery

  **Top 10 General Competitors:**
  1. **Shopify (Sidekick):** AI assistant embedded in admin; generates reports, makes site edits, drafts emails. Still heavily relies on a traditional admin interface.
  2. **Wix (Aria):** AI website builder with a conversational interface for setup, but post-setup management remains traditional.
  3. **Square (Square AI):** Offers AI tools for product descriptions and background removal, plus conversational analytics, but lacks a centralized, cross-capability daily action feed.
  4. **HubSpot (Breeze):** Specialized AI agents (Prospecting, Customer Service, Content). Very powerful but extremely complex and CRM-centric; not suitable for a sole operator like Carlos or Maya without heavy setup.
  5. **Notion (Notion AI & Agents):** Incredible enterprise search, meeting notes, and custom agents for recurring workflows. Highly customizable, but requires the user to build the workflows rather than working out-of-the-box for a local service owner.
  6. **Squarespace (Blueprint):** AI-guided design and content generation.
  7. **WooCommerce:** AI product description generators and SEO.
  8. **WeCom (Tencent):** Ubiquitous in China for customer relationships and operations, blending chat and business tasks, but lacks a tailored western-SMB AI assistant.
  9. **DingTalk (Alibaba):** Comprehensive collaboration and operations suite; very dense and administrative.
  10. **Microsoft Copilot:** Deeply integrated into M365 (Word, Excel, Teams); highly secure and capable for knowledge workers, but not designed as an operations command center for local operators.

  **Top 10 AI-Native Competitors:**
  1. **Durable:** 30-second website generation and basic CRM; ultra-fast onboarding but limited post-launch operations capability.
  2. **10Web:** AI WordPress manager.
  3. **Mixo:** Idea validation and instant lead-capture pages.
  4. **Framer AI:** Vibe coding and generative design.
  5. **Lindy.ai:** Personal AI Executive Assistant handling email triage and scheduling.
  6. **Relevance AI:** Platform to build autonomous AI workforces.
  7. **Skyvern:** AI browser automation for repetitive manual tasks.
  8. **11x.ai (Alice & Julian):** Autonomous digital workers for sales and inbound calls.
  9. **Intercom Fin:** Customer support resolution engine.
  10. **AGI (On-Device):** Smartphone OS-level actions.

  ### Track 2: Deep-Dive Competitor Audit (Shopify Sidekick & HubSpot Breeze)

  **Shopify Sidekick:**
  - **Capabilities:** Direct access to Shopify data. Edits themes, generates reports (e.g., "Give me a weekly performance summary"), sets up discount codes, audits shipping configs.
  - **Success Factors:** Deep integration with the Shopify ecosystem and access to the user's specific data context.
  - **User Sentiment Audit:** Users praise the potential to avoid manual configuration (e.g., "I love that Sidekick can see my real sales data"). However, complaints highlight that the core Shopify admin is still overwhelmingly complex for simple operators, and Sidekick is a chat window *beside* the complexity, not a replacement for it.

  **HubSpot Breeze:**
  - **Capabilities:** Customer Agent (resolves inquiries), Prospecting Agent (finds leads, personalized outreach), Data Agent (instant CRM answers).
  - **Success Factors:** Deep CRM tie-in, ability to create custom automations, robust ROI tracking.
  - **User Sentiment Audit:** Excellent for mid-market and enterprise (praise for scaling support), but heavily criticized by small businesses for extreme complexity, steep learning curve, and high cost of entry.

  ### Track 3: OHC Gap & Pain Point Identification

  **OHC Feature Audit:**
  Current OHC features (Work Intake, Customer Relationships, Scheduling, Offers, Knowledge, Decisions) are robust.
  **Gap Matrix (OHC vs Competitors):**

  | Feature | Shopify | HubSpot | Notion | Wix | OHC Current | OHC Target |
  | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
  | Assistant-First UI | No | No | No | No | Partial | **Yes (Command Center)** |
  | Cross-Domain Agents | Yes | Yes | Yes | No | Partial | **Yes (Unified)** |
  | Proactive Briefing | No | No | No | No | No | **Yes (Daily Summary)** |
  | One-Tap Executions | No | Partial | Partial | No | No | **Yes (Action Feed)** |

  **Unresolved Pain Points:**
  - *Maya (Baker):* Needs to know exactly which 3 orders need action today, rather than looking at an analytics dashboard.
  - *Carlos (Handyman):* Misses leads while driving/working; needs the assistant to tee up a drafted quote for a missed call, ready for one-tap approval.

  ### Track 4: Deeper Focused Research & Agentic Solutions

  **Agentic Solution Design: The "Proactive Daily Briefing & One-Tap Action Feed"**
  Instead of an admin portal with charts, OHC's primary home screen must be an "Assistant-First Shell."
  - **The Briefing:** A plain-language summary generated every morning: "Good morning Maya. You have 3 cake orders to fulfill by Friday. 2 new DMs came in overnight, and I've drafted replies for both."
  - **The Feed:** A prioritized list of actionable cards. Each card represents a cross-agent coordinated task (e.g., Work Triage + Customer Assistant + Sales Assistant).
  - **One-Tap Action:** Each card has a primary action button (e.g., "Approve & Send Quote", "Schedule Delivery", "Remind Customer").

  ---

  ## Design Doc

  **High-Level Architecture:**
  - **Entity Types:** `DailyBriefing` (tenant-scoped, generated daily), `ActionCard` (tasks/alerts requiring owner action, linked to underlying resources like `Message`, `Booking`, or `Invoice`).
  - **Key Relationships:** `ActionCard` belongs to `Tenant` and references `SourceResource` (polymorphic). `ActionCard` has a status (`Pending`, `Approved`, `Dismissed`).
  - **Integration Points:** The AI Job Queue (PostgreSQL `SKIP LOCKED`) runs a nightly job per tenant to generate the `DailyBriefing` using the Finance/Decision Assistant and Work Triage capabilities. Real-time events (new DM, new lead) trigger the Work Triage agent to generate new `ActionCard`s.

  **UI Wireframes & Mobile UX Flow (375px first):**
  1. **Home Screen (The Command Center):**
     - Top Section: Translucent glass card with the plain-language **Proactive Daily Briefing**.
     - Middle Section: **One-Tap Action Feed**. A vertical scrolling list of `ActionCards`.
     - Bottom Navigation: Standard OHC mobile tab bar.
  2. **Action Card UX:**
     - Minimalist Apple/Ubiquiti-style hierarchy.
     - Title: "New Inquiry: Custom Cake"
     - Subtitle: "Customer asked for vegan options for Saturday."
     - Agent Draft Box (collapsible): Shows the AI-drafted reply.
     - Action Buttons (44x44px min): [Approve & Send] [Edit] [Dismiss]

  **AI Agent Integration Points:**
  - **Work Triage Agent:** Listens for inbound events, creates `ActionCard`s.
  - **Customer Assistant Agent:** Drafts replies attached to `ActionCard`s.
  - **Decision Assistant Agent:** Compiles the `DailyBriefing`.

  ---

  ## Implementation Prompt

  **User-Facing Outcome:**
  When Maya (or any owner) opens the OHC Flutter/PWA app on her 375px-wide phone, she no longer sees a dashboard of charts. She sees a friendly, plain-language greeting summarizing her business state today, followed immediately by a prioritized feed of actions (drafted replies, pending quotes, low inventory alerts). She can approve these actions with a single tap, clearing her feed and keeping momentum without needing technical expertise.

  **Critical User Journey (CUJ):**
  1. User logs into OHC on a mobile device.
  2. User views the Home screen, displaying the Daily Briefing.
  3. User scrolls to the Action Feed and sees a card for an unread customer message with an AI-drafted reply.
  4. User taps "Approve & Send" on the card.
  5. The card visually resolves/disappears, the reply is sent, and the user's workload is reduced.

  **Acceptance Criteria:**
  - The Home screen layout must be strictly mobile-first (optimized for 375px) with no horizontal scrolling.
  - Touch targets for actions must be at least 44x44px.
  - The UI must use truthful states (loading, empty, error) with zero hardcoded mock data. All data must originate from the real backend.
  - The feature must be fully covered by Playwright E2E tests executing the CUJ via the real UI and real API.
  - Visual design must utilize OHC Premium Tokens (translucent materials, clear status tokens).

  ---

  ## Visual Excellence

  ### Competitive Landscape Diagram
  ```mermaid
  graph TD;
      OHC[OHC: Owner Assistant] --> Traditional[Traditional Admin Suites];
      OHC --> AINative[AI-Native Tools];

      Traditional --> Shopify[Shopify: Sidekick];
      Traditional --> HubSpot[HubSpot: Breeze];
      Traditional --> Square[Square: Square AI];

      AINative --> Durable[Durable: 30s Site];
      AINative --> Lindy[Lindy: AI EA];

      OHCDiff((OHC Advantage: Proactive Briefing & One-Tap Action Feed));
      OHC --> OHCDiff;
  ```

  ### Feature Gap Heatmap
  | Capability | OHC Target | Shopify Sidekick | HubSpot Breeze | Wix Aria | Notion AI |
  | :--- | :--- | :--- | :--- | :--- | :--- |
  | **Assistant-First UI Shell** | 🟢 | 🔴 | 🔴 | 🔴 | 🔴 |
  | **Proactive Daily Briefings** | 🟢 | 🟡 | 🔴 | 🔴 | 🔴 |
  | **One-Tap Agent Actions** | 🟢 | 🟡 | 🟡 | 🔴 | 🟡 |
  | **Mobile-First (375px) Focus** | 🟢 | 🟡 | 🟡 | 🟢 | 🟢 |

  ---

  ## References & Sources (50+ URLs Analyzed)

  ### Shopify
  1. https://www.shopify.com/sidekick
  2. https://www.shopify.com/magic
  3. https://www.shopify.com/editions/spring2026
  4. https://changelog.shopify.com/posts/create-customers-and-companies-with-sidekick
  5. https://changelog.shopify.com/posts/payments-and-web-performance-data-available-in-sidekick
  6. https://changelog.shopify.com/posts/sidekick-conversation-history-moves-to-your-sidebar
  7. https://www.shopify.com/blog/what-is-shopify
  8. https://www.shopify.com/pricing
  9. https://apps.shopify.com/
  10. https://themes.shopify.com/

  ### Wix
  11. https://www.wix.com/
  12. https://www.wix.com/ai-website-builder
  13. https://www.wix.com/website/design
  14. https://www.wix.com/business-software
  15. https://www.wix.com/business-software/crm
  16. https://www.wix.com/studio
  17. https://www.wix.com/ecommerce/website
  18. https://www.wix.com/mobile/wix-app
  19. https://www.wix.com/features/main
  20. https://www.wix.com/manage/analytics

  ### HubSpot
  21. https://www.hubspot.com/
  22. https://www.hubspot.com/products/artificial-intelligence
  23. https://www.hubspot.com/products/crm/ai-crm
  24. https://www.hubspot.com/products/aeo
  25. https://www.hubspot.com/products/artificial-intelligence/ai-customer-service-agent
  26. https://www.hubspot.com/products/sales/ai-prospecting-agent
  27. https://www.hubspot.com/products/artificial-intelligence/ai-data-agent
  28. https://www.hubspot.com/use-case/scale-customer-service-support
  29. https://www.hubspot.com/use-case/build-sales-pipeline
  30. https://www.hubspot.com/products/crm/starter

  ### Square
  31. https://squareup.com/us/en
  32. https://squareup.com/us/en/ai
  33. https://squareup.com/us/en/software
  34. https://squareup.com/us/en/point-of-sale
  35. https://squareup.com/us/en/messages
  36. https://squareup.com/us/en/marketing
  37. https://squareup.com/us/en/banking
  38. https://squareup.com/us/en/appointments
  39. https://squareup.com/us/en/restaurants
  40. https://squareup.com/us/en/retail

  ### Notion
  41. https://www.notion.com/product/ai
  42. https://www.notion.com/product/agents
  43. https://www.notion.com/product/ai-meeting-notes
  44. https://www.notion.com/product/enterprise-search
  45. https://www.notion.com/help/custom-agent
  46. https://www.notion.com/help/notion-ai-faqs
  47. https://www.notion.com/help/notion-ai-security-practices
  48. https://www.notion.com/product/ai/use-cases
  49. https://www.notion.com/teams
  50. https://www.notion.com/personal

  ### General Competitors / AI Natives
  51. https://durable.co/
  52. https://www.10web.io/
  53. https://mixo.io/
  54. https://www.framer.com/ai/
  55. https://www.lindy.ai/
  56. https://relevanceai.com/
  57. https://skyvern.com/
  58. https://www.11x.ai/
  59. https://www.intercom.com/fin
  60. https://www.microsoft.com/en-us/microsoft-365-copilot
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
