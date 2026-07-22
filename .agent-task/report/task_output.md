issue_title: "Zero-Click Agentic Storefront Generation (Architecture & Design)"
issue_description: |
  # Zero-Click Agentic Storefront Generation

  ## Problem Statement
  Current e-commerce and booking platforms (Shopify, Wix, Squarespace) require substantial manual configuration. Non-technical users, like Maya (baker) or Carlos (handyman), experience "Setup Paralysis" when faced with a blank canvas, complex menus, app installations, and theme configuration. Traditional platforms can take hours or days to configure properly. The gap is clear: SMB owners need an AI that executes and creates, not just an AI that advises. OHC must provide a "Zero-Click Generation" flow where a single conversational prompt autonomously provisions the database schema, product catalog, pricing, and storefront layout.

  ## Research Report
  Based on competitive analysis of Shopify Sidekick, Wix Studio, and modern AI builders (Durable, Framer AI, Hocoos):
  1. **The "App Tax" Fatigue**: Traditional platforms force users to patch together bookings, reviews, and commerce via third-party apps, adding cost and complexity.
  2. **Advisory vs. Executory AI**: Shopify Sidekick acts as a manual/advisor. Users want an executory agent that takes instructions (e.g., "Set up a custom cake store with deposit payments") and builds the state directly.
  3. **Generative Onboarding Gap**: 73% of non-technical users abandon complex setups. AI builders like Durable show the appeal of <30s generation, but often lack the deep operational backend (inventory, bookings, payments) that real businesses need. OHC can bridge this by generating both the beautiful frontend and the functional multi-tenant backend schema.

  ## Design Doc

  ### Architectural Diagram
  ```mermaid
  sequenceDiagram
      actor Owner
      participant MobileUI as OHC Mobile UI (Flutter)
      participant OnboardingAgent as Agent: Onboarding / Genesis
      participant OpsAgent as Agent: Operations
      participant MultiTenantDB as OHC Database (Postgres)
      participant AssetGen as Asset Generator

      Owner->>MobileUI: Prompt: "I'm a baker in Austin, I sell custom vegan cakes"
      MobileUI->>OnboardingAgent: Submit Genesis Prompt
      OnboardingAgent->>OpsAgent: Request Schema & Catalog Definition
      OpsAgent->>MultiTenantDB: Execute CRUD (Create Products, Categories, Booking Slots)
      OnboardingAgent->>AssetGen: Request Layout & Hero Images
      AssetGen-->>OnboardingAgent: Assets & Layout Config
      OnboardingAgent->>MultiTenantDB: Save Storefront Configuration
      OnboardingAgent-->>MobileUI: Return Generated Storefront Link
      MobileUI-->>Owner: Display Live Preview (375px first)
  ```

  ### Data Model & Invariants
  *   **`StorefrontConfig`**: Holds layout metadata, theme variables (OHC Premium Token library), and localized copy.
  *   **`ProductCatalog`**: Automatically populated with variants, pricing tiers, and deposit requirements.
  *   **Multi-tenant Isolation**: All generated entities must strictly map to `tenant_id` and utilize PostgreSQL Row Level Security (RLS).

  ### Mobile UX Flow (375px)
  1.  **Genesis Screen**: A simple, translucent chat interface asking "What do you do?". Native keyboard integration.
  2.  **Loading/Generation State**: Animated shimmering cards detailing background tasks ("Creating your catalog...", "Setting up booking calendar...").
  3.  **Preview Screen**: Fully functional, zero-scroll-required mobile preview of the storefront with real interactive buttons (e.g., a "Book Now" CTA).
  4.  **Acceptance**: A single "Publish" or "Tweak" button.

  ### AI Agent Integration
  *   **Genesis Agent (Gemini Pro/GPT-4o)**: Acts as the orchestrator. Takes the raw prompt and parses it into structured domain requirements (products vs. services).
  *   **Operations Agent (Worker)**: Executes database transactions securely using the repository patterns (ProductRepo, BookingRepo) to provision the backend state.
  *   **Asset/Content Agent**: Generates placeholder copy, localized strings, and visual configuration.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the Zero-Click Agentic Storefront Generation flow from the mobile frontend to the backend agent orchestrator.
  **CUJ (Critical User Journey)**:
  1. As a new user on the mobile app, I enter a single sentence describing my business.
  2. I wait a few moments while the system generates my setup.
  3. I am presented with a fully configured store containing sample products, a layout, and ready-to-use booking/payment options.
  **Acceptance Criteria**:
  - The UI must be implemented in Flutter targeting a 375px mobile breakpoint first, applying the OHC translucent glass design system.
  - The backend must expose a new gRPC/REST endpoint for the `Genesis` request.
  - The AI generation must insert real records into the database (Products, Settings) scoped to the user's `tenant_id`.
  - Zero mock data in the final UI; the generated data must be persisted in Postgres.
  - E2E Playwright tests must be added verifying the end-to-end flow from prompt submission to storefront rendering.

  ## Scope & Priority
  **Estimated Scope**: Large
  **Priority**: P0 (Critical path for user acquisition and activation)
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []