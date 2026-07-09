issue_title: "Implement Zero-Click Onboarding Agent"
issue_description: |
  # Mission Brief: Zero-Click Onboarding Agent

  ## Problem Statement
  Small business owners and operators (e.g., Maya the Home Baker, Carlos the Field Service Owner) experience severe setup paralysis when adopting new platforms. Market research shows that 34% of small business owners abandon setup due to "technical complexity" when faced with traditional monoliths like Shopify. The initial blank canvas is terrifying for non-technical users. OHC aims to close the gap to competitors like Durable (which boasts a <1 minute AI generation) while retaining the deep operations coordination found in OHC's backend.

  ## Research Report
  - **Shopify (Sidekick)**: Chatbot assistant that acts as a manual/advisor but fails to autonomously configure complex setups (e.g., shipping zones, app integration). The "App Tax" fatigue forces SMBs to piece together disparate tools.
  - **Durable AI**: Generates sites in 30 seconds but lacks deep customizability and SEO, relying on simple list dashboards instead of deep commerce/booking data models.
  - **OHC Gap**: OHC requires fully native mobile onboarding (< 10 minutes) with an agent that *executes* state changes rather than just advising. The setup must bundle commerce and booking modules into a unified data schema out-of-the-box.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Landing/Auth Screen**: Simple Google OIDC / Email login.
  2. **The Prompt (Onboarding)**: Single chat interface. "What do you do?" User inputs a single sentence: "I'm a baker in Austin and I need to take custom cake deposits."
  3. **Generation State**: Shimmering translucent glass overlay indicating AI worker progress (Generating DB Schema -> Populating Catalog -> Building UI).
  4. **The Assistant Command Center (Home)**: The owner feed is populated. Agent suggests a next step (e.g., "Connect Stripe for custom deposits").

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Mobile Client (Flutter)
      participant API as Backend Service (Go/Rust)
      participant Orchestrator as KAIROS Orchestrator
      participant Agent as Onboarding Agent
      participant DB as Postgres (Tenant Data)

      Owner->>API: Submit Prompt ("I'm a baker...")
      API->>Orchestrator: Enqueue Onboarding Job
      Orchestrator->>Agent: Delegate to Departmental AI Worker
      Agent->>Agent: Extract Persona (Commerce + Booking)
      Agent->>DB: Provision Tenant & Schema (RLS)
      Agent->>DB: Create Seed Catalog & Services
      Agent->>DB: Configure Default Assistant Feed
      Orchestrator-->>API: Completion Event
      API-->>Owner: Stream UI Ready State
  ```

  ### Key Design Decisions
  - **Mobile First**: Flutter application ensures a native feel on a 375px viewport with native keyboard integration.
  - **Departmental AI Workers**: The Onboarding Agent must have permissions (via SPIFFE identity) to execute CRUD operations securely to provision the tenant environment.
  - **Multi-Tenant Isolation**: Ensure row-level security is correctly applied on generation.

  ### AI Agent Integration Points
  - **KAIROS Orchestrator**: Manages the prompt ingestion and dispatches a background job.
  - **LLM Prompting**: A system prompt optimized to convert natural language business descriptions into structured OHC data models (Offers, Services, Tasks).
  - **Visual Workflows**: Use parallel fan-out generation tasks (e.g., generating product photos while creating default business policies).

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your task is to implement the "Zero-Click Onboarding Agent" API and corresponding mobile-first UI wrapper.
  - **User Journey**: An unauthenticated user creates an account, lands on a minimalist 375px chat interface, inputs a single sentence describing their business, and within 30 seconds is redirected to a fully populated OHC command center.
  - **Acceptance Criteria**:
    - The API endpoint securely ingests the user's prompt and provisions a new tenant.
    - The backend uses an LLM to parse the prompt and populate the tenant's initial state (products, services, booking configuration).
    - The UI is mobile-first, utilizing translucent glass styling, and provides real-time loading feedback.
    - Zero mock data in the final state; all generated entries must be persisted real data in Postgres.
    - Playwright E2E tests must cover the entire onboarding journey from prompt to populated command center.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, onboarding]
assignees: []
