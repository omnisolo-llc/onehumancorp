issue_title: "Implement Zero-Click Agentic Mobile-First Onboarding & Provisioning"
issue_description: |
  ## Title
  Implement Zero-Click Agentic Mobile-First Onboarding & Provisioning

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) face significant setup paralysis when adopting new operational software. Our research shows that 34% of small business owners abandon setup due to "technical complexity" when faced with a blank canvas or multi-step configuration menus (as seen in traditional monoliths like Shopify, Wix, and Squarespace). Users want an AI assistant that *does* the work, rather than an AI chatbot that just *tells* them how to do it. Currently, OHC requires manual setup that can take up to an hour. We must close this gap by guiding the user from an unclear idea to a clear next action, provisioning their entire business environment in less than 10 minutes.

  ## Research Report
  Based on our competitive analysis (`agentic_autonomous_website_builders_smb_platform_gap_analysis.md` and `ohc_owner_work_assistant_competitive_research.md`):
  - **Shopify / Wix / Squarespace**: Highly capable but complex. Setup takes 30-60+ minutes. AI assistants like Shopify Sidekick primarily act as advisors, providing tutorials instead of taking autonomous action. Users experience "App Tax" fatigue.
  - **Durable / 10Web**: AI-native competitors that excel at rapid, low-friction setup, generating functional sites in under a minute via simple prompts. However, they lack deep operational business flows like bookings, inventory tracking, and POS integration.
  - **OHC Gap**: OHC currently lacks the "Zero-to-One" autonomous experience found in AI-native competitors. To differentiate, OHC must combine the immediate setup of Durable with the operational depth of Shopify, wrapped in an assistant-first mobile interface.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner as Owner (Mobile 375px)
      participant UI as OHC Assistant Shell
      participant Orch as KAIROS Orchestration (Hub)
      participant Agent as Onboarding Agent
      participant DB as Postgres (Tenant Scoped)

      Owner->>UI: "I'm a baker in Austin selling custom cakes"
      UI->>Orch: Start Zero-Click Onboarding
      Orch->>Agent: Extract Intent & Generate Payload
      Agent-->>Orch: Schema, Product Catalog, Storefront Design
      Orch->>DB: Provision Tenant, Roles, & Auth
      Orch->>DB: Insert Products, Categories, Initial Action Cards
      Orch-->>UI: Onboarding Complete (Render Assistant Feed)
      UI-->>Owner: Display Action Cards (e.g., "Add Deposit Link")
  ```

  ### UI Wireframes & Screen Flow (375px Target)
  1. **Splash/Input Screen**: A clean, distraction-free screen on mobile. A simple translucent glass input field: "Tell me about your business..."
  2. **Generation Loading State**: Real-time streaming status indicators using Apple/Ubiquiti-style tokens (e.g., "Configuring Stripe...", "Drafting product catalog...").
  3. **Assistant-First Shell (Feed)**: Instead of a complex dashboard, the user lands in their new Command Center. They see their business generated as "Action Cards" (e.g., "Approve custom cake product listing", "Connect bank account for deposits").

  ### Mobile UX Flow
  - Entire interaction occurs within a single chat-like interface.
  - No complex nested navigation menus or tabs.
  - Use of `rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px)` and 16px corner radii on all card elements.
  - Touch targets are strictly >44x44px. Forms utilize native mobile keyboards.

  ### AI Agent Integration Points
  - **Intake Intent Parser**: Uses the `minimax.reason()` fallback structure to parse the initial prompt into a structured `IntakeData` JSON representation (business type, tone, required modules like POS or Bookings).
  - **Departmental Setup Agents**: The KAIROS Hub orchestrates specialized workers (e.g., Inventory Agent, Finance Agent) to populate default seed data relevant to the parsed business type, strictly adhering to row-level tenant isolation via `tenant_id`.

  ### Key Design Decisions
  - **Assistant-First vs Dashboard-First**: Start the user in an action feed, not a blank settings dashboard. This drives immediate momentum and value realization.
  - **Progressive Disclosure**: Keep advanced setup and technical details completely hidden until explicitly needed.
  - **Multi-Tenant State Management**: Ensure all generated data is strictly partitioned via PostgreSQL RLS before presenting to the user.

  ## Implementation Prompt
  **Context**: We are building the "Zero-Click Generation" onboarding flow for OHC, targeting mobile business owners.
  **Task**: Implement a conversational onboarding UI that takes a single user prompt and coordinates with the KAIROS backend to automatically provision the user's business context.
  **CUJ**:
  1. New owner opens the app on their phone (375px width).
  2. Owner types a single sentence describing their business and submits.
  3. The system processes the request via the KAIROS Hub, showing a dynamic loading state.
  4. The system provisions the necessary DB schemas, creates an initial product catalog, and lands the owner on the "Assistant-First Shell" feed containing action cards to approve or edit the generated data.
  **Acceptance Criteria**:
  - The UI must be fully functional on a 375px viewport with no horizontal scrolling.
  - All styling must use the OHC Premium Token library (Translucent Glass materials, 16px border radius, 44x44px min touch targets).
  - The feature must include full Playwright E2E test coverage verifying the path from prompt submission to landing on the populated Assistant Feed without any mock data in the frontend code.
  - The backend endpoints must support this flow dynamically and ensure all records are tied to the newly created `tenant_id`.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
