issue_title: "Implement Mobile-First Agent Approval Feed UI"
issue_description: |
  **Problem Statement**
  Small business owners like Maya and Carlos need to operate their entire business from their phones without being overwhelmed by complex configuration forms. Legacy platforms rely on desktop dashboards, breaking the mobile-first operational paradigm.

  **Research Report**
  As detailed in `docs/business/market_research/ohc_smb_mobile_first_design_research.md`, legacy platforms fail mobile users by hiding complex configurations behind desktop walls. OHC's key differentiator is automating these workflows and presenting the user with an intuitive, unified "Agent Approval Feed" accessible entirely on a 375px display. This allows non-technical owners to approve complex actions (e.g., launching an ad campaign or drafting a promotional email) with a single tap.

  **Design Doc**
  - **Architecture Diagram**:
    ```mermaid
    graph TD
      A[Unified Mobile Feed] -->|Fetch| B[Agent Queue API]
      B --> C[Operations Agent]
      B --> D[Marketing Agent]
      B --> E[Advisory Agent]
      A -->|Action| F[Approve/Reject Endpoint]
    ```
  - **UI Wireframes/Screen Flow**:
    - The main dashboard replaces traditional charts with a vertical scrolling feed of Action Cards.
    - Each card displays context (e.g., "Drafted Instagram Post") and large, touch-friendly primary actions ("Approve & Post", "Edit").
  - **Mobile UX Flow**:
    - App Open -> Fetches Agent Proposals -> Renders Stack of Cards (375px width, no horizontal scroll) -> Tap "Approve" -> Transitions to Success state -> Shows next card.
  - **AI Agent Integration Points**:
    - The feed aggregates outputs from Operations, Marketing, and Advisory departments.
    - Card templates vary slightly based on the originating AI department (e.g., Marketing cards show image previews).

  **Implementation Prompt**
  Implement the "Unified Agent Feed" mobile UI view for the OHC Tauri App.
  1. Build a responsive, 375px-first feed component that polls or subscribes to a backend stream of pending agent actions.
  2. Implement card variants for Operations, Advisory, and Marketing proposals, utilizing OHC Premium Tokens (Glassmorphism, Outfit font, 44x44px minimum touch targets).
  3. Wire up an action flow where clicking "Approve" dispatches the confirmation back to the agent and optimistically updates the UI.
  4. Ensure 100% test coverage using Playwright E2E tests, verifying that a seeded test user can approve an agent action entirely within the mobile layout.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
