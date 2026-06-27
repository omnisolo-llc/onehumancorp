issue_title: "Zero-Click Generation: Agentic Mobile-First Onboarding Flow"
issue_description: |
  # Research Report: Agentic 10-Minute Mobile Onboarding (Zero-Click Generation)

  ## Problem Statement
  Current onboarding experiences on legacy platforms (Shopify, Wix) suffer from "Setup Paralysis". Non-technical SMB owners (like Carlos the Handyman or Maya the Baker) are greeted with a terrifying blank canvas and complex configuration menus (shipping zones, tax rates, inventory catalogs). They abandon the setup before realizing any value. Existing AI tools are mostly reactive chatbots that offer advice rather than executing state changes. SMBs don't want a tool to build a business; they want an assistant that builds it for them.

  ## Research Report
  - **Competitor Analysis:** Shopify takes 30-60 minutes for a basic setup and requires a desktop for serious configuration. Wix and GoDaddy are simpler but result in static sites without robust operational backends. AI tools like Durable generate sites in 30 seconds but lack deep e-commerce and POS capabilities.
  - **The OHC Differentiator:** OHC's vision is "anyone can launch and run a real small business from their phone or browser in under 10 minutes". We must move from an advisory AI to an executing AI.
  - **Proposed Solution:** A conversational onboarding flow where a single prompt (e.g., "I'm a baker in Austin needing a pre-order site") triggers autonomous agents to generate the entire business scaffold (storefront, database schema, catalog, booking slots, and initial copy).

  ## Design Doc
  ### Architectural Design
  - **Core Component:** `OnboardingOrchestrator` service.
  - **Data Model:** A flexible tenant schema where the initial seed generates `TenantConfig`, `ProductCatalog`, and `AvailabilitySchedule` based on the LLM's classification of the business type.
  - **Agent Coordination:**
    - **Intake Agent:** Parses the user's natural language input to extract business type, locale, and tone.
    - **Operations Agent:** Generates the underlying data structures (e.g., service types vs physical goods).
    - **Marketing Agent:** Drafts SEO-optimized copy, hero images (via image generation), and local SEO metadata.
  - **State Management:** The generation process uses server-sent events (SSE) or WebSockets to stream progress to the mobile client in real-time.

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant User (Mobile 375px)
      participant OnboardingOrchestrator
      participant IntakeAgent
      participant OperationsAgent
      participant MarketingAgent
      participant PostgreSQL

      User (Mobile 375px)->>OnboardingOrchestrator: "I fix ACs in Miami"
      OnboardingOrchestrator->>IntakeAgent: Extract intent & domain
      IntakeAgent-->>OnboardingOrchestrator: Domain: Service, Location: Miami
      OnboardingOrchestrator->>OperationsAgent: Generate service catalog & booking schema
      OperationsAgent->>PostgreSQL: Seed initial services (AC Repair, Maintenance)
      OnboardingOrchestrator->>MarketingAgent: Generate site copy & assets
      MarketingAgent->>PostgreSQL: Save storefront configuration
      OnboardingOrchestrator-->>User (Mobile 375px): Stream progress updates
      OnboardingOrchestrator-->>User (Mobile 375px): Onboarding Complete - Present Dashboard
  ```

  ### Mobile UX Flow (375px First)
  1. **Splash Screen:** Single input field: "Tell us about your business." with voice-to-text option.
  2. **Generation Screen:** A visually premium (macOS Translucent Glass) progress screen showing steps: "Structuring database...", "Writing copy...", "Preparing storefront...".
  3. **Reveal:** The fully generated dashboard tailored to their business type (e.g., Carlos sees an upcoming bookings calendar; Maya sees an inventory list).
  4. **Refinement:** 1-tap buttons to "Regenerate Design" or "Launch".

  ### AI Agent Integration Notes
  - Must use strict structured output (JSON schema) from the LLM to ensure the generated catalog and schema can be safely injected into the PostgreSQL database.
  - Rely on SPIFFE/SPIRE for agent identity when making cross-service calls during generation.

  ## Implementation Prompt (For Implementer Agent)
  **Feature Name:** Zero-Click Agentic Onboarding Flow
  **Target Persona:** Carlos the Handyman (Android phone, no technical skills, relies on word of mouth).
  **Outcome:** Carlos downloads OHC, types "I do home repairs in Austin", and within 60 seconds has a fully functional booking site and service catalog ready to accept jobs.
  **Critical User Journey (CUJ):**
  1. User opens the OHC mobile app (375px).
  2. Enters a single sentence describing their business.
  3. UI displays a real-time progress indicator as agents scaffold the backend.
  4. User is presented with a fully populated storefront and dashboard.
  **Acceptance Criteria:**
  - Build the `OnboardingOrchestrator` backend service that coordinates the agents.
  - Implement the Flutter mobile UI (375px optimized) with translucent glass styling.
  - Zero mock data; the generated catalog must be persisted to the real PostgreSQL database.
  - Include full Playwright E2E tests verifying the flow from prompt to generated storefront.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []