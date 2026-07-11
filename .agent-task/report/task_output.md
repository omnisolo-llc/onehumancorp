issue_title: "AI-Native Zero-Click Omnichannel Storefront & Booking Generation"
issue_description: |
  # OHC Feature Brief: AI-Native Zero-Click Omnichannel Storefront & Booking Generation

  ## Title
  AI-Native Zero-Click Omnichannel Storefront & Booking Generation

  ## Problem Statement
  Non-technical small business owners (like Carlos the Handyman and Maya the Baker) experience "setup paralysis" when joining traditional platforms (Shopify, Wix, Squarespace). They are greeted with blank canvases, complex menus, and the immediate need to stitch together 5-10 apps (e.g., e-commerce, bookings, SEO) to achieve parity with modern customer expectations. This fragmented "App Tax" costs time and money. They need an assistant that instantly sets up their business infrastructure based on a single conversation, not a software manual they have to learn.

  ## Research Report
  **Competitive Landscape:**
  - **Shopify:** Extremely powerful backend but zero-click setup does not exist. Relies heavily on a complex ecosystem of third-party apps for basic functions like service bookings, which adds to the monthly cost and fractures the UX.
  - **Wix/Squarespace:** Offers native booking and commerce, but requires manual drag-and-drop configuration.
  - **AI Builders (Durable, Mixo):** Can generate a site in 30 seconds, but lack deep operational integration (e.g., tenant-isolated multi-product databases, recurring booking schedules, integrated AI agents for customer success).

  **OHC Gap:**
  OHC aims for sub-10-minute time-to-value. Currently, OHC needs a robust, fully autonomous initialization flow where a single prompt (e.g., "I'm a baker in Austin selling custom cakes and hosting weekend workshops") triggers the generation of the entire backend tenant, frontend UI, product catalogs, and booking availability logic seamlessly.

  ## Design Doc

  **Architecture Diagram:**
  ```mermaid
  sequenceDiagram
      participant Owner (Mobile)
      participant OHC Triage Agent
      participant Setup/Operations Agent
      participant OHC Tenant DB

      Owner (Mobile)->>OHC Triage Agent: "I bake custom cakes and run classes."
      OHC Triage Agent->>Setup/Operations Agent: Parse intent, extract entities (products, services)
      Setup/Operations Agent->>OHC Tenant DB: Provision tenant schema, catalog, booking slots
      Setup/Operations Agent->>Owner (Mobile): Deliver interactive mobile-first preview
      Owner (Mobile)->>Setup/Operations Agent: Approve / "Make it more professional"
      Setup/Operations Agent->>OHC Tenant DB: Update tenant config & theme
  ```

  **Mobile UX Flow (375px):**
  1. **Acquisition/Onboarding:** A single chat interface. "Tell me about your business."
  2. **Generation State:** A dynamic, Apple-style loading visualization showing agents working (e.g., "Drafting copy...", "Setting up booking calendar...").
  3. **Interactive Preview:** The owner sees a fully functional mobile preview (translucent glass styling, clear typography). No drag-and-drop editor; modifications are made via chat (e.g., "Change the colors to warm pastels").
  4. **Launch:** One-tap approval to go live and connect Stripe.

  **AI Agent Integration Points:**
  - **Triage Agent:** Analyzes the initial onboarding prompt.
  - **Operations Agent:** Sets up the booking availability blocks and product catalog.
  - **Marketing Agent:** Generates SEO-optimized copy, meta tags, and placeholder imagery.

  **Key Design Decisions:**
  - **No Drag-and-Drop Editors:** All UI customization is handled conversationally through agents to avoid setup paralysis.
  - **Unified Model:** Both physical products (cakes) and services (workshops) use the same generated catalog/booking core, eliminating the need for separate apps.
  - **Zero Mock Data:** The generated storefront immediately uses live tenant-isolated database rows so it is functional on day one.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the "Zero-Click Storefront & Booking Generation" user journey. Build a conversational onboarding mobile-first interface (targeting 375px width) where a business owner inputs a single natural language description of their business. The interface must communicate with the backend to trigger the creation of a realistic, fully populated store and booking calendar (persisted in the database, no frontend mock data).

  The output should be an interactive preview of their business (products and service slots) that they can refine via further chat commands. Ensure the UI adopts the OHC Premium Token glassmorphism design system. Provide full Playwright E2E test coverage for the onboarding flow from initial prompt to generated live preview.

  ## Priority
  P2

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
