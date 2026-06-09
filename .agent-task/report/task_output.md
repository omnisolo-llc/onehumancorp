issue_title: "Implement Mobile-First Zero-Click Autonomous Onboarding & Store Generation"
issue_description: |
  ## Title
  Implement Mobile-First Zero-Click Autonomous Onboarding & Store Generation

  ## Problem Statement
  Small business owners and operators like Maya (the baker) and Carlos (the handyman) face severe "setup paralysis." When they try to move their operations online, they are greeted by traditional builders (like Shopify, Wix, or Squarespace) with overwhelming blank canvases, hundreds of configuration menus, and fragmented app ecosystems. The onboarding process takes hours of piecing together tools and themes. From a non-technical owner's perspective, this initial barrier prevents them from using the platform because they just want to input their business idea and have a working storefront and booking system ready to go. They want AI that *executes* rather than just *advises*.

  ## Research Report
  Our competitive gap analysis and deep-dive into the market ("Agentic Autonomous Website Builders & SMB Platform Gap Analysis") reveals the following:
  *   **Shopify / Wix / Squarespace:** Highly capable but very complex. Shopify's Sidekick advises but doesn't autonomously execute end-to-end storefront generation. Onboarding takes 30-60 minutes at a minimum and requires installing multiple paid plugins.
  *   **GoDaddy / Weebly:** Simpler but rigid, lacking the deeply integrated AI scheduling and booking components.
  *   **AI-Native Competitors (Durable, 10Web, Framer AI):** Fast generation (Durable generates in 30 seconds) but often superficial. They produce static landing pages without the underlying commerce, catalog, and service booking logic needed by service operators.
  *   **The OHC Opportunity:** 73% of non-technical users abandon complex setups. OHC must capture this audience by providing a true "Zero-Click Generation" flow where a single prompt ("I'm a baker in Austin needing deposits and cake variant choices") generates the fully functional catalog, storefront UI, and booking calendar simultaneously.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      autonumber
      actor Owner as Maya / Carlos (Mobile UI)
      participant MobileApp as OHC Flutter App
      participant OnboardingAgent as Onboarding Agent
      participant OpsAgent as Operations Agent
      participant OHCBackend as OHC Backend (Postgres)

      Owner->>MobileApp: Enters Single Prompt (e.g., "I'm a plumber in Miami")
      MobileApp->>OnboardingAgent: Generate Business Profile & Context
      OnboardingAgent->>OpsAgent: Request Default Service Catalog & Schema
      OpsAgent->>OHCBackend: Autonomously Provision DB Schema, Products, Pricing
      OpsAgent-->>OnboardingAgent: Confirm Provisioning Success
      OnboardingAgent-->>MobileApp: Return Storefront UI Config & Catalog
      MobileApp-->>Owner: Display Functional Mobile Storefront (Ready for Review)
  ```

  ### UI Wireframes & Screen Flow (375px first)
  1.  **Screen 1: The Prompt:** A clean, edge-to-edge macOS translucent glass interface. A single prominent text field "What do you do?" with a microphone icon for voice input. A pulsating "Generate My Business" button.
  2.  **Screen 2: Autonomous Generation State:** An animated, dynamic loading state. "Agents are building your service menu...", "Designing your storefront...", "Configuring booking deposits...". No complex spinners—just plain-language progress.
  3.  **Screen 3: The Generated Result:** The fully functional app shell. The owner sees their generated profile, 3 default services/products auto-filled with stock imagery and pricing, and a prominent "Publish & Share Link" button.

  ### Mobile UX Flow
  *   **Input:** Single text/voice prompt on a 375px screen. Native keyboard pops up automatically.
  *   **Transition:** Seamless handover to the agent generation layer without any further owner intervention.
  *   **Output:** The user lands directly into the "Operations Assistant" view, fully populated with their new business context, ready to accept customers.
  *   **Validation:** Touch targets > 44px, no horizontal scrolling.

  ### AI Agent Integration Points
  *   **Onboarding Agent (Entry):** Parses the initial prompt, identifies the business vertical, and determines the required capabilities (Products vs. Bookings).
  *   **Operations Agent (Execution):** Creates the underlying default records (e.g., standard pricing, hours of operation, booking policies) in the database via the MCP tool gateway.
  *   **Visualizer Agent (Presentation):** Selects appropriate UI layout tokens, color schemes, and stock imagery tailored to the vertical.

  ### Key Design Decisions and Why
  *   **Single-Prompt Input:** Replaces standard multi-step forms (Name, Category, Address, Products) to eliminate setup fatigue.
  *   **Autonomous CRUD Execution:** Agents actively provision the backend (PostgreSQL) instead of just generating a static design mock. This ensures the app is functional instantly.
  *   **Mobile-First Exclusivity for Onboarding:** Forces the engineering team to keep the onboarding process extremely lightweight. If it requires desktop, it fails the "Zero-Click" mandate.

  ## Implementation Prompt
  **Goal:** Implement the "Zero-Click Mobile Onboarding" Critical User Journey (CUJ).

  **User Persona:** Carlos, a non-technical handyman using a low-end Android phone.

  **Expected Outcome:** When a new user logs in, instead of seeing a blank dashboard or a multi-step form, they are presented with a single, full-screen conversational prompt (e.g., "Describe your business"). Submitting this prompt must trigger backend AI agents to autonomously generate and persist a fully populated storefront, including a default catalog of services/products, pricing, and business settings. The UI must then immediately reflect this generated state on a 375px mobile layout.

  **Acceptance Criteria:**
  1.  **UI/UX:** Build a premium, translucent-glass styled input screen for the initial prompt, fully responsive at 375px without horizontal scrolling.
  2.  **Agent Trigger:** Submitting the prompt triggers a backend agent workflow that successfully parses the natural language input.
  3.  **Autonomous Execution:** The agents autonomously create necessary backend entities (business profile, default products/services) without further user input.
  4.  **Instant Reflection:** The mobile app UI transitions smoothly from the prompt screen to a fully populated, functional dashboard showing the generated items.
  5.  **Test Coverage:** Provide complete Playwright E2E tests validating this exact flow: logging in, submitting the single prompt, and verifying the resulting populated dashboard elements exist in the DOM.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
