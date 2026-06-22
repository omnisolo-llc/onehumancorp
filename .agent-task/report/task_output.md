issue_title: "Implement 'Zero-Click Generation' Mobile-First Storefront Builder Onboarding"
issue_description: |
  ## Problem Statement
  Small business owners (SMBs) like Maya the Baker or Carlos the Handyman experience "setup paralysis" when faced with a blank canvas or complex multi-step onboarding forms found in traditional platforms (Shopify, Wix). Over 70% of non-technical users abandon complex setups. They need a system that acts rather than advises. Existing solutions also require desktop computers for meaningful store creation, alienating mobile-first users. The goal is to provide a fully native, mobile-first onboarding where a single conversational prompt autonomously generates the DB schema, product catalog, and storefront layout.

  ## Research Report
  - **Shopify/Wix:** Rely on complex, multi-page configuration forms and desktop-first editors. Their AI tools (like Sidekick) often function as advisory chatbots rather than executing agents that build the store end-to-end.
  - **AI-Native Competitors (Durable, Mixo):** Fast generation but often result in superficial landing pages lacking deep integration with bookings, physical inventory, and omnichannel commerce.
  - **OHC Opportunity:** OHC differentiates by offering a true "Zero-Click Generation" flow. By leveraging the `OnboardingAgent`, OHC can parse a single natural language input ("I'm a baker in Austin") to intelligently construct a complete business structure (`IntakeData`), including products, pricing models, and website templates, instantly rendering a functional, mobile-optimized storefront. This bridges the gap between AI generation speed and full-platform commerce capabilities.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      User([Business Owner - Mobile]) --> |Natural Language Prompt| Frontend(Flutter + PWA Mobile App)
      Frontend --> |POST /api/v1/builder/zero-click-builder/generate| API[Growth API]

      API --> Agent[Onboarding Agent]
      Agent <--> |Prompt + Schema| LLM(Gemini / Minimax)
      LLM --> |Structured JSON IntakeData| Agent

      Agent --> |Provision Tenant & Data| DB[(PostgreSQL)]

      API --> |Response: Auth & Redirect| Frontend
      Frontend --> |Render Live Preview| LivePreview[Live Storefront Preview 375px]
  ```

  ### Mobile UX Flow (375px First)
  1. **Landing Screen (375px):** A clean, premium glassmorphism interface with a single large text area prompting: "Describe your business in a few sentences."
  2. **Input:** User types or uses voice dictation (e.g., "I run a custom cake shop in Austin. I need a gallery and custom order forms.").
  3. **Action:** User taps a massive, touch-friendly (>44px) "Generate My Store" primary button.
  4. **Processing State:** A vibrant, reassuring loading animation indicates the AI is building the catalog, configuring settings, and designing the layout.
  5. **Completion & Handoff:** The backend provisions the tenant and returns auth tokens. The app seamlessly transitions to the `LivePreview` of the newly generated storefront, ready for minor block-based tweaks.

  ### AI Agent Integration Points
  - **Onboarding Agent (`The Promoter`):** Receives the initial text prompt. Utilizes LLMs to extract `business_name`, `business_type`, `categories`, and generates `initial_products` with smart defaults (e.g., inferring a $10.00 base price or typical service offerings).

  ### Key Design Decisions
  - **Single Input Field:** Drastically reduces cognitive load compared to multi-step wizards.
  - **Mobile-First Execution:** Entire flow is designed for a 375px viewport without horizontal scrolling or tiny touch targets.
  - **Premium UI Tokens:** Heavy reliance on translucent glass materials (`backdrop-filter: blur(20px) saturate(200%)`) and rounded corners to convey a modern, native app feel.

  ## Implementation Prompt
  Implement the "Zero-Click Generation" onboarding flow in the mobile-first Flutter frontend.
  1. Create a new onboarding screen targeting a 375px width. Use OHC Premium Design Tokens (Glassmorphism, rounded corners).
  2. The screen should contain a single large text input for the user's prompt and a "Generate Store" button (minimum 44x44px touch target).
  3. On submit, call the existing `POST /api/v1/builder/zero-click-builder/generate` endpoint with the prompt.
  4. Implement a visually engaging loading state.
  5. Upon success, handle the returned `ZeroClickGenerateResponse` (which includes `organization_id` and `user_id`) to authenticate the user and navigate them to their newly generated storefront dashboard or live preview.
  6. Ensure all UI components are fully operable on mobile and pass interaction verification.
  7. Provide Playwright E2E tests covering this exact Critical User Journey (CUJ).

  **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []