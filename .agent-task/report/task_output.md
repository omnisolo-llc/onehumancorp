issue_title: "[Architecture] Autonomous AI Business Ingestion & Migration Engine"
issue_description: |
  # [Architecture] Autonomous AI Business Ingestion & Migration Engine

  ## Title
  Autonomous AI Business Ingestion & Migration Engine

  ## Problem Statement
  The primary friction point for a non-technical small business owner (like Maya the baker or Carlos the handyman) is the initial platform setup. Competitors like Shopify or Wix require hours of manual data entry, uploading photos one by one, formatting menus, copying business hours, and importing reviews. This manual labor is the #1 cause of abandonment during the "Acquisition to Activation" phase.

  For OneHumanCorp (OHC) to achieve the "zero → live business in under 10 minutes" promise, we cannot rely on manual data entry or complex CSV uploads. We need a system where a user simply pastes a link to their existing digital footprint (e.g., an Instagram handle, a Google Maps link, or a Yelp profile), and our AI invisibly scrapes, normalizes, and populates the entire OHC business data model in under 60 seconds.

  ## Research Report
  ### Market Context
  - **Shopify:** Provides tools like "Store Importer" but it heavily relies on CSVs or specialized apps that users have to figure out how to configure. High friction.
  - **Wix:** Has an AI website builder but it usually creates placeholder text and stock images rather than intelligently porting over the user's real business data from social platforms.
  - **Durable:** Generates a site in 30 seconds using localized Google Maps data, but lacks deep e-commerce ingestion (like pulling Maya's cake photos from Instagram DMs and turning them into shoppable catalog items).

  ### Small Business Persona Gaps
  - **Maya (Baker, 28):** Her entire business lives on Instagram. She has no CSV of products. She needs OHC to look at her Instagram feed, identify posts that look like cakes, extract the descriptions (which often contain pricing), and automatically build a visual catalog.
  - **Fatima (Food Cart, 50):** Has a Yelp page and Google Maps listing. She needs her menu, operating hours, and location scraped and translated if necessary, creating a live pre-order storefront with zero typing.

  ### The OHC Opportunity
  By using our existing event-driven AI agent architecture, the **Operations Agent** and **Marketing Agent** can coordinate a web-scraping and vision-parsing pipeline. The user pastes a link; the system goes to work and produces an instantly activated platform ready for revenue generation.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  sequenceDiagram
      participant User as User (Mobile App)
      participant KAIROS as KAIROS Orchestrator
      participant Scout as Scout Resource Integrator
      participant Marketing as Marketing Agent
      participant Ops as Operations Agent
      participant DB as OHC Unified DB (Ledger/Catalog)

      User->>KAIROS: Pastes Link (e.g., "instagram.com/mayascakes")
      KAIROS->>Scout: Dispatch scraping job for target URL
      Scout-->>KAIROS: Return raw HTML, JSON, Images
      KAIROS->>Marketing: Parse brand vibe, colors, description, hours, reviews
      Marketing-->>DB: Upsert Tenant Config, Theme, Storefront Data
      KAIROS->>Ops: Vision-parse images to detect catalog items & prices
      Ops-->>DB: Upsert Product Entities, Variants, Pricing
      DB-->>KAIROS: Ingestion Complete
      KAIROS->>User: "Your store is ready! Review your catalog." (Under 60s)
  ```

  ### Key Design Decisions and Why
  1. **Pasted Link vs. OAuth Authorization:** OAuth is preferred for continuous sync, but for the *initial ingestion*, a simple public URL paste (Instagram, Yelp, Google Maps, Facebook) minimizes drop-off. We prioritize scraping public data instantly to achieve the "Aha!" moment without requiring the user to remember passwords.
  2. **Vision-Parsing Catalog Items:** For visual businesses (Maya), the Operations Agent uses Vision-Language Models (VLMs) to look at scraped images, recognize items (e.g., "Vegan Chocolate Cake"), and estimate categorization rather than relying solely on text descriptions.
  3. **Asynchronous but Optimistic UI:** The scraping and AI analysis will take 20-60 seconds. During this time, the mobile UI must display an engaging, optimistic "building your business" skeleton loading state to prevent the user from abandoning the app.
  4. **Tenant Isolation:** All scraped data is immediately sandboxed via SPIFFE/SPIRE identity rules.

  ### Mobile-First UX Flow (375px Viewport)
  1. **Screen 1 (Input):**
     - Clean translucent glass card: "Where is your business right now?"
     - Single, prominent text input field with placeholder: "Paste your Instagram, Google Maps, or Website link".
     - Large, thumb-friendly primary button: "Build My Store".
  2. **Screen 2 (Loading / Optimistic):**
     - Lottie animation showing AI building blocks.
     - Text updates dynamically: "Scanning photos...", "Setting up your calendar...", "Formatting your menu..."
     - No complex developer jargon; friendly, supportive tone.
  3. **Screen 3 (The "Aha!" Reveal):**
     - High-fidelity preview of the newly generated mobile storefront.
     - A floating bottom action bar: "Looks Good! Go Live" or "Edit".
     - Grandmother Test: If Fatima can't understand that her menu is ready to accept orders just by looking at this screen, we fail.

  ### AI Agent Integration Points
  - **Scout Resource Integrator:** Handles the actual headless browser scraping, bypassing basic bot protections and downloading assets.
  - **Marketing Agent:** Analyzes the text content to generate SEO-optimized descriptions, extract business hours, and define the brand color palette based on image dominant colors.
  - **Operations Agent:** Takes raw image/text pairs and turns them into structured Product or Service Catalog entries in the database.

  ## Implementation Prompt
  **User-Facing Outcome:** The user can paste a single URL (e.g., an Instagram profile or Google Maps link) during onboarding. Within 60 seconds, they are presented with a fully populated OHC storefront containing their products/services, business hours, cover photos, and location data, requiring zero manual data entry.

  **Core User Journey (CUJ):**
  1. User opens the OHC mobile app for the first time.
  2. User selects "Import from existing site/social".
  3. User pastes their Instagram profile URL.
  4. User watches a 30-second progress screen.
  5. User is presented with a fully built storefront featuring their latest Instagram photos as shoppable catalog items.

  **Acceptance Criteria:**
  - The ingestion pipeline must accept at least one social or maps URL and successfully extract standard business metadata (Name, Hours, Bio/Description).
  - The pipeline must extract at least 5 images and convert them into draft catalog items using an AI vision model.
  - The entire process must complete in under 60 seconds.
  - The UI must be fully responsive and pass the 375px mobile-first design standard with translucent glass materials.
  - All created entities must be strictly scoped to the user's `tenant_id`.
  - Do not prescribe specific database schemas or API signatures; design the event payloads and background worker queues that enable this asynchronous flow.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
