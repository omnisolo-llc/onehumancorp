issue_title: "Implement AI-Native Zero-Click Mobile Onboarding Flow"
issue_description: |
  # Research Report: AI-Native Zero-Click Mobile Onboarding Flow

  ## Problem Statement
  Currently, SMB platforms (like Shopify, Wix) require users to navigate complex, desktop-first configuration flows that cause significant drop-off among non-technical users. For owner/operators like Maya (a baker running her business on an iPhone) or Carlos (a handyman operating via Android), the standard "blank canvas" onboarding process is paralyzing. They do not have 30-60 minutes to select themes, configure schema, and write initial product copy. The product gap is clear: OHC requires a mobile-first onboarding flow that transforms a simple natural language prompt into a fully initialized platform.

  ## Research Report & Gap Analysis
  - **Competitor Setup Paralysis**: 73% of non-technical users abandon complex setups. Platforms like Shopify present a vast array of menus and settings before the user sees value, while AI-assisted tools like Sidekick act only as advisors, not executors.
  - **The OHC Advantage**: Instead of piecing together discrete tools and apps, OHC’s "Zero-Click Generation" will use an AI agent to build the initial schema, product/service catalog, and storefront layout in one unified step.
  - **Persona Evidence**: Maya needs an integrated booking and custom-order deposit system without the "App Tax" of third-party plugins. Carlos needs a service listing with a booking calendar and deposit payments immediately ready to go. The flow must accommodate both physical products and service-based business structures.

  ## Design Doc

  ### Architecture Diagram (Mermaid)
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile Owner (Maya/Carlos)
      participant UI as OHC Flutter Mobile UI (375px)
      participant Backend as OHC Core Backend
      participant Agent as Builder Agent (LLM)

      Owner->>UI: Enters business description (e.g., "I'm a baker in Austin")
      UI->>Backend: Submit natural language prompt
      Backend->>Agent: Request schema & catalog generation
      Agent-->>Backend: Return initialized DB schema, catalog items, & layout configuration
      Backend->>Backend: Provision Multi-Tenant Database Schema & Configurations
      Backend-->>UI: Return ready-to-launch workspace state
      UI-->>Owner: Display personalized, fully-functioning dashboard (Zero-Click setup complete)
  ```

  ### UI / Mobile UX Flow (375px Baseline)
  1. **Greeting Screen**: A clean, macOS-style Translucent Glass greeting. "What kind of business are you building?"
  2. **Prompt Input**: A single, prominent text area utilizing the native mobile keyboard. Example hint: "I teach guitar locally," or "I sell custom cakes."
  3. **Agent Action State**: A polished, UniFi-style modular loading screen illustrating the AI agent actively constructing the workspace (creating catalog, setting up bookings, writing copy).
  4. **Workspace Reveal**: The owner lands directly on the OHC Assistant Feed and Storefront preview, fully populated with relevant sample products/services and actionable tasks.

  ### AI Agent Integration Points
  - **Builder Agent (Operations Manager Protocol)**: Acts upon the natural language prompt to define data schemas (e.g., categorizing the business as Service vs. Physical Product).
  - **Copywriting Agent**: Generates relevant descriptions, FAQs, and policy placeholders tailored to the specific business vertical.

  ### Key Design Decisions
  - **Zero-Trust & Multi-Tenancy**: The Builder Agent must execute strictly within a new, isolated tenant namespace, ensuring row-level security is enforced upon creation.
  - **Mobile-First Exclusivity**: The entire onboarding journey must be effortlessly completable on a 375px mobile viewport without horizontal scrolling or tiny touch targets.

  ## Estimated Scope
  Large

  ## Implementation Prompt
  Implement the AI-Native Zero-Click Mobile Onboarding feature for the Flutter frontend and Go + Bazel backend.

  **User Journey:** The user (e.g., Maya the Baker) opens the app on her phone, types "I make custom vegan cakes in Austin," and within seconds, her OHC workspace is generated with a product catalog, booking calendar, and AI-drafted store copy.

  **Acceptance Criteria:**
  1. Create a Flutter UI sequence starting at a 375px width that captures a single text prompt.
  2. Implement a backend service endpoint that orchestrates the Builder Agent and Copywriting Agent to process this prompt.
  3. The backend must autonomously provision the correct tenant DB entities (e.g., products vs. services) and initial content based on the agent's output.
  4. Ensure a smooth transition in the UI from the loading/generation state to the fully populated owner dashboard.
  5. The implementation must include full Playwright E2E tests validating the complete journey from prompt entry to dashboard render.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
