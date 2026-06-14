issue_title: "Implement Zero-Click Onboarding Agent"
issue_description: |
  **Title**: Implement Zero-Click Onboarding Agent

  **Problem Statement**:
  Small business owners face significant "setup paralysis" when starting with traditional platforms (like Shopify or Wix). They are presented with a blank canvas and complex configuration menus (shipping zones, payment gateways, theme customization). 34% of small business owners abandon setup due to technical complexity. Maya (the baker) wants to sell cakes, not configure DNS or database schemas.

  **Research Report**:
  - **Traditional Flow**: Hours of manual data entry, theme selection, and app installation.
  - **AI-Native Rivals (Durable, 10Web)**: Generate sites in under a minute based on a prompt. However, these are often just "brochure" sites without deep operational backends.
  - **OHC Opportunity**: Implement a "Zero-Click Onboarding Agent." The user simply describes their business (or provides an Instagram handle), and the agent provisions the tenant DB, selects a premium Glassmorphism theme, generates initial product catalogs, and sets up Stripe placeholders—all autonomously. The user is guided from unclear work to a clear next action (reviewing the generated site) in minutes.

  **Design Doc**:
  - **Architecture diagram**:
    ```mermaid
    graph TD
        A[User Prompt/Input] --> B(Setup Agent)
        B --> C{Orchestration Hub}
        C --> D[Provision Tenant DB Schema]
        C --> E[Generate Theme/UI Assets]
        C --> F[Create Initial Products/Services]
        C --> G[Configure Stripe Placeholders]
        D --> H[Live Preview Ready]
        E --> H
        F --> H
        G --> H
    ```
  - **Mobile UX flow (375px)**:
    1. **Onboarding Screen**: A simple chat interface. "Tell me about your business..."
    2. **User Input**: "I'm a baker in Austin selling custom vegan cakes."
    3. **Agent Action State**: A visually engaging loading screen showing the agent's progress ("Provisioning database...", "Designing storefront...", "Generating cake catalog...").
    4. **Result State**: A success card displaying the generated storefront preview and a "Publish & Start Selling" button.
  - **AI agent integration points**:
    - **Setup Agent (LLM)**: Parses the user's intent to determine the business type (Product, Service, Food) and necessary data schemas.
    - **Operations Agent**: Creates initial dummy inventory or booking slots based on the business type.
  - **Key design decisions**:
    - The onboarding process must happen entirely through a conversational interface, not form fields.
    - The backend must dynamically apply the correct multi-tenant schema based on the agent's interpretation of the business.
    - The generated UI must strictly adhere to OHC's Premium Tokens (Glassmorphism, 375px responsiveness).

  **Implementation Prompt**:
  Build the "Zero-Click Onboarding" flow. Create a mobile-first chat interface where a user can describe their business. Implement the `Setup Agent` that takes this prompt, calls the backend APIs to create a new tenant, populates a default product/service catalog relevant to the prompt, configures basic settings, and returns a live preview link. Ensure the entire process takes less than 3 minutes and requires no technical knowledge from the user.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
