issue_title: "Implement 'Zero-Click' Conversational Mobile Onboarding Agent"
issue_description: |
  ## Problem Statement
  Small business owners face high drop-off rates during initial setup due to "Setup Paralysis". Traditional platforms like Shopify and Wix require manual navigation of complex settings, theme selection, and plugin installations. For our core personas like Maya (Home Baker) and Carlos (Handyman), this technical hurdle prevents them from activating. They need an assistant that builds their business context simply by talking to them.

  ## Research Report
  - **Competitor Analysis**: Shopify and Wix have added AI assistants (Shopify Sidekick, Wix Studio AI), but they still expect users to use traditional dashboards to complete setup. AI-native tools like Durable.co have gained traction by generating a complete business website in 30 seconds from a single prompt, but they lack deep operational/commerce capabilities.
  - **Market Gap**: OHC requires a seamless, chat-driven "Zero-Click" mobile onboarding flow. 73% of non-technical users abandon complex setups. A conversational interface where the user just answers questions ("I am a baker in Austin", "Here is a photo of my cake") and the AI automatically generates the DB schema, product catalog, and storefront layout is a critical differentiator.
  - **References**: Durable (durable.co), Shopify Sidekick, Wix AI, reddit.com/r/smallbusiness/comments/1910675662/shopify_setup_struggles/

  ## Design Doc
  ### Architectural Design
  - **Component Diagram**:
    ```mermaid
    graph TD
      MobileApp[Flutter Mobile App - 375px] --> |Natural Language / Images| OnboardingAPI[Onboarding Service API]
      OnboardingAPI --> SetupAgent[KAIROS Setup Agent]
      SetupAgent --> |Gen Schema| DBTenant[Tenant Schema Provisioning]
      SetupAgent --> |Gen Content| CatalogService[Product & Catalog Service]
      SetupAgent --> |Payment Setup| PaymentService[Payment Gateway Integration]
      SetupAgent --> |UX Generation| StorefrontAPI[Edge Cached Storefront]
    ```
  - **Mobile UX Flow (375px Baseline)**:
    1. **Welcome Screen**: Clean, translucent glass UI. "What do you do?" with a voice input / text input box.
    2. **Conversational Steps**: Agent asks 3-4 follow up questions in a chat format. (e.g. "Do you take deposits?", "Upload a photo of your work").
    3. **Magic Moment**: Progress indicator showing autonomous agents executing tasks (Provisioning DB, Generating Theme, Creating Products).
    4. **Launch**: "Your business is ready." showing a preview of the storefront.
  - **AI Agent Integration Points**:
    - The `Setup Agent` is invoked via KAIROS orchestration engine to translate the conversational prompt into standard REST/gRPC API calls to provision the tenant, populate the database with default products (derived from image/text), and set up basic configurations.
  - **Key Design Decisions**:
    - **No Dashboards during Setup**: The entire onboarding must feel like texting an assistant.
    - **Progressive Enhancement**: Only ask for advanced settings (shipping zones, tax info) post-activation via the unified agent feed when it becomes relevant.

  ## Implementation Prompt
  **User Persona**: Maya (Home Baker, 28) using an iPhone (375px width).
  **Critical User Journey (CUJ)**: Maya opens the OHC mobile app for the first time. She sees a chat interface instead of a form. She types "I make custom vegan cakes in Austin." The agent asks for an example photo. She uploads one. The agent replies "Give me a minute...", and autonomously creates her tenant, a "Vegan Cake" product with an AI-generated description, configures a deposit requirement, and generates her mobile-optimized storefront link.

  **Acceptance Criteria**:
  1. Build a Flutter mobile onboarding screen that utilizes a conversational UI.
  2. Implement backend logic to receive onboarding chat inputs and trigger the Setup Agent.
  3. The Setup Agent must be able to translate user context into functional tenant records (products, basic business info) in Postgres.
  4. Ensure 100% usability on a 375px viewport with native mobile keyboards and touch targets >= 44x44px.
  5. Provide an E2E Playwright test simulating this conversational onboarding flow to verify successful tenant creation and product generation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
