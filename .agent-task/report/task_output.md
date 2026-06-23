issue_title: "Agentic Zero-Click Onboarding & Business Context Engine"
issue_description: |
  ## Title
  Agentic Zero-Click Onboarding & Business Context Engine

  ## Problem Statement
  Small business owners and non-technical operators (like Maya the Baker or Carlos the Handyman) experience "Setup Paralysis" when adopting new platforms. Traditional platforms like Shopify or WooCommerce require navigating complex dashboards, configuring DNS, setting up shipping zones, and manually building product catalogs. This technical hurdle causes a massive drop-off rate (upwards of 34%) before the user ever publishes their site. Business owners want to sell their services or products immediately, not learn how to become systems administrators. The gap lies in moving from intent to an operational storefront without manual configuration.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify / WooCommerce:** Highly robust but extremely manual. Setting up a storefront takes hours or days. They offer reactive "wizards" or chatbots (like Shopify Sidekick), but these assistants wait for user instructions rather than autonomously building the context.
  - **Durable / 10Web:** AI-native platforms that generate websites in under 30 seconds using a few prompts. However, they lack deep e-commerce, complex booking, and back-office integration. They produce a static brochure rather than a functioning operational backend.
  - **Wix / Squarespace:** Provide AI onboarding quizzes that select templates, but still require extensive drag-and-drop customization and manual product entry.
  - **OHC Opportunity:** By introducing an "Agentic Zero-Click Onboarding Engine", OHC can ingest an owner's existing digital footprint (e.g., an Instagram handle, a few photos, or a quick voice memo) and autonomously synthesize a complete business context. This engine will build the storefront, configure local delivery/shipping zones based on location, populate a starter product catalog, and set up booking calendars—all in under 2 minutes. The owner just reviews and approves.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile App / Web UI] -->|Intent / Voice / Photo input| B(Onboarding Gateway)
      B --> C[Business Context Synthesis Engine]
      C --> D{KAIROS Orchestrator}
      D --> E[Operations Agent]
      D --> F[Marketing Agent]
      D --> G[Finance Agent]
      E -->|Setup DB| H[(Tenant Database)]
      F -->|Generate Site & Copy| I[Edge Storefront Configuration]
      G -->|Configure Stripe & Ledger| J[Payment Gateway Sync]
      D --> K[Push Notification: "Your Business is Ready"]
      K --> A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Screen 1: The Intake (Mobile)**: A single chat-like interface. Prompt: "Hi, I'm your OHC Assistant. What do you want to build today? (Tip: Just give me your Instagram handle or describe what you sell)."
  - **Screen 2: The Magic Loader**: A translucent glass loading card displaying real-time agentic actions (e.g., "The Operations Agent is configuring your delivery zones...", "The Marketing Agent is writing your product descriptions...").
  - **Screen 3: The Big Reveal**: The "Action Required: Approve Store" card. Shows a mobile preview of the fully functional storefront, a sample product, and a "Connect Stripe" button to start taking payments instantly.
  - **Visual Design**: Uses OHC Premium Token library with Apple/Ubiquiti-style hierarchy. Clean spacing, readable typography, and translucent materials.

  ### AI Agent Integration Points
  - **Business Context Synthesis Engine (LLM Layer)**: Uses Gemini Pro to extract business entity data (Name, Vibe, Products, Service Radius) from the user's initial unstructured input.
  - **Operations Agent**: Automatically provisions the PostgreSQL tenant, sets up shipping/delivery configuration, and populates the initial database schemas (Products, Services).
  - **Marketing Agent**: Generates SEO-optimized product descriptions, "About Us" copy, and configures the edge-cached storefront layout.

  ### Key Design Decisions
  - **Conversational to Operational**: Transitioning the user immediately from a natural language or media input into a fully provisioned backend. No intermediate forms or dropdowns.
  - **Deferred Complexity**: Advanced settings (DNS, tax overrides, specific shipping weight rules) are completely hidden behind an "Advanced Paths" toggle, keeping the initial experience radically simple.
  - **Approval-Based Workflow**: The AI executes the heavy lifting, but the owner retains agency by explicitly tapping "Approve & Publish" before anything goes live.

  ## Implementation Prompt
  **User-Facing Outcome:** As a non-technical business owner, I can open the OHC app, speak a 10-second description of my home baking business, and within 2 minutes receive a fully populated, ready-to-publish storefront with my first three product drafts and a localized delivery zone configured.
  **CUJ & Acceptance Criteria:**
  1. Create a natural language input endpoint that accepts a business description string.
  2. The Business Context Synthesis Engine must parse this string and return structured JSON defining the business name, category, and 3 inferred products/services.
  3. The KAIROS Orchestrator provisions the tenant data (products, basic delivery zone) in the database based on the structured JSON.
  4. The mobile-first UI (375px) displays the parsed results as an "Approve Store" review screen.
  5. Provide Playwright E2E tests: A user submits a business description, waits for processing, and verifies the pre-populated products and settings on the review screen.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
