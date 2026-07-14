issue_title: "[architecture] Agentic Zero-Click Onboarding and Unified Mobile Agent Feed"
issue_description: |
  ## Problem Statement
  Small business owners, such as Maya (baker) and Carlos (handyman), abandon traditional SaaS setups like Shopify or Wix due to "blank canvas syndrome" and extreme configuration complexity. Furthermore, current platforms require a desktop computer for initial setup and advanced store management. Non-technical owners need an assistant that operates purely from their 375px mobile screen, translating natural language intents into fully configured business operations (e.g., website creation, DB schema, Stripe integration, bookings) in under 10 minutes without manuals.

  ## Research Report
  ### Competitive Mapping & Gap Analysis
  - **Legacy Monoliths (Shopify, Wix, Squarespace)**: Provide vast customization but have steep learning curves (30-60 minutes setup). Their mobile apps act as companion dashboards for metrics and basic fulfillment, forcing users to a desktop browser for initial store setup and complex marketing rules. Their AI offerings (e.g., Shopify Sidekick) are largely advisory chatbots, not autonomous executors.
  - **AI-Native Gen Tools (Durable, Mixo, 10Web)**: Deliver 30-second website generation but lack deep operational backends (e.g., native booking, complex variants, custom deposits). They solve the "initial build" but not the "daily operations."
  - **Mobile-First Creators (Linktree, Stan Store)**: High mobile usability but extremely limited in operational capabilities (no agentic workflows, complex shipping).

  ### The OHC Gap
  OHC currently has the backend services for bookings, POS, and quoting but lacks the autonomous "Zero-to-One" onboarding and an intent-driven mobile operations UI. SMBs want an AI that **executes** rather than just advises.

  ## Design Doc

  ### 1. Architecture Design (Zero-Click Onboarding & Feed)

  The solution requires two major components:
  1. **Agentic Onboarding Engine**: A conversational interface that parses natural language ("I am a baker in Austin doing custom vegan cakes") to generate multi-tenant DB schemas, product catalogs, and Stripe deposit links.
  2. **Unified Agent Feed**: Replaces the traditional hamburger menu with a vertical feed of "Action Cards" on a 375px viewport.

  ```mermaid
  graph TD;
      User[Mobile User 375px] -->|Natural Language Prompt| IntentLayer[Intent & Context Resolution LLM];
      IntentLayer -->|Provisioning Command| BuilderAgent[Operations & Builder Agent];
      IntentLayer -->|State Change| DB[(Multi-Tenant PostgreSQL RLS)];

      BuilderAgent -->|Configure Stripe| StripeService[Stripe API Integration];
      BuilderAgent -->|Generate Site| StorefrontService[Edge-Cached Storefront];

      Webhook[External Webhooks/Events] --> EventBus[Redis Pub/Sub];
      EventBus --> AgentFeedEngine[Agent Feed Engine];
      AgentFeedEngine -->|Generate Action Card| FeedUI[Unified Mobile Feed UI];

      FeedUI -->|Approve Card| IntentLayer;
  ```

  ### 2. Mobile UX Flow (375px First)
  1. **Onboarding**: The app opens to a chat-like interface. User inputs their business description. The Operations Agent responds, "Building your store, setting up deposits, and configuring your menu."
  2. **The Unified Feed**: Post-onboarding, the home screen is a vertical, scrollable feed of Action Cards using OHC Premium Tokens (Glassmorphism, 44x44px touch targets).
  3. **Card Interaction**:
      - A card appears: "Maya, 3 new Instagram DMs asking about custom cakes. Draft responses ready."
      - User taps the card to expand.
      - User taps the massive "Approve & Send" button to execute the agentic workflow. No horizontal scrolling is permitted.

  ### 3. AI Agent Integration Points
  - **Operations & Marketing Agent**: Handles zero-click provisioning of the user's workspace, applying default DB configurations and deploying the storefront.
  - **Agent Feed Engine**: Hooks into the backend event bus (PostgreSQL SKIP LOCKED / Redis Pub/Sub) to generate contextual LLM drafts that are serialized into UI Action Cards.

  ## Implementation Prompt
  **Objective:** Implement the Mobile-First Unified Agent Feed UI and its backend data provider.
  **User-Facing Outcome:** When a business owner opens the Flutter app or PWA on a phone, they see a clean, unified vertical feed of actionable cards (e.g., "Drafted reply for Carlos", "Inventory low on Vegan Cakes - Order more?").
  **Critical User Journey (CUJ):**
  1. User logs in on a 375px viewport.
  2. User is presented with a vertical feed of Agent Action Cards (No traditional dashboard charts by default).
  3. User taps a card detailing an AI-generated proposal (e.g., "Approve 20% discount on summer dresses").
  4. User taps a large, 44x44px minimal "Approve" button.
  5. The system successfully dispatches the approval back to the AI Job Queue and updates the feed optimistically.
  **Acceptance Criteria:**
  - The UI strictly adheres to 375px width constraints.
  - Employs OHC Premium Tokens (Translucent Glass materials, clear typography).
  - The backend provides an endpoint to fetch pending Action Cards for the authenticated tenant.
  - E2E Playwright tests must verify clicking "Approve" on a card successfully processes the action.
  - Zero mock data in the final implementation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
