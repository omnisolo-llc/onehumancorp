outcome: actionable
issue_title: "OHC-04/05/06: Native Client AI Broker Architecture and BYOK Inference Accounting"
issue_description: |
  # Native Client AI Broker Architecture and BYOK Inference Accounting

  ## Problem Statement
  Currently, the OneHumanCorp platform lacks a measured deployment cost model, an invoice-grade meter, and a clear distinction between provider-native client subscription modes (e.g. Anthropic Claude Code unmodified binary hosting versus API/Cloud authentication). The existing Codex adapter and API/OAuth credential types are reuse assets but not a demonstration of a supported native-client subscription mode. Furthermore, customer-paid BYOK inference is not cleanly separated from OHC's provider expense and OHC inference debit.

  ## Research Report
  ### Market Benchmark & Evidence
  - **OpenAI:** Recommends API keys for programmatic CLI work and Enterprise access tokens for trusted private Codex automation, not general API calls. No approval for arbitrary hosted subscription brokering.
  - **Anthropic:** Permits hosting the **unmodified** Claude Code binary under stated conditions where users authenticate and pay directly. Prohibits third-party login/token relay and end-user usage resale. An OHC-owned Agent SDK/API requires API/cloud authentication.
  - **Google:** Gemini API billing ties API keys to projects and Cloud Billing. A generic Workspace subscription is not automatically a Gemini API entitlement.

  ### OHC Gap & Invisible Agentic Solution
  The platform needs a robust cost accounting and metering capability that supports BYOK and distinguishes it from OHC-funded model/tool usage. This includes durable, deduplicated events, tenant-specific reads, integer subunits for precision, atomic reservations before spending, and clear unknown or pending reconciliation.

  ## Design Doc
  - **Architecture Details:**
    - Introduce an invoice-grade metering abstraction.
    - Implement durable, idempotent usage events using a deduplication key (tenant/project/task/attempt, provider request IDs).
    - Distinguish between AI provider payer modes (Managed API, Customer API key, Provider-native client, Customer machine).
    - Exclude customer-paid BYOK inference from OHC's provider expense.
  - **UI Wireframes/Screen Flow:**
    - Mobile-first (375px) cost UI showing **estimated task cost and maximum authorized spend**, itemized OHC bill, and separate customer-direct provider usage.
  - **AI Agent Integration:**
    - Agents must consult the metering service and perform atomic reservations before initiating paid external API calls.

  ```mermaid
  graph TD
      A[Agent Request] --> B{BYOK or Managed?}
      B -- BYOK --> C[Customer Direct Provider Usage]
      B -- Managed --> D[Atomic Reservation]
      D --> E[Provider API Call]
      E --> F[Durable Idempotent Usage Event]
      F --> G[Invoice Grade Metering]
      C --> G
  ```

  ## Implementation Prompt
  Implement the invoice-grade metering and BYOK cost accounting separation. Add tests that verify atomic reservations before spending, durable idempotent usage, and tenant-specific reads. Ensure that BYOK inference is not rebilled as OHC consumption. Provide UI components (mobile-first) to display estimated task cost, maximum authorized spend, itemized OHC bill, and customer-direct provider usage.

  ## Strategy Admission
  - OHC Target ID: OHC-05, OHC-10
  - Launch/Run Stage: Run
  - Evidence Level: Documented/Implemented
  - Dependencies: Existing telemetry and billing stubs
  - Scope: Large
  - Priority: P1

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
