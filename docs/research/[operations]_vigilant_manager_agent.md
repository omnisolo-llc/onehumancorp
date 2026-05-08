# Title
The Vigilant Manager: Proactive Inventory and Operations AI Agent

# Problem Statement
Small business owners like Priya (Boutique Owner) and Fatima (Food Cart Operator) struggle with operational fatigue. They lack real-time visibility into their stock and operations because they are too busy serving customers to update spreadsheets or clunky dashboards. Manual inventory tracking leads to "Sold Out" signs, missed sales, and stress. They need a system that acts as an invisible manager, anticipating shortages and proactively suggesting restocks or operational adjustments without requiring manual data entry.

# Research Report
- **Target Personas**: Priya (Boutique Owner, 35) - struggles with inventory sync across in-store and online; Fatima (Food Cart, 50) - busy cooking, cannot manage mobile app inventory during rush.
- **Pain Points Addressed**: Operational Fatigue (Rank 2, 68% frequency), Mobile Gaps (Rank 7, 42% frequency).
- **Competitor Analysis**: Shopify and Wix rely on reactive alerts or third-party apps for inventory management. Neither offers a proactive, conversational agent that suggests draft orders.
- **Evidence**: App Store reviews indicate users find it too hard to update inventory quickly from their phones during busy hours. Trustpilot shows complaints about selling out of items unexpectedly.
- **AI Differentiation**: The Vigilant Manager agent monitors the event mesh in real-time, predicting stock-outs based on sales velocity and queuing 1-tap restock approvals on the owner's dashboard.

# Design Doc
- **High-Level Architecture**:
  - **Entity Types**: `InventoryItem`, `SalesEvent`, `ActionTask`.
  - **Key Relationships**: `SalesEvent`s decrease `InventoryItem` counts. The agent generates `ActionTask`s when `InventoryItem` levels drop below dynamically predicted thresholds.
  - **Integration Points**: Hooks into the backend event mesh (NATS/PostgreSQL triggers) to process sales events in the background.
- **UI Wireframes/Screen Flow Description**:
  - The Action Feed (Mobile 375px first): A sleek, Glassmorphic feed displaying priority alerts.
  - Alert Card: "Vanilla Cupcakes are selling fast! You have 5 left. Draft a restock order for tomorrow?"
  - Interaction: A single "Approve" button or a "Snooze" swipe. No complex forms required.
- **AI Agent Integration Points**: The background agent continuously analyzes the stream of sales data against historical velocity, employing lightweight predictive models to flag anomalies or impending stock-outs before they happen.

# Implementation Prompt
Build the Vigilant Manager agent in the backend orchestration layer that subscribes to sales events. It should track inventory velocities and create actionable tasks (e.g., "Approve Restock") in the central task queue when items are predicted to run low. Update the Slint mobile UI to surface these tasks in a centralized "Action Required" feed, allowing the user to approve the suggested action with a single tap. The Critical User Journey (CUJ) is a store owner receiving an automated low-stock alert with a pre-drafted restock order and approving it instantly from their phone. Acceptance criteria include the agent successfully catching a simulated low-stock event and the UI rendering the 1-tap approval card correctly.

# Priority
P1

# Estimated Scope
Medium
