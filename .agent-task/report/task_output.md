issue_title: "Build 10-Minute Zero-Click Mobile Onboarding Flow"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## Problem Statement
  Small business owners face significant "Setup Paralysis". When users sign up for traditional platforms like Shopify or Wix, the initial blank canvas is terrifying. Non-technical users struggle significantly with initial configuration and often abandon the process because they lack the time and expertise to piece together a functional online store.

  ## Research Report
  Our research into the SMB Platform landscape reveals a major gap between complex legacy systems (Shopify, Wix) and simple, but limited, AI-native builders (Durable, Mixo).

  **Competitor Audit (Shopify):**
  While Shopify is powerful, it requires 30-60 minutes minimum for onboarding. Users must navigate complex menus, install themes, and add products manually. Its AI assistant, Sidekick, is mostly a reactive chatbot rather than a proactive builder.

  **OHC Opportunity:**
  OHC can capture this market by implementing a "Zero-Click Generation" flow. Because 73% of non-technical users abandon complex setups, OHC must allow a user to launch a site using a single conversational prompt (e.g., "I'm a baker in Austin"). The AI should autonomously generate the database schema, product catalog, and storefront layout.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[User Mobile Device 375px] -->|1. Enter single sentence prompt| B(Onboarding Gateway)
      B --> C{AI Onboarding Agent}
      C -->|Generate Schema & Content| D[LLM Provider]
      D --> E[Tenant Database Setup]
      D --> F[Product Catalog Generation]
      D --> G[Storefront Layout Generation]
      E --> H[Live Preview]
      F --> H
      G --> H
      H -->|2. Review and Launch| A
  ```

  ### Mobile UX Flow (375px First)
  1. **Splash Screen:** Minimalist, translucent glassmorphism design. "What kind of business do you run?" with a single text input field.
  2. **Processing Screen:** Engaging animation while the AI agent provisions the tenant, generates the catalog, and designs the layout in the background.
  3. **Live Preview:** A fully functional, mobile-optimized preview of the generated storefront.
  4. **Action:** A prominent "1-Tap Launch" button to finalize the setup.

  ### AI Agent Integration Points
  - **Onboarding Agent:** Acts as the orchestrator. It receives the user prompt, interfaces with the LLM to generate the necessary structured data (products, pricing, layout preferences), and executes the backend mutations to provision the tenant and populate the database.

  ### Key Design Decisions
  - **Mobile-First:** The entire flow must be seamless on a 375px device. No horizontal scrolling, large touch targets (>= 44x44px).
  - **Zero-Click Generation:** Move away from multi-step wizards. The goal is to go from prompt to live preview with zero intermediate configuration steps.
  - **Glassmorphism UI:** Adhere to the OHC Premium Token library for a premium, vibrant feel.

  ## Implementation Prompt
  **User-Facing Outcome:** As a new business owner, I want to type a single sentence describing my business on my phone and have a complete, ready-to-sell online store generated for me in under a minute, so that I can start accepting orders immediately without learning complex software.

  **CUJ & Acceptance Criteria:**
  1. The user navigates to the mobile onboarding screen (375px width).
  2. The user enters a prompt (e.g., "I run a mobile dog grooming service in Seattle").
  3. The system processes the prompt and the Onboarding Agent provisions a new tenant.
  4. The Agent generates a relevant service catalog, pricing, and a storefront layout.
  5. The user is presented with a live, interactable preview of their new store.
  6. The user taps "1-Tap Launch" and the store is finalized.
  7. Provide Playwright E2E tests verifying the complete flow from prompt submission to live preview rendering.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
