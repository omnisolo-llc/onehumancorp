issue_title: "Research: Evaluate Managed API and BYOK usage charging and economics"
issue_description: |
  # Managed API and BYOK Usage Economics Audit

  ## Title
  Evaluate API Provider Payer Modes and Integration

  ## Problem Statement
  We need to verify if the OmniSolo platform can reliably measure and bill for resource usage across different payer modes: an OHC-managed API account and a customer-provided BYOK account, ensuring no duplicate billing, proper budget reservations, and accurate tenant allocation before committing to an architecture plan.

  ## Research Report
  - **Managed API Mode**: OHC utilizes its own contracted API accounts. The system must account for OHC compute/resources and explicitly priced API consumption. Reselling bare API access without added value or pooling consumer subscriptions typically violates provider terms and must be avoided.
  - **BYOK (Bring Your Own Key) Mode**: Customers authenticate using their own API keys or cloud accounts. In this mode, OHC charges for hosting, storage, and tools but *must not* rebill the customer-paid inference as OHC consumption.
  - **Current Implementation Gaps**: Existing usage event pipelines (e.g., `auditor.rs` and `hub.rs`) capture some telemetry but lack a robust invoice-grade meter. Deficiencies exist around durable idempotency, accurate payer/auth attribution, reservation ceilings before starting work, and reconciling unknown outcomes. The integration needs tests to prevent double debiting for BYOK.

  ## Design Doc
  - The billing and metrics subsystems (e.g., `auditor.rs`, `billing/service.rs`, `budget.rs`) will need to be refactored to tag every usage event with a `payer_mode` (`MANAGED` or `BYOK`).
  - Budget checks must verify reservations prior to dispatching LLM calls.
  - The telemetry pipeline must ensure idempotent, tenant-specific writes.
  - BYOK inference costs will be visually tracked for the user's dashboard but explicitly zeroed out in OHC's internal revenue ledger.

  ## Implementation Prompt
  Implement a unified metering interface that distinguishes between OHC-managed API usage and BYOK usage. Ensure budget reservation precedes execution. Ensure telemetry events have robust idempotency keys, payer mode tags, and tenant scoping. Exclude BYOK tokens from OHC billing totals while still recording them for user visibility.
  - **Acceptance Criteria**: Metering events successfully record both Managed and BYOK usage. Billing engine correctly isolates BYOK costs from OHC invoices. Budget reservation reliably blocks over-limit requests.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
