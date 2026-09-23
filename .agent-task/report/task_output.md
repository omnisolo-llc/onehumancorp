issue_title: Compute and API Charging Evaluation
issue_description: |
  **Title**: Scribe: Evaluate Compute/API Charging and BYOK

  **Problem Statement**:
  The project needs to evaluate charging for compute and AI API usage, plus customer BYOK (Bring Your Own Key), provider-permitted native subscription access, and local inference, to understand existing code and real owner needs before committing to a plan.

  **Superpowers Provenance**:
  - Skill used: `using-superpowers`, `brainstorming`
  - Upstream Revision: 5bf4e78011075bcfc0dc295f0724994cd123ee71 (fetched into `.agent-task/scratch/superpowers`)
  - The `using-superpowers` skill was invoked to plan the execution approach and correctly frame the boundaries of this research task based on `RESEARCH.md`.

  **Research Report**:
  Based on the active business-capability map and scope priorities in `RESEARCH.md`, and the current audit/remediation ledger in `docs/research/native_migration_and_remediation.md`:

  1. **Current Capabilities**:
     - Usage accounting exists (`hub.rs`, `auditor.rs`) but requires fixing a repeated accounting issue (F01).
     - Global cost summaries read global totals rather than tenant-specific totals (F02).
     - A budget monitor exists but does not act as a hard spending reservation (F03).
     - Model paths disagree on usage; the proposal adapter returns default usage (F04).
     - Telemetry is not invoice-grade and lacks stable event identity, payer attribution, etc. (F05).
     - The connection flow returns 501 for secure connections (F06).
     - BYOK vs. subscriptions is distinct; the API proxy rejects unsupported subscription-relay modes, but provider-permitted native-client subscription hosting is not generally implemented (F13).

  2. **Economic Modes**:
     - *Managed API*: OHC's contracted account. Needs tracking of compute/resources + API consumption.
     - *Customer API Key (BYOK)*: Customer pays directly. OHC must not rebill customer-paid inference as managed inference.
     - *Provider-Native Client*: Depends on provider terms (e.g., Claude Code allows unmodified hosting but prohibits token relay).
     - *Local Inference*: Customer uses their own hardware/local models. OHC must ensure local paths (like `minimax.rs`) are metered only for cloud resources actually consumed, not double-charging for local model compute.

  3. **Prerequisites for Billing**:
     - Durable, deduplicated usage events with tenant/project/task/attempt, provider IDs, payer/auth mode, and rate-card version.
     - Immutable adjustments, trusted producers, tenant-specific reads.
     - Hard budget reservations and invoice reconciliation.

  **Design Doc**:
  ```mermaid
  flowchart TD
      A[API Request] --> B{Auth Mode}
      B -->|Managed API| C[Meter & Debit OHC Ledger]
      B -->|BYOK| D[Meter for Visibility, No Debit]
      B -->|Provider-Native| E[Ensure Terms Compliance]
      B -->|Local Inference| I[Ensure Metering Ignores Local Compute]
      C --> F[Atomic Reservation]
      D --> F
      E --> F
      I --> F
      F --> G[Execution]
      G --> H[Settle & Reconcile]
  ```

  **Feature Matrix**:
  | Billing Mode | Current State | Required State |
  | --- | --- | --- |
  | Managed API | Fragmented, non-invoice-grade | Idempotent, tenant-scoped, hard-capped |
  | BYOK | Keys accepted but mixed with managed | Separated logic, no double charging |
  | Native Sub | Not implemented | Validated per provider terms |
  | Local Inference | Basic paths exist | Fully verified resource non-billing |

  **Verified Source URLs**:
  - `docs/research/business_capability_and_usage_economics_audit.md`
  - `docs/research/native_migration_and_remediation.md`

  **Implementation Prompt**:
  Before implementing billing rates:
  - Fix the repeated usage accounting loop in `hub.rs`/`auditor.rs` (F01).
  - Scope cost summaries to the authenticated tenant (F02).
  - Implement hard budget reservations (F03).
  - Standardize usage capture across all model paths, including local models and proposals (F04).
  - Upgrade telemetry to invoice-grade (F05).
  - Verify and secure the connection flow (F06).
  - Strictly separate BYOK from Managed API charging (F13).
  - Wait for actual workload costs, owner interviews, and test results before applying the suspended $99/mo or 300-step allowances.

  **Priority**: P0 (Correctness before expansion)
  **Estimated Scope**: Needs to be broken down into individual remediation PRs based on the F01-F15 findings.
issue_priority: P0
issue_category: Research
issue_type: Report
issue_label: ohc:lane:finance
assignees: []
