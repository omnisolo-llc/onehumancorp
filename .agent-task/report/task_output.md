issue_title: "Implement the 'Zero-Click Onboarding' Architecture"
issue_description: |
  # Research Report: "Zero-Click Onboarding" Architecture

  ## Title
  Implement the "Zero-Click Onboarding" Architecture

  ## Problem Statement
  Small business owners face "Setup Paralysis". When they sign up for a traditional e-commerce or booking platform, they are confronted with complex menus, shipping zones, tax configurations, and an empty site that requires manual effort to build. Our persona, Maya (Home Baker), wants to sell cakes, not configure DNS. If we fail the "Grandmother Test" during onboarding, we lose the user before activation.

  ## Research Report
  Our competitive analysis indicates:
  - **Shopify:** Complex onboarding, taking up to days to configure fully.
  - **Durable.co:** Offers AI website generation in under a minute, capturing a segment of non-technical users.
  - **OHC Opportunity:** By leveraging the 'Marketing Agent' and the 'Operations Agent', we can create an onboarding flow where the user provides a single natural language description of their business. The system should then autonomously generate the storefront layout, product catalog (e.g., extracting items from text or photos), and configure basic booking/deposit logic.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App 375px] -->|Natural Language Prompt/Audio| B(Onboarding Gateway)
      B --> C[Orchestrator]
      C --> D{KAIROS Intent Router}
      D -->|Store Generation| E[Marketing Agent]
      D -->|Catalog Generation| F[Operations Agent]
      D -->|Settings Config| G[Finance Agent]
      E --> H[Storefront DB Schema]
      F --> I[Products/Services DB Schema]
      G --> J[Payments/Deposits Settings]
      H & I & J --> K[Published OHC Storefront]
      K -->|Push Notification| A
  ```

  ### Mobile UX Flow
  1. **Welcome Screen:** A simple input box: "Tell me about your business." with a microphone button for audio input.
  2. **Processing State:** While processing, display a loading screen indicating agent actions ("Building storefront...", "Adding 3 initial products...").
  3. **Activation Screen:** The owner receives a link to their live, fully populated storefront, pre-configured based on their description, with a single call to action: "Review & Launch."

  ### Key Design Decisions
  - **Single Prompt Input:** Reduce friction by asking only one open-ended question.
  - **Agentic Generation:** Use the existing AI agent infrastructure to infer business category, generate copy, create placeholder products (or use uploaded photos), and configure base settings (e.g., deposits for custom orders).
  - **Mobile-First:** The entire flow must be seamless on a 375px device.

  ## Implementation Prompt
  Implement a progressive onboarding wizard that accepts a natural language description (text or voice) of a small business. Route this input to the KAIROS orchestrator to generate a basic storefront layout, a catalog with at least 3 relevant products/services, and default payment settings (e.g., deposits for services). The flow must culminate in a functional storefront URL in under 60 seconds. Include a 375px optimized mobile UI and Playwright E2E tests verifying the complete Zero-Click Onboarding CUJ.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
