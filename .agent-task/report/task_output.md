issue_title: "[research] Build Mobile-First, AI-Assisted Unified Onboarding Flow"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## Problem Statement
  Small business owners (SMBs) struggle with the complexity of setting up an online presence. Existing platforms (Shopify, Wix, Squarespace) require users to act as part-time web developers, copywriters, and marketers. The onboarding process is often overwhelming, taking hours or days to complete, and is poorly optimized for mobile devices. Users face a blank canvas and are expected to understand complex settings, shipping zones, and plugin ecosystems.

  ## Research Report
  Our analysis of the market reveals two distinct categories:
  - **Traditional Builders (Shopify, Wix):** Powerful but complex. They offer tools, not solutions. Onboarding is a massive point of friction.
  - **AI-Native Builders (Durable, Mixo):** Fast setup but often shallow. They generate a landing page but lack deep e-commerce and operational integrations needed to actually run a business.

  **The OHC Opportunity:** OHC will bridge this gap by providing a "Zero-Setup" vision. We aim for sub-10-minute time-to-value. The key differentiator is moving from a manual configuration process to an autonomous execution model. The AI doesn't just advise; it builds the store, configures the database, and drafts the initial copy based on a simple conversational prompt.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App 375px] -->|Conversational Prompt| B(AI Setup Agent)
      B --> C{Intent & Context Resolution}
      C --> D[Generate DB Schema/Tenant Config]
      C --> E[Generate Product/Service Catalog]
      C --> F[Generate Storefront UI Layout]
      C --> G[Generate Initial Copy & SEO]
      D --> H(PostgreSQL Central Ledger)
      E --> H
      F --> I(OHC Platform Backend)
      G --> I
      H --> J[Live Storefront]
      I --> J
  ```

  ### Mobile UX Flow (375px)
  1. **Conversational Entry:** User opens the app and is greeted by a simple chat interface: "Tell me about your business in a few words (e.g., I'm a baker in Austin)."
  2. **Agent Processing:** A loading state ("Agent is building your store...") while the backend orchestrates the setup.
  3. **Review & Refine:** The app presents a preview card of the generated storefront, catalog, and settings.
  4. **Approval:** A single "Publish" button to take the store live.

  ### AI Agent Integration
  - The `Setup Agent` (powered by the LLM provider) receives the user prompt.
  - It uses structured outputs to generate JSON configurations for the tenant, initial products/services, and site layout.
  - It triggers internal APIs to provision the resources.

  ## Implementation Prompt
  **Feature Name:** OHC Zero-Click Agentic Onboarding

  **Target Persona:** Maya the Baker (relies on Instagram DMs, overwhelmed by complex e-commerce setups).

  **Outcome:** Maya can launch a fully functional OHC storefront (products, booking capabilities, and basic copy) simply by describing her business in a few sentences on her iPhone.

  **Critical User Journey (CUJ):**
  1. Maya downloads the OHC mobile app and signs up.
  2. Instead of a complex dashboard, she sees a conversational prompt: "Describe your business."
  3. She types: "I sell custom vegan cakes in Austin."
  4. The Setup Agent generates a complete store profile, including sample "Vegan Chocolate Cake" products, a booking calendar for custom orders, and localized SEO copy.
  5. Maya reviews the generated preview and taps "Launch Store".

  **Acceptance Criteria:**
  - Must provide a fully mobile-first (375px) conversational onboarding interface.
  - Must use an AI agent to parse the input and generate actionable configuration data.
  - Must automatically provision the necessary database records (tenant, products, services) without manual user intervention.
  - Must include E2E tests verifying the flow from prompt to a published (or previewable) store state.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
