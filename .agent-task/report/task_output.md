issue_title: "Implement Distributed Inventory POS Sync & Agent Resolution"
issue_description: |
  # Mission Queue Protocol: Centralized Inventory & Distributed POS Architecture

  ## Problem Statement
  Priya, the boutique owner, needs seamless real-time synchronization between her physical storefront (using a tap-to-pay POS terminal) and her online catalog. Currently, OHC lacks strong consistency when an item is simultaneously purchased online and in-store. This leads to double-booking, frustrated customers, and disjointed analytics, preventing Priya from confidently using OHC to drive demand.

  ## Research Report
  - **Market Context**: Shopify POS provides robust omnichannel sync but burdens SMEs with complex inventory layers. Stripe Terminal solves hardware needs but lacks built-in agentic resolution for conflict scenarios.
  - **Identified Gap**: OHC requires a Redis Redlock-backed inventory reservation mechanism tied to both online checkout flows and offline Terminal transactions. When conflict arises, the "Operations Agent" must automatically reconcile and provide Priya with owner-friendly actionable solutions (e.g., automated restock drafting).
  - **Design Justification**: Leveraging the existing Central Ledger (PostgreSQL) as the source of truth combined with Redis for ephemeral transactional locks ensures no double-spending. The POS client must employ optimistic UI updates, reverting safely if the lock fails.

  ## Design Doc
  ### Mobile UX Flow (375px Target)
  1. Priya selects the "Red Dress" and taps "Charge $50.00".
  2. The POS client instantly shows a localized loading state.
  3. Under the hood, the client invokes the POS Terminal Checkout endpoint, attempting to secure a 15-second Redis Redlock on the inventory item.
  4. Once secured, the Terminal transaction finalizes.
  5. If an online customer attempts checkout simultaneously, the system immediately returns an "Out of Stock" error.
  6. Priya receives a dashboard notification from the Operations Agent confirming the sync and optionally suggesting a re-order draft.

  ### AI Agent Integration
  - **Operations Agent**: Monitors the locking system for repeated conflicts. In the event of an out-of-stock scenario, it automatically triggers a workflow to notify the online customer (via CS Agent) and drafts a restock order for Priya's approval.

  ## Implementation Prompt
  Implement the backend synchronization protocol and corresponding POS API endpoint for distributed inventory locking.

  **Acceptance Criteria**:
  1. Create a Redis Redlock mechanism applied to `ohc:lock:{tenant_id}:inventory:{product_id}`.
  2. The lock should support dynamic timeouts (e.g., 15 seconds for POS, 5 minutes for online carts).
  3. Integrate this locking mechanism into the checkout flow (backend API) to prevent double-booking.
  4. Provide a structured event/hook for the Operations Agent to detect stock-out situations and trigger downstream notification workflows.
  5. Ensure API errors gracefully fallback for the mobile-first frontend.

  **Note**: Do not prescribe the specific database schema or internal function signatures. The implementer should design those within the existing `src/server/` boundaries.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
