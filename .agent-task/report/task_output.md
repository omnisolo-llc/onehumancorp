issue_title: Usage Economics and BYOK Billing Architecture
issue_description: |
  # Title
  Usage Economics and BYOK Billing Architecture

  # Superpowers Workflow Provenance
  - Loaded Skills: `using-superpowers`, `brainstorming`, `writing-plans`, `systematic-debugging`, `subagent-driven-development`
  - Loaded from Repository: `https://github.com/obra/superpowers/`
  - Revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks: Codebase file review (`src/server/api/invoice.rs`, `src/server/integrations/registry.rs`) and documentation audit (`docs/research/business_capability_and_usage_economics_audit.md`, `RESEARCH.md`).
  - Outcomes: Identified gaps in usage reporting, invoice-grade metering, budget reservation capabilities, and strict BYOK boundaries. Recommended implementing durable, deduplicated events and a two-phase budget commit.

  # Problem Statement
  OneHumanCorp (OHC) requires an architecture to measure and bill for AI API usage, distinguishing between OHC-funded compute/resources and customer Bring Your Own Key (BYOK) or provider-permitted native subscriptions. The system must support stable usage identity, payer/model/rate attribution, atomic reservations, integer subunit accounting, tenant-specific reads, and invoice reconciliation without duplicate charging for direct BYOK inference.

  # Research Report
  ## Source Dates
  - Research performed on 2026-09-19 based on the current codebase structure.
  - Sourced documents (`RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md`) accessed and reviewed on 2026-09-18.

  ## Study Populations
  - The context stems from the Federal Reserve's March 2026 employer-firm report, which identifies reaching customers/growing sales as the leading operational challenge, alongside the 2025 nonemployer report highlighting stable solo businesses.

  ## Metric Definitions
  - **OHC Serving Cost:** Model/tool usage + active CPU/Mem/GPU/Sandbox time + infra (storage/DB/queues) + external paid tools + payment collection + support.
  - **BYOK/Customer Keys:** Customer pays provider directly. OHC tracks usage for analytics/quotas but does not bill the customer for the inference cost. Financial quantities must use integer subunits (cents/micro-cents).

  ## Uncertainties
  - It is currently unknown which specific service niche exhibits the strongest repeated pain regarding usage reporting, and which channel/payment integrations owners prefer to connect based on real-world adoption.
  - Acceptable monthly cost and autonomy limits for owners regarding automated external actions require empirical validation through real-world pilot execution.

  ## Exact Commands Run
  - `git clone https://github.com/obra/superpowers/ .scratch/superpowers`
  - `cd .scratch/superpowers && git log -1 --format="%H"`
  - `cat docs/research/business_capability_and_usage_economics_audit.md`
  - `cat src/server/integrations/registry.rs | grep -n stripe`
  - `cat src/server/api/invoice.rs`

  ## Current Codebase Analysis
  - The telemetry/cost system uses `auditor.rs` and `hub.rs` to write metrics, but as noted in the audit (`F05`), it is not an invoice-grade meter.
  - Budget reservations (`pricing/budget.rs`) incremented before reporting over-limit, which was repaired (`F03`), but atomic reservation before spend and reconciliation capabilities need strengthening.
  - Provider paths currently disagree on usage reporting (`F04`), with some adapters returning default or absent usage metrics, undermining accurate billing.
  - The integration registry (`src/server/integrations/registry.rs`) does not currently implement isolation between OHC API keys and customer-provided keys for model inference.
  - BYOK requires clear boundaries. For example, Anthropic allows hosting unmodified Claude Code but prohibits proxying Claude.ai tokens. OpenAI differentiates API keys from ChatGPT Plus subscriptions.

  ## Technical Requirements
  1. **Event Capture:** Durable, deduplicated events containing `tenant_id`, `project_id`, `task_id`, `attempt_id`, `provider_request_id`, `payer` (OHC vs. Customer), `auth_mode` (Managed vs. BYOK), `provider`, `model`, input/output/cache amounts, resource units, interval, rate-card version, status (estimated/reserved/settled/refunded).
  2. **Accounting:** Use integer subunits (cents/micro-cents) for financial values. Immutable ledger-style adjustments, no silent history edits.
  3. **Reservations:** Implement hard budget reservations before initiating paid external work.
  4. **Reconciliation:** Handle late, out-of-order, or failed provider events. Do not convert failures into successful billed tokens.

  ## Scope
  The proposed scope is cross-cutting, involving the API, database persistence, and specific harness adapters. The focus is exclusively on establishing correct measurement and attribution. It explicitly excludes instituting a generic assistant, ERP system, or modifying current UI journeys outside of providing headless measurement logic.

  # Design Doc
  ## Architecture
  ```mermaid
  flowchart TD
      Worker[Harness Worker] -->|Usage Event| MeteringAPI[Metering API]
      MeteringAPI -->|Reserve| BudgetService[Budget & Quota Service]
      BudgetService -->|Check/Update| LedgerDB[(Ledger & Memory DB)]
      Worker -->|External Call| Provider[AI Provider]
      Provider -->|Response/Usage| Worker
      Worker -->|Settle Event| MeteringAPI
      MeteringAPI -->|Commit/Release| BudgetService
      BudgetService -->|Persist| LedgerDB
      Finance[Finance Module] -->|Reconcile| LedgerDB
  ```

  ## UI Wireframes
  (N/A - Headless backend service design)

  ## Mobile UX Flow
  (N/A - Backend infrastructure)

  ## AI Agent Integration Points
  - `omnisolo.memory` and harness local services must intercept model calls to inject context identity (`session_id`, `task_id`) and capture usage metrics returned by adapters.
  - The `AdapterLlm::chat` implementation across providers must parse usage data strictly and populate the `Usage` struct with non-zero integers in test mode to satisfy CI gates.

  # Implementation Prompt
  1. Implement `MeteringService` in `src/server/services/billing/` to process usage events.
  2. Define `UsageEvent` struct with fields: `id`, `tenant_id`, `task_id`, `payer` (Enum: `OhcManaged`, `CustomerByok`), `provider`, `model`, `input_tokens`, `output_tokens`, `amount_micros`.
  3. Update `pricing/budget.rs` to implement two-phase commit: `reserve_budget(tenant, amount)` and `settle_budget(reservation_id, final_amount)`.
  4. Modify `AdapterLlm` traits to require returning `Usage` objects and enforce correct mock values in tests.
  5. Ensure 100% unit test coverage for the metering and budget modules.

  # Priority
  High

  # Estimated Scope
  Large (Cross-cutting concern across API, database, and harness adapters)
issue_priority: High
issue_category: Finance
issue_type: Epic
issue_label: ohc:lane:finance
assignees: []
