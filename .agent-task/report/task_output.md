issue_title: "Implement Autonomous Zero-Click Onboarding Agent"
issue_description: |
  # Mission Queue Protocol: Autonomous Zero-Click Onboarding Agent

  ## Title
  Implement Autonomous Zero-Click Onboarding Agent

  ## Problem Statement
  Small business owners like Maya (the home baker) and Carlos (the handyman) experience "setup paralysis" when trying to launch their digital presence. Research shows that 34% of small business owners abandon setup due to technical complexity. They want to focus on their craft, not configure DNS, payment gateways, or layout themes. OHC currently takes ~1 hour of manual setup, whereas AI-native competitors like Durable achieve a 30-second setup. OHC needs an autonomous onboarding agent that guides the user from login to a published product link using only natural language in under 10 minutes.

  ## Research Report
  - **Shopify / Squarespace:** Take days to set up properly, requiring manual configuration of themes, products, and shipping.
  - **Durable:** Generates a complete business website, CRM, and invoicing in under a minute with zero technical hurdles.
  - **OHC Current State:** Has a robust orchestration engine and specialized services but lacks the "Zero-to-One" autonomous experience.
  - **Target Persona:** Maya (Home Baker). She needs a storefront with a photo catalog and deposit-based custom orders without dealing with technical configurations.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      User[User/Owner] -->|Natural Language| UI[Mobile UI - Assistant Feed];
      UI --> Triage[Work Triage Agent];
      Triage --> Onboard[Onboarding Agent];
      Onboard --> API[OHC API Gateway];
      API --> Tenant[Tenant DB Provisioning];
      API --> Stripe[Stripe Checkout/Identity Config];
      API --> Catalog[Catalog Service];
      API --> Storage[GCS / MinIO];
      Onboard --> Config[Configuration Engine];
      Config --> Deploy[Storefront Deployment];
  ```

  ### UI Wireframes & Mobile UX Flow (375px first)
  - **Screen 1 (Welcome & Prompt):** Translucent glass card asking, "What kind of business are you starting today?" with native mobile keyboard.
  - **Screen 2 (Agent Working):** Animated status indicators showing background tasks: "Setting up tenant...", "Configuring Stripe deposits...", "Generating product variants...".
  - **Screen 3 (Refinement):** Assistant presents a draft storefront (photo catalog, pre-configured deposit link). Owner can say "Make it look more premium" or "Add a vegan cake option".
  - **Screen 4 (Success):** A single "Publish & Get Link" button (44x44px touch target) with a clean, Apple/Ubiquiti-style hierarchy.

  ### AI Agent Integration Points
  - **Onboarding Agent (Gemini Pro):** Receives the user's initial description and breaks it down into structured tool calls for the backend APIs.
  - **Memory:** Uses tenant-scoped memory to remember the business type and preferences.
  - **Action Space:** The agent uses tools to call the internal `Tenant Service` (create tenant), `Payment Service` (setup Stripe Checkout for deposits), and `Catalog Service` (create products with AI-generated images).

  ### Key Design Decisions
  - **Assistant-First Shell:** The entire onboarding process occurs through a chat-like feed, hiding all technical forms.
  - **Progressive Disclosure:** Advanced settings (like custom DNS or tax rates) are hidden behind an "Advanced Settings" switch.
  - **Stripe Pre-config:** Deposits and payment links are auto-configured in test mode (or via standard onboarding) to reduce friction.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to implement the "Zero-Click Onboarding Agent" workflow in OHC.
  1. Build a new `OnboardingAgent` in the backend that processes natural language prompts to provision a new tenant, configure basic Stripe deposits, and create a starting product catalog.
  2. Implement the frontend assistant flow (Flutter/Web) starting at 375px viewport. Provide a chat-like interface where the user describes their business, and the agent responds with status updates and a final generated storefront preview.
  3. The Critical User Journey (CUJ) must start from a fresh login, prompt the user for their business idea, and result in a published product link using only natural language.
  4. Use real OHC services (Tenant, Catalog, Payment) and do not use mock data. Write E2E Playwright tests that execute this flow. Ensure the UI adheres to the macOS Translucent Glass and UniFi layout standards.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
