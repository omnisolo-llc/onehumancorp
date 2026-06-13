issue_title: "Implement Autonomous Zero-Click Onboarding Agent for SMB Setup"
issue_description: |
  ## Problem Statement
  Small business owners and non-technical operators (like Maya the Baker or Carlos the Handyman) frequently abandon the setup of traditional platforms (e.g., Shopify, Wix) due to "technical complexity" and the overwhelming blank canvas. They want to sell their services and products immediately, not configure DNS settings, database schemas, or complex app integrations. We need a "Zero-Click Onboarding Agent" that transitions a user from an initial text prompt to a fully provisioned storefront and CRM in under 10 minutes.

  ## Research Report
  - **Market Context**: Traditional platforms (Shopify) rely on complex admin dashboards and pieced-together app ecosystems, creating "setup paralysis." AI-native builders (Durable, 10Web) are proving that sub-minute site generation is a powerful acquisition tool, but they often stop at basic brochure sites without deep commerce/ops integration.
  - **OHC Positioning**: OHC must differentiate by combining the speed of AI generation with the depth of its existing KAIROS engine and operational modules (booking, quoting, POS).
  - **Persona Fit**:
    - *Maya*: Needs to instantly convert her Instagram bio into a structured deposit-taking product page.
    - *Carlos*: Needs to turn a one-sentence business description into a quote-generation and booking portal.
  - **Data Insight**: 73% of non-technical users abandon complex setups. Reducing the initial friction to a single conversational interaction is a massive growth lever.

  ## Design Doc
  ### AI Agent Integration Points
  - **Onboarding LLM Router**: Intercepts the initial user prompt and categorizes the business type (e.g., physical goods, services, food).
  - **Schema Generation Agent**: Translates the categorization into initial database records (products, services, initial inventory).
  - **Content & Design Agent**: Generates the UI layout using the OHC Premium Token library, selects appropriate translucent glass styles, and drafts copy based on the initial prompt.
  - **Infrastructure Agent**: Provisions the multi-tenant `tenant_id` in PostgreSQL, sets up Stripe placeholder configurations, and generates the unique public URL.

  ### Mobile UX Flow (375px First)
  1. **Landing/Auth Screen**: Simple login/signup.
  2. **The Magic Prompt**: A single full-screen conversational input field: "Tell me about your business..."
  3. **Agent Loading State**: "OHC is building your business... (Provisioning database... Generating products... Designing storefront...)".
  4. **The "Ta-Da" Moment**: The user lands directly on the "Assistant-First Shell" with a generated mock storefront link and a pre-populated product/service.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User
      participant Onboarding UI (Mobile)
      participant OHC Backend
      participant Setup Agent
      participant Schema Agent
      participant Postgres (Tenant DB)

      User->>Onboarding UI: Enters: "I run a custom cake shop in Austin"
      Onboarding UI->>OHC Backend: POST /api/v1/onboard/agentic { prompt }
      OHC Backend->>Setup Agent: Analyze intent & extract business type
      Setup Agent-->>OHC Backend: Type: Food/Bakery, Name: Custom Cakes Austin
      OHC Backend->>Schema Agent: Generate default products & policies
      Schema Agent-->>OHC Backend: JSON { products: [Vegan Cake, Wedding Cake] }
      OHC Backend->>Postgres (Tenant DB): Create Tenant, Apply RLS, Insert Products
      OHC Backend-->>Onboarding UI: Success { tenant_id, dashboard_url }
      Onboarding UI-->>User: Redirect to Assistant Feed
  ```

  ### Key Design Decisions
  - **Conversational Entry**: Replace all multi-step wizards with a single unstructured text prompt.
  - **Safe Defaults**: The system will make opinionated choices (e.g., Stripe defaults, basic return policy) that the user can edit later, rather than blocking setup.
  - **Tenant Isolation**: The provisioning process must strictly adhere to the existing row-level security (RLS) model.

  ## Implementation Prompt
  Implement the "Zero-Click Onboarding Flow".
  - **User Journey**: The user should land on an onboarding screen on their mobile device (375px), see a single text input to describe their business, and submit it. The system should process this prompt, provision a new tenant context, generate 2-3 sample products/services based on the prompt, and drop the user into the OHC Assistant feed with a fully configured initial state.
  - **Acceptance Criteria**:
    - The UI must be fully functional on a 375px screen without horizontal scrolling.
    - The backend must accept the prompt, interact with the LLM provider (Gemini/MiniMax/OpenAI) to extract the business profile, and successfully create the necessary tenant records in the database.
    - The entire process must complete without requiring the user to navigate complex settings menus.
    - Must include E2E Playwright tests simulating a user (e.g., Maya) signing up and describing her business, verifying the resulting generated products.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []