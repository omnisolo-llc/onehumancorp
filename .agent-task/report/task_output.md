issue_title: "Implement Zero-Click Agentic Onboarding Flow"
issue_description: |
  ## Title
  Implement Zero-Click Agentic Onboarding Flow

  ## Problem Statement
  Small business owners (like Maya the Baker or Carlos the Handyman) suffer from "Setup Paralysis". Traditional platforms like Shopify or Wix require them to act as part-time web developers, copywriters, and system administrators, taking hours or days to configure a functional storefront. The current OHC onboarding process is still a manual, widget-based flow that takes ~1 hour. This friction violates our core promise that "anyone can launch and run a real small business from their phone or browser in under 10 minutes." We need a Zero-Click Onboarding Agent that uses conversational AI to extract business context and autonomously provisions the store, products, and operational settings in the background.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Durable & Mixo:** Generate complete business websites and CRMs in under 30 seconds from a single prompt. They excel at zero technical hurdle onboarding but lack deep operational capabilities (like POS and inventory) post-setup.
  - **Shopify & Squarespace:** Offer guided "AI" setup but still rely heavily on manual configuration of DNS, shipping zones, and product details. They feel like traditional software with an AI skin.
  - **OHC Opportunity:** By leveraging the KAIROS orchestration engine and The Ambassador Agent, OHC can combine the 30-second setup speed of Durable with the deep operational robustness of Shopify. The onboarding process becomes a friendly conversation ("What do you sell?", "Upload a photo of your best work") rather than a form-filling exercise.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Web/App 375px] -->|Chat Interface| B(Onboarding Gateway)
      B --> C[The Ambassador Agent]
      C -->|Natural Language Intent| D{KAIROS Orchestrator}
      D -->|Provision Tenant| E[Tenant DB Setup]
      D -->|Generate Theme| F[Theme Engine]
      D -->|Extract Products| G[Product Catalog Engine]
      D -->|Configure Ops| H[Operations Config]
      E --> I[Unified Storefront Ready]
      F --> I
      G --> I
      H --> I
  ```

  ### Mobile UX Flow & UI Wireframes (375px First)
  - **Screen 1 (The Greeting):** A clean, chat-like interface. The Ambassador Agent introduces itself. "Hi! I'm your OHC assistant. Let's get your business online. What's the name of your business and what do you do?" Native keyboard integration, large text input area.
  - **Screen 2 (The Upload):** The agent asks for a few photos. "Great! Upload a couple of photos of your cakes." A large touch target (44x44px min) for photo upload.
  - **Screen 3 (The Magic Moment):** A visually pleasing loading state (translucent glassmorphism) with dynamic text: "Analyzing photos... Drafting product descriptions... Setting up your calendar...".
  - **Screen 4 (The Reveal):** A preview of the generated storefront. A prominent "Publish" button and an "Edit with AI" button. No complex settings menus.

  ### AI Agent Integration Points
  - **The Ambassador Agent (Customer Success):** Acts as the friendly frontend conversationalist, guiding the user and extracting context.
  - **The Marketing Agent:** Takes the uploaded photos, enhances them, and generates SEO-optimized product descriptions and site copy.
  - **The Operations Agent:** Configures business hours, default shipping/pickup options, and booking availability based on the conversational context.

  ### Key Design Decisions
  - **Conversation over Forms:** Eliminate all traditional forms during onboarding. The user only interacts via chat and photo uploads.
  - **Progressive Disclosure:** Advanced settings (taxes, complex shipping zones) are entirely hidden during onboarding and handled autonomously or deferred until necessary.
  - **Mobile-First Chat UI:** The entire flow must feel as natural as texting a friend on a 375px screen.

  ## Implementation Prompt
  **User-Facing Outcome:** As a new business owner, I can open OHC on my phone, chat with an AI assistant for 3 minutes, upload two photos, and instantly receive a fully functional, published storefront with products, pricing, and booking configured.

  **CUJ & Acceptance Criteria:**
  1. A new user initiates the onboarding flow via the mobile UI.
  2. The user interacts with the chat interface, providing basic business details and uploading at least one photo.
  3. The system parses the conversation and autonomously creates a new Tenant record.
  4. The AI agents generate and save at least one product with a description and price based on the chat/photo.
  5. The system publishes the initial storefront and presents a preview to the user.
  6. Provide Playwright E2E tests: A new user completes the chat flow, the database verifies the created tenant and product, and the user successfully views the generated storefront preview.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
