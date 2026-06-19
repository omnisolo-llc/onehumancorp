issue_title: "Implement Intelligent Cart Recovery Agentic Workflow"
issue_description: |
  # Implementation Prompt: Intelligent Cart Recovery Agent

  ## Problem Statement
  Small business owners are losing significant revenue from abandoned carts. Traditional e-commerce platforms (like Shopify) require users to manually set up complex email marketing flows or install expensive third-party apps (like Klaviyo) to recover these carts. OHC needs a zero-configuration, AI-driven solution.

  ## Research & Context
  According to our market research (`docs/business/market_research/ai_agentic_workflows_research.md` and `docs/business/market_research/ai_agentic_cart_recovery_research.md`), cart recovery is a major pain point. OHC's differentiation is "Invisible AI Agents" that do the work for the user. We need an agent that autonomously monitors for abandoned carts, drafts personalized recovery messages, and surfaces them for approval via the Agent Feed.

  ## Architectural Design

  ### 1. Data Model
  - Introduce an `abandoned_carts` table tracking cart state (e.g., `cart_id`, `customer_email`, `items`, `abandoned_at`, `status`).
  - *Note: the implementer agent will design the exact schema, but it must enforce RLS.*

  ### 2. Event Pipeline & Trigger
  - Instead of relying on manual configuration, a background job (or event listener) will identify carts that have been inactive for a certain period (e.g., 1 hour).
  - This trigger will dispatch a task to the `Customer Success Agent` via the `Teammate Mesh`.

  ### 3. Agent Execution (The "Brain")
  - The agent will use the `OHC_LLM_PROVIDER` to generate a personalized recovery message based on the cart contents and customer history.
  - The drafted message will be inserted into the `agent_feed_items` table as a `PENDING_APPROVAL` action.

  ### 4. UI / UX (Mobile First)
  - The business owner receives a notification (via the existing Agent Feed UI in `src/ui/tauri`).
  - The feed card will display the drafted email/SMS.
  - The owner can click "Approve" (which sends the message) or "Edit" (to modify the draft before sending).

  ## Execution Steps for Implementer
  1.  **Schema:** Create a database migration for tracking abandoned carts and recovery attempts.
  2.  **Detection Logic:** Implement the background logic to detect abandoned carts and trigger the agent workflow.
  3.  **Agent Workflow:** Write the logic for the `Customer Success Agent` to generate the recovery draft using the configured LLM and insert it into the Agent Feed.
  4.  **E2E Test:** Write a comprehensive Playwright test (in `src/e2e/`) simulating the entire flow: a customer abandoning a cart -> the agent generating a draft -> the owner logging in on mobile and approving it from the feed. No mocks allowed for the core flow.

  ## Acceptance Criteria
  - [ ] Database migration applied and RLS enforced.
  - [ ] Agent correctly identifies an abandoned cart and generates a draft.
  - [ ] Draft appears in the Agent Feed UI.
  - [ ] Owner can approve the draft from a mobile viewport (375px).
  - [ ] `bazel test //...` passes completely.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
