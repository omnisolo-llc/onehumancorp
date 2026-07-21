issue_title: "[Research] Zero-Setup Agentic Business Ingestion Engine"
issue_description: |
  ## Title
  Zero-Setup Agentic Business Ingestion Engine

  ## Problem Statement
  Small business owners face immense "Initial Setup Paralysis" when migrating to or starting on a new platform. For example, Maya (the baker) already has 50 cake photos and descriptions on her Instagram. Carlos (handyman) has a basic service list in a PDF. Legacy platforms like Shopify and Wix require manual, tedious data entry or complex CSV formatting to create a product catalog, causing high abandonment rates during onboarding. The transition from "intent" to "live storefront" is too long.

  ## Research Report
  - **Shopify & Wix:** Rely on manual form entry or CSV imports for initial catalog creation. This assumes the user understands structured data and takes hours.
  - **Durable & AI Builders:** Can generate a beautiful static site in 30 seconds but hallucinate the business data (fake services, fake prices). The user still has to go back and replace all fake data with their real catalog, which feels like double work.
  - **OHC Opportunity:** OHC should bridge this gap with an "Agentic Business Ingestion Engine." The owner simply provides an unstructured source of truth (Instagram handle, existing messy website URL, PDF menu, or a raw text description). The AI Agent autonomously parses the source, extracts products/services, infers prices, standardizes variants, downloads and optimizes images, and structures the actual OHC database catalog.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile UI (Owner)
      participant Gateway as OHC API Gateway
      participant IntakeAgent as Agent: Business Ingestion
      participant VisionAgent as Agent: Vision & OCR
      participant DB as Postgres (Tenant DB)

      Owner->>Gateway: Submit Data Source (URL, PDF, IG Handle)
      Gateway->>IntakeAgent: Queue Ingestion Job (SKIP LOCKED)
      IntakeAgent->>VisionAgent: Scrape & Extract Entities
      VisionAgent-->>IntakeAgent: Return Unstructured Catalog (Products, Prices, Images)
      IntakeAgent->>IntakeAgent: Standardize Schema (Variants, Categories)
      IntakeAgent->>DB: Upsert Draft Catalog Items (Tenant Scoped)
      IntakeAgent-->>Gateway: Ingestion Complete
      Gateway-->>Owner: Prompt to Review Draft Catalog
  ```

  ### Mobile UX Flow (375px First)
  1. **Onboarding Screen:** A simple input field: "Where is your business right now?" (Paste a link to your Instagram, old website, or upload a menu PDF).
  2. **Loading State:** A translucent glass card shows the agent working: "Scanning Instagram...", "Extracting 12 products...", "Optimizing images...".
  3. **Review Screen:** A swipeable deck or vertical list of drafted products (Image, Name, Price). The owner can tap a checkmark to approve or quickly edit text.
  4. **Completion:** One tap to publish the catalog to the new OHC storefront.

  ### AI Agent Integration Points
  - **Vision & Scraping Agent:** Uses Playwright or direct API integration to visit the provided source, leveraging multimodal LLM capabilities to understand the visual layout and text of menus/posts.
  - **Structuring Agent:** Takes raw extracted text/images and maps them to OHC's strict multi-tenant product schema (handling edge cases like variant detection—e.g., "Size: Small / Large").

  ## Implementation Prompt
  Implement the Zero-Setup Agentic Business Ingestion Engine.
  - Create the API endpoints for accepting a data source (URL, IG handle, PDF).
  - Implement a background job using the PostgreSQL `SKIP LOCKED` pattern to process the ingestion asynchronously.
  - Build the integration with the primary LLM provider (Gemini Pro) to parse the unstructured data into the OHC catalog schema.
  - Design the mobile-first UI for the "Onboarding" and "Review" states following OHC's Translucent Glass and UniFi layout guidelines.
  - The final state should present a draft catalog to the owner for approval. Ensure all database writes are strictly scoped by `tenant_id`.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []