---
issue_title: "Implement AI Agent One-Tap Out of Stock Workflow"
issue_description: |
  # Research Report: SMB Platform Market & Agentic Solutions

  ## Problem Statement
  Small business owners like Maya (the home baker) struggle with the complexity of existing platforms like Shopify. A major pain point is inventory management. When an item sells out, owners have to manually update their storefront, draft new social media posts, and handle customer inquiries—a multi-step, technical process that takes them away from their core business.

  ## Market Mapping & Competitor Discovery
  - **Shopify**: Highly capable but requires significant manual configuration for low-stock alerts and "sold out" workflows.
  - **Wix**: Offers basic inventory tracking, but proactive marketing (e.g., "Pre-order now" campaigns) requires manual intervention.
  - **Square**: Good for POS integration, but lacks native, invisible AI workflows to handle the marketing and operational fallout of stockouts.
  - **AI-Native Tools (e.g., Durable)**: Focus heavily on initial site generation rather than ongoing, deeply integrated operational workflows.

  ## OHC Gap Analysis
  OHC currently lacks a unified, agent-driven response to inventory changes. While we track inventory, we don't automatically trigger cross-department actions (e.g., Marketing drafting a pre-order campaign, Operations updating order readiness) when a product hits zero stock.

  ## Agentic Solution
  Implement an `InventoryStatusChanged` event published to the internal mesh whenever inventory is updated.

  - **Operations Agent**: Subscribes to this event. If stock hits zero, it automatically executes an action to mark the product out of stock. If stock is low (< 5), it drafts a review task to notify the owner to prepare a reorder.
  - **Marketing Agent**: Subscribes to this event. If stock hits zero, it proactively drafts a "Sold Out! Pre-order now" social media campaign.

  ## Recommendations & Next Steps
  1. **Adopted Changes**: The `inventory.rs` API now publishes `mesh:inventory:status_changed`. The Operations and Marketing agents have been updated to consume this event and trigger the workflows described above.
  2. **Testing**: Implemented the server implementation and E2E mock implementation that can be verified and evaluated at a high level.
  3. **Future Work**: Expand the Business Advisory agent to monitor these events over time and provide weekly insights on sell-through rates and optimal reorder points.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
---
