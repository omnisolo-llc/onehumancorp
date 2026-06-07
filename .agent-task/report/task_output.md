issue_title: "Implement Agentic Autonomous Storefront Builder"
issue_description: |
  # Mission Queue Protocol: Agentic Autonomous Storefront Builder

  ## Problem Statement
  Small business owners (like Maya the baker and Carlos the handyman) suffer from "Initial Setup Paralysis" when trying to build their digital presence. Traditional platforms (Shopify, Wix) require users to act as part-time web designers, content writers, and IT administrators. Owners stare at blank templates and do not know what to write, what images to use, or how to structure their services. This is the #1 pain point (28%) holding back SMBs from launching their online business.

  ## Research Report
  Based on the competitive analysis against Shopify, Wix, Squarespace, and AI-native builders (Durable, 10Web):
  - Current SMB platforms fail because they provide *tools* instead of *outcomes*.
  - Generative AI builders (like Durable) prove that a 30-second setup is possible, but they often lack deep business operations (bookings, inventory, tap-to-pay) behind the generated site.
  - OHC's differentiation: Combine zero-setup generative storefronts with full-stack business operations. The owner simply chats with the OHC Assistant ("I sell custom vegan cakes in Brooklyn"), and the assistant autonomously generates the storefront, populates initial products, and configures the booking/deposit system.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      Owner[Owner / Operator] -->|Natural Language Prompt| OHC_Chat[OHC Assistant Chat UI]
      OHC_Chat --> Work_Triage_Agent[Work Triage Agent]
      Work_Triage_Agent --> Storefront_Agent[Storefront Generator Agent]
      Storefront_Agent -->|Generates JSON Schema| Theme_Engine[Dynamic Theme Engine]
      Storefront_Agent -->|Generates Initial Data| Product_DB[(Product & Services DB)]
      Theme_Engine --> Mobile_UI[Mobile-First Edge-Cached Storefront]
      Product_DB --> Mobile_UI
  ```

  ### Mobile UX Flow (375px First)
  1. **Onboarding Chat**: User opens the app. No complex forms. A single chat interface: "What kind of business are you running?"
  2. **Generation State**: Translucent glass loading screen showing real-time agent progress ("Drafting services...", "Selecting layouts...", "Writing copy...").
  3. **Preview & Edit**: A fully functional 375px preview of the generated storefront.
  4. **Approval**: A sticky bottom action button (44x44px minimum touch target): "Publish" or "Tweak it". If "Tweak it", the user just texts the agent ("Make the colors warmer").

  ### AI Agent Integration Points
  - **Storefront Generator Agent**: A specialized capability prompted to map natural language to OHC's internal `StorefrontConfig` and `Product` schemas.
  - **Memory/Context**: The agent saves the generated business context (brand voice, target audience) into the tenant's episodic memory for future interactions.
  - **Theme Engine**: A Flutter/Web widget system that safely renders the agent-generated JSON schema into beautiful, accessible UI components.

  ### Key Design Decisions
  - **Zero-Code Interface**: No drag-and-drop. The primary editing interface is natural language chat with the AI assistant.
  - **Schema-Driven UI**: The agent does not write code. It generates a strict JSON schema that the frontend safely interprets, ensuring high performance, accessibility, and zero chance of broken layouts.
  - **Immediate Usability**: The generated storefront is immediately wired to OHC's payment and scheduling backend.

  ## Implementation Prompt
  **Outcome**: Build the "Agentic Autonomous Storefront Builder" flow in the Tauri/Flutter frontend and the Rust backend.
  **CUJ**: A new owner signs up, describes their business in one sentence, and within 30 seconds, has a published storefront with sample products/services and a working contact form.
  **Acceptance Criteria**:
  - The UI must feature a conversational onboarding flow, completely replacing manual form-based setup.
  - The backend must provide a new AI capability/agent that takes the business description and returns a structured `StorefrontConfig` and seed products.
  - The generated UI must strictly follow OHC's mobile-first (375px) and translucent glass design system.
  - Playwright E2E tests must verify the entire flow from chat prompt to published storefront visibility without mocking the frontend-backend connection.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
