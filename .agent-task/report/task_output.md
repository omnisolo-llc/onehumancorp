issue_title: "Implement the Unified Agent Feed for Mobile Operations MVP"
issue_description: |
  ## Issue Brief: Unified Agent Feed (Mobile Operations MVP)

  ### 1. Problem Statement
  Legacy e-commerce platforms like Shopify and Wix require users to operate from a complex, desktop-focused dashboard. Our small business owners (like Maya the baker and Carlos the handyman) operate their businesses primarily from 375px mobile screens. They experience significant pain dealing with multiple apps (booking, payments, marketing) and find the standard "admin panel" overwhelming. They do not want tools to figure out how to do their tasks; they want an assistant to take action and present a simple approval flow. The critical gap is the lack of a mobile-first, consolidated interface that coordinates work and pushes actionable notifications—essentially making the complex simple through an "Approval Interface."

  ### 2. Research Report
  Based on the `ohc_smb_mobile_first_design_research.md` and `agentic_autonomous_website_builders_smb_platform_gap_analysis.md`, the rise of mobile-first creators using Link-in-Bio tools proves the value of a single, simple mobile interface. However, these lack robust backend operations. Shopify excels at operations but its mobile app primarily serves to check metrics and fulfill orders, rather than build and automate workflows without leaving the phone.

  Current OHC capabilities must differentiate by shifting from "Manual Configuration" to "Autonomous Execution." The ideal paradigm is the **Agent Feed**, where various OHC AI departments (Operations, Customer Support, Marketing) process events (e.g., a DM asking for product availability, or an abandoned cart) and proactively generate "Action Cards" for the user to review and approve, all strictly adhering to a 375px viewport limit with large touch targets.

  ### 3. Design Doc

  #### 3.1 Architecture Overview
  The architecture relies on an event-driven pipeline where webhooks or internal state changes trigger classification by the LLM layer. The LLM uses tenant-scoped memory (RAG) to draft context-aware actions, which are then queued as Action Cards in the Agent Feed.

  ```mermaid
  graph TD;
      Event[Webhook / State Change] --> Ingestion[Event Pipeline (Redis/Kafka)];
      Ingestion --> Classifier[Intent & Context Resolution (LLM)];
      Classifier --> RAG[(Tenant Memory / Knowledge)];
      RAG -.-> Classifier;
      Classifier --> ActionGenerator[Draft Proposed Action];
      ActionGenerator --> Feed[Unified Agent Feed];
      Feed --> UX[Mobile UI - 375px];
      UX -- "User Taps 'Approve'" --> Executor[Background Task Queue];
      Executor --> StateUpdate[Database / External API Update];
  ```

  #### 3.2 Mobile UX Flow (375px First)
  - **Screen 1 (Home/Feed)**: Vertical scroll of "Action Cards." Each card has a subtle icon indicating the Agent department (e.g., Marketing, Operations).
  - **Interaction**: The user sees a card: "Drafted reply for new DM inquiry about vegan cakes."
  - **Screen 2 (Card Detail - Optional)**: User taps the card to expand and review the drafted message or proposed action.
  - **Action**: A prominent `Approve` button (touch target >= 44x44px) sits at the bottom.
  - **Visuals**: Uses OHC Premium Tokens (translucent glassmorphism, clean typography, soft background blur). No developer jargon or complex settings are visible.

  #### 3.3 AI Agent Integration Points
  - **Intent Classification Pipeline**: Hooked into existing `server/workers` to intercept events and request LLM classification.
  - **Drafting Protocol**: Agents (e.g., `src/agents/builtin`) will output structured JSON representing the proposed action and UI copy.
  - **Approval Pipeline**: An explicit API route to consume the "Approve" action, transitioning the state of the card and executing the associated background task.

  ### 4. Implementation Prompt
  **Role**: Implementer Agent
  **Objective**: Build the foundational UI and backend API for the "Unified Agent Feed" ensuring strict 375px mobile compatibility.

  **Critical User Journey (CUJ)**:
  1. As a business owner (e.g., Maya), log in to the OHC app.
  2. The home screen displays a vertical feed of at least two mocked Action Cards (e.g., "Operations: Fulfill 3 new orders", "Customer: Drafted reply to Carlos").
  3. The layout must be perfectly sized for a 375px width, with no horizontal scrolling.
  4. The user taps the primary "Approve" button on a card.
  5. The UI shows a loading state (spinner inside the button), makes a request to a new `/api/feed/approve` endpoint, and removes the card from the feed upon success.

  **Acceptance Criteria**:
  - The UI is built using the Flutter/web or current UI framework following the macOS Translucent Glass design system.
  - Touch targets for actionable buttons must be at least 44x44px.
  - A new backend API endpoint `/api/feed/approve` is implemented to handle the state transition (marking a card as resolved).
  - A Playwright E2E test is added that verifies the feed rendering, button tap, API call, and card removal on a simulated 375px mobile viewport. No hardcoded mock data in the production frontend code; feed data must come from the actual database or API layer.

  ### 5. Meta Information
  - **Priority**: P0
  - **Estimated Scope**: Medium
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
