issue_title: "OHC Inventory Reorder Auto-Draft Agent Capability"
issue_description: |
  # Research Report: Agentic Inventory Reorder & Replenishment

  ## Problem Statement
  For business owners (like Priya the boutique owner or Maya the baker), manually monitoring inventory levels and drafting reorder emails to suppliers is tedious, error-prone, and distracting. The existing OHC system notifies the Operations agent when stock is low (`LowStockAlert`), but it stops short of actually completing the task. The business owner still has to navigate to a different tool, remember vendor details, and draft an email manually.

  ## Research Report
  Our competitive analysis (Shopify, Wix, Square) shows that inventory alerts are common, but they are purely reactive ("You are low on X"). Advanced ERP systems can auto-reorder, but they are too complex and expensive for micro-SMEs. OHC's unique value proposition is the *actionable* agent. By completing the last mile—drafting the supplier email and presenting it for single-tap approval—OHC transforms from a passive monitoring tool into an active operations assistant.

  ## Design Doc

  ### Architecture
  1.  **Trigger**: The existing `LowStockAlert` event (generated in `InventoryService::commit_inventory`) creates an `agent_action_requests` entry for the Operations agent with `action_type = 'Reorder'`.
  2.  **Processing (Agent)**: The Operations agent process (e.g., a background worker or a new capability in the Rust agent service) must listen for these `Reorder` requests.
  3.  **Context Retrieval**: The agent retrieves the product details (name, current stock) and looks up associated supplier/vendor information (which needs to be added to the data model or mocked via a settings/config if not present).
  4.  **Generation**: The LLM provider (Gemini/MiniMax) is called to draft a professional reorder email to the supplier, requesting a standard restock quantity (e.g., default batch size).
  5.  **Presentation (UI)**: The drafted email is placed into the owner's `agent_feed_items` or a dedicated "Drafts" view with a "Review & Approve" status.

  ### Mobile UX Flow (375px)
  - Owner opens the OHC app.
  - The Home Feed shows an alert: "Operations Agent drafted a restock order for Red Dress."
  - Tap -> Opens a modal displaying the drafted email to `supplier@example.com`.
  - Buttons: [Approve & Send] [Edit] [Cancel].

  ### Implementation Prompt (For Implementer)
  - **Goal**: Implement the agentic worker capability that processes `Reorder` action requests, generates a draft email using the configured LLM, and creates a user-facing approval item.
  - **Tasks**:
    1.  Create a worker/processor in the Rust backend that polls or listens for `agent_action_requests` where `action_type = 'Reorder'`.
    2.  Integrate with the LLM service to generate the email text.
    3.  Persist the generated draft so it appears in the UI for the owner to approve.
    4.  Ensure the relevant UI component (or E2E test) verifies that the drafted message is accessible and actionable.

  ## Priority: P1
  ## Estimated Scope: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
