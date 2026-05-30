issue_title: "[Architecture] Autonomous 1-Click Platform Migration Engine"
issue_description: |
  # Title: Autonomous 1-Click Platform Migration Engine

  ## Problem Statement
  Small business owners like Priya (Boutique owner) often feel trapped on legacy platforms like Shopify or Wix. While OHC offers a superior "10-minute setup" experience for new businesses, migrating an existing store with hundreds of products, variants, customer data, and active subscriptions requires massive cognitive load. Current solutions require merchants to export CSVs, manually map column headers, re-upload images, and re-create variants—an impossible task on a 375px mobile screen. Priya needs an invisible, autonomous agent that simply asks for her current store URL or login, and invisibly scrapes, maps, and ports her entire business to OHC in minutes.

  ## Research Report
  *   **Current Architecture Limits:** OHC's current onboarding assumes a blank slate or manual conversational entry. There is no infrastructure for bulk programmatic ingestion from third-party storefronts via scraping or API proxying.
  *   **Competitor Analysis:**
      *   *Shopify:* Offers "Store Importer" apps, but they still heavily rely on CSV templates and often fail on complex variants or high-res image imports.
      *   *Wix / Squarespace:* Require manual product-by-product recreation or clunky third-party plugins like Cart2Cart, which charge hundreds of dollars per migration.
  *   **Discovery:** OHC needs a dedicated "Migration Agent" within the Onboarding Department. This engine must be capable of ingesting a seed URL (e.g., `priyasboutique.myshopify.com`), utilizing headless browser scraping (or secure API token ingestion) to read the catalog, using Vision AI to understand product imagery, and autonomously populating the OHC Universal Ledger with 100% fidelity.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ ONBOARDING_UI : "Provides Seed URL / Auth"
      ONBOARDING_UI ||--o{ MIGRATION_AGENT : "Triggers"
      MIGRATION_AGENT }|--|| CRAWLER_MESH : "Dispatches"

      CRAWLER_MESH {
          string target_platform "Shopify, Wix, Custom"
          json scraped_payload
      }

      CRAWLER_MESH ||--o{ VISION_AI_PROCESSOR : "Analyzes Images/Variants"
      VISION_AI_PROCESSOR ||--o{ MIGRATION_AGENT : "Returns Structured Data"

      MIGRATION_AGENT ||--o{ CORE_LEDGER : "Populates Catalog & CRM"
      MIGRATION_AGENT ||--o{ OPERATIONS_AGENT : "Notifies on Completion"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Customer/Merchant View (OHC Mobile App - 375px):**
      *   **Action:** Priya selects "I already have a store" during onboarding.
      *   **Input Screen:** A clean, Translucent Glass card with a single input field: "Enter your current website URL."
      *   **Processing Screen:** A UniFi-style modular dashboard card appears showing a pulsing gradient and the text: "✨ AI is packing up your shop..." Below it, small glass chips pop in as tasks complete: `[Products Found]`, `[Images Enhanced]`, `[Customer List Securely Moved]`.
      *   **Completion:** A satisfying haptic buzz. A large green checkmark over a frosted background: "Your 342 products are ready in OHC. Review Catalog."

  ### Key Design Decisions
  *   **Zero-Config Extraction:** The system must use AI to interpret the source site's DOM structure or API responses. The merchant should never need to specify "which column is the SKU."
  *   **Multi-Modal Fidelity:** Vision AI must be used to preserve or even upscale product imagery during the migration, ensuring the OHC storefront looks premium immediately.
  *   **Idempotent & Reversible:** Migrations must happen in an isolated, versioned staging state within the tenant. The merchant can preview the imported data and either "Commit" or "Discard," ensuring no permanent damage to their new OHC ledger if the scrape is imperfect.
  *   **Zero Trust:** Any OAuth tokens or credentials used to access legacy platforms via API must be ephemeral, stored in secure enclaves, and destroyed immediately after the migration job completes.

  ### AI Agent Integration Points
  *   **Onboarding Agent:** Acts as the orchestrator, communicating progress to the merchant in plain, reassuring language (e.g., "I've moved 50 of your dresses over, still working on the accessories!").
  *   **Operations Agent:** Post-migration, this agent audits the new catalog for missing data (e.g., "I noticed 3 products don't have prices, want me to set them to your average price of $45?").

  ## Implementation Prompt
  Implement the Autonomous 1-Click Platform Migration Engine. The system must allow merchants to seamlessly migrate their entire product catalog, images, and basic CRM data from legacy platforms (like Shopify or Wix) simply by providing a URL or authenticating via a standard OAuth flow. Focus on building a robust, headless crawling/ingestion mesh that utilizes AI to map unstructured or third-party data schemas into the OHC Universal Ledger. Ensure the migration process is transparently communicated via the mobile UI, completely asynchronous, and occurs in a staging environment for merchant approval before final commit. Acceptance criteria include the successful migration of a 100-item test Shopify store including complex size/color variants and high-resolution images, without requiring any manual data mapping from the user.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
