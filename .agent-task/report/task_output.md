issue_title: "Implement the OHC Setup Agent for Zero-Click Site Generation"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & OHC Gap Analysis

  ## Track 1: Market Mapping & Competitor Discovery
  Our research across the e-commerce platform landscape reveals two distinct categories:
  - **Traditional Legacy Giants:** Shopify, Wix, Squarespace, GoDaddy. These platforms provide tools to run a business but are highly complex to configure and require users to learn web development.
  - **AI-Native Emerging Players:** Durable, 10Web, Framer, Dorik. These platforms offer AI website generation but often lack deep operational integration (e.g., native booking, advanced POS sync) required to run a business smoothly.

  ## Track 2: Deep-Dive Competitor Audit - Shopify
  Shopify is incredibly powerful but overwhelmingly complex for micro-SMBs.
  - **The "App Tax":** Users often need 5-10 third-party apps to reach parity with basic modern expectations.
  - **Setup Paralysis:** The initial configuration flow is daunting and leads to high abandonment rates.
  - **AI Role:** Shopify Sidekick functions merely as an advisory chatbot rather than a proactive executor.

  ## Track 3: OHC Gap & Pain Point Identification
  - **Persona Focus:** Maya (Home Baker) & Carlos (Handyman). They need a platform that sets itself up.
  - **The Gap:** OHC currently lacks a zero-click onboarding flow that leverages AI to completely provision a storefront, database schema, and initial content from a single natural language prompt.

  ## Track 4: Architecture & Design Doc

  ### Data Model (PostgreSQL)
  - `Tenant`: Base configuration for the SMB.
  - `StorefrontConfig`: Themes, layout preferences, and AI-generated copy.
  - `Product`/`Service`: Autonomously generated initial catalog items.

  ### AI Agent Coordination (The "Setup Agent")
  - **Ingestion:** A simple natural language input ("I am a baker in Austin selling custom cakes").
  - **Processing:** The Setup Agent (LLM-driven) parses the prompt to infer business category, tone, target audience, and required operational modules (e.g., booking vs. e-commerce).
  - **Execution:**
    - Provisions a new tenant DB schema.
    - Selects an appropriate premium Glassmorphism theme.
    - Generates localized copy and initial product placeholders.
    - Sets up necessary agent connections (e.g., enabling the 'Ambassador Agent' if the business relies heavily on DMs).

  ### Mobile-First UX Flow
  1. User opens the OHC app (375px viewport).
  2. A single conversational input field asks for a brief business description.
  3. A dynamic loading screen shows the Setup Agent building the components.
  4. The user is presented with a fully functional, populated storefront and dashboard. They only need to connect their Stripe account to go live.

  ## Implementation Prompt (For Engineering Swarm)
  **Target Persona:** Carlos the Handyman
  **Outcome:** Carlos can launch his service listing site by simply typing "I am a handyman in Chicago doing drywall and plumbing." The AI sets up his booking calendar, pricing placeholders, and site layout autonomously.

  **Next Actions:**
  1. Implement the `Setup Agent` workflow capable of orchestrating tenant provisioning.
  2. Design the single-input mobile onboarding screen.
  3. Ensure the LLM pipeline reliably maps natural language inputs to structural OHC data models (Products/Services/Storefront configs).
  4. Build automated E2E tests for the zero-click generation flow.

  **Do NOT prescribe specific database schema details in the implementation phase.** Focus on the end-to-end user journey and the reliability of the AI-driven generation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
