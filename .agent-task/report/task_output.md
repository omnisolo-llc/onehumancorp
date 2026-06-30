issue_title: "[Architecture] Autonomous Zero-Friction Competitor Migration Engine"
issue_description: |
  ## Mission Queue Protocol Brief

  ### Title
  [Architecture] Autonomous Zero-Friction Competitor Migration Engine

  ### Problem Statement
  Small business owners (like Priya the boutique operator or Maya the baker) are often trapped in legacy platforms (Shopify, Wix, Squarespace) because migrating products, customer data, and booking histories is technically daunting and time-consuming. "Platform lock-in friction" prevents them from moving to OHC even when they recognize the superior agentic workflows. They lack the time and technical skill (CSV mapping, API integrations, DNS configuration) to manually copy their business state over. To acquire these users rapidly, OHC needs an engine that can autonomously ingest and recreate their entire business state from a single URL or minimal authentication.

  ### Research Report
  - **Competitive Audit**:
    - **Shopify/Wix**: Offer basic CSV import tools or rely on expensive third-party migration services (e.g., Cart2Cart) that require technical mapping and take days.
    - **OHC Advantage**: By leveraging KAIROS and our multi-modal agent departments (Marketing, Operations), we can turn a migration into an invisible, fully automated onboarding process. The owner provides their current URL or platform credentials, and the agents autonomously scrape, map, and import the catalog, generating the OHC storefront instantly.
  - **Key Findings**:
    - 65% of surveyed SMBs on legacy platforms want to switch but cite "migration pain" as the #1 blocker.
    - Owners are highly sensitive to downtime and data loss during migration.
    - A "magic migration" flow (URL -> Live OHC Store in 5 minutes) acts as a massive acquisition catalyst.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      subgraph Owner Mobile UI
          A[Enter Legacy URL or Login]
          B[Review Draft Store & Catalog]
          C[1-Tap Approve & Launch]
      end

      subgraph KAIROS Orchestrator
          D[Migration Intake Job]
          E[Progressive Sync Engine]
      end

      subgraph AI Agent Departments
          F[The Scout - Web Scraping/API Crawler]
          G[The Analyst - Data Mapping & Cleansing]
          H[The Builder - Storefront Generation]
      end

      subgraph OHC Data Core
          I[(Tenant Ledger: Products/Customers/Content)]
      end

      A -->|Submit| D
      D --> F
      F -->|Raw Data/Images| G
      G -->|Normalized Entities| I
      G -->|Triggers| H
      H -->|Draft Link| B
      B -->|Approve| C
      C -->|Finalize DNS| E
  ```

  #### Key Architectural Decisions
  1. **Asynchronous Progressive Migration**: The Scout agent works in the background (via PostgreSQL `SKIP LOCKED` job queue) to scrape public data or ingest via authorized APIs. The owner gets an optimistic UI showing the migration progress, allowing them to close the app and return later.
  2. **Multi-Tenant Normalization**: The Analyst agent uses LLM-based structured extraction (e.g., Gemini Pro) to map disparate schemas (Shopify JSON, Squarespace HTML) into OHC's unified `Product`, `Variant`, and `Customer360` entities, ensuring strict `tenant_id` isolation via Row Level Security.
  3. **Visual Integrity Preservation**: The system must automatically download, compress (to WebP), and re-host product images on OHC's GCS/MinIO storage, maintaining the owner's brand asset quality.

  #### Mobile UX Flow (375px First)
  1. **The Hook (Acquisition Screen)**: A simple glassmorphic card: "Bring your store to OHC in minutes. Enter your current website URL to start."
  2. **The Magic Progress**: While KAIROS orchestrates the agents, a visually engaging sequence (UniFi-style skeleton loaders) shows progress: "Analyzing your catalog... 42 products found," "Saving product images...," "Drafting your new OHC storefront..."
  3. **The Reveal (Draft Store)**: A 375px-optimized preview of the imported catalog within the OHC design system.
  4. **The 1-Tap Launch**: A single, prominent button (OHC Primary Green) to approve the import, with an optional "Advanced Mode" for technical DNS switchover steps (hidden by default).

  #### AI Agent Integration Points
  - **The Scout**: Uses headless browser techniques or known API patterns to extract legacy data robustly.
  - **The Analyst**: Employs prompt-based semantic mapping to convert unstructured or foreign-schema data into OHC's internal domain models (e.g., recognizing "Color: Red, Size: M" as a standard OHC Variant).

  ### Implementation Prompt
  **Goal**: Implement the backend infrastructure for the "Autonomous Zero-Friction Competitor Migration Engine."

  **Core User Journey (CUJ)**:
  1. **Intake**: Priya (Boutique Operator) enters her Shopify URL into the OHC onboarding app.
  2. **Extraction**: The system creates an asynchronous job. The backend agent pipeline scrapes/fetches her product catalog, descriptions, pricing, and images.
  3. **Normalization**: The data is mapped to OHC's `tenant`-isolated PostgreSQL schema, and images are cached locally/cloud.
  4. **Presentation**: Priya receives a notification and views her fully populated OHC draft store.

  **Acceptance Criteria**:
  - Implement a queue-based `MigrationWorker` that accepts a URL and a `tenant_id`.
  - Design the data mapping service that takes raw extracted JSON/HTML and translates it to core OHC domain entities (Products, Variants).
  - Ensure all database writes enforce the multi-tenant RLS invariant.
  - Create the API endpoints to poll migration status and retrieve the generated draft for the frontend.
  - Provide full unit test coverage (100%) and a Playwright E2E test verifying the flow from URL submission to draft store viewing.

  ### Priority
  P1 (High) - Critical for user acquisition and overcoming platform lock-in.

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
