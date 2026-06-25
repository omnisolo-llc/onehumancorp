issue_title: "Implement Autonomous Zero-Click Agentic Onboarding Engine"
issue_description: |
  # Architecture Design: Autonomous Zero-Click Agentic Onboarding Engine

  ## Problem Statement
  Currently, onboarding SMBs (like Maya the Baker or Carlos the Handyman) into a unified platform is tedious. They face a "blank canvas" problem, requiring them to manually configure their storefront, services, products, bookings, and policies across complex settings screens. This manual setup paralyzes non-technical users, leading to high abandonment rates (73% of non-technical users abandon complex setups based on our gap analysis). Shopify, Wix, and Squarespace require 30-60 minutes of configuration and third-party app curation. The current OneHumanCorp (OHC) setup lacks a zero-click, agent-driven approach to completely automate business generation from a single natural language input.

  ## Research Report
  Based on our analysis in `agentic_autonomous_website_builders_smb_platform_gap_analysis.md`, the AI website builder market is fragmented:
  1. Traditional monoliths (Shopify, Wix) offer powerful capabilities but are too complex and desktop-centric. Their AI offerings are mostly advisory "chatbots" that guide users rather than execute tasks.
  2. "AI Toy Builders" (Durable, Mixo) can generate basic websites in 30 seconds but lack the depth for real business operations (integrated payments, unified inventory, bookings).

  **The OHC Opportunity:** We need to position OHC in the top-right quadrant: "Simple / Mobile-First" AND "Autonomous Execution". We must implement a "Zero-Click Generation" flow where a single prompt (e.g., "I am Maya, a custom cake baker in Austin") triggers "Departmental AI Workers" to autonomously provision the database schema, product catalog, visual storefront, and booking rules without any manual clicks.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner as Maya (Owner)
      participant MobileApp as OHC Mobile App (Flutter)
      participant Gateway as API Gateway / Auth
      participant DirectorAgent as Director Agent
      participant DeptAgents as Departmental Agents (Ops, Sales, Web)
      participant DB as Central Ledger (Postgres)

      Owner->>MobileApp: "I make custom cakes in Austin."
      MobileApp->>Gateway: POST /api/v1/onboarding/generate { prompt }
      Gateway->>DirectorAgent: Trigger Zero-Click Pipeline

      par Parallel Generation
          DirectorAgent->>DeptAgents: Generate Catalog & Pricing (Sales)
          DirectorAgent->>DeptAgents: Generate Booking Rules (Ops)
          DirectorAgent->>DeptAgents: Generate Visual Theme & UI (Web)
      end

      DeptAgents->>DB: Execute DML & Provision Tenant Data
      DeptAgents-->>DirectorAgent: Acknowledge Completion
      DirectorAgent-->>MobileApp: Return Generated Workspace Context
      MobileApp->>Owner: Display Fully Functional Storefront & Dashboard
  ```

  ### Mobile UX Flow (375px First)
  1. **The Entry:** A single, immersive full-screen translucent glass card on a 375px viewport with a large text input and voice dictation button: "Tell us what you do, and we'll build your business engine."
  2. **The Generation State:** As the agentic engine runs, the user sees a sleek loading state with real-time text updates detailing the agent's actions (e.g., "Designing storefront...", "Adding 5 custom cake templates...", "Setting up payment links...").
  3. **The Reveal:** A clean, UniFi-style modular dashboard appears, pre-populated with realistic mock products (e.g., "Vegan Chocolate Cake - $45") and a functional "Share Store Link" button. Zero blank slates.

  ### AI Agent Integration Points
  - **Director Agent:** Acts as the orchestrator. It receives the natural language prompt, analyzes the business type (e.g., Physical Product, Service, Booking), and delegates tasks to specialized sub-agents.
  - **Sales Agent:** Responsible for generating a structured product catalog, pricing tiers, and descriptions.
  - **Operations Agent:** Configures the scheduling rules, delivery radii (for the baker), or working hours.
  - **Web Agent:** Generates the design tokens (colors, typography) and the initial landing page layout structure.

  ### Key Design Decisions
  - **Single Input Modality:** To eliminate setup paralysis, we strictly require only ONE input from the user. All other configuration is inferred and can be edited later.
  - **Synchronous Illusion / Asynchronous Execution:** The API call will kick off an asynchronous pipeline but provide a websocket or polling endpoint for the UI to show real-time progress, ensuring the user stays engaged.
  - **Pre-populated Data:** The system must generate tangible, usable data (not generic 'Lorem Ipsum') so the owner immediately understands how the platform works.

  ## Implementation Prompt
  **Target Persona:** Maya the Baker
  **CUJ:** Maya downloads the OHC app on her iPhone. She enters "I'm a baker specializing in custom vegan cakes in Austin." Within 45 seconds, she is presented with a fully functional dashboard containing a generated product catalog of 3 cake types, a booking calendar for consultations, and a ready-to-share storefront link.

  **Instructions for Implementer Agent:**
  1. Create the backend gRPC/REST endpoint `/api/v1/onboarding/generate` that accepts a single text prompt.
  2. Implement the `DirectorAgent` coordination logic to parse the prompt, determine the business category, and orchestrate the parallel generation of tenant data (products, services, theme).
  3. Ensure the generated data is persisted directly to the PostgreSQL Central Ledger under the new `tenant_id`.
  4. Develop the Flutter mobile-first (375px) UI for the entry screen, loading state, and the finalized dashboard reveal, adhering to the macOS translucent glass and UniFi modular design system.
  5. **Acceptance Criteria:** A user can input a single sentence and receive a fully provisioned workspace with no manual configuration steps required. Ensure 100% test coverage and E2E Playwright validation of this flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, zero-click-onboarding]
assignees: []