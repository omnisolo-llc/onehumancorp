issue_title: "Implement Zero-Click Agentic Onboarding Flow & Departmental AI Workers"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## 1. Problem Statement
  Small business owners (SMBs) like Carlos the handyman or Fatima the food cart operator are overwhelmed by the setup process of traditional platforms like Shopify or Wix. The "Setup Paralysis" caused by a blank canvas leads to high abandonment rates (73% of non-technical users abandon complex setups). They need a system that translates a single natural language prompt into a fully functional storefront, database schema, and operational configuration without clicking through dozens of menus.

  ## 2. Research Report
  - **Market Mapping**:
    - *Traditional*: Shopify (complex, app tax), Wix (visual, disjointed e-commerce), GoDaddy (simple but limited).
    - *AI-Native*: Durable (30-sec site generation), Mixo (landing pages), Framer AI (design focused).
  - **The OHC Opportunity**: OHC must move beyond "AI as an advisor" (like Shopify Sidekick) to "AI as an executor". OHC should implement a "Zero-Click Generation" flow where an agent autonomously builds the entire business architecture from a single prompt.
  - **Competitor Gaps**:
    - *Shopify Sidekick*: Tells you *how* to do things, doesn't do them for you.
    - *Durable/Mixo*: Generates simple frontends but lacks deep operational backend (inventory, bookings, payments).

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[User Enters Prompt: 'I am a baker in Austin'] --> B(Onboarding Agent Gateway)
      B --> C{Orchestrator Agent}
      C --> D[Operations Agent: DB Schema & Inventory]
      C --> E[Sales Agent: Pricing & Offers]
      C --> F[Marketing Agent: Storefront Copy & SEO]
      C --> G[Design Agent: Theme & Layout]
      D --> H(PostgreSQL Central Ledger)
      E --> H
      F --> I(Static Asset Pre-rendering)
      G --> I
      H --> J[Owner Dashboard: Ready to Launch]
      I --> J
  ```

  ### Mobile UX Flow (375px)
  1. **The Single Prompt**: A clean, full-screen input field: "Tell us about your business." User types: "I sell custom vegan cakes in Austin."
  2. **The Loading State**: A visually engaging sequence showing the "Departmental AI Workers" (Operations, Sales, Marketing, Design) building the business in real-time. No loading spinners; show the actual work (e.g., "Drafting menu...", "Configuring deposit payments...").
  3. **The Reveal**: The generated storefront and dashboard are presented. The user has 1-tap options to "Publish" or "Refine."

  ### AI Agent Integration
  - **Departmental AI Workers**: Separate agents handle specific domains to reduce hallucinations and improve quality.
    - *Operations*: Defines `Product` vs `Service`, sets up inventory logic.
    - *Sales*: Suggests pricing models (e.g., deposits for custom cakes).
    - *Marketing*: Generates product descriptions and SEO tags.
    - *Design*: Selects appropriate UI components and translucent glass materials.

  ## 4. Implementation Prompt
  **Feature Name**: Zero-Click Agentic Onboarding Flow
  **Target Persona**: Fatima the Food Cart Operator
  **Outcome**: Fatima types "Halal food cart pre-orders" and within 60 seconds has a fully functional mobile menu with sold-out toggles and pre-order payment flows configured.

  **Next Actions**:
  1. Create the `OrchestratorAgent` service that parses the initial prompt and delegates tasks to domain-specific agents.
  2. Implement the "Departmental Agents" (Operations, Sales, Marketing, Design) using the `ohc_builtin_agent` to generate structured JSON payloads representing the business configuration.
  3. Build the Flutter/Tauri mobile-first UI for the single-prompt input and the real-time building visualization.
  4. Ensure the generated configuration correctly initializes the PostgreSQL database and Stripe payment settings.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
