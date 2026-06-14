issue_title: "Implement Zero-Click Agentic Onboarding Architecture"
issue_description: |
  # Research Report: Zero-Click Agentic Onboarding Architecture

  ## 1. Problem Statement
  Small business owners (e.g., Maya the Home Baker) abandon the onboarding process on traditional platforms because of technical complexity (e.g., configuring DNS, setting up Stripe schemas, establishing shipping zones). OHC's current setup requires manual data entry (taking up to an hour) rather than leveraging our AI agents for a "Zero-to-One" instantaneous launch. Users want the platform to understand their business from a single sentence or image and automatically configure the storefront and underlying operational databases.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require manual configuration and multiple plugin installations. AI-native competitors like Durable create basic web pages in 30 seconds but lack backend operational tools (inventory, POS, advanced bookings).
  - **The OHC Opportunity**: By introducing a `Setup Agent` combined with an autonomous onboarding flow, OHC can capture non-technical users by providing an operational storefront with initialized databases in under 10 minutes.
  - **Competitor Gaps**:
    - *Shopify*: High setup paralysis (days to configure fully).
    - *Durable*: Quick frontend setup but weak operational/backend capabilities.
    - *Wix*: Guided setup but still manual.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      User[User Natural Language Input] --> IntentClassifier[LLM Setup Classifier]
      IntentClassifier --> DataGen[Tenant & Schema Generator]
      DataGen --> DB[PostgreSQL Tenant DB Provisioning]
      DataGen --> MediaGen[Gemini Vision Media & Copy Gen]
      DB --> StoreFront[Edge Cached Storefront]
      MediaGen --> StoreFront
      StoreFront --> MobileUX[Mobile First Dashboard]
  ```

  ### Mobile UX Flow (375px)
  1. **Welcome Screen**: A simple chat-like interface. Prompt: "Tell me about your business in one sentence, or upload a photo of what you do."
  2. **Loading State**: An engaging "Agent at work" screen explaining actions (e.g., "Drafting your menu...", "Configuring payments...").
  3. **Launch Screen**: A complete storefront preview and populated admin feed.

  ### AI Agent Integration Points
  - **Setup Agent**: Parses user input, determines business category, and orchestrates other agents.
  - **Operations Agent (Provisioning)**: Initializes the tenant context in PostgreSQL and sets default rules for shipping/availability based on the category.
  - **Promoter Agent**: Generates initial catalog items and descriptions from the provided input/image.

  ### Key Design Decisions
  - **Single Input Entry**: The flow must start with a single, unstructured input (text or image) to eliminate friction.
  - **Immediate Provisioning**: The Tenant schema and default context MUST be provisioned dynamically without manual form submissions.
  - **Mobile-First Priority**: The entire setup and preview must be fully functional and readable on a 375px wide screen.

  ## 4. Implementation Prompt
  **Target Persona**: Maya the Home Baker
  **User Facing Outcome**: Maya speaks one sentence into her phone ("I make custom vegan cakes in Brooklyn"), and within a few minutes, she has a live, mobile-optimized storefront with pre-populated products, a deposit-based payment structure ready to go, and an AI agent configured to reply to Instagram DMs.

  **Acceptance Criteria**:
  1. Implement the chat interface for the initial onboarding prompt, optimized for 375px.
  2. Create the backend `Setup Agent` workflow to classify the business and trigger tenant provisioning.
  3. Integrate the LLM (Gemini Pro/Vision) to generate default products, copy, and policies based on the input.
  4. Ensure a fully functional, auto-configured "Preview Store" state is presented immediately after the flow.

  **Note for Implementer**: Focus on the orchestration and the seamless user journey. The underlying multi-tenancy rules and database must be populated seamlessly in the background.

  ## 5. Priority
  P0

  ## 6. Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
