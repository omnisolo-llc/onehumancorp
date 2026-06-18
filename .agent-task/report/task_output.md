issue_title: "Implement Edge-Cached Dynamic Storefront Generation"
issue_description: |
  # Research Report: Implement Edge-Cached Dynamic Storefront Generation

  ## Problem Statement
  Currently, small business platforms require users to manually design their website layouts, select complex themes, and optimize their pages for SEO and performance. This is a massive friction point for our personas (like Maya the baker or Carlos the handyman), who just want to turn their offline business into a beautiful, performant online store without acting as web developers. OHC currently lacks an automated mechanism to generate an edge-cached, highly performant dynamic storefront tailored to their business type.

  ## Research Report
  Our competitive analysis (from `agentic_autonomous_website_builders_smb_platform_gap_analysis.md`) shows that tools like Shopify and Wix require 30-60 minutes minimum for setup, while new AI-native platforms (like Durable) generate a basic site in seconds but lack customization and deep operational integration.

  To solve this, OHC needs an **Edge-Cached Dynamic Storefront Generator**. Instead of a static theme, the storefront should be dynamically composed by an AI agent based on the user's business profile and seamlessly distributed to a CDN/Edge caching layer (e.g., using robust HTTP cache-control, generated static assets via a Next.js/Vercel-like architecture or our own high-speed Rust/Go CDN endpoints).

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[Storefront Agent] -->|Reads Business Profile| B(Dynamic Page Generator)
      B -->|Produces HTML/CSS| C[Edge Cache Layer / CDN]
      C -->|Serves Request| D[Customer Browser]
      E[Product/Inventory Updates] -->|Cache Invalidation Event| C
  ```

  ### Mobile UX Flow (375px)
  1. The user (owner) views their 'Storefront' tab.
  2. If a storefront doesn't exist, the UI presents an AI prompt: "Describe your business in one sentence" (or auto-fills from onboarding).
  3. The Storefront Agent generates a high-quality, translucent glass-styled landing page preview.
  4. The owner taps "Publish".
  5. The backend compiles the page, sets appropriate Edge-Cache headers, and makes it available globally.

  ### AI Agent Integration
  - **Storefront Agent:** Takes the business context (e.g., "bakery selling custom cakes", "handyman services") and selects the appropriate component templates (Hero, Catalog, Contact).

  ### Key Design Decisions
  - **Zero Configuration:** The user does NOT manually configure DNS, caching rules, or themes.
  - **Performance:** All storefront pages must be edge-cacheable to ensure sub-100ms response times for buyers. Cache invalidation happens automatically on inventory updates.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the backend services and mobile-first UI for the Edge-Cached Dynamic Storefront Generator.

  **Acceptance Criteria:**
  1. Create a `StorefrontAgent` service that accepts a business profile and returns a compiled HTML/CSS template optimized for mobile (375px).
  2. Implement backend endpoints to serve this generated storefront with proper `Cache-Control` headers for edge caching.
  3. Ensure that when inventory or profile changes occur, a cache invalidation signal is emitted.
  4. Create a mobile-first UI component (following OHC's translucent glass design) where the owner can preview and publish their dynamic storefront.
  5. Ensure 100% unit test coverage and E2E Playwright tests verifying the end-to-end publishing flow.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
