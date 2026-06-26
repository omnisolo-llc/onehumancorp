issue_title: "Implement Autonomous Zero-Click Onboarding Engine"
issue_description: |
  ## Mission Queue Protocol: Autonomous Zero-Click Onboarding Engine

  ### 1. Problem Statement
  Small business owners (e.g., Maya, the 28-year-old home baker) face "setup paralysis" when adopting new digital tools. Currently, OHC requires manual configuration: adding services, setting up deposits, configuring DNS, and creating storefronts. Real-world users abandon the platform because they want to sell products, not configure software. Competitive research shows that AI-native platforms like Durable generate complete business setups in under a minute via natural language. We need to close this gap by building a "Zero-Click Onboarding Engine" that provisions a fully functional workspace, catalog, and agent feed from a simple conversational prompt.

  ### 2. Research Report
  - **Codebase & Docs Audit**: OHC currently uses manual creation flows (`src/server/domain/organization.rs` and related frontend onboarding screens). We have strong building blocks like the internal KAIROS orchestration engine and specialised services (`booking`, `quoting`), but they are not linked into a unified, zero-click conversational onboarding flow.
  - **Competitor Insights**:
    - *Durable.co*: 30-second setup generating a full website, CRM, and invoicing.
    - *Shopify Magic*: Still requires structural setup (shipping zones, domains) despite AI assistance.
  - **Observed Gap**: Through manual product use of the onboarding flow, a user must manually input business details, add their first product, and connect services. The "Zero-Click" experience is absent. The smallest complete product change is to introduce a conversational onboarding interface that automatically translates an owner's natural language input into a populated database schema (tenant, products, agent config).

  ### 3. Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD;
      User[Owner/Maya] -->|Natural Language Input| OnboardingUI[Mobile Conversational Onboarding UI]
      OnboardingUI -->|Streaming Prompt| OHCBackend[OHC API / Blueprint Service]
      OHCBackend -->|Intent & Context Resolution| LLMLayer[LLM / Intent Analyzer]
      LLMLayer -->|Schema Generation| OHCBackend
      OHCBackend -->|Tenant Setup| DB[(PostgreSQL Row-Level Security)]
      OHCBackend -->|Create Products/Services| Catalog[Catalog & Service Modules]
      OHCBackend -->|Configure AI Agents| AgentFeed[Agent Feed Config]
      Catalog --> SuccessUI[Dashboard / Agent Feed]
      AgentFeed --> SuccessUI
  ```

  #### Mobile UX Flow (375px First)
  1. **Landing Screen**: A translucent glass card centered on the screen saying "Tell me about your business." with a single text input (or voice memo).
  2. **Processing Screen**: A loading state (using OHC Premium Tokens) with dynamic text: "Setting up your storefront... Configuring AI assistants... Preparing your booking calendar..."
  3. **Success Screen**: The user is dropped directly into the **Agent Feed**, with pre-populated action cards (e.g., "Review your new Custom Cake offering", "Connect Stripe for deposits").

  #### AI Agent Integration Points
  - **Blueprint Agent**: An internal onboarding agent that takes the unstructured user prompt and translates it into structured payloads (business name, category, initial product offerings, suggested policies).
  - **Operations Agent**: Automatically provisions the necessary calendar slots and inventory structures based on the Blueprint Agent's output.

  #### Key Design Decisions
  - **Conversational Input**: Bypasses traditional form-based onboarding completely.
  - **Agentic Orchestration**: Uses the existing KAIROS engine to orchestrate the provisioning of multiple services (catalog, bookings, settings) asynchronously.
  - **Immediate Value Delivery**: Drops the user into the Agent Feed with actionable items rather than an empty dashboard, ensuring they know what needs attention today.

  ### 4. Implementation Prompt
  **To the Implementer Agent:**
  Your task is to implement the "Autonomous Zero-Click Onboarding Engine".
  - **CUJ**: A non-technical user (Maya) opens the app for the first time on her 375px phone, inputs "I sell custom vegan cakes on Instagram", and within 30 seconds is placed into her Agent Feed with a pre-configured business, a draft product for "Custom Vegan Cake", and a prompt to connect her Instagram account.
  - **Acceptance Criteria**:
    1. Introduce a single conversational onboarding screen (Flutter/Tauri UI) that accepts a natural language description.
    2. Build an API endpoint that processes this description using the builtin LLM provider to extract business details and initial catalog items.
    3. Persist the extracted entities into the PostgreSQL database under a new tenant with Row-Level Security.
    4. Transition the user directly to the Agent Feed showing at least one auto-generated "Review Product" action card.
    5. Write at least 5 Playwright E2E tests verifying the complete flow from input to Agent Feed without mock data.
    6. Ensure the design adheres to the translucent glass macOS-style and UniFi modular dashboard cards.
    7. Unit tests for the new onboarding pipeline must be at 100% coverage.

  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
