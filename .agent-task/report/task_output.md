issue_title: "Autonomous Cross-Platform Migration & Ingestion Engine"
issue_description: |
  # [Architecture] Autonomous Cross-Platform Migration & Ingestion Engine

  ## Problem Statement
  Small business owners often already have a fragmented digital presence when they decide to professionalize. Maya (baker) has 400 cake photos and descriptions on Instagram, along with scattered DM orders. Priya (boutique owner) has an old, clunky Wix site with 200 inventory items, and maybe an Etsy store. Migrating these assets to a new platform is a massive barrier to entry. They do not have the technical skills to export/import CSVs, map database fields, or manually download and re-upload hundreds of images. This friction prevents activation and onboarding. They need a system where they can just provide a link to their Instagram, Etsy, or Wix site, and an AI agent invisibly scrapes, categorizes, and builds their new OneHumanCorp storefront with all their existing data, inventory, and media in under 10 minutes.

  ## Research Report
  We investigated how competitors handle onboarding and migration, finding a significant gap in automated, multi-modal ingestion.

  ### Competitive Analysis
  | Platform | Migration Approach | Key Constraint |
  |---|---|---|
  | Shopify | Store Importer app (CSV based) or third-party apps (e.g., Matrixify). | Highly technical. Requires formatted CSVs. Non-technical users struggle with field mapping. |
  | Wix | Import from CSV. Some limited direct imports for specific platforms. | Still relies heavily on manual file manipulation. |
  | Squarespace | Import tool for WordPress, Shopify, Etsy. | Brittle. Often fails on images or complex variants. Doesn't support social media scraping for catalog generation. |
  | **OHC (Target)** | **Autonomous, URL-based multi-modal scraping and ingestion.** | **Must abstract all data mapping, image processing, and formatting from the user. Zero CSVs.** |

  ### Industry Findings
  - **High Drop-off in CSV Imports:** Data shows that non-technical users abandon platform migrations at a 70%+ rate when asked to format CSV files.
  - **Social as Primary Catalog:** For micro-merchants (like Maya), their Instagram feed *is* their current catalog. Traditional platforms cannot ingest unstructured social feeds into structured e-commerce products.
  - **The LLM Advantage:** Modern vision-language models can analyze an Instagram post ("Vanilla bean wedding cake with buttercream... DM to order. $150 deposit required.") and perfectly extract the product name, description, price, and image into a structured catalog entity.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  flowchart TD
      User([User: Maya/Priya]) -->|Provides URL or Auth| IngestionUI[Onboarding Ingestion UI]
      IngestionUI --> MigrationOrchestrator[Migration Orchestrator Agent]

      MigrationOrchestrator --> ScrapingDept[AI Data Ingestion Dept]
      ScrapingDept -->|Instagram/Social| VisionLLM[Vision-Language Models]
      ScrapingDept -->|Wix/Shopify/Etsy| WebScraper[Headless Browser / API Extractors]

      VisionLLM --> DataNormalizer[AI Normalization & Mapping Agent]
      WebScraper --> DataNormalizer

      DataNormalizer -->|Creates structured entities| CorePlatform[OHC Core Ledger & Catalog]
      CorePlatform -->|Assets| CDN[Edge CDN / Image Optimization]

      CorePlatform --> Notification[Notification Dept]
      Notification -->|Sends 'Store Ready' SMS| User
  ```

  ### Data Model & Invariants
  ```mermaid
  erDiagram
      Tenant ||--o{ CatalogItem : owns
      Tenant {
          string id
          string name
          string default_currency
      }
      CatalogItem {
          string id
          string title
          string description
          float price
          string source_url
      }
      CatalogItem ||--o{ MediaAsset : contains
      MediaAsset {
          string id
          string cdn_url
          string original_url
      }
      Tenant ||--o{ MigrationJob : manages
      MigrationJob {
          string id
          string status
          string source_url
      }
  ```
  **Multi-Tenancy Rules:** Strict row-level security (RLS) ensuring `MigrationJob` and `CatalogItem` records are isolated by `tenant_id`. Zero-Trust verified using SPIFFE/SPIRE for the microservices hitting the ScrapingDept.

  ### UI Wireframes & Mobile UX Flow
  **Target: 375px viewport (Mobile First)**
  - **Screen 1 (The Hook):** "Bring your existing business to OHC in 1 tap." Input field for an Instagram handle, Etsy store link, or existing website URL. Big, translucent glass button: "Import My Business".
  - **Screen 2 (The Magic):** A beautiful loading screen with Apple-style fluid animations. Text updates dynamically: "Scanning Instagram...", "Found 45 products...", "Enhancing images...", "Building your storefront...".
  - **Screen 3 (The Reveal):** "Your store is ready." Shows a preview of the new OHC mobile storefront, fully populated with their existing items. Two buttons: "Looks Perfect - Go Live" or "Review Items".

  ### Performance & Offline Targets
  - **UI Latency:** UI must not block. Migration jobs must be pushed to a high-performance background queue (e.g., NATS or similar).
  - **Offline Capability:** The onboarding wizard must allow queuing the ingestion link while offline; it triggers automatically when the connection is restored.
  - **Zero Trust Security:** Secure identity validation between the `MigrationOrchestrator` and `ScrapingDept` to prevent unauthorized cross-tenant extraction.

  ### AI Agent Integration Points
  - **Ingestion Agent:** Given a URL, determines the source type (Social, E-commerce competitor) and selects the right scraping strategy.
  - **Vision & Mapping Agent:** Uses LLMs to parse unstructured text and images into structured OHC Data Models (e.g., extracting price, variants, and descriptions from an Instagram caption).
  - **Enhancement Agent:** Automatically upscales low-quality scraped images and removes messy backgrounds if needed, preparing them for the premium OHC storefront.

  ### Key Design Decisions
  1. **Zero-CSV Policy:** The system will never ask the user to upload a CSV. All data must be sourced via URL scraping, OAuth integrations, or direct image uploads.
  2. **Asynchronous Processing:** Migration can take minutes. The user is freed from the app immediately, and the Operations Agent will send an SMS or WhatsApp message when the store is fully generated.
  3. **Optimistic Ingestion:** It's better to ingest a product with slightly imperfect categorization that the user can fix later, than to fail the migration or ask the user complex mapping questions upfront.

  ## Implementation Prompt
  **User-Facing Outcome:** A user can paste their Instagram handle or old website link during onboarding, put their phone down, and receive a text 5 minutes later with a link to their fully built, production-ready OHC storefront containing all their historical products, images, and descriptions.

  **Core User Journey (CUJ):**
  1. User enters their Instagram handle or Wix URL in the mobile onboarding flow.
  2. The user sees an engaging loading screen confirming the system is working.
  3. Behind the scenes, the system scrapes the source, uses LLMs to structure the data into OHC Product entities, downloads and optimizes the images to the OHC CDN, and attaches them to the new store.
  4. User receives an SMS notification.
  5. User clicks the link in the SMS and views their populated store.

  **Acceptance Criteria:**
  - The engine must support at least two ingestion sources: Instagram (public profile) and a generic website URL (HTML meta-scraping).
  - Extracted data must map correctly to the OHC core catalog schema (Title, Description, Price, Image URL).
  - The entire process must be asynchronous and not block the user's mobile device or require the app to stay open.
  - The UI must adhere to the premium translucent glass design language and be perfectly usable on a 375px screen.

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
