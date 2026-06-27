issue_title: "Zero-Click Generation: Agentic Autonomous Storefront & Schema Setup"
issue_description: |
  ## Mission Queue Protocol Brief
  **Problem Statement:**
  Non-technical business owners (like Maya the baker or Carlos the handyman) face "Setup Paralysis" when adopting new platforms. Traditional builders like Shopify present a blank canvas that requires 30-60 minutes of configuration, selecting themes, adding products manually, and dealing with fragmented apps. SMBs want an AI that executes and builds the setup based on a simple prompt ("I'm a baker in Austin"), rather than glorified chatbots that only offer advice.

  ## Research Report
  - **Market Context**: Legacy platforms (Shopify, Wix) require high cognitive load for setup and are inherently desktop-first. AI-native tools (Durable, Hocoos) generate static sites but lack deep operational schemas (inventory, bookings, payments).
  - **Competitive Analysis**: Shopify's Sidekick advises but doesn't autonomously execute structural changes. OHC must differentiate by bridging the gap between "Zero-Click" site generation and operational multi-tenant database schema deployment.
  - **Proposed Solution**: A "Zero-Click Generation" flow where a single natural language prompt autonomously generates the database schema (products, services), multi-tenant data boundaries, and a fully functional 375px-optimized storefront.

  ## Design Doc
  ### High-Level Architecture
  - **Component 1**: `PromptIngestionService` - Captures the owner's single-sentence intent.
  - **Component 2**: `SchemaGenerationAgent` - Converts intent into OHC's internal data model representation (e.g., creating specific Product and Booking types tailored to the business).
  - **Component 3**: `StorefrontBuilderAgent` - Selects optimal UI layout templates and translucent design tokens, populating them with generated content.
  - **Component 4**: `MultiTenantProvisioner` - Ensures complete SPIFFE/SPIRE Zero-Trust isolation and row-level security setup for the new tenant.

  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  sequenceDiagram
      actor Owner
      Owner->>+PromptIngestion: "I am a handyman in Miami"
      PromptIngestion->>+SchemaGenerationAgent: Generate Data Model
      SchemaGenerationAgent-->>-PromptIngestion: Returns Product/Booking schemas
      PromptIngestion->>+MultiTenantProvisioner: Provision Tenant & DB Rows
      MultiTenantProvisioner-->>-PromptIngestion: Tenant ID
      PromptIngestion->>+StorefrontBuilderAgent: Generate Mobile UI
      StorefrontBuilderAgent-->>-PromptIngestion: UI Layout Config
      PromptIngestion-->>-Owner: Redirect to Live Mobile Storefront
  ```

  ### Mobile UX Flow (375px First)
  1. **Landing Screen**: A single, prominent text input: "What kind of business do you run?" with a voice input option.
  2. **Loading State (Glassmorphism)**: Engaging translucent loading screen ("Building your catalog...", "Setting up bookings...") taking no more than 15 seconds.
  3. **Live Storefront View**: The fully interactive mobile storefront, ready for review.
  4. **Agent Feed Prompts**: Action cards pop up asking "Do you want to adjust pricing for these services?" to refine the initial generation.

  ### AI Agent Integration Points
  - **Operations Department**: Receives the initial schema and sets up default workflows (e.g., typical handyman appointment durations).
  - **Marketing Department**: Drafts initial SEO-friendly descriptions and service names.

  ## Implementation Prompt
  **Goal:** Implement the `ZeroClickGenerator` API and its mobile-first onboarding UI.
  **CUJ:** A new user opens the app, types "I sell custom vegan cakes," and within 15 seconds is dropped into a functional, multi-tenant-secured storefront loaded with 3 placeholder products and a booking calendar.
  **Acceptance Criteria:**
  - Create the UI with a single text field on a 375px width screen.
  - Connect to a mocked backend endpoint (in test environment) that returns a generated tenant configuration.
  - E2E Playwright test must verify the flow from text input to the rendered storefront screen.
  - Ensure zero mock data is hardcoded in the frontend components; everything must flow from the API.

  ## Priority & Scope
  **Priority:** P0 (Critical for Acquisition)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []