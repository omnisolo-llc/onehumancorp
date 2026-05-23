issue_title: "[Architecture] Zero-Click Autonomous Platform Migration Engine"
issue_description: |
  # [Architecture] Zero-Click Autonomous Platform Migration Engine

  ## Problem Statement
  For non-technical small business owners like **Priya (boutique owner)** and **Leo (music tutor)**, the biggest barrier to adopting OneHumanCorp (OHC) is the switching cost. Existing platforms (Shopify, Wix, Calendly) hold their business data hostage. To migrate, users are currently forced to figure out complex CSV exports, manually download and re-upload gigabytes of product photos, and manually re-create their service booking availability. This manual migration process fails the "grandmother test" and leads to high abandonment during onboarding. When Priya wants to switch her boutique from Shopify to OHC, she doesn't want to spend three days mapping database columns; she wants an invisible assistant to do it for her.

  ## Research Report
  - **Competitor Landscape**: Shopify provides a store importer app that requires users to generate API tokens or upload CSVs formatted specifically for their schema. Wix offers a limited URL importer but often fails to grab variants or high-res images. No competitor offers a truly autonomous, multi-agent migration.
  - **Business Impact**: High friction in onboarding causes massive drop-off for established businesses (which have higher LTV than new businesses).
  - **The OHC Differentiator**: OHC will use an autonomous migration engine. Instead of a CSV uploader, the user simply pastes their existing store/booking URL or connects via secure OAuth. AI agents automatically crawl, parse, classify, and rebuild the catalog, service list, images, and prices natively into the OHC multi-tenant ledger.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Priya (Mobile 375px)
      participant Onboarding as OHC Onboarding flow
      participant KAIROS as KAIROS Orchestrator
      participant Scout as Migration Scout Agent
      participant Ops as AI Operations Agent
      participant Storage as Secure Tenant Ledger

      User->>Onboarding: Pastes Shopify/Wix URL
      Onboarding->>KAIROS: Triggers Migration Event
      KAIROS->>Scout: Spin up headless browser & API scraper
      Scout->>Scout: Crawl legacy site & extract raw data
      Scout->>Ops: Pass raw text/images
      Ops->>Ops: Structure data (Products, Variants, Prices)
      Ops->>Storage: Upsert to OHC multi-tenant ledger
      KAIROS->>User: Push Notification: "Your store is ready! 1-Tap to review."
  ```

  ### Mobile UX Flow (375px First)
  1. **Input Screen**: Clean glass-morphism card: "Moving from another platform? Just paste your link." (Input field + 'Migrate' button).
  2. **Loading State**: Engaging loading animation showing AI agents "working" (e.g., "Scouting your catalog...", "Polishing product images...").
  3. **Review Screen**: The user receives a push notification when complete. Tapping opens a summarized review card: "We imported 142 products, 320 photos, and your service menu. Looks good?"
  4. **Action**: A single "1-Tap Approve" button to publish the new OHC storefront.

  ### AI Agent Integration Points
  - **Migration Scout Agent**: Operates in the background to fetch raw data via scraping or public APIs.
  - **AI Operations Agent**: Parses unstructured HTML/JSON into the strict OHC data model invariants. Generates product embeddings for the Memory Consolidation Layer.

  ### Key Design Decisions
  - **Zero-Trust**: Migration tasks run in isolated sandboxes. Imported data is strictly bound to the user's `tenant_id`.
  - **Agentic Delegation**: The migration is entirely asynchronous and non-blocking. The user is encouraged to close the app while the AI works.
  - **Resilience**: The system gracefully handles missing data (e.g., if an image fails to load) by substituting intelligent defaults or generative placeholders.

  ## Implementation Prompt
  Implement the "Zero-Click Autonomous Platform Migration Engine" starting with Shopify and Wix as the initial source platforms. Create the backend worker queues and the Migration Scout Agent responsible for extracting data from a provided URL. Build the AI pipeline that structures this raw data into OHC's product and service models, ensuring strict `tenant_id` isolation. Finally, implement the mobile-first React Native (or equivalent web) 375px UI components that allow the user to input a URL and asynchronously receive a completion notification to approve the imported catalog. Ensure performance targets are met such that the background ingestion does not impact core KAIROS event processing.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []