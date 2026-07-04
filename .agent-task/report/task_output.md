issue_title: "Implement Autonomous AI Order Escalation & Resolution Agent"
issue_description: |
  # Autonomous AI Order Escalation & Resolution Agent

  ## Problem Statement
  For operators like Fatima (food cart) or Jun (location manager), high volume periods often lead to order exceptions: missed pickups, late fulfillments, or stockouts. Currently, identifying these exceptions requires manually scanning order dashboards and mentally tracking elapsed time. When an issue occurs, the owner must manually draft apologies, coordinate with staff, or process refunds. This friction causes customer dissatisfaction and distracts the operator from immediate operational needs. A core promise of OHC is to proactively tell the owner what needs attention and suggest the next move.

  ## Research Report
  - **Market Gap:** Traditional platforms (Square, Shopify) offer order status tracking but lack proactive, agent-driven escalation. They rely on the user to check a dashboard. AI-native tools are starting to automate customer service but often lack the deep operational context to make fulfillment-based decisions.
  - **Persona Focus:** Fatima (Food Cart Operator) dealing with a high volume of pre-orders during a lunch rush. Jun (Location Manager) tracking delivery dispatch delays.
  - **Proposed Solution:** Implement an autonomous "Escalation Agent" that monitors order lifecycles in the background. If an order exceeds a defined SLA (e.g., pending for > 30 minutes, or unpicked up 15 minutes post-schedule), the agent triggers an escalation event. This event is routed to the Agent Feed, proposing an immediate action (e.g., "Draft apology SMS to customer with a 10% discount link" or "Alert kitchen staff to prioritize order #123").

  ## Design Doc
  - **Architecture:**
    - A background worker (`EscalationWorker`) periodically (or via event triggers) scans the `orders` table for SLA violations based on `tenant_id` and order `status`.
    - Upon detecting a violation, it pushes an event to the `AgentFeedService`.
    - The LLM context builder generates an intent like `order_sla_breach` and drafts a contextual action (e.g., SMS apology draft, refund proposal).
    - The `AgentFeedItem` is persisted and pushed to the UI for the owner.
  - **Mobile UX Flow (375px):**
    - The owner receives an OHC push notification or sees a high-priority card at the top of their home feed: "⚠️ Order #123 is 15 mins late."
    - Tapping the card opens a translucent modal showing the order details and the AI-drafted action: "Send SMS apology & $5 coupon to customer."
    - Two large (44x44px min) buttons: `[Send Now]` (Primary) and `[Edit]` (Secondary).
  - **AI Agent Integration:** The Escalation Agent operates silently in the background, utilizing the existing job queue and `AgentFeedService` to surface actionable intelligence rather than raw data.

  ## Implementation Prompt
  Implement the backend service and database queries required for the Autonomous AI Order Escalation Agent.
  1. Create a periodic job or event listener that identifies orders exceeding a configurable SLA threshold.
  2. Integrate this detection with the existing `AgentFeedService` to generate an `AgentFeedItem` with a proposed resolution action.
  3. Ensure the context payload includes order details and customer contact info to allow the LLM to draft a relevant response.
  4. Write comprehensive unit and integration tests simulating an SLA breach and verifying the feed item generation.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
