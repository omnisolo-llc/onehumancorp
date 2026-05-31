issue_title: "[Research] Autonomous AI Discovery Agent for Local & LLM Search"
issue_description: |
  # [Research] AI Discovery Agent (GEO) for Local & LLM Search

  ## Problem Statement

  Small business owners like Carlos (Handyman) and Fatima (Food Cart) are essentially invisible online when they start. They do not understand SEO, cannot afford marketing agencies, and lack the technical vocabulary to optimize metadata, structure markup, or claim business profiles. When they launch a site, it receives zero traffic, leading to "Invisible Discovery"—one of the top 10 SMB pain points (52%). They need an invisible partner that automatically ensures their business is discovered not just on traditional search engines like Google, but also by LLM-driven search tools (e.g., Perplexity, ChatGPT web search) and local directories.

  ## Research Report

  Based on our market analysis of platforms (Shopify, Wix, Squarespace) and SMB pain points:

  *   **Competitor Landscape:**
      *   **Shopify/Wix:** Rely heavily on manual SEO configuration (titles, meta descriptions, alt tags). Even with some AI text generation, the user is burdened with understanding *where* to put it.
      *   **Squarespace:** Good basic structure but lacks proactive local directory synchronization.
      *   **None** of the legacy platforms are natively optimizing for Generative Engine Optimization (GEO)—ensuring LLMs synthesize the business correctly in conversational search.
  *   **The Opportunity:** OHC has the unique opportunity to treat SEO and Discovery as an autonomous, background process handled by the "Marketing & Advertising" AI Department (The Promoter/Discovery Agent). By automating Google Business Profile creation, structured schema markup generation, and localized content injection, we eliminate the need for the owner to understand "SEO."
  *   **Key Needs for Personas:**
      *   **Carlos (Handyman):** Needs to show up when someone in his zip code searches "emergency plumber near me" on their phone or asks an LLM.
      *   **Fatima (Food Cart):** Needs her operating hours, location, and halal menu structured so it's instantly visible in map searches.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  graph TD
      subgraph OHC Platform
          tenant(Tenant DB)
          agent(AI Discovery Agent - Marketing Dept)
          queue(AI Job Queue - PostgreSQL SKIP LOCKED)
          content_service(Content / Storefront Service)
      end

      subgraph External Platforms
          gbp(Google Business Profile API)
          search_engines(Search Engines - Google, Bing)
          llms(LLM Search - Perplexity, OpenAI)
      end

      tenant -- "Business Info Update (Location, Hours, Menu)" --> queue
      queue -- Dequeue Job --> agent
      agent -- "Generate Structured Data (JSON-LD)" --> content_service
      agent -- "Optimize Meta Tags & Content" --> content_service
      agent -- "Sync Profile Info" --> gbp

      content_service -- "Serve Optimized Site" --> search_engines
      content_service -- "Serve Semantic HTML" --> llms
  ```

  ### UI Wireframes / Mobile UX Flow (375px First)

  **The "1-Tap Approval" Paradigm**
  *   **Screen 1 (Dashboard Notification):** A prominent, glassmorphic card on the main dashboard: "Your AI Discovery Agent has localized your service pages. [Review & Approve]"
  *   **Screen 2 (Approval View):**
      *   *Visual:* Clean, un-cluttered view. "We generated technical SEO data to help people in [City Name] find your handyman services."
      *   *Details (hidden behind accordion for non-technical users):* "Updated meta descriptions, added JSON-LD schema for local business, optimized image alt tags."
      *   *Action:* A large primary button: "Approve & Publish."
  *   **Screen 3 (Success State):** Micro-animation showing fireworks/stars. "Your site is now optimized for search engines and AI assistants."

  ### AI Agent Integration Points

  *   **Trigger:** Whenever a tenant updates their core business info, adds a new product/service, or initially completes onboarding.
  *   **Tools:**
      *   `generate_schema_markup`: Creates JSON-LD for LocalBusiness, Product, Event, etc.
      *   `optimize_meta_tags`: Uses Gemini Pro to craft high-converting, keyword-rich title and meta description tags based on the business context.
      *   `sync_local_directory`: (Future integration) Pushes updates to Google Business Profile.

  ## Implementation Prompt

  **Prompt for Implementer Agent:**

  Implement the core logic for the AI Discovery Agent within the Marketing & Advertising department.

  **User Story:** As a non-technical business owner, when I add a new service (e.g., "Plumbing Fixes") or update my business location, my website's technical SEO (meta tags, JSON-LD schema) is automatically generated and updated in the background, so I can be found on search engines and LLM searches without writing any SEO code myself.

  **Acceptance Criteria:**
  1.  Create an AI job processor (using the existing `SKIP LOCKED` pattern or similar job queue) that listens for `BusinessProfileUpdated` or `ProductAdded` events.
  2.  The processor should use the LLM provider interface to generate appropriate JSON-LD structured data (e.g., `@type: LocalBusiness` or `@type: Product`) and optimized meta titles/descriptions based on the tenant's data.
  3.  Store this generated SEO metadata in the database linked to the specific entity (Tenant/Storefront or Product).
  4.  Ensure the storefront API or rendering engine injects this metadata into the `<head>` of the tenant's public pages.
  5.  All database schemas and background workers must respect strict multi-tenant isolation.
  6.  *Testing:* Write at least one Playwright E2E test verifying that a user can update their business info via the mobile UI, and the resulting public storefront page contains the updated, AI-generated meta tags and JSON-LD schema. Ensure 100% unit test coverage for the new AI agent logic.

  **Priority:** P1
  **Estimated Scope:** Medium

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
