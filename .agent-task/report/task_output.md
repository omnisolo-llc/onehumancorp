issue_title: "[Research] Autonomous 10-Minute Onboarding Agent Architecture"
issue_description: |
  # Research Report: Autonomous 10-Minute Onboarding Agent Architecture

  ## 1. Problem Statement
  OneHumanCorp (OHC) aims to serve small business owners (like Maya the Baker or Carlos the Handyman) who abandon platform setup due to "technical complexity." Traditional platforms (Shopify, Wix) take hours or days to configure properly, requiring users to act as web developers and sysadmins. OHC's vision is a "Zero-Click Generation" flow where a single conversational prompt can autonomously provision the entire business setup (domain, database schema, product catalog, Stripe integration, and initial storefront layout) in under 10 minutes from a mobile device (375px viewport). Currently, OHC relies on a manual 1-hour setup process or a complex wizard.

  ## 2. Research Report
  - **Market Context:** Our internal research (`agentic_autonomous_website_builders_smb_platform_gap_analysis.md`) highlights that 73% of non-technical users abandon complex setups.
  - **Competitors:**
    - AI-Native builders like Durable.co generate sites in 30 seconds but lack deep commerce features.
    - Shopify Sidekick provides advice but requires manual execution of complex workflows.
  - **The OHC Differentiator:** OHC's "Zero-Click Onboarding Agent" must go beyond answering questions. It must actively execute state changes: creating tenants, setting up Stripe deposits, generating AI product photos, and configuring the `Agent Feed` directly from a chat interface.

  ## 3. Design Doc: High-Level Architecture

  ### Architecture Diagram
  ```mermaid
  graph TD
      User[User (Mobile 375px)] -->|Natural Language Prompt| FlutterApp[Flutter UI: Chat Interface]
      FlutterApp -->|gRPC / REST| OnboardingService[Go API: Onboarding Service]
      OnboardingService -->|Intent & Schema Generation| LLM[LLM Provider: Gemini Pro / OpenAI]
      OnboardingService -->|Provision Tenant| CoreDB[PostgreSQL (Row-Level Security)]
      OnboardingService -->|Dispatch Agents| AIQueue[AI Job Queue (Pg SKIP LOCKED)]
      AIQueue --> OpsAgent[Operations Agent: Setup Stripe & Settings]
      AIQueue --> MarketingAgent[Marketing Agent: Gen Product & Copy]
      OpsAgent -.-> CoreDB
      MarketingAgent -.-> CoreDB
      CoreDB -->|Sync State| FlutterApp
  ```

  ### Mobile UX Flow (375px First)
  1. **Splash/Entry:** Clean, translucent glass UI asking: "What kind of business do you run?"
  2. **Conversational Intake:** User responds (e.g., "I'm Maya, I sell custom vegan cakes in Austin via IG.").
  3. **Agentic Processing:** A loading state showing real-time agent tasks (e.g., "Drafting menu...", "Configuring payments...", "Building storefront..."). No complex form fields.
  4. **Review & Approval:** User is presented with a fully populated Agent Feed containing Action Cards to approve the generated products, policies, and Stripe connect link.
  5. **Launch:** One tap to publish the storefront.

  ### AI Agent Integration Points
  - **Intent Parsing:** LLM extracts business type (e.g., food & beverage), required features (e.g., custom orders, deposits), and persona (Maya).
  - **Zero-Shot Schema Provisioning:** The Onboarding Service translates the LLM intent into a standardized OHC tenant schema.
  - **Handoff:** The `OnboardingAgent` coordinates sub-agents (`Setup Agent`, `Marketing Agent`) via Redis distributed locks to avoid race conditions during DB population.

  ### Key Design Decisions
  - **No Forms:** Replace multi-step wizards with conversational and generative AI input.
  - **Execute, Don't Advise:** The LLM must be connected to internal CRUD APIs via a secure tool-execution protocol.
  - **Multi-Tenant Safety:** All generated data must strictly adhere to the `tenant_id` RLS policies in PostgreSQL.

  ## 4. Implementation Prompt
  **User-Facing Outcome:** Implement the "Zero-Click Onboarding" conversational flow. A user should be able to open the app, type "I'm Carlos, a handyman in Chicago doing emergency plumbing," and receive a fully functional OHC tenant pre-populated with 3 dummy services, a booking calendar, and an AI-drafted welcome email in their Agent Feed.

  **CUJ & Acceptance Criteria:**
  1. The Flutter UI provides a chat-based entry point for onboarding.
  2. The Go backend handles the prompt, calls the LLM, and orchestrates the creation of a new tenant, products/services, and settings without manual form inputs.
  3. The final state is the user landing on the Agent Feed, fully populated with their new business context.
  4. E2E Playwright tests must verify the flow from the chat input to the populated dashboard, ensuring no mock data is hardcoded in the UI.

  **Priority**: P0
  **Estimated Scope**: Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
