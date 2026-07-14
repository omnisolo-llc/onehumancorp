issue_title: "Zero-Click Onboarding Agent: Autonomous Tenant & Storefront Provisioning"
issue_description: |
  # Zero-Click Onboarding Agent: Autonomous Tenant & Storefront Provisioning

  ## Problem Statement
  Small business owners face "setup paralysis." When Maya (a baker) or Carlos (a handyman) sign up for traditional platforms (like Shopify or Wix), they are confronted with a blank canvas and complex configuration menus (DNS, shipping zones, tax settings, theme customization). As noted in our competitor analysis, 34% of small business owners abandon setup due to this technical complexity. They want to start selling immediately, but the platform expects them to become administrators first. Currently, OHC requires a manual ~1-hour setup process, lacking the "Zero-to-One" autonomous experience found in AI-native platforms like Durable.

  ## Research Report
  - **Market Mapping:** General platforms (Shopify, Wix) are powerful but complex. Their AI tools (Shopify Sidekick, Wix Studio AI) assist but still require the user to navigate the admin panel. In contrast, AI-native competitors like Durable create a full website and business setup in 30 seconds from a single prompt, focusing on immediate time-to-value for service providers.
  - **The Gap:** OHC lacks an autonomous onboarding process. We have robust backend orchestration (KAIROS) and multi-tenancy, but the initial provisioning is too manual.
  - **Opportunity:** We must leverage our agentic architecture to build a "Zero-Click Onboarding Agent." This agent should take a single conversational input ("I'm Maya, I sell custom vegan cakes in Austin, TX") and autonomously generate the tenant context, a mobile-first storefront, database schema (products/services), and configure basic operations (booking/deposits) without requiring the user to navigate complex settings.

  ## Design Doc

  ### Architecture
  ```mermaid
  graph TD
      A[User Prompt: 'I sell custom cakes in Austin'] --> B(Zero-Click Onboarding Agent)
      B --> C{Intent & Entity Extraction}
      C --> D[Tenant Provisioning]
      C --> E[Data Schema Generation]
      C --> F[Storefront/UI Generation]
      D --> G(PostgreSQL: Tenant Registry)
      E --> H(PostgreSQL: Products/Services DB)
      F --> I(Edge Cache / CDN)
      G --> J[Unified Agent Feed]
      H --> J
      I --> J
      J --> K[Mobile Dashboard (375px)]
  ```

  ### Mobile UX Flow (375px First)
  1. **Landing/Signup:** Clean interface with a single text/voice input box: "Tell me about your business."
  2. **Generation State:** A dynamic loading screen showing the agent's progress ("Registering tenant...", "Designing storefront...", "Setting up payment links..."). No more than 30 seconds.
  3. **The Reveal:** The user lands on the "Unified Agent Feed" (mobile dashboard) and sees their first Action Card: "Your store is ready! Tap here to review your catalog or connect Stripe."
  4. **Storefront Preview:** A fully functional, mobile-optimized preview of their store is immediately accessible.

  ### AI Agent Integration Points
  - **Onboarding Agent (The Architect):** A specialized initial agent responsible for translating the user's prompt into a structured JSON representation of the business (name, category, initial products/services, tone).
  - **Operations Agent (The Manager):** Triggered by the Architect to create the necessary database records (Tenant, Initial Catalog, Availability Blocks for services).
  - **Marketing Agent (The Promoter):** Generates initial copywriting for the storefront and SEO metadata.

  ### Key Design Decisions
  - **Single Prompt Entry:** Remove all traditional sign-up form fields (address, industry dropdowns, etc.) in favor of conversational AI extraction.
  - **Immediate Actionable State:** The generated state is not a mock; it creates real database records in the PostgreSQL ledger (with row-level security enabled) so the user can immediately accept a booking or payment.
  - **Progressive Disclosure:** Advanced settings (custom domains, complex shipping) are hidden and introduced later via the Agent Feed only when relevant.

  ## Implementation Prompt

  **Feature:** Zero-Click Onboarding Agent

  **Target Persona:** Maya the Baker

  **User-Facing Outcome:** Maya opens the OHC app, types "I'm Maya and I sell custom vegan cakes in Austin", and within 30 seconds, she is presented with a fully functional mobile storefront featuring an initial catalog of cakes, ready to accept custom order deposits.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. User accesses the `/onboarding` route on a 375px viewport.
  2. User inputs a business description prompt.
  3. The system processes the prompt and autonomously provisions a new `Tenant` in the database.
  4. The system generates 1-3 initial `Product` or `Service` records based on the prompt context.
  5. The user is redirected to the `/dashboard` (Unified Agent Feed) and sees a success card with a link to their live storefront.
  6. **Automated Verification:** Implement a Playwright E2E test that starts at the onboarding screen, submits a prompt, waits for the generation to complete, and asserts that the new tenant dashboard and storefront preview are accessible and contain data relevant to the prompt. Use the `ai-judge` helper if validating the generated content quality.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []