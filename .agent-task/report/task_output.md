issue_title: "Agentic Zero-Click Storefront Generation & Onboarding Flow"
issue_description: |
  ## Title
  Agentic Zero-Click Storefront Generation & Onboarding Flow

  ## Problem Statement
  Small business owners (e.g., Maya the Home Baker, Carlos the Handyman) experience severe "Setup Paralysis" when adopting traditional e-commerce platforms like Shopify or Wix. These platforms present a daunting blank canvas, requiring non-technical users to manually configure DNS, design themes, set up product catalogs, and define shipping zones before they can launch. This complexity leads to an estimated 34% abandonment rate during the initial setup phase. Owners want to focus on their craft and sales, not software configuration. They need an assistant that builds the store for them based on a simple conversational prompt.

  ## Research Report
  **Market Context & Competitor Analysis:**
  - **Shopify & Wix:** Both provide powerful customization but demand significant manual effort. Shopify's "Sidekick" chatbot assists with edits but doesn't autonomously generate the entire storefront structure from scratch. Wix Studio AI speeds up visual design but still relies on the user to piece together the operational backend (products, variants, inventory).
  - **Durable & Mixo:** These AI-native builders excel at rapid website generation (e.g., generating a site in 30 seconds). However, their outputs are often shallow, lacking robust e-commerce capabilities, deep inventory management, and integrated agentic workflows.
  - **OHC Opportunity:** OHC can differentiate by combining the speed of Durable with the operational depth of Shopify. The "Zero-Click Onboarding Agent" will take a single natural language input (e.g., "I sell custom vegan cakes in Austin, TX") and autonomously provision the tenant space, generate the initial product catalog (with variants), set up payment intent flows, and pre-render a mobile-optimized storefront.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile UX - Onboarding Prompt] --> B(Onboarding Gateway API)
      B --> C[Zero-Click Onboarding Agent]
      C --> D{LLM Reasoning & Generation}
      D --> E[Tenant Provisioning]
      D --> F[Product Catalog Generation]
      D --> G[Storefront Content & Theme]
      E --> H[(PostgreSQL: Tenant & Config)]
      F --> I[(PostgreSQL: Products & Inventory)]
      G --> J[Edge-Cached Storefront]
      H --> K[Push Notification: Store Ready]
      I --> K
      J --> K
      K --> L[Mobile UX - Approval Feed]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Landing/Prompt Screen:** A clean, uncluttered screen optimized for 375px viewports with a single text input field: "Tell me about your business..." and an optional photo upload.
  2. **Loading State (Glassmorphism):** While the agent generates the store, the user sees a premium, translucent loading screen with dynamic text explaining the agent's actions ("Setting up your catalog...", "Applying a clean, modern design...").
  3. **Approval Feed Card:** The user's home feed displays a card: "Your storefront is ready."
  4. **Storefront Preview:** Tapping the card opens a full-screen, 375px optimized preview of the generated store.
  5. **Quick Edits & Launch:** The user can tap "Launch" to immediately go live, or "Edit" to chat with the Operations Agent to make adjustments (e.g., "Change the prices to be 10% higher").

  ### AI Agent Integration Points
  - **Onboarding Agent:** Orchestrates the setup process. It decomposes the user's prompt into structured JSON data representing the business profile, products, and site theme.
  - **Operations Agent (Handoff):** Once the store is generated, the Onboarding Agent hands off to the Operations Agent, which then manages ongoing modifications and inventory tracking.

  ### Key Design Decisions
  - **Conversational Entry:** Eliminating traditional complex forms (business name, address, industry, etc.) in favor of a single natural language prompt.
  - **Opinionated Defaults:** The agent applies opinionated, high-converting defaults for layout, typography, and payment configuration to minimize decision fatigue for the user.
  - **Zero Trust & Multi-Tenancy:** The provisioning process strictly adheres to OHC's Row-Level Security (RLS) model in PostgreSQL, ensuring the generated data is perfectly isolated to the new `tenant_id`.

  ## Implementation Prompt
  **User-Facing Outcome:** As a new user, I open the OHC app and type "I'm a dog walker in Seattle." In less than a minute, I have a fully functional booking page, predefined service packages (30-min walk, 1-hour walk), and a connected payment flow, ready to share with clients.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. The user submits a plain-text business description via the mobile onboarding UI.
  2. The system triggers the Onboarding Agent, which parses the intent and generates a structured payload (Tenant details, Services/Products, Theme).
  3. The system executes database transactions to create the necessary records.
  4. The generated storefront is immediately accessible via a dedicated route.
  5. **E2E Test Requirement:** Implement a Playwright test simulating the submission of a prompt, waiting for the generation process to complete, and verifying the presence of the newly generated products on the user's dashboard and storefront preview.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
