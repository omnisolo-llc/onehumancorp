issue_title: "Implement Zero-Click Agentic Mobile Onboarding & Blueprint Generation"
issue_description: |
  # Research Report: Agentic Autonomous Website Builders & SMB Platform Gap Analysis

  ## Track 1: Market Mapping & Competitor Discovery
  Competitors like Shopify dominate e-commerce but are notoriously complex for micro-SMEs, often leading to "setup paralysis." AI-native builders like Durable and 10Web offer fast setup but lack deep operational capabilities (inventory, POS, agentic inbox). OHC has a unique opportunity to merge zero-click setup with an autonomous agentic backend.

  ## Track 2 & 3: Deep-Dive & OHC Gap Identification
  - **Persona Focus:** Carlos (Handyman) and Maya (Baker) who run their businesses entirely from their phones.
  - **The Gap:** OHC currently lacks a fully native, mobile-first (375px) onboarding flow that bypasses traditional configuration forms. Users should not have to manually configure shipping zones, product schemas, or booking calendars before launching.
  - **The "Now What?" Syndrome:** After setting up, non-technical users need the AI to transition from "setup assistant" to "operational manager."

  ## Track 4: Architecture Design (Zero-Click Blueprint Generation)

  ### Architecture
  1. **Conversational Intake:** A single text input or voice prompt on mobile: "I'm a baker in Austin selling custom cakes."
  2. **LLM Blueprint Pipeline (Gemini Pro):**
     - Parses the intent and infers the business model (Physical Goods vs. Services).
     - Generates a `BusinessBlueprint` JSON object containing: Catalog schema, initial dummy inventory, booking availability (if applicable), and default policies.
  3. **Provisioning Engine (Rust Backend):**
     - Accepts the `BusinessBlueprint` and executes database migrations/seed data for the specific tenant within PostgreSQL.
     - Initializes the Agentic Departments (Operations, Ambassador, Promoter) with context specific to the blueprint.

  ### Mobile UX Flow (375px)
  1. **Welcome Screen:** "What do you do?" (Text box / Voice input).
  2. **Loading State:** Translucent glass spinner with text: "Agent drafting your catalog...", "Agent setting up your booking calendar...".
  3. **Review Screen:** A swipeable deck of generated assets (Storefront preview, AI-drafted welcome email, sample booking slot).
  4. **Action:** "Launch My Business" button.

  ## Implementation Prompt (For Engineering Swarm)
  **Feature Name:** Zero-Click Agentic Onboarding Blueprint
  **Target Persona:** Carlos (Handyman) who wants a service booking site.

  **Outcome:** Carlos inputs "I do home repairs in Miami" on his phone. The OHC backend generates a service catalog, a booking calendar schema, and an initial "Ambassador Agent" prompt without any manual configuration.

  **Critical User Journey (CUJ):**
  1. User opens the OHC mobile app (375px).
  2. User inputs a single sentence describing their business.
  3. The system calls the Gemini LLM to generate a `BusinessBlueprint`.
  4. The Provisioning Engine applies the blueprint to the tenant's database schema.
  5. The user is presented with a fully functional storefront and operational dashboard.

  **Next Actions:**
  - **Step 1:** Define the `BusinessBlueprint` Rust struct and JSON schema.
  - **Step 2:** Implement the LLM pipeline to generate the blueprint from a short text prompt.
  - **Step 3:** Build the mobile-first (375px) conversational intake UI and connect it to the provisioning backend.
  - **Step 4:** Write a Playwright E2E test to verify the end-to-end "Zero-Click" onboarding flow.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
