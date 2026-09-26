issue_title: '💰 Miser: Audit of Usage Economics and Compute Pricing'
issue_description: |
  # Usage Economics and Compute Pricing Audit

  ## Problem Statement
  The product owner needs to understand the economic baseline for providing AI capabilities, how compute/API pricing compares to previous flat-rate subscription models, and evaluate BYOK (Bring Your Own Key) options vs. managed API models before committing to a final product pricing strategy.

  ## Research Report
  The audit `docs/research/business_capability_and_usage_economics_audit.md` indicates that previous assumptions of a $99/month fixed fee and $299 setup are suspended. New requirements mandate evaluating actual computational resource costs, explicit API usage vs reserved capacity, and different hosting structures (managed API vs customer account API key vs provider-native client).

  ### Current Codebase Capabilities
  - **Billing Framework**: A foundational billing crate (`src/server/services/billing`) is in use, moving away from simple request queues towards a pipeline capturing tenant usage.
  - **Budget Verification**: Budget checks (`src/server/pricing/budget.rs`) and pricing calculations exist.
  - **BYOK Support**: The API proxy currently supports rejecting unsupported subscription relays and bounding API keys.
  - **Missing Elements**: The codebase lacks integration with a comprehensive customer UI to display granular API/inference costs, limits, and reservations in the exact units demanded by the usage economics audit. The audit emphasizes needing durable, deduplicated events, which the current `server_pricing` tests confirm exist to some degree, but need tighter integration with the frontend to expose owner visibility.

  ### Evaluated Modes
  1.  **Managed API**: This places all token and generation costs on OHC's accounts. It demands robust telemetry to avoid runaway inference, which is currently present via `Miser telemetry` logs.
  2.  **BYOK (Customer API Key)**: Avoids OHC's token liability, but OHC still incurs platform and hosting costs. It requires strict separation in the ledger.

  ### Findings
  The infrastructure for capturing telemetry and limiting budgets exists in `src/server/pricing` and is being verified by tests. However, an end-to-end reconciliation system mapping direct provider invoices to OHC's tracked usage, and displaying that to the owner before billing, is still a gap. The business logic for "durable reservations" is present, avoiding overruns.

  ## Design Doc

  ### Architecture Update
  No immediate structural changes are recommended, but the telemetry and usage recording pipeline must strictly enforce the separation of BYOK events vs Managed API events so the ledger correctly prices OHC resources independently of inference tokens.

  ## Implementation Prompt
  *   This is a research-only issue; no direct code changes to `src/server/pricing` or `src/server/services/billing` are prescribed here, as they are functioning and currently passing the budget unit tests.
  *   Continue monitoring telemetry to refine the model's token consumption accuracy, especially concerning "cache creation input tokens" vs "base input tokens".

  ## Superpowers Workflow Provenance
  - Loaded Skills: `skills/using-superpowers/SKILL.md`
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks: Investigated the `src/server/pricing` implementations, specifically `budget.rs`, `calculator.rs`, and checked tests. Verified the state of the native build checks using `make doctor`. Started cargo tests in the background.
  - Outcomes: The codebase matches the audit's finding that budget increment and check mechanisms exist, but explicit integration with Stripe/payment for precise per-token billing requires further verification before going live. No code modifications were required for this audit.

issue_priority: P1
issue_category: research
issue_type: audit
issue_label: ohc:lane:finance
assignees: []
