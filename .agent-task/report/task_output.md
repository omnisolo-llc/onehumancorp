issue_title: "Implement Resource-Based Charging and BYOK Billing Architecture"
issue_description: |
  **Problem Statement:**
  Non-technical owner/operators like Nora (solo web/design professional) need predictable costs when using an AI team. Currently, they lack transparent mechanisms to bring their own keys (BYOK) or use provider-permitted native subscriptions, leading to uncertainty around margin, API overages, and mixed billing of hosting versus inference. The gap lies in the absence of a resource-based charging model that clearly separates OHC infrastructure fees from customer-funded model usage.

  **Research Report:**
  *Market Benchmarks:*
  - **ChatGPT Work:** Uses files, connected tools and desktop apps to create work products; supports plugins and recurring tasks.
  - **Gemini in Workspace:** Assistance within the productivity suite, source-grounded research and no-code flows with connectors/custom extensions.
  - **Claude SMB update:** Announces 43 workflows and 27 new integrations, including commerce/finance tools; covers reporting, inquiries and proposals. Its Mothership Coffee example describes fragmented operational data brought together for reporting.

  *Operator Quotes:*
  - N/A

  **Design Doc:**
  *High-Level Architecture:*
  ```mermaid
  graph TD
      A[Owner UI / Mobile App] --> B[Billing & Auth Gateway]
      B --> C{Execution Mode}
      C -->|OHC Managed API| D[OHC Organization Account]
      C -->|BYOK / Customer API| E[Customer Cloud Account]
      C -->|Provider Native| F[Customer Subscription]
      B --> G[Usage Meter & Budget Enforcer]
      G --> H[Invoice Reconciliation Ledger]
  ```
  *UI Wireframes & Mobile UX Flow (375px first):*
  - **Screen 1 (Setup):** "Choose Your AI Plan" - Toggle between "OHC Managed (Pay as you go)" and "Bring Your Own Key (Save on inference)".
  - **Screen 2 (Budget):** "Set Monthly Cap" - A simple slider to set maximum OHC infrastructure or BYOK spend.
  - **Screen 3 (Ledger):** "Monthly Estimate" - Split view showing OHC serving costs vs. Customer-direct provider usage.
  *AI Agent Integration Points:*
  - Agents must request budget reservation from the `Usage Meter` before initiating tasks.

  **Implementation Prompt:**
  *Critical User Journey:*
  1. The owner navigates to Billing Settings and selects BYOK mode.
  2. The owner securely inputs their API key and sets a cap.
  3. The AI team executes a campaign; the system reserves budget, logs the tokens, and deducts from the cap without billing OHC's accounts.
  *Acceptance Criteria:*
  - Tenant-specific reads and budget reservations enforce the spending cap.
  - Usage events capture provider request IDs, payer, model, and resource units.
  - Exhausted budgets pause agent execution gracefully without silent failures.

  **Priority:** P2

  **Estimated Scope:** Large

  **Strategy Admission:**
  - **OHC Target ID:** OHC-10
  - **Launch/Run Stage:** Days 46-90
  - **Observed/Inferred Gap:** Observed gap in isolating OHC compute from customer-funded inference.
  - **Evidence Level:** Documented provider access boundaries and test-verified API mechanics.
  - **Baseline and Measurable Result:** Goal is pilot users successfully configuring and tracking BYOK or OHC-managed caps (N/A for baseline metric).
  - **Dependencies/Reuse:** Reuses existing hub.rs and auditor.rs.
  - **Non-Goals:** Building a new LLM provider; pooling consumer subscriptions.
  - **Authority Class:** Owner standing authority required for API key storage.
  - **Cost/Measurement Plan:** Track active/reserved compute, failed retries, and token volume separately from OHC serving cost.
  - **Happy-path and Failure Acceptance Checks:** Happy path verifies budget deduction and accurate logging. Failure path verifies strict agent suspension when budget hits $0 or API key revokes.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
