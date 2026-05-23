issue_title: "[Architecture] Autonomous Plain-Language Business Intelligence Engine"
issue_description: |
  # Research Report & Findings

  Small business owners like Maya (baker) and Fatima (food cart operator) do not have the time, energy, or training to interpret complex charts, conversion funnels, and pivot tables provided by traditional platforms (Shopify, Wix, Quickbooks). They need an invisible Analyst that translates raw ledger data into plain-language, actionable business intelligence pushed directly to their phone.

  ## Problem Statement
  Small business owners are busy running their operations. When they open their current software tools, they are greeted with complex dashboards featuring line charts, conversion funnels, CAC metrics, and pivot tables. They do not have the time, energy, or training to interpret these charts. They need an invisible Analyst that translates raw ledger data into plain-language, actionable business intelligence pushed directly to their phone.

  ## Proposed Next Steps
  We need to implement the Autonomous Plain-Language Business Intelligence Engine. This involves building a backend service and agent coordination.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Universal Ledger & Inventory Mesh] --> B[Data Aggregator Service]
      C[Omnichannel AI Inbox / CRM] --> B
      B --> D[The Analyst - AI Intelligence Agent]
      D --> E[Recommendation Engine]
      E --> F[Plain-Language Generation LLM]
      F --> G[Mobile Notification & Action Card 375px]
      G --> H[1-Tap Approval / Execution]
      H --> I[The Operator / Promoter Agents]
  ```

  ### Mobile UX Flow (375px)
  1. **The Morning Briefing**: At 8:00 AM, the owner receives a push notification on their phone.
  2. **Plain-Language Summary**: Tapping the notification opens a simple glass-morphic card with a brief greeting.
  3. **Actionable Suggestions**: Below the summary, a 1-tap action card asks for confirmation on a suggested action.
  4. **Approval**: The owner taps "Yes, do it." The Analyst agent hands off the task to the Manager agent, which updates the storefront instantly.
  5. **Advanced Toggle**: An "Advanced Settings" switch is hidden at the bottom, which reveals the raw data and charts for users who actually want to see them.

  ### Key Design Decisions
  *   **No Mandatory Dashboards**: The default UI contains zero charts. The AI agent speaks in plain text and actionable questions.
  *   **Proactive Push**: The system does not wait for the user to open an analytics tab; it pushes critical insights to them at the optimal time.
  *   **Multi-Tenant Isolation**: The Data Aggregator strictly filters events by tenant ID before passing them to the LLM for summarization, guaranteeing zero cross-tenant data leakage.

  ### Implementation Prompt
  **Objective**: Build the backend service and agent coordination for the Autonomous Plain-Language Business Intelligence Engine.
  **CUJ**: A business owner receives a daily plain-language insight generated from their sales, inventory, and communication data, with a 1-tap action card to resolve a problem or capture an opportunity.
  **Acceptance Criteria**:
  - Implement a `DataAggregator` service that securely reads from the tenant's ledger and inventory mesh without exposing PII.
  - Create "The Analyst" AI agent that processes the aggregated data and generates a `DailyBrief` object containing a plain-language summary and 0-3 suggested actions.
  - Ensure the UI renders the `DailyBrief` in a clean, macOS-glass style card on a 375px viewport, passing the "grandmother test."
  - The `DailyBrief` must support triggering downstream AI agent actions upon 1-tap approval.
  - Ensure strict zero-trust multi-tenant data isolation.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
