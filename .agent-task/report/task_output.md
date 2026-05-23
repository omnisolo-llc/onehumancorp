issue_title: "[architecture] Universal Social Link-in-Bio & Commerce Engine"
issue_description: |
  # Issue Brief: Universal Social Link-in-Bio & Commerce Engine

  ## Title
  Universal Social Link-in-Bio & Commerce Engine

  ## Problem Statement
  Creators and service-based solopreneurs (like Leo the music tutor or Maya the baker) rely heavily on social media (TikTok, Instagram) as their primary top-of-funnel acquisition channel. The single link in their social profile is critical real estate. Currently, they use tools like Linktree or Beacons to aggregate links, but these tools are disconnected from their actual business operations (booking, inventory, payments). This forces a "Frankenstein stack": a user clicks a Linktree link, goes to a Calendly page to book, then gets a PayPal invoice. This disjointed journey causes high drop-off rates and "Conversion Friction". Small business owners need a unified, instantly generated "Link-in-Bio" page that acts as a full-fledged commerce storefront, directly integrated with the KAIROS engine for seamless booking, payments, and digital product delivery without ever leaving the social app's in-app browser.

  ## Research Report
  - **Competitive Audit**:
    - **Linktree / Beacons / Stan Store**: Excellent at simple link aggregation and basic digital product sales, but lack deep operational integration (e.g., complex service booking with deposits, physical inventory sync). They operate as standalone silos.
    - **Shopify Starter**: Offers a link-in-bio tool (Linkpop), but it is heavily biased towards physical products and lacks native, fluid service booking or robust AI autonomy.
    - **Wix / Squarespace**: Too slow and complex to act as a lightweight, instant-loading social bio link.
    - **OHC Advantage**: By treating the Link-in-Bio as just another "surface" for the unified KAIROS data model (where inventory, calendar, and AI agents live), OHC can offer a 1-tap setup that immediately pulls the existing catalog into a mobile-perfect, high-converting social landing page.
  - **Key Findings**:
    - 75% of social traffic drops off if required to navigate through more than 2 distinct external platforms to complete a purchase.
    - Service providers (tutors, consultants) need a bio link that isn't just links, but a dynamic, real-time availability calendar.
    - The "Aha!" moment is generating this entire page automatically from the user's existing Instagram content or basic business description.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      SOCIAL_PROFILE ||--o{ LINK_IN_BIO_SURFACE : "points to"
      LINK_IN_BIO_SURFACE }|--|| KAIROS_EDGE_CACHE : "served from"

      LINK_IN_BIO_SURFACE {
          string custom_slug "e.g., ohc.bio/leo"
          jsonb layout_config "Cards, Calendar, Digital Products"
          string theme_id
      }

      KAIROS_EDGE_CACHE ||--o{ UNIFIED_CATALOG : "fetches"
      KAIROS_EDGE_CACHE ||--o{ BOOKING_ENGINE : "fetches availability"

      UNIFIED_CATALOG {
          string product_id
          string type "Physical, Digital, Service"
          int price
      }

      LINK_IN_BIO_SURFACE ||--o{ COMMERCE_CHECKOUT : "triggers 1-click"
      COMMERCE_CHECKOUT ||--o{ PAYMENT_GATEWAY : "Stripe/Native"
  ```

  ### AI Agent Integration Points (The Marketer & The Receptionist)
  - **The Marketer Agent**: During onboarding, scans the user's social media profile (via public APIs or manual upload) to automatically generate the bio page layout, extract brand colors, and write high-converting copy for the call-to-action buttons.
  - **The Receptionist Agent**: Integrates into the bio page as a persistent, floating "Ask Me Anything" chat bubble. If a customer clicks the bio link and asks "Do you have slots next week?", the Receptionist instantly checks the `BOOKING_ENGINE` and replies with available times right on the page.

  ### Key Architectural Invariants
  1. **Ultra-Low Latency Edge Serving**: The Link-in-Bio page must be served from edge locations (e.g., Cloudflare Workers or similar) to ensure sub-200ms First Contentful Paint (FCP). It is critical for mobile social traffic.
  2. **1-Click Commerce Integration**: Transactions (booking a slot, buying an eBook) must happen directly on the bio page without redirecting to a separate cart domain, utilizing universal payment elements (Apple Pay, Google Pay).
  3. **Zero-Config Sync**: Any change to inventory or calendar in the main OHC dashboard must instantly invalidate the edge cache and reflect on the bio page without manual republishing.

  ### Mobile UX Flow (375px First)
  1. **Discovery**: Customer taps the link in Leo's TikTok bio (`ohc.bio/leo_guitar`).
  2. **The Bio Page**: A fast-loading, frosted glass UI opens in TikTok's in-app browser.
     - Top: Leo's profile picture and a short AI-generated bio.
     - Section 1: "Book a Lesson" (A horizontal scrolling calendar widget).
     - Section 2: "Buy my Chords eBook" (A product card with a 1-tap "Buy with Apple Pay" button).
     - Floating Bottom Right: "Chat with AI" bubble.
  3. **Action**: Customer taps a calendar slot and authenticates via Apple Pay in one motion.

  ## Implementation Prompt
  **Goal**: Build the "Universal Social Link-in-Bio & Commerce Engine" to provide a high-converting, instantly generated social landing page that natively hooks into the core OHC commerce and booking systems.

  **Core User Journey (CUJ)**:
  1. Leo clicks "Create Social Bio Link" in the OHC mobile app.
  2. The AI Marketing Agent instantly generates a preview, pulling his existing lesson packages and calendar availability into a clean, mobile-optimized stack of cards.
  3. Leo approves the design and pastes the `ohc.bio/leo` link into his Instagram.
  4. A follower clicks the link, sees real-time availability, and books a $50 lesson using Apple Pay without leaving the Instagram app.

  **Acceptance Criteria**:
  - Implement the `LINK_IN_BIO_SURFACE` data model and routing to serve these pages at a dedicated path (e.g., `/bio/:slug`).
  - Develop the frontend components using the OHC Visual Mandate (Glassmorphism, 44x44px touch targets) specifically optimized for 375px mobile viewports and in-app browsers.
  - Integrate the `BOOKING_ENGINE` and `UNIFIED_CATALOG` so that availability and products are rendered natively on the bio page, not as external links.
  - Implement a streamlined 1-click checkout flow overlay directly on the bio page.
  - Ensure the page architecture supports aggressive edge caching for extreme performance.

  ## Priority
  P0 (Critical) - High demand from creator and service-provider personas.

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []