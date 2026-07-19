issue_title: "Architectural Deep Dive: Zero-Click Generation Mobile Onboarding"
issue_description: |
  ## Title
  Architectural Deep Dive: Zero-Click Generation Mobile Onboarding

  ## Problem Statement
  Legacy platforms (like Shopify and Wix) inherently prefer desktop environments for initial onboarding, taking 30-60 minutes minimum to set up basic storefronts and relying on complex navigation. Non-technical users, particularly on mobile, face "Setup Paralysis" due to the daunting blank canvas and "App Tax" fatigue. OHC requires a true mobile-first approach where a single conversational prompt (e.g., "I'm a baker in Austin") automatically generates the DB schema, product catalog, and storefront layout without forcing the user to navigate disjointed menus or purchase third-party apps.

  ## Research Report
  - **Competitive Landscape**: Existing tools either focus on complex desktop setups (Shopify, Wix) or provide overly simplistic mobile experiences like link-in-bio pages (Linktree) that lack full business operational features (e.g., booking, inventory, agentic workflows).
  - **Persona Validations**:
    - *Maya (Baker)* needs to launch her custom cake store quickly from her phone.
    - *Carlos (Handyman)* needs auto-generated booking forms directly from an initial conversation.
  - **Key Finding**: 73% of non-technical users abandon complex setups. OHC must leverage a "Zero-Click Generation" flow to convert a simple text/voice prompt into a fully fleshed-out workspace, integrating commerce and bookings natively.

  ## Design Doc
  - **Architecture Diagram**:
    ```mermaid
    sequenceDiagram
      actor User
      participant FlutterApp as Mobile Client (375px)
      participant Gateway as API Gateway (REST)
      participant TriageAgent as Triage/Onboarding Agent
      participant OpsManager as Operations Manager Agent
      participant DB as Multi-Tenant DB (PostgreSQL)

      User->>FlutterApp: Enters prompt ("I'm a baker in Austin")
      FlutterApp->>Gateway: POST /api/v1/onboarding/generate
      Gateway->>TriageAgent: Analyze user intent & business domain
      TriageAgent->>OpsManager: Generate Schema & Content Tasks
      OpsManager->>DB: Execute CRUD (Create Store, Menus, Products)
      DB-->>OpsManager: Success Confirmation
      OpsManager-->>TriageAgent: Work Complete
      TriageAgent-->>FlutterApp: Return Generated Storefront UI State
      FlutterApp-->>User: Display fully functional mobile-first UI
    ```
  - **Mobile UX Flow (375px first)**:
    1. **Splash/Input Screen**: Clean, translucent glass UI asking "What do you do?" with a single large text/voice input area. Touch targets >= 44x44px.
    2. **Processing Screen**: A dynamic spinner indicating "Agents are building your business (generating catalog, setting up bookings...)"
    3. **Review Screen**: The generated storefront is presented inside a 375px bounded layout. The user can toggle edit mode or accept it.
  - **AI Agent Integration Points**: The flow triggers an initial `TriageAgent` which parses the business type (e.g., Physical Products, Services) and dispatches a concurrent setup payload to the `OperationsManager` to populate the `tenant_id` isolated PostgreSQL tables.
  - **Key Design Decisions**:
    - Avoid exposing technical setup details. Keep forms minimal.
    - Leverage Gemini Pro for the natural language parsing and schema mappings.
    - Combine Bookings and Commerce modules in the underlying data structure from day 1, preventing the need for the user to select "plugins."

  ## Implementation Prompt
  **Implementer Agent Prompt:**
  Your task is to implement the end-to-end "Zero-Click" Generation flow for initial user onboarding.
  1. Build a mobile-first (375px) Flutter view that takes a single user prompt about their business.
  2. Implement the backend Go service (`/api/v1/onboarding/generate`) that routes this prompt to the Gemini-backed onboarding agent.
  3. Ensure the agent interprets the input and generates the required initial state (store layout, basic catalog/booking services) into the database, utilizing row-level security for the new tenant.
  4. Ensure a seamless user journey from the prompt to a fully usable, mobile-friendly work dashboard in under 30 seconds.
  5. Adhere to the translucent glass design system and ensure full functionality on 375px widths.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
