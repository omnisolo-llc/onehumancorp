issue_title: "Implement Zero-Click Mobile-First Autonomous Store Onboarding Agent"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya, a home baker, or Carlos, a field service operator) face "Setup Paralysis" when trying to transition their business online. Legacy platforms (Shopify, Wix) demand desktop setups, complex configurations (DNS, payment gateways, product catalogs), and an average onboarding time of 30-60 minutes. This creates a massive drop-off, as non-technical owners abandon setup. OHC needs a "Zero-Click Onboarding Agent" that can generate a fully functional, mobile-ready store (database schema, catalog, UI, and payment integration) from a single conversational prompt on a mobile device in under 10 minutes.

  ## Research Report
  Based on competitive mapping and user sentiment analysis (Track 1 & Track 2):
  - **Shopify & Wix**: Great scalability but poor onboarding for non-technical users. Shopify Sidekick only "advises" rather than "executes."
  - **AI-Native Competitors (Durable, Mixo)**: Offer fast generation (e.g., Durable's 30-second site) but lack deep e-commerce operations (inventory sync, automated bookings).
  - **The Missing Link**: 73% of SMBs abandon complex setups. Modern platforms succeed by removing the technical friction (like Link-in-Bio tools) but fall short on deep business ops.
  - **Conclusion**: OHC has an architectural gap in fully autonomous onboarding. The platform needs an orchestrator agent that translates natural language (e.g., "I'm a baker in Austin selling custom cakes") directly into instantiated DB rows, pre-configured Stripe links, and tailored storefronts, without requiring the user to navigate complex settings.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      User[Mobile App User] -->|Natural Language Prompt| API[Mobile App Agent Interface]
      API --> Orchestrator[Onboarding Orchestrator Agent]
      Orchestrator -->|RAG / Intent Classification| LLM[LLM Provider]
      LLM --> Orchestrator
      Orchestrator -->|Execute Schema Generation| DB[PostgreSQL Multi-Tenant DB]
      Orchestrator -->|Configure Payments| PaymentGate[Stripe / Payments Integration]
      Orchestrator -->|Draft Catalog/Copy| ContentStore[Storage / CDN]
      Orchestrator --> Feed[Unified Agent Feed]
      Feed --> User
  ```

  ### Mobile UX Flow
  1. **Initial Screen**: Clean, minimalist UI (375px) with a single chat interface. "Tell me about your business."
  2. **Agent Interaction**: User types/speaks: "I run a mobile dog grooming service in Seattle."
  3. **Generation State**: A translucent glass loading state shows real-time progress: "Drafting services...", "Configuring booking calendar...", "Setting up payment gateway...".
  4. **Agent Feed Presentation**: An actionable card appears in the Unified Agent Feed: "Your store is ready. Review and Publish."
  5. **Review & Action**: User taps the card, reviews the auto-generated service list (e.g., "Small Dog Groom", "Large Dog Groom"), and taps a large (44px+) "Approve & Go Live" button.

  ### AI Agent Integration Notes
  - The Onboarding Orchestrator Agent will utilize the existing `minimax.reason()` or Gemini Pro for intent classification and struct generation.
  - The Agent needs specific sub-skills: `DbSchemaGeneratorSkill`, `CatalogDraftingSkill`, and `PaymentConfigSkill`.
  - The generated output must be seamlessly pushed to the user's Unified Agent Feed (`AgentFeedRepository`).

  ## Implementation Prompt
  **User-Facing Outcome**: A fully autonomous onboarding flow where a user provides a single natural language description of their business on their mobile device, and OHC generates a complete store (catalog, booking capability, copy).
  **Critical User Journey (CUJ)**:
  1. User starts on the mobile onboarding screen (375px viewport).
  2. User inputs a short business description.
  3. The system processes the request via the Onboarding Orchestrator Agent.
  4. The user receives a notification/card in their Agent Feed.
  5. The user reviews the drafted store and clicks a single "Approve & Go Live" button.
  **Acceptance Criteria**:
  - The onboarding UI is entirely mobile-first, adhering to 375px width constraints and OHC Premium Tokens (Glassmorphism, 44px+ touch targets).
  - The backend agent successfully translates the natural language input into persisted multi-tenant database records (tenant setup, products/services).
  - A comprehensive E2E Playwright test (`bazel test //src/e2e:playwright`) covers this exact flow, starting from the single prompt to the final published store view.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
