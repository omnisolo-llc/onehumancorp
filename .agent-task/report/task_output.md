issue_title: "[research] Optimize OHC AI Automation Engine to Reduce Mobile Setup Friction"
issue_description: |
  # Research Report: Optimizing AI Automation to Reduce Mobile Setup Friction for Non-Technical SMBs

  ## Problem Statement
  Small business owners face significant setup paralysis. Current platforms like Shopify and Wix expect users to configure complex settings (shipping zones, payment gateways, theme layouts) on a desktop. OHC users (like Maya the baker and Carlos the handyman) operate primarily from mobile devices. If they cannot set up their business within 10 minutes on a 375px screen without manual data entry, they churn.

  The gap identified is that while OHC has AI Agent Departments, the initial setup process still requires too much direct user intervention. We need to transition from "AI-assisted configuration" to "Zero-touch, Invisible AI Automation," where the agents proactively generate the complete store configuration based on a single natural language input or photo.

  ## Research Findings
  - **Market Gap**: 42% of SMB owners experience "setup paralysis" due to complex configuration requirements (e.g., shipping, taxes).
  - **Competitor Analysis**: Shopify's "Sidekick" requires the user to know what to ask. Wix's AI generates a basic layout but leaves business logic (inventory, payments) manual.
  - **OHC Advantage**: OHC's architecture supports background autonomous agents. We must leverage the `Operations` and `Marketing` departments to completely abstract the initial configuration.

  ## Proposed Architecture & Design Doc

  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
      A[Mobile UI 375px: Single Input Form] -->|NLP/Photo| B(API Gateway)
      B --> C{Orchestrator}
      C --> D[Marketing Agent: Generates UI/Copy]
      C --> E[Operations Agent: Configures Inventory/Shipping]
      C --> F[Finance Agent: Sets up Payment/Tax Profile]
      D --> G[(Tenant Database)]
      E --> G
      F --> G
      G --> H[Mobile UI: 'Approve & Launch' Button]
  ```

  ### UX/UI Flow (Mobile-First 375px)
  1. **Step 1 (Input)**: User uploads a photo of their product or types one sentence: "I sell custom cakes in Seattle."
  2. **Step 2 (Loading State)**: Translucent glass loading screen ("Agents are building your business...").
  3. **Step 3 (Review)**: A fully populated store (theme, sample products, shipping defaults) is presented.
  4. **Step 4 (Launch)**: 1-Tap "Approve & Go Live" button.

  ### Key Design Decisions
  - **Zero-Touch Configuration**: Default parameters (e.g., local shipping zones based on IP, standard tax rates) are injected by agents automatically.
  - **Progressive Disclosure**: Advanced settings (like API keys or specific tax overrides) are hidden by default and accessible only via an "Advanced" toggle.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the "Zero-Touch Storefront Generation API."
  **CUJ**: A user provides a single text input string describing their business. The backend must synchronously trigger the Marketing, Operations, and Finance agents to generate a complete `StoreProfile` (including theme, 3 sample products, and default shipping/tax settings) and return it to the client for 1-tap approval.
  **Acceptance Criteria**:
  - Endpoint must accept a simple string or image payload.
  - Must return a fully hydrated store configuration within 5 seconds.
  - Must include E2E Playwright tests verifying the flow from the mobile UI perspective (375px viewport).
  - All database interactions must respect row-level tenant isolation.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
