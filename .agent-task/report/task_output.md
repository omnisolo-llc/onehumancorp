issue_title: "Architecture: Zero-Click Intelligent Mobile Onboarding & Agentic Generation"
issue_description: |
  # Research Report: Zero-Click Intelligent Mobile Onboarding & Agentic Generation

  ## Problem Statement
  Small business owners and non-technical operators (e.g., Maya the baker, Carlos the handyman) face immense friction during the initial setup of traditional platforms like Shopify or Wix. The "blank canvas" problem is terrifying; onboarding takes 30-60 minutes, involves navigating complex menus (often desktop-only), configuring databases/catalogs manually, and installing multiple third-party apps just to start accepting bookings or selling items. This complexity causes a massive drop-off rate for solopreneurs who expect an assistant-like experience right from their 375px mobile screen. They need an application that transitions from an "unclear idea" to a "ready-to-operate digital business" with practically zero clicks.

  ## Research Report & Gap Analysis
  - **Shopify / Wix / Squarespace:** All rely on manual configuration. Shopify's onboarding is desktop-first, highly complex, and expects users to construct their store catalog from scratch. AI tools like "Sidekick" are strictly conversational advisors rather than executors; they tell the user *how* to set up products instead of *doing it* autonomously.
  - **AI Toy Builders (Durable, Mixo):** These generate landing pages fast but lack deep backend integration (like POS, multi-tenant database isolation, inventory sync, or Stripe integration). They are purely visual.
  - **OHC Architecture Gap:** OHC currently lacks a "Zero-Click" onboarding pipeline where a single natural language input ("I'm a baker in Austin selling custom vegan cakes") directly provisions a fully functional multi-tenant workspace, generates the database schema (products, prices, booking availability), and drafts a mobile-first storefront in under 30 seconds.

  ## Design Doc
  ### Architectural Overview
  The onboarding process must shift from manual CRUD operations to an **Agent-Driven State Generation** model.

  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  graph TD
      A[Mobile App - 375px] -->|Single Prompt String| B(Gateway / Auth Layer)
      B --> C[Workspace Orchestrator]
      C --> D[Provisioning Agent]

      D -->|Multi-Tenant Context| E[Database Provisioning]
      D -->|Catalog Draft| F[Operations Agent]
      D -->|Site Generation| G[Marketing Agent]

      E --> H[(PostgreSQL Central Ledger)]
      F --> H
      G --> H

      H --> I[Unified Memory & Event Mesh]
      I --> J[Owner Dashboard - Live Preview]
  ```

  ### Mobile UX Flow
  1. **Step 1: The One-Question Setup (375px)**
     - Screen displays a simple, native text input: "What do you do?"
     - User speaks or types: "I repair appliances in Miami."
  2. **Step 2: Translucent Loading & Agent Execution**
     - A macOS-style translucent glass loading screen appears showing live steps: "Creating workspace...", "Drafting service catalog...", "Setting up booking calendar...".
  3. **Step 3: The 'Ready' Dashboard**
     - User lands on the Assistant-First Shell. It displays their generated catalog (e.g., "Fridge Repair - $80/hr"), an empty but configured booking calendar, and an AI-drafted welcome post for Instagram.

  ### AI Agent Integration Points
  - **Provisioning Agent (The Builder):** Parses the initial prompt to deduce the business type (service vs. product). Generates the `Tenant` record in PostgreSQL.
  - **Operations Agent (The Manager):** Creates default `Service` or `Product` entities tailored to the prompt (e.g., automatically adding "Vegan Chocolate Cake" for Maya).
  - **Security & Multi-Tenancy:** The Provisioning Agent uses strict tenant-scoped DB transactions ensuring row-level security (`tenant_id` filtering) is enforced from the first generated row.

  ### Key Design Decisions
  - **Zero-Click Execution:** Eliminate all standard onboarding forms (company name, address, industry drop-downs). The LLM extracts this from context.
  - **Immediate Usability over Perfection:** The generated store does not need to be final; it needs to be 90% there so the owner can easily edit ("Action Required: Approve Catalog") rather than create from scratch.
  - **Mobile-First Exclusivity:** The entire onboarding flow must be comfortable on a mobile device without requiring a keyboard for more than one sentence.

  ## Implementation Prompt
  **User-Facing Outcome:** As a new business owner, I open the OHC app, enter a single sentence describing my business, and within 30 seconds I am dropped into a fully functioning dashboard with my services, pricing, and booking calendar already generated and ready for approval.

  **CUJ & Acceptance Criteria:**
  1. Develop the "Zero-Click Onboarding" mobile UI (375px target) accepting a single text prompt.
  2. Implement the `Provisioning Agent` service in Rust/Go that receives the prompt and calls the LLM (Gemini Pro/MiniMax) to extract business profile data.
  3. The agent must orchestrate database transactions to create a new Tenant, default Catalog (Products/Services), and basic Settings.
  4. Ensure row-level multi-tenant security is strictly enforced during generation.
  5. **Automated Verification:** Write Playwright E2E tests where a simulated new user enters a prompt like "I am a local plumber," waits for generation, and verifies the dashboard displays auto-generated plumbing services without any manual catalog entry.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
