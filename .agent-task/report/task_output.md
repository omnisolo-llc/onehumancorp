issue_title: Establish Trustworthy Economics and Cost Visibility Before Pricing
issue_description: |
  # Trustworthy Compute/API Usage Economics

  ## OHC Serving Cost Model
  As documented in `RESEARCH.md`, the serving cost is calculated as:
  ```text
  OHC serving cost = OHC-funded model/tool usage
                   + allocated CPU, memory and GPU resources
                   + databases, storage, queues, backups and logs
                   + browser/runtime capacity and network egress
                   + paid external tools and communication
                   + payment collection, support and incident handling
                   + attributable idle/shared capacity
  ```

  This does NOT include customer-direct provider bills (like their own API keys). The distinction must be maintained.

  ## Blocked Prerequisites
  The task is a **no-work/blocked** outcome because the current system lacks real-world prerequisites:
  1. **Invoice Reconciliation**: We do not currently have a reconciled provider invoice to base exact telemetry unit pricing off of.
  2. **Owner Interviews & Actual Workload Cost Measurement**: There is no measured representative serving cost or workload distribution available from actual customers. Extrapolating from an unverified public demographic does not fulfill the requirement for exact willingness to pay and active/reserved resources.
  3. **Usage Billing Evidence**: Minimum evidence requires durable, idempotent events with tenant/project/task/attempt and provider request IDs. Current codebase telemetry/cost reports are not an invoice-grade meter.

  Per the directive, we must not fabricate baselines, interviews, or dummy implementations. A concrete price card, Stripe integration, or usage billing system requires this true evidence first.

  ## Superpowers Workflow Provenance
  - **Loaded Skill Paths**: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`, `skills/brainstorming/visual-companion.md`
  - **Resolved Revision**: `5bf4e78011075bcfc0dc295f0724994cd123ee71` (fetched into `.scratch/superpowers`)
  - **Checks**: Executed codebase checks, reviewed `RESEARCH.md` and `native_migration_and_remediation.md`. Ran `make lint` and `make test`, noting the existing `next` missing error causing `build-web` to fail.
  - **Outcomes**: Documented a blocked task limitation correctly utilizing `.agent-task/report/task_output.md`.
issue_priority: P0
issue_category: Cost Engineering
issue_type: Implementation Blocked/Research
issue_label: ohc:lane:finance
assignees: []
