issue_title: "Architecture: Autonomous Competitor Migration & Ingestion Engine"
issue_description: |
  # Title: [Architecture] Autonomous Competitor Migration & Ingestion Engine

  ## Problem Statement
  When we convince small business owners to switch to OneHumanCorp (OHC) from legacy platforms like Shopify, Wix, or Squarespace, they immediately hit a brick wall: data migration. Non-technical users like Maya (baker) or Priya (boutique owner) do not understand how to export CSV files from Shopify, map the correct column headers, and import them into a new system. They do not know how to bulk download their product images and re-upload them.
  This friction is the single biggest barrier to acquisition. If a user has 50 products on Wix, the thought of manually copying and pasting them into OHC will cause them to abandon the onboarding process entirely. The "Cost of Switching" is too high.
  We need an invisible, zero-click migration engine where the user simply provides their current website URL, and our AI agents autonomously ingest, structure, and populate their entire catalog and storefront on OHC.

  ## Research Report
  - **Competitor Solutions:** Platforms like Shopify offer "Import Tools" that rely heavily on the user formatting CSVs correctly or installing third-party migration apps (like Cart2Cart) which are expensive, highly technical, and prone to error.
  - **Competitor Gap:** No platform currently offers an intelligent, visual-first scraping and ingestion engine that requires zero data export from the source platform.
  - **OHC Advantage:** OHC can leverage our `Sourcing Agent` and visual reasoning AI to act as a web crawler. By simply providing a URL, the AI can visit the existing storefront, identify product cards, extract titles, prices, descriptions, and high-resolution images, and map them directly into OHC's `Universal Capacity Ledger` and `Product` schemas.

  ## Design Doc

  ### Business Journey Mapping
  1. **Acquisition & Onboarding**: Maya signs up for OHC. The onboarding wizard asks, "Do you already have a website?" Maya enters `mayas-cakes.myshopify.com`.
  2. **Ingestion (The Magic Moment)**: Maya sees a progress screen ("Our AI is carefully moving your 42 cakes to your new home..."). Behind the scenes, the engine crawls her old site, extracting products, variants, and images.
  3. **Activation**: Under 5 minutes later, Maya's new OHC storefront is live, fully populated with her exact inventory, but upgraded with OHC's AI agents and premium design.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant User as Maya (Mobile 375px)
      participant OHC as OHC Dashboard
      participant Hub as KAIROS Orchestrator
      participant Scraper as The Ingestor (Crawler Agent)
      participant Vision as Vision AI (Image extraction)
      participant Ledger as Universal Capacity Ledger

      User->>OHC: Enters existing website URL
      OHC->>Hub: Trigger `MigrationEvent`
      Hub->>Scraper: Task: Crawl and extract catalog
      Scraper->>Scraper: Map sitemap & identify product pages
      Scraper->>Vision: Send screenshots/DOM for extraction
      Vision-->>Scraper: Return structured JSON (Title, Price, Desc, Images)
      Scraper->>Ledger: Insert `Product` and `Capacity` entities
      Hub-->>User: Notification: "Migration Complete. 42 items imported."
  ```

  ### Data Model & Invariants
  - **Migration Job**: Tracks the status of the URL ingestion.
  - **Ingestion Mapping**: The AI extracts unstructured DOM data and maps it to our strict `Universal Capacity Ledger` entity model.
  - **Image Pipeline**: Images extracted from the competitor site must be autonomously downloaded, compressed, and re-hosted on OHC's edge CDN.

  ### AI Department Coordination
  - **The Ingestor Agent (Ops/Migration)**: Orchestrates the web crawling, handling rate limits, CAPTCHAs, and pagination on the source platform.
  - **The Visualizer (Vision AI)**: When standard DOM parsing fails, it uses visual reasoning to identify product images, prices, and variant options from rendered pages.

  ### Mobile-First UX Flow (375px First)
  - **Screen 1: The Input**: A simple, single input field: "What's your current website address?"
  - **Screen 2: The Magic Loader**: A highly polished, satisfying loading screen with Translucent Glass materials. Text updates dynamically: "Found 12 products...", "Downloading images...", "Setting up your inventory...".
  - **Screen 3: The Reveal**: A summary card showing the migrated catalog. A single large button: "Review & Publish." No mapping of CSV columns.

  ## Implementation Prompt
  **Task for Implementer**: Build the core Autonomous Competitor Migration & Ingestion Engine.

  **Core User Journey (CUJ)**:
  A user provides the URL to their existing Shopify or Wix store during onboarding. The system queues a migration job, autonomously crawls the URL, extracts product data (title, description, price, variants, and images), and populates their new OHC catalog without any manual data entry or CSV mapping.

  **Acceptance Criteria**:
  1. Implement an async ingestion job queue that accepts a source URL.
  2. Integrate a web crawling and scraping mechanism capable of extracting product metadata from common eCommerce DOM structures or via visual LLM extraction.
  3. Build a pipeline to download remote product images and store them in OHC's blob storage.
  4. Ensure the extracted data accurately maps to the `Universal Capacity Ledger` and `Product` models.
  5. Provide a webhook or polling endpoint so the mobile UI can display real-time progress to the user.
  6. The user must not be asked to map columns or handle technical data mapping.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
