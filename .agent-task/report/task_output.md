issue_title: "Implement Zero-Click Agentic Mobile Onboarding Flow"
issue_description: |
  **Problem Statement**
  Currently, OHC requires manual configuration of services, products, and profile settings during setup. This traditional flow takes nearly an hour and creates "setup paralysis." Our target persona, Maya (Home Baker), finds this overwhelming—she wants to sell cakes, not configure complex software. Data indicates that 73% of non-technical users abandon complex setups, and 34% drop off purely due to "technical complexity" when trying to launch their digital presence. OHC must guide users from an unclear work state to a clear next action in minutes via an AI-driven, zero-click onboarding flow.

  **Research Report**
  A competitive audit of the SMB platform market reveals a sharp divide:
  - **Traditional Giants (Shopify, Wix, Squarespace):** Focus on manual configuration. Shopify's setup is comprehensive but complex. Its AI (Sidekick) acts as an advisory chatbot rather than an execution engine, leading to onboarding fatigue (often 30-60 minutes).
  - **AI-Native Competitors (Durable, 10Web, Mixo):** Focus on instant generation. Durable can generate a website, CRM, and invoicing setup in under 30 seconds based on a simple prompt. However, they lack deep operational tools.

  **Gap to Close:** OHC has powerful background agents but lacks an initial autonomous generation experience. By deploying a "Zero-Click Onboarding Agent," OHC can bridge the gap between Durable's rapid setup and Shopify's operational depth, targeting a < 10-minute setup time natively on mobile.

  **Design Doc**

  *Architecture Diagram*
  ```mermaid
  sequenceDiagram
      actor Owner
      participant MobileUI as OHC Mobile App (Flutter)
      participant OnboardingAgent as Zero-Click Agent
      participant KAIROS as Orchestration Engine
      participant DB as Multi-Tenant Database

      Owner->>MobileUI: Submits natural language prompt ("I am a home baker in Austin")
      MobileUI->>OnboardingAgent: Trigger Onboarding Mission
      OnboardingAgent->>KAIROS: Request capability provisioning
      KAIROS->>DB: Provision DB Schema & Tenant Isolation
      KAIROS->>DB: Seed initial catalog & Stripe config
      KAIROS-->>OnboardingAgent: Confirmation
      OnboardingAgent-->>MobileUI: Return generated mobile storefront & dashboard
      MobileUI-->>Owner: Display personalized work feed & first recommended action
  ```

  *Mobile UX Flow (375px First)*
  1. **Welcome Screen:** Clean, distraction-free screen with a single conversational input field and a native keyboard. "Tell us about your business in one sentence."
  2. **Generation Screen:** Engaging loading state displaying translucent glass cards that pop in as the agent builds the database, products, and booking system in the background.
  3. **Assistant-First Shell:** The user lands directly on the "Today" work feed. The system is fully configured with sample products, an auto-generated booking link, and a suggested next action (e.g., "Review your new Cake Menu").

  *AI Agent Integration Points*
  - **Onboarding Agent (Gemini Pro/GPT-4o):** Interprets the natural language prompt, determines the business type (e.g., physical vs. booking), and emits structured setup commands to the backend.
  - **Operations Agent:** Immediately starts monitoring the generated environment to create initial actionable tasks for the owner.

  *Key Design Decisions*
  - **Mobile-First Conversational Input:** Eliminates complex web forms.
  - **Agentic Execution, Not Advice:** The AI doesn't just suggest a schema; it actively provisions the database and creates mock product listings.
  - **Immediate Usability:** The user lands on a personalized feed instead of a blank dashboard.

  **Implementation Prompt**
  "Implement a Zero-Click Onboarding flow for new tenants. The user should be greeted with a single text input on mobile (375px targeted). When they submit a description of their business, the backend should use the LLM to autonomously configure their tenant workspace, including generating a basic product catalog or service booking setup based on their business type. The user must land on a populated, functional work feed within a few seconds, with no manual form-filling required beyond the initial prompt. Ensure the UI uses the OHC Premium Token library with translucent materials. All network calls must handle transient failures gracefully."

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
