issue_title: "Implement Autonomous Link-in-Bio Portfolio Engine"
issue_description: |
  # Title: Autonomous Edge-Cached Link-in-Bio & Portfolio Engine

  ## Problem Statement

  Small business owners, independent creators, and service providers—like Leo the music tutor—often run their entire business through a single link on TikTok or Instagram. Their pain point is extreme fragmentation. They currently string together Linktree for links, Calendly for bookings, Venmo for payments, and Google Drive for file delivery. If a follower wants to book a lesson or buy a digital guide, they have to click out of the social app, load a slow third-party page, and navigate a clunky multi-step checkout. This friction kills conversions. They need a single, instantly loading "link-in-bio" portfolio that automatically integrates their calendar, accepts payments, and delivers digital products—all looking beautiful and working flawlessly on a mobile phone without requiring them to code or connect multiple apps.

  ## Research Report

  The modern creator and solo service provider rely heavily on the "Link in Bio" as their primary storefront. Current market solutions fail to provide an all-in-one, highly performant, and intelligent solution:

  - **Linktree:** The dominant player, but primarily a link router. It lacks native deep integrations for bookings, subscriptions, or complex digital product delivery without relying on third-party widgets that degrade performance and UX.
  - **Stan Store:** Very popular among creators for digital products and simple calendar bookings. However, it operates as a rigid, standardized storefront that lacks true portfolio flexibility, physical product support, or deep AI conversational integration.
  - **Linkpop (Shopify):** Good for physical products if already on Shopify, but weak for service providers (bookings) and digital creators (subscriptions, courses). It lacks native scheduling capabilities.
  - **Wix/Squarespace:** Too heavyweight for a simple link-in-bio. Their mobile load times are often unoptimized for the split-second attention spans of social media users, and the setup process requires a desktop and hours of design work.
  - **The Opportunity for OneHumanCorp:** Provide an edge-cached, instant-loading portfolio that serves as a unified entry point. It must natively support OHC's existing booking, digital delivery, and subscription engines, wrapping them in a macOS-glass, UniFi-style modular card layout that looks premium out of the box. An invisible AI agent manages layout optimization, dynamic content rendering based on visitor intent, and automated follow-ups.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      SOCIAL_MEDIA_LINK ||--o{ EDGE_CDN_NODE : "resolves to"
      EDGE_CDN_NODE ||--o{ PORTFOLIO_PAGE : "serves cached"
      PORTFOLIO_PAGE ||--o{ MODULAR_CARD : "contains"
      MODULAR_CARD }|--|| OHC_CORE_SERVICES : "interacts with via APIs"

      OHC_CORE_SERVICES {
          string BookingEngine "Handles Calendars & Time Slots"
          string DigitalDeliveryEngine "Handles Files & Links"
          string SubscriptionEngine "Handles Recurring Payments"
      }

      PORTFOLIO_PAGE ||--o{ AI_VISITOR_AGENT : "monitored by"
      AI_VISITOR_AGENT }|--|| OHC_CORE_SERVICES : "triggers automations (e.g. abandoned cart)"
  ```

  ### UI Wireframes & Screen Flow (375px First)

  - **Header Section:** Full-bleed background image with a subtle blur (Translucent Glass). A circular profile avatar, business name, and a one-sentence bio.
  - **Modular Cards (UniFi Style):**
    - **Book a Lesson (Card 1):** Clean white/glass card displaying "1-on-1 Session". Tapping it smoothly expands inline (no page load) to reveal available calendar slots for the next 3 days.
    - **Digital Guide (Card 2):** Thumbnail of an eBook, title, and price button. Tapping the button invokes a native mobile-optimized bottom-sheet checkout (Apple Pay / Google Pay support).
    - **Social Links (Card 3):** Pill-shaped horizontal scrolling list of social icons.
  - **Interactions:** Every tap yields immediate haptic feedback and fluid 60fps micro-animations. Checkouts and calendar selections happen in bottom-sheet overlays to keep the user anchored to the main portfolio page.

  ### Mobile UX Flow

  1. **Discovery:** User clicks the link in Leo's TikTok bio.
  2. **Instant Load:** The page loads instantly from an edge cache, presenting the glass-morphic portfolio.
  3. **Intent Action:** User taps "Book a Lesson". A bottom sheet slides up with Calendar slots.
  4. **Checkout:** User selects a time and confirms with Apple Pay/Google Pay directly in the sheet.
  5. **Confirmation:** Sheet dismisses with a success animation. The AI backend instantly texts the user the meeting link.

  ### Zero Trust & Security Flow

  - **Multi-Tenant Isolation:** Edge cache key routing must explicitly mandate OHC_TENANT_ID namespaces for every CDN pop response, ensuring cross-tenant bleed is impossible even when URLs are scraped.
  - **SPIFFE/SPIRE Workload Identity:** Interaction between the Edge Functions (handling bottom-sheet checkouts) and OHC Core Services (BookingEngine, DigitalDeliveryEngine) must leverage ephemeral short-lived certificates validated by SPIRE. A consumer making a purchase assumes an ephemeral `checkout-session-intent` identity mapping to their device.

  ### AI Agent Integration Points

  - **Dynamic Layout Optimization:** The AI agent analyzes which links/cards are getting the most engagement and autonomously reorders them for maximum conversion, notifying the owner in their daily briefing.
  - **Conversational Concierge:** A floating, non-intrusive chat bubble allows visitors to ask "Do you have any availability next week?" and the AI agent checks the booking engine to provide direct booking links in the chat.
  - **Proactive Follow-up:** If a user initiates a booking but abandons the checkout sheet, the Marketing AI department records the intent and can trigger a retargeting workflow if identity is known.

  ### Key Design Decisions and Why

  - **Edge-Caching as Default:** To survive viral TikTok traffic spikes and prevent bounce rates, the portfolio must be statically generated and globally distributed at the edge. Dynamic personalized data (like available calendar slots) must be fetched asynchronously or hydrated via edge functions.
  - **Bottom-Sheet Overlays:** By keeping all interactions (checkout, booking, subscribing) within bottom sheets rather than navigating to new URLs, we reduce friction and context switching, passing the "grandmother test."
  - **Translucent Glass & Modular Cards:** Ensures a premium, trustworthy look and feel by default. Small businesses shouldn't need to be designers to have a beautiful brand presence.

  ## Implementation Prompt

  **Prompt for Implementer Agent:**
  Implement the "Autonomous Link-in-Bio Portfolio Engine" for OneHumanCorp.
  - **User Facing Outcome:** A business owner can activate a single-page portfolio in one tap. Visitors clicking this link from a mobile social app experience an instantly loading, premium glass-morphic page. Visitors can book appointments, buy digital goods, or subscribe directly from this page using inline bottom sheets without navigating away.
  - **Core User Journey (CUJ):**
    1. A creator (like a music tutor) toggles "Link-in-Bio" on their OHC mobile dashboard.
    2. They add three modules: A booking calendar, a digital download, and social links.
    3. A customer clicks the generated link, sees the fast-loading page, taps the booking module, selects a time from an inline bottom sheet, and pays via a native mobile wallet.
  - **Acceptance Criteria:**
    - The portfolio page achieves a 99+ Lighthouse Mobile Performance score through edge caching and minimal initial payload.
    - The UI exactly mimics the macOS translucent glass and modular card design language.
    - All complex interactions (booking, checkout) happen via bottom-sheet overlays on a 375px viewport.
    - An integration hook is provided for the AI agent to dynamically query and reorder modular cards based on telemetry.
    - No database schemas, exact API routes, or specific library choices are prescribed here—design the most robust technical implementation to meet these business outcomes.

  ## Priority
  `P1`

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
