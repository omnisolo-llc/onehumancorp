issue_title: "Implement 'Zero-Click' Agentic Website & Storefront Builder"
issue_description: |
  ## Problem Statement
  Small business owners like Carlos (Handyman) and Maya (Baker) need a professional web presence to attract customers and accept orders. Existing website builders (Shopify, Wix, Squarespace) are generic, require too much configuration ("blank canvas syndrome"), have steep learning curves, and don't natively integrate with booking and payment systems out of the box—especially from a mobile device. SMBs abandon website creation if it takes more than 1 hour. They need an AI agent to build the site *for* them in seconds based on a simple prompt, which they can then easily tweak from their 375px mobile device.

  ## Research Report
  - **Market Context**: Traditional builders (Shopify, Wix) focus on "chatbots that advise" and require a "Franken-stack" of apps (e.g., $100-$300/mo in app subscriptions for reviews, bookings, email marketing).
  - **OHC Differentiator**: OHC uses "AI that executes." Instead of piecing together disparate tools, the OHC `Marketing & Advertising Agent` directly mutates state and creates a cohesive storefront natively integrating commerce and bookings.
  - **Mobile Constraints**: Over 70% of local searches are mobile. OHC's builder must be 375px-first, operable with one thumb (no drag-and-drop, use up/down arrows), and feature touch targets ≥ 44x44px.
  - **The "Zero-Click" Generation Flow**: A user enters one sentence (e.g., "I'm a baker in Austin"). The agent autonomously generates the DB schema, product catalog, SEO meta tags, and storefront layout in under 10 seconds.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Intake Prompt**: User enters a single sentence prompt.
  2. **Agentic Generation (Loading state)**: The `MarketingAgent` parses the prompt, generates the site schema, provisions the tenant, applies a Premium Glassmorphism theme, and writes SEO metadata.
  3. **Live Preview (375px)**: A fully functional 375px preview is shown.
  4. **Block-Based Editor**: A bottom sheet shows semantic blocks (Hero, Product Grid, Booking Calendar). Tapping elements opens native mobile keyboards. UI uses "Move Up/Down" buttons instead of drag-and-drop.
  5. **Publish**: "1-Tap Launch" provisions an OHC subdomain or custom domain.

  ### AI Agent Integration Points
  - **The Promoter (MarketingAgent)**: Handles the initial prompt-to-site generation, SEO meta tags, and initial copy generation.
  - **State Mutations**: Agents act directly on the `StorefrontSchema` and `ProductCatalog` entities in PostgreSQL.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      subgraph Frontend "Tauri App (Mobile-First 375px)"
          Prompt[Single Sentence Prompt]
          Preview[Live Storefront Preview]
          Editor[Block-Based Editor]
      end

      subgraph Backend "Rust + Axum API"
          API[Storefront Builder API]
          Theme[Theme & Rendering Engine]
          PubSub[Publishing Engine]
      end

      subgraph AI "KAIROS Agent Mesh"
          SetupAgent[Setup / Marketing Agent]
      end

      subgraph Storage "Persistence"
          DB[(PostgreSQL - Multi-Tenant)]
          CDN[Edge CDN]
      end

      Prompt --> API
      API --> SetupAgent
      SetupAgent --> DB : Gen Schema, Products, SEO
      DB --> Theme
      Theme --> API
      API --> Preview
      Preview --> Editor
      Editor --> API
      API --> PubSub
      PubSub --> CDN
  ```

  ### Visual & Premium Standards
  - Apply **Translucent Glass** materials (`rgba(255, 255, 255, 0.65)` light / `rgba(22, 22, 26, 0.7)` dark, `backdrop-filter: blur(30px) saturate(210%)`).
  - Use Apple macOS/UniFi curves (`8px` for inputs/buttons, `16px` for container cards).

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Build the end-to-end "Zero-Click" Storefront Generation flow in the Tauri frontend and Rust backend.
  **CUJ**: A new owner logs into the Tauri app on a simulated 375px mobile device. They see an empty state and a single text input: "Describe your business." They type "I am a local handyman in Chicago offering plumbing and electrical repairs." They tap "Generate Site." Within 10 seconds, a fully rendered 375px mobile preview of a site with a Hero block, Service List (Plumbing, Electrical), and Booking form appears. The user taps "Move Down" on the Service List to swap its order with the Booking form, then taps "1-Tap Launch" to publish.
  **Acceptance Criteria**:
  1. Frontend (Tauri): Implement the single-prompt UI, loading state, 375px live preview, and block-based mobile editor (using up/down arrows, 44x44px touch targets, and Translucent Glass styling).
  2. Backend (Rust): Create the `Storefront Builder API` endpoints to receive the prompt.
  3. AI (Rust): Wire the `MarketingAgent` (or equivalent Setup Agent) to parse the prompt, generate a default site schema (Hero, Services/Products, Contact), and save it to PostgreSQL with the tenant ID.
  4. Playwright E2E: Write a test covering the exact CUJ above, ensuring NO mock data is used in the UI, and verifying the `bazel test //src/e2e:playwright` passes.

  ## Priority & Scope
  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
