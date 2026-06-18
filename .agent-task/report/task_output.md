issue_title: "Develop Zero-Touch Mobile Onboarding Flow for Maya (Baker Persona)"
issue_description: |
  # Research Report: Zero-Touch Mobile Onboarding for Small Business

  ## Title
  Zero-Touch AI Mobile Onboarding Flow for Maya (Baker Persona)

  ## Problem Statement
  Non-technical small business owners like Maya (a baker who relies on Instagram DMs) experience setup paralysis when using traditional e-commerce platforms like Shopify or Wix. The initial blank canvas is terrifying, and the "App Tax" required to piece together tools (booking, inventory, messaging) creates immense friction. OHC currently lacks a zero-touch, conversational onboarding flow that takes a user from a simple prompt ("I'm a baker in Austin") to a fully generated storefront and database schema natively from their mobile device.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify/Wix:** Rely on complex desktop-first setups and third-party apps to add core functionality like bookings or custom deposits. Setup takes 30-60 minutes minimum.
  - **Durable/10Web:** AI-native platforms generate sites quickly (in 30 seconds), but these are often just landing pages without deep operational capabilities or unified databases.
  - **OHC Opportunity:** Implement a "Zero-Click Generation" flow where an AI Setup Agent (The Promoter) autonomously generates the DB schema, product catalog, storefront layout, and books the setup tasks in under 10 minutes, entirely from a 375px mobile viewport.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Onboarding UI 375px] -->|Natural Language Prompt| B(Setup Agent API Gateway)
      B --> C{The Setup Agent}
      C -->|Generate Schema & Catalog| D[PostgreSQL Central Ledger]
      C -->|Design Storefront| E[Flutter/PWA Storefront Service]
      C -->|Configure Bookings| F[Operations Agent]
      D --> G[Tenant Scope Isolated]
      E --> H[Live Preview Ready]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Step 1: The Prompt:** A single input field on a clean, translucent glass card: "Tell me about your business." Maya enters, "I bake custom vegan cakes in Austin."
  - **Step 2: Generation Screen:** An animated loading state explaining the work happening in the background ("Creating your catalog", "Setting up booking slots").
  - **Step 3: The Reveal:** A fully functional live preview of her store with generated products (e.g., "Custom Vegan Birthday Cake"), pricing placeholders, and a functional deposit-booking calendar.
  - **Step 4: 1-Tap Approval:** A prominent "Launch My Store" button.

  ### AI Agent Integration Points
  - **The Setup Agent:** Parses the user's natural language input, interfaces with Gemini to generate standard products and descriptions tailored to a baker, and seeds the PostgreSQL database.
  - **The Operations Agent:** Configures default calendar availability blocks for cake pickups/deliveries based on industry norms (which Maya can later adjust).

  ### Key Design Decisions
  - **Mobile-First Exclusively:** The entire flow must be designed for and perfectly usable on a 375px screen.
  - **Action over Advice:** The AI does the heavy lifting of generating data, creating schemas, and building the UI, rather than just telling the user how to do it.
  - **Unified Operations:** The generated store seamlessly integrates physical products (cakes) with services (consultations/bookings) natively without apps.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya opens the OHC mobile app for the first time. She types "I'm a custom baker in Austin specializing in vegan cakes." Within 3 minutes, she has a fully populated storefront with AI-generated product images, a deposit booking system, and a clean UI, ready to share on her Instagram.

  **CUJ & Acceptance Criteria:**
  1. User navigates to the `/onboard` route on a 375px viewport.
  2. User enters a description of their business into the input field and submits.
  3. The `SetupAgentService` is triggered, calls the LLM, and generates a structured JSON response containing sample products, business details, and default booking settings.
  4. The generated data is persisted to the Postgres database under a new isolated tenant.
  5. The user is redirected to their live generated storefront preview.
  6. **Testing:** Provide a Playwright E2E test simulating the mobile viewport that enters a prompt, waits for generation, and verifies the generated products appear on the storefront. No mocked backend data in the final test.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
