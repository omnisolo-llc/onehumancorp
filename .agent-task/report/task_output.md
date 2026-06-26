issue_title: "Implement Autonomous Zero-Click Onboarding Agent"
issue_description: |
  # Research Report: Autonomous Zero-Click Onboarding Agent

  ## Problem Statement
  Small business owners (e.g., Maya the Home Baker, Carlos the Handyman) experience severe "setup paralysis" when launching their business online. Traditional platforms like Shopify or Wix require them to navigate complex menus, understand DNS, configure payment gateways, and manually build out product catalogs before making a single sale. This technical and cognitive barrier leads to high abandonment rates and delays time-to-revenue. The current OHC setup, while capable, still relies on a manual, multi-step wizard that feels like an admin portal rather than an assistant.

  ## Research Report
  - **Competitive Landscape**:
    - **Shopify/Wix/Squarespace**: Require significant manual configuration. Their AI tools (like Shopify Sidekick) are advisory chatbots, not autonomous executors.
    - **Durable/Mixo**: Generate landing pages quickly but lack deep, native e-commerce and operational integration (bookings, inventory, multi-tenant separation).
  - **The Gap**: Users need an "invisible" setup experience where they simply describe their business in natural language, and the platform autonomously provisions the necessary infrastructure, schema, and initial content.
  - **OHC Opportunity**: By leveraging the existing KAIROS orchestration and `onboarding_agent.rs`, OHC can introduce a "Zero-Click Generation" flow. A single prompt (e.g., "I'm Maya, I sell custom vegan cakes in Austin via Instagram") should trigger a coordinated swarm of agents to build the store, configure Stripe, set up an initial catalog, and generate SEO-optimized copy, all within minutes.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1.  **The Prompt**: The user is greeted with a clean, translucent glass interface and a single, large text input: "Tell me about your business..."
  2.  **The Assembly**: A vibrant, animated loading state (macOS style) shows the AI departments at work:
      - "Provisioning secure database..."
      - "Designing your storefront..."
      - "Drafting initial product catalog..."
  3.  **The Reveal**: The user is presented with a fully functional, mobile-optimized storefront preview.
  4.  **The Action**: A primary CTA (e.g., "Looks Good - Go Live") approves the generated state.

  ### AI Agent Integration Points
  -   **Onboarding Agent (Coordinator)**: Parses the initial prompt using LLM to extract business type, tone, location, and core offerings.
  -   **Operations Agent**: Autonomously provisions the tenant database schema, initial `services` or `products`, and sets up the Stripe integration config.
  -   **Marketing Agent**: Generates SEO meta tags, product descriptions, and storefront copy based on the extracted business context.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      User[User Prompt] --> OHA[Onboarding Agent];
      OHA --> Intent[Extract Business Intent & Entities];
      Intent --> OpsAgent[Operations Agent];
      Intent --> MktgAgent[Marketing Agent];

      OpsAgent --> DB[(Tenant DB)];
      OpsAgent --> Stripe[Stripe Config];
      MktgAgent --> Content[Generate Copy & SEO];

      DB --> Preview[Mobile Storefront Preview];
      Content --> Preview;
      Stripe --> Preview;
  ```

  ### Key Design Decisions
  -   **Conversational vs. Wizard**: Replace the multi-step `onboarding/page.tsx` wizard with a single-prompt entry point to drastically reduce cognitive load.
  -   **Agentic Execution**: Move from the LLM just suggesting JSON (advisory) to the agents actually executing the database mutations and service provisioning before presenting the result to the user.
  -   **Mobile Parity**: The entire setup must be doable from a 375px screen, meaning no complex drag-and-drop or dense configuration tables.

  ## Implementation Prompt
  **Feature Name**: Autonomous Zero-Click Onboarding Agent
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya can launch a fully functional OHC storefront with product listings and payment capabilities using only a natural language description of her business.

  **Acceptance Criteria**:
  1.  Create a new single-prompt onboarding entry point optimized for mobile (375px).
  2.  Update the `onboarding_agent.rs` (or create a new flow) to coordinate the creation of the tenant, initial services/products, and storefront configuration based entirely on the LLM's parsing of the prompt.
  3.  Ensure the generated storefront is immediately previewable and functional upon user approval.
  4.  The entire process must not require the user to manually configure settings or navigate complex menus.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
