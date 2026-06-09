issue_title: "Architecture & Capability Design: Universal Agentic Product Generation (Zero-Click Onboarding)"
issue_description: |
  # Mission Queue Protocol: Universal Agentic Product Generation (Zero-Click Onboarding)

  ## Problem Statement
  Non-technical owners (e.g., Maya the Baker, Carlos the Handyman) experience extreme setup paralysis when launching a new business online. The industry standard (Shopify, Wix) presents a blank canvas or a generic template, forcing the user to manually configure databases, create product variants, upload images, write descriptions, and set up pricing. This "blank canvas" friction leads to high abandonment rates. While AI chat tools exist (like Shopify Sidekick), they are advisory rather than executable. OHC needs a "Zero-Click Generation" flow where a single natural language prompt autonomously provisions the entire business state.

  ## Research Report
  - **Market Context**: Competitors like Durable or 10Web offer "AI website builders," but they mostly generate front-end HTML/templates. They do not deeply integrate commerce backends, dynamic inventory, or complex variant structures natively.
  - **The OHC Differentiator**: OHC's architecture must go beyond visual generation. The onboarding prompt must trigger a coordinated effort among AI Agents (Operations, Marketing) to dynamically generate the correct PostgreSQL data schemas, populate realistic product catalogs (with variants and stock), establish pricing models, and draft storefront copy—all before the user clicks a second button.
  - **Persona Fit**: Maya types "I sell custom vegan cakes in Austin." OHC instantly generates a product catalog for "6-inch Vegan Chocolate", "8-inch Vegan Vanilla", sets up deposit payment rules for custom orders, and drafts an Instagram-ready bio.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as User (Mobile UI)
      participant Gateway as API Gateway (gRPC/REST)
      participant Onboarding as Onboarding Service
      participant Swarm as AI Job Queue (PostgreSQL SKIP LOCKED)
      participant Architect as System Architect Agent
      participant Ops as Operations Agent
      participant DB as Central Ledger (PostgreSQL)

      Owner->>Gateway: POST /onboard "I sell custom vegan cakes..."
      Gateway->>Onboarding: Initiate Generation
      Onboarding->>Swarm: Enqueue Zero-Click Task
      Swarm->>Architect: Dequeue Task
      Architect->>Architect: Parse Intent & Infer Business Model
      Architect->>Ops: Delegate Catalog & Schema Generation
      Ops->>DB: INSERT Products, Variants, Pricing (RLS Isolated)
      Ops->>DB: INSERT Storefront Configuration
      Ops-->>Architect: Generation Complete
      Architect-->>Onboarding: State Ready
      Onboarding-->>Gateway: Return Success & Preview URL
      Gateway-->>Owner: Redirect to populated Dashboard
  ```

  ### Mobile UX Flow (375px)
  1. **The Single Input Screen**: A clean, distraction-free screen with a single large text area: "Tell us about your business in a sentence."
  2. **The Loading State**: A pulsing, premium translucent glass animation showing agent progress (e.g., "Operations Manager is stocking your shelves...", "Marketing Agent is writing your bio...").
  3. **The Reveal**: The user lands on a fully populated Dashboard. The Operations Agent shows a summary card: "I've set up 4 initial products based on your prompt. Tap to edit or approve."

  ### AI Agent Integration Points
  - **System Architect Agent (New Capability)**: Orchestrates the generation pipeline, deciding if the business needs physical inventory (products) or calendar bookings (services).
  - **Operations Agent**: Executes the database writes, generating realistic default products, prices, and variant structures based on the LLM's world knowledge of the business type.

  ### Key Design Decisions
  - **Strict Multi-Tenancy**: All generated data must be rigorously scoped to the newly created `tenant_id` using PostgreSQL Row-Level Security (RLS) to ensure Zero-Trust isolation.
  - **Real Data, Not Mocks**: The generated data must be real, persisted records in the database, not frontend mock states. The user must be able to immediately edit these records.

  ## Implementation Prompt
  **Outcome**: Implement the backend service and AI swarm coordination for the "Zero-Click Generation" onboarding flow. When a user provides a business description, the system must autonomously generate a realistic product catalog (including variants) and save it to the database.

  **Acceptance Criteria**:
  1. Create a new API endpoint (e.g., `POST /api/onboarding/generate`) that accepts a text prompt.
  2. Implement an AI worker flow that uses an LLM (Gemini Pro) to parse the prompt, infer at least 3 relevant products with appropriate pricing and variants.
  3. The worker must persist these products into the PostgreSQL database under the user's `tenant_id`.
  4. Ensure strict multi-tenant isolation (RLS) is maintained during generation.
  5. The mobile-first frontend must provide a smooth UX flow from prompt entry to viewing the populated catalog.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
