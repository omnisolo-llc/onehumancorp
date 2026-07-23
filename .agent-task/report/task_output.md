issue_title: "Feature: Zero-Click Mobile-First Autonomous Storefront Onboarding Agent"
issue_description: |
  # Mission Queue Protocol: Zero-Click Autonomous Onboarding Agent

  ## Problem Statement
  Small business owners like Maya (the home baker) often suffer from setup paralysis. Traditional platforms like Shopify or Wix require users to manually configure themes, DNS, inventory databases, payment gateways, and shipping zones. This can take days and requires technical knowledge, leading to a 34% abandonment rate during onboarding. Maya doesn't want to build a website; she wants to sell custom cakes directly from her phone. We need a "Zero-to-One" flow that completely abstracts the setup process.

  ## Research Report
  Based on competitive analysis across the SMB e-commerce landscape:
  - **Traditional Builders (Shopify, Wix, Squarespace):** Focus heavily on providing the *tools* for a user to build a site, but demand high manual configuration. Shopify's "Sidekick" is mostly an advisory chatbot.
  - **AI Builders (Durable, Mixo, Framer):** Provide incredibly fast AI-generated sites based on a text prompt, but are often simplistic and lack robust operational backend systems (inventory, custom deposits, multi-channel POS).
  - **The OHC Opportunity:** OHC must bridge this gap by offering a fully autonomous onboarding agent. Instead of dropping the user onto a dashboard, OHC starts with a natural language conversation, autonomously generating the database schema, creating the storefront, setting up payment flows (e.g., Stripe Custom Deposits), and publishing the first product—all within 10 minutes and completely manageable from a 375px mobile screen.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile UI 375px] -->|Chat/Voice Prompt| B(Onboarding Agent)
      B --> C{Context & Intent Resolution}
      C --> D[KAIROS Orchestrator]
      D --> E[Stripe Integrations]
      D --> F[Catalog / Inventory API]
      D --> G[Storefront Delivery Network]
      E -->|Setup Payment Intents| H[(PostgreSQL Ledger)]
      F -->|Create Initial Products| H
      G -->|Generate Edge UI| I[Live Storefront URL]
      D --> J[Mobile Notification: 'Your Store is Live']
  ```

  ### Mobile UX Flow (375px First)
  1.  **Welcome Screen:** User downloads the app and enters the chat interface instead of a traditional form. "What kind of business are you building today?"
  2.  **Conversation:** Maya inputs: "I am a home baker in Austin selling custom cakes. I need to take a 50% deposit on orders."
  3.  **Autonomous Setup:** A loading screen showing the Onboarding Agent at work (e.g., "Configuring Stripe deposits...", "Generating cake product templates...").
  4.  **Review Card:** A glassmorphism Action Card appears in the Agent Feed: "Your bakery is ready. We've set up a custom cake product with a 50% deposit requirement. [Preview Store] [Launch]".
  5.  **Completion:** 1-tap approval publishes the store and provides a shareable link.

  ### AI Agent Integration Points
  -   **Onboarding Agent:** Acts as the primary interface for initial setup. It translates the user's natural language into structured operations (e.g., parsing "50% deposit" into a Stripe PaymentIntent requirement).
  -   **Promoter Agent (Handoff):** Once setup is complete, the Onboarding Agent hands off to the Promoter Agent to suggest the first Instagram post announcing the launch.

  ### Key Design Decisions
  -   **Conversational UI over Forms:** Eliminates the cognitive load of traditional onboarding flows.
  -   **Opinionated Defaults:** The agent makes smart assumptions (e.g., default shipping zones, standard policies) to prevent decision fatigue. The user can edit these later in "Advanced Settings".
  -   **Mobile-Native:** The entire onboarding process must be completable on a phone without ever needing a desktop browser.

  ## Implementation Prompt
  **User-Facing Outcome:** As a non-technical owner, I want to type a single sentence describing my business and have the app automatically build my store, configure payments, and add my first product, so I can start selling immediately from my phone.

  **CUJ (Critical User Journey):**
  1.  User opens the OHC mobile app (simulated 375px).
  2.  User inputs a prompt describing their business into the chat interface.
  3.  The backend Onboarding Agent parses the prompt, orchestrates the necessary backend service calls (Catalog, Settings, Integration/Stripe).
  4.  The Agent Feed updates with an "Action Required" card containing the generated store preview and a "Publish" button.
  5.  User taps "Publish" and receives the live storefront URL.

  **Acceptance Criteria:**
  -   Implement the `Onboarding Agent` capability using the LLM provider (Gemini/MiniMax).
  -   The agent must successfully orchestrate the creation of a tenant, a base product, and payment settings based on a text prompt.
  -   Develop the mobile-first (375px) chat and action card UI.
  -   Provide Playwright E2E tests verifying the complete Zero-Click Onboarding flow from login to published store link.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
