issue_title: "Implement Zero-Click Generation Agentic Flow for Mobile Onboarding"
issue_description: |
  # Zero-Click Generation Agentic Flow for Mobile Onboarding

  ## Problem Statement
  Current e-commerce platforms like Shopify, Wix, and Squarespace require significant manual configuration (30-60+ minutes) to set up a basic storefront. For non-technical SMB owners (like Maya the Baker or Carlos the Handyman), the initial "blank canvas" is overwhelming and leads to high abandonment rates (73%). They need an AI assistant that doesn't just offer advice, but actively executes the setup process. We need a "Zero-Click Generation" flow where a user can provide a single descriptive sentence (e.g., "I'm a baker in Austin selling custom vegan cakes") and the AI agent autonomously generates the product catalog, storefront layout, database schema, and initial copy, all accessible via a mobile-first interface.

  ## Research Report
  - **Market Gap**: Competitors (Shopify Sidekick) offer chatbots that advise *how* to use the platform, but do not execute state changes autonomously.
  - **User Need**: SMBs want an AI that executes setup tasks.
  - **Solution**: A conversational prompt-based onboarding flow that triggers a backend AI agent to generate the initial business state.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User (Mobile)
      participant OHC API Gateway
      participant Onboarding Agent
      participant OHC Database
      participant UI/Storefront Engine

      User (Mobile)->>OHC API Gateway: Submit prompt ("Baker in Austin...")
      OHC API Gateway->>Onboarding Agent: Trigger zero-click generation
      Onboarding Agent->>OHC Database: Generate Schema & Initial Data (Products, Prices)
      Onboarding Agent->>UI/Storefront Engine: Generate Storefront Layout & Copy
      Onboarding Agent-->>OHC API Gateway: Generation Complete (Returns tenant config)
      OHC API Gateway-->>User (Mobile): Redirect to generated storefront dashboard
  ```

  ### Mobile UX Flow (375px first)
  1. **Landing Screen**: A simple, clean, Translucent Glass-styled screen with a large text input area. Prompt: "Tell us about your business in one sentence."
  2. **Loading State**: An engaging animation showing the AI agent at work ("Generating product catalog...", "Designing storefront layout...").
  3. **Dashboard / Preview**: The user is immediately dropped into their newly generated storefront dashboard, populated with placeholder (but relevant) products, copy, and layout.

  ### AI Agent Integration Points
  - **Onboarding Agent**: A dedicated agent responsible for interpreting the initial prompt and executing the necessary CRUD operations to set up the tenant's workspace.
  - **Data Generation**: Uses LLM capabilities to infer product types, pricing, and business logic from the user prompt.

  ### Key Design Decisions
  - **Mobile-First Execution**: The entire flow must be seamless on a 375px screen without requiring complex form inputs.
  - **Agentic Execution**: The AI must perform actual database writes (with user consent/preview) rather than just generating text suggestions.
  - **Translucent Glass UI**: Use OHC's premium design tokens for a polished feel.

  ## Implementation Prompt
  **User Facing Outcome**: The user can create a fully functional, personalized storefront by submitting a single sentence describing their business on their mobile device.

  **Critical User Journey (CUJ)**:
  1. User opens the OHC mobile app (or PWA).
  2. User enters a description (e.g., "Handyman in Chicago").
  3. User sees a loading screen indicating AI agent activity.
  4. User is presented with a fully populated dashboard/storefront tailored to a handyman service, including placeholder services, pricing, and a booking calendar.

  **Acceptance Criteria**:
  - Implement a mobile-first (375px) onboarding screen that accepts a single text prompt.
  - Develop a backend service (Onboarding Agent) that processes this prompt.
  - The agent must generate and persist relevant initial data (e.g., product categories, placeholder items, default layout settings) in the database for the new tenant.
  - The user must be redirected to their generated workspace seamlessly.
  - End-to-end Playwright tests must verify the entire flow from prompt submission to dashboard rendering.
  - 100% unit test coverage for the new backend service and frontend components.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
