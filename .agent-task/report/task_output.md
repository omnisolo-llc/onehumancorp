issue_title: "Zero-Click AI Storefront Generation & Mobile Onboarding Flow"
issue_description: |
  ## Issue Brief

  ### Title
  Zero-Click AI Storefront Generation & Mobile Onboarding Flow

  ### Problem Statement
  Small business owners and operators (like Maya the Baker or Carlos the Handyman) experience extreme "Setup Paralysis" when trying to transition their business online using legacy platforms like Shopify, Wix, or Squarespace. These platforms greet users with complex, blank-canvas desktop admin panels requiring 30-60 minutes of configuration, app installation ("App Tax"), and manual data entry just to see a basic storefront. The current OHC offering lacks a fully native, instantly gratifying mobile onboarding flow. If a user cannot generate a functional, visually appealing storefront with a single natural language prompt from their phone in under 10 minutes, OHC fails its core promise of "unclear work -> clear next action in minutes" without technical manuals.

  ### Research Report
  - **Market Context**: Competitor research (`docs/business/market_research/agentic_autonomous_website_builders_smb_platform_gap_analysis.md`, `ohc_smb_market_report.md`) shows that setup paralysis is the #1 pain point (28%) for SMBs adopting e-commerce.
  - **Competitor Flaws**: Shopify's "Sidekick" acts only as an advisory chatbot rather than executing state changes. Wix and Squarespace require significant manual drag-and-drop design. Current AI builders (e.g., Durable) generate toy sites that lack real operational depth (e.g., integrating payments or bookings).
  - **The OHC Gap**: OHC currently lacks the "Zero-Click Generation" flow where an SMB owner simply says "I'm a baker in Austin selling custom vegan cakes" and the system autonomously structures the database schema, creates sample product variants, configures a Stripe-ready checkout flow, and renders a mobile-optimized UI.
  - **Opportunity**: By integrating our existing AI capabilities (like the Vision/Agent Harness) with the mobile-first Flutter frontend, OHC can deploy an "Onboarding Agent" that transitions the user from an empty state to a fully configured, multi-tenant isolated storefront purely through conversational interaction on a 375px screen.

  ### Design Doc
  **Architecture Overview (Mermaid)**
  ```mermaid
  sequenceDiagram
      participant Owner as User (Mobile 375px)
      participant Onboarding UI as Flutter Mobile App
      participant API as OHC Go Backend
      participant KAIROS as KAIROS Orchestration
      participant AI as Onboarding Agent (LLM)
      participant DB as Postgres (Tenant DB)

      Owner->>Onboarding UI: Enters prompt ("I sell custom vegan cakes")
      Onboarding UI->>API: Submit Prompt Payload
      API->>KAIROS: Dispatch Generation Workflow
      KAIROS->>AI: Generate Schema, Content, Flow
      AI-->>KAIROS: Return JSON (Products, Pricing, Layout)
      KAIROS->>DB: Seed Tenant Data (Row-Level Security)
      KAIROS-->>API: Storefront Ready Event
      API-->>Onboarding UI: Render Live Preview
      Onboarding UI-->>Owner: Display Storefront & "Ready for Setup" Action
  ```

  **Mobile UX Flow (375px First)**
  1. **Landing Screen**: A clean, distraction-free screen asking one question: "What does your business do?" with a large native keyboard input field. No complex menus or settings.
  2. **Loading/Agent State**: While KAIROS processes the prompt, show a premium "Translucent Glass" loading state explaining what the agents are doing (e.g., "Designing catalog...", "Setting up booking logic...").
  3. **Live Preview**: The generated storefront appears seamlessly. The user sees their products/services with AI-generated placeholder images and copy.
  4. **The "Next Action"**: A clear, single call-to-action button (e.g., "Connect Bank" or "Publish") guides the owner to the next vital step, keeping them in the flow without navigating complex admin dashboards.

  **AI Agent Integration Points**
  - **Onboarding Agent**: Parses the initial natural language prompt to determine the business type (e.g., Service vs. Physical Product) and generates the necessary JSON structure for the KAIROS engine to seed the database.
  - **Operations Agent**: Automatically configures default booking rules or shipping zones based on the inferred business type.

  **Key Design Decisions**
  - **Zero-Click Execution**: AI must *do* the work (seed DB, configure settings), not just tell the user how to do it.
  - **Mobile Native**: The entire flow must be built and tested for a 375px viewport (Flutter iOS/Android targets).
  - **Immediate Gratification**: The user must see a functional preview before being asked to create an account or provide payment details, lowering the barrier to entry.

  ### Implementation Prompt
  **User Persona**: Maya (Home Baker) / Carlos (Field Service)
  **Critical User Journey (CUJ)**:
  1. User opens the OHC mobile app (375px viewport).
  2. User enters a single sentence describing their business.
  3. User taps "Generate".
  4. The system securely provisions a new tenant space.
  5. The AI agent generates relevant products/services, pricing, and a basic storefront layout.
  6. The user is presented with a fully functional preview of their storefront and a single "Next Step" action (e.g., Connect Payment).

  **Acceptance Criteria**:
  - Implement the UI flow in the Flutter application, strictly adhering to the 375px mobile-first requirement and OHC Translucent Glass design tokens.
  - Create the API endpoint in the Go backend to accept the onboarding payload.
  - Integrate the `ohc-builtin-agent` to process the user prompt and generate structured business data.
  - Ensure the KAIROS orchestration correctly seeds the PostgreSQL database with the generated data under strict multi-tenant RLS isolation.
  - All interactive elements in the new flow must be thoroughly verified with Playwright E2E tests, simulating a realistic owner prompt and validating the resulting storefront state. NO mocked backend data allowed.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []