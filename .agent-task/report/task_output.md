issue_title: "Instant 30-Second Mobile-First Storefront Generation Flow"
issue_description: |
  ## Mission Queue Protocol Report: Zero-Click Onboarding Agent

  ### 1. Problem Statement
  The onboarding friction for most ecommerce platforms is far too high for non-technical small business owners like Maya (the baker) and Carlos (the handyman). They experience setup paralysis. A 10-minute setup with 11 steps feels like a chore, and an empty canvas is intimidating. 73% of non-technical users abandon complex setups. OHC needs to reduce "Time to Live" for the initial storefront and operation system to under 60 seconds.

  ### 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for booking, causing "app tax" and setup fatigue. Wix Harmony and Durable offer AI generation ("vibe coding" and "Get online in 30 seconds" flows), proving users want the system to do the work, not just advise on it.
  - **OHC Current State**: The OHC onboarding process is comprehensive but takes multiple manual steps. However, `OnboardingAgent::process_intake` in the backend already intercepts conversational prompts and extracts metadata (like `business_type`, `company_name`, `company_description`, `selling_categories`).
  - **Competitor Gaps**: Traditional platforms wait for the user to configure menus and settings. AI-native tools generate landing pages but lack deep operational backends (like OHC's `booking` and `pos` modules). OHC can leapfrog by generating not just a visual site, but an operational business model.

  ### 3. Design Doc
  **Architecture Diagram (Mermaid.js)**
  ```mermaid
  graph TD
      A[User enters 1 paragraph bio] --> B{The Advisor (Onboarding Agent)}
      B --> C[Extract Name/Type/Metadata]
      B --> D[Generate Tagline & Copy]
      B --> E[Draft First Product / Service]
      B --> F[Configure Storefront Layout]
      C & D & E & F --> G[Live Preview Generated]
      G --> H[User Clicks Launch]
  ```

  **Mobile UX Flow (375px first)**
  1. **Conversational Entrypoint**: A single text area or microphone input: "Tell us about your business." Example: "I'm Maya, I sell custom celebration cakes in Oakland."
  2. **Agentic Processing State**: A clean, translucent loading state (using OHC premium design tokens) showing the AI agents at work: "Generating products...", "Setting up bookings...".
  3. **Instant Preview**: A fully populated mobile-first storefront with smart defaults (e.g., local delivery for bakers, appointments for handymen).
  4. **Launch or Refine**: A sticky "Launch Now" button at the bottom, and a "Chat to adjust" input to tweak the result.

  **AI Agent Integration Points**
  - **OnboardingAgent (The Advisor)**: Receives the unstructured prompt, parses it to deduce the business model, and queues sub-tasks in the KAIROS Shared Task List.
  - **The Promoter**: Synthesizes the layout and marketing copy.
  - **Operations Agent**: Configures necessary database modules based on extracted intent (e.g., enabling the booking module and setting up `AvailabilityBlock` schemas if the user is a service business like Carlos or Leo).

  **Key Design Decisions**
  - Replace the multi-step `SetupWizard` with a single-prompt entry point for users optimizing for speed.
  - Rely heavily on smart defaults derived from the location and industry text extraction rather than explicit form questions.
  - Use the established KAIROS orchestration to run `generate_initial_products` and template selection in parallel.

  ### 4. Implementation Prompt
  **Outcome**: Maya, the home baker, can create her online store from her iPhone by simply typing: "I bake custom vegan cakes in Oakland and need to take pre-orders."
  **CUJ**:
  1. User navigates to the start screen on a mobile device (375px viewport).
  2. User inputs a single descriptive paragraph about their business.
  3. The system parses the intent via the `OnboardingAgent`, configuring the database modules, initial products, and website template autonomously.
  4. User is presented with a live, functional preview of their storefront in under 60 seconds.
  5. User taps "Launch" and the storefront goes live.
  **Acceptance Criteria**:
  - A new "Instant Build" UI component must be created and fully usable at 375px.
  - The backend `process_intake` logic must successfully parse a single paragraph and route to parallel agentic generation.
  - At least 5 Playwright E2E tests verify the conversational generation flow from the home page.
  - ZERO mock data; the generated preview must reflect the actual database state created by the agents.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []