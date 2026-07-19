issue_title: "Implement Autonomous Zero-Click Onboarding Agent"
issue_description: |
  ## Mission Queue Protocol

  **Problem Statement:**
  Small business owners (like our persona Maya, the home baker) experience "setup paralysis" when trying to transition their business online. Our market research indicates that 34% of small business owners abandon setup due to technical complexity (e.g., DNS configuration, Stripe setup, catalog creation). Currently, OHC requires a manual, multi-step process (~1 hour) to configure services, storefronts, and payment gateways. Owners need a "Zero-Click" onboarding experience that transforms natural language (e.g., "I want to sell custom vegan cakes") into a fully provisioned storefront and business configuration instantly.

  **Research Report:**
  We mapped the 2025 landscape of owner/operator work assistants.
  - **Traditional Giants** like Shopify (Sidekick) and HubSpot (Breeze) offer proactive agents but still require significant manual configuration of core settings and integrations. Setup takes hours or days.
  - **AI-Native Competitors** like Durable.co offer a "30-Second Setup" generating a complete business website, CRM, and invoicing via a single prompt. This zero-technical-hurdle approach is highly appealing to service providers and non-technical owners.
  - **OHC Gap:** While OHC possesses robust orchestration capabilities (KAIROS) and specialized services (booking, quoting, pos), it lacks the seamless, autonomous "Zero-to-One" onboarding flow. OHC's goal is <10 minutes of agent-led setup.

  **Design Doc:**
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    graph TD;
        Owner[Owner (Maya)] -->|Natural Language Input| MobileUI[Mobile App 375px];
        MobileUI -->|Submit Prompt| OnboardingAgent[Zero-Click Onboarding Agent];
        OnboardingAgent --> KAIROSEngine[KAIROS Orchestration Engine];

        KAIROSEngine -->|Provision| DomainService[Domain & DNS Config];
        KAIROSEngine -->|Configure| PaymentIntegration[Stripe / Payments];
        KAIROSEngine -->|Generate| CatalogService[Product/Service Catalog];
        KAIROSEngine -->|Initialize| TenantConfig[Tenant DB & Settings];

        DomainService --> ReadyState[Ready-to-Sell State];
        PaymentIntegration --> ReadyState;
        CatalogService --> ReadyState;
        TenantConfig --> ReadyState;

        ReadyState -->|Confirmation & Link| MobileUI;
    ```
  - **Mobile UX Flow (375px first):**
    1. **Welcome Screen:** A clean, translucent glass UI card greeting the owner. Single text input field: "Tell me about the business you want to run."
    2. **Processing State:** A dynamic loading animation with engaging text ("Setting up your storefront...", "Configuring payments...", "Generating your first product...").
    3. **Success Screen:** A celebratory card displaying the new live URL, a generated sample product (e.g., "Vegan Custom Cake"), and a clear primary action button ("View Store" or "Share Link").
  - **AI Agent Integration Points:**
    - A new specialized agent (`OnboardingAgent`) managed by KAIROS.
    - Deep integration with the multi-tenant configuration API and third-party provisioning APIs (Stripe, Domain registrars).
  - **Key Design Decisions:**
    - **Conversational Interface:** Replace complex forms with a single natural language prompt.
    - **Agent-Driven Provisioning:** The agent executes a series of backend tasks (catalog generation, payment setup) in the background without user intervention.
    - **Immediate Value:** The user receives a functional, shareable link within minutes.

  **Implementation Prompt:**
  As an Implementer agent, build the "Zero-Click Onboarding Agent" feature.
  1. Implement the UI flow for mobile (375px) starting from a single text input prompt.
  2. Create the backend `OnboardingAgent` that interprets the prompt, generates initial business metadata, and provisions a basic product/service catalog and tenant settings.
  3. Ensure the onboarding process results in a verifiable "Ready-to-Sell" state with a shareable link.
  4. Ensure all changes follow the macOS Translucent Glass and UniFi layout design tokens.
  5. The Critical User Journey (CUJ) is: Owner inputs "I sell custom cakes" -> Agent processes -> Owner sees a live link and a sample product generated in their catalog.
  6. Add comprehensive Playwright E2E tests validating this exact flow.

  **Priority:** P0 (Critical)
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []