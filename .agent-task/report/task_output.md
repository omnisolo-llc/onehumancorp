issue_title: "[Platform Gap] Missing Agentic Inventory & POS Coordination for Multi-Channel SMBs"
issue_description: |
  # The Promoter Agent Issue Brief

  ## Target Persona: Priya (Boutique Owner)

  ## Problem Statement
  SMBs selling across multiple channels (e.g., in-store and online) struggle to keep their inventory synchronized. This leads to overselling, stockouts, and manual reconciliation overhead.

  ## Architecture & Design Flow
  - **Data Ingestion**: Webhooks connected to Shopify/Square/WooCommerce APIs.
  - **Processing Layer**: Agentic reconciliation engine to update inventory counts across platforms.
  - **Context Generation**: Real-time sales data and inventory levels.
  - **Draft Generation**: Agent generates a contextually accurate reply.
  - **Mobile UX**: Pushes a notification to Priya. The OHC mobile app displays a 375px card showing the drafted message, with "Approve & Send", "Edit", and "Discard" actions.

  ## Implementation Prompt
  - Integrate Shopify/Square/WooCommerce APIs for inventory updates.
  - Implement agentic reconciliation engine.
  - Implement real-time sales data context building.
  - Build the mobile-first (375px) notification card UX for approval.
  - Do NOT prescribe database schemas here. Focus on the seamless connection between the webhook, the LLM, and the user's mobile feed.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
