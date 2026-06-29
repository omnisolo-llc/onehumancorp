issue_title: "Implement Zero-Click AI Storefront Generation Flow"
issue_description: |
  # Implement Zero-Click AI Storefront Generation Flow

  ## Problem Statement
  Non-technical small business owners (e.g., Carlos the Handyman, Maya the Baker) often experience "setup paralysis" when confronting the blank canvas of legacy e-commerce platforms like Shopify or Wix. The requirement to manually design a storefront, configure a database schema, write copy, and connect payment gateways prevents them from launching. They abandon the platform because it offers them tools rather than executing the setup on their behalf.

  ## Research Report
  - **Market Context:** Traditional builders (Shopify, Wix) take 30-60 minutes for initial onboarding and demand significant manual configuration. AI-native tools (Durable, 10Web) can generate sites in 30 seconds but often lack deep integration with complex operational backends (e.g., inventory, booking, multi-tenant multi-channel POS).
  - **The Gap:** OHC currently lacks an autonomous onboarding capability that bridges the gap between conversational input and a fully structured, operational multi-tenant backend.
  - **The OHC Opportunity:** Introduce a "Zero-Click Generation" flow where a single natural language prompt ("I am a baker in Austin needing custom order deposits") autonomously provisions the database schemas, populates initial product catalogs, and renders a fully functional, mobile-optimized storefront.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User
      participant OHCMobileWeb as OHC Mobile Web (Flutter)
      participant SetupAgent as Setup Agent (LLM)
      participant CoreAPI as Core API (Rust)
      participant Ledger as Ledger (Postgres)

      User->>OHCMobileWeb: Enters business prompt (e.g., "I'm a baker")
      OHCMobileWeb->>SetupAgent: Send prompt context
      SetupAgent->>CoreAPI: Generate Schema & Catalog Definitions
      CoreAPI->>Ledger: Provision Tenant & Insert Seed Data
      CoreAPI-->>SetupAgent: Setup Complete
      SetupAgent-->>OHCMobileWeb: Return Storefront Metadata
      OHCMobileWeb->>User: Display rendered 375px Storefront
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  1. **Onboarding Screen:** A clean, uncluttered interface with a single text input area taking up 50% of the screen. Soft translucent glass styling.
  2. **Generation State:** A dynamic loading screen where the user sees the AI "working" (e.g., "Designing menu...", "Configuring booking engine...").
  3. **Launch Screen:** The generated storefront appears seamlessly in the 375px viewport with a large sticky action button at the bottom: "Approve & Go Live" or "Regenerate".

  ### AI Agent Integration Points
  - **Setup Agent (The Architect):** Interprets the unstructured prompt, identifies the business category (Service, Digital, Physical), and maps it to a structured OHC template.
  - **Marketing Agent (The Copywriter):** Generates SEO-optimized product descriptions and policies based on the business type.

  ### Key Design Decisions
  - **Zero-Trust & Multi-Tenancy:** The Setup Agent must execute configuration via the Core API with strict tenant isolation, ensuring it only modifies the newly created tenant namespace.
  - **Immediate Value:** Bypassing traditional setup wizards reduces abandonment rates. We prioritize immediate visual feedback over perfect initial data.
  - **Mobile-First Execution:** The entire flow must be designed for and perfectly usable on a 375px touch interface, utilizing native mobile keyboards and large touch targets (44x44px min).

  ## Implementation Prompt
  **Feature Name:** Zero-Click AI Storefront Generation Flow
  **Target Persona:** Carlos the Handyman (wants to accept bookings and deposits but has no website).

  **Outcome:** Carlos downloads the OHC app, types "I'm a handyman in Miami and need to book appointments with a $50 deposit", and within 45 seconds, the app generates a live, bookable storefront.

  **Critical User Journey (CUJ) / Acceptance Criteria:**
  1. User is presented with a conversational input field on first launch.
  2. User submits a short description of their business.
  3. The Setup Agent parses the input and determines the necessary modules (e.g., Bookings, Deposits).
  4. The system provisions the PostgreSQL tenant schema and inserts relevant mock/seed products or services.
  5. A fully functional, mobile-responsive (375px) storefront is presented to the user.
  6. The user can click an "Approve" button, at which point the storefront is finalized.
  7. Include E2E Playwright tests simulating the prompt submission and verifying the generated UI elements.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
