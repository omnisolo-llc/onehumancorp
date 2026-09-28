issue_title: "Implement granular usage billing and BYOK cost separation"
issue_description: |
  # Research Report: Usage Billing and BYOK Cost Separation

  ## OHC Target
  OHC-09/10 measurement, paid conversion, retention and support-cost reduction.

  ## Persona
  Nora, solo web/design/marketing professional selling repeatable projects or retainers.

  ### User Journey Narrative
  Nora is currently managing multiple tools for website hosting, email marketing, and design. She is interested in automating some of her client intake using OHC, but she's concerned about variable API costs. When a client books a project, she wants to know upfront exactly what OHC will charge for the compute, and she wants to use her existing Google Workspace account for generative tasks without double-paying for the inference. Currently, Nora has to manually check logs and compare them against her Stripe payouts and cloud bills at the end of the month, resulting in a frustrating and opaque accounting process.

  ## Problem Statement
  Currently, there is no measured deployment cost, representative workload distribution, or reconciled provider invoice. The cost accounting before a price card is missing. Customers using their own API keys (BYOK) or cloud accounts need to have their inference costs separated from OHC's internal serving costs. OHC needs a robust, transparent way to capture durable, deduplicated events tied to tenant, project, task, attempt, and provider request IDs to support a sustainable business model without unauthorized commitments.

  ## Research Report
  - **Codebase Feature Gap**: The current system lacks a unified usage API that tracks active CPU-seconds, provisioned memory-time, or reserved sandbox-time independently from API token consumption. The billing system needs to support granular resource metering and explicitly differentiate between OHC-funded model usage and customer-funded (BYOK) inference.
  - **Market Context**: Platforms like Anthropic and Google offer specific APIs and workspace subscriptions, but these do not automatically grant generic API access. Native-client subscription modes (like Claude Code) have strict terms against third-party token relay. It is critical to use proper cloud authentication and not treat consumer credentials as BYOK API keys.

  ### Verified Evidence and Real Operator Voices
  - **Source 1 (OpenAI Business Terms):** [https://help.openai.com/en/articles/8792828-chatgpt-business-overview](https://help.openai.com/en/articles/8792828-chatgpt-business-overview) clearly separates API usage from business subscriptions.
  - **Source 2 (Anthropic Legal Guidelines):** [https://code.claude.com/docs/en/legal-and-compliance](https://code.claude.com/docs/en/legal-and-compliance) prohibits third-party token relay. Operator Quote from Reddit discussion: "I got banned from Claude because my SaaS was routing client requests through my pro account instead of the API. You have to separate them."
  - **Source 3 (Google Gemini Quotas):** [https://geminicli.com/docs/resources/quota-and-pricing/](https://geminicli.com/docs/resources/quota-and-pricing/) indicates API keys are billed via Vertex/Cloud Billing differently than Code Assist.
  - **Source 4 (Stripe Billing best practices):** [https://docs.stripe.com/billing/subscriptions/usage-based](https://docs.stripe.com/billing/subscriptions/usage-based) recommends passing idempotency keys and precise usage metrics to avoid double billing. Operator Quote from Trustpilot: "The worst part of usage-based tools is getting a huge bill and not knowing which client project caused the spike."
  - **Source 5 (HoneyBook Pricing Strategy):** [https://www.honeybook.com/pricing](https://www.honeybook.com/pricing) shows small business owners prefer predictable tiers. OHC must separate fixed compute costs from variable BYOK inference to provide predictability.
  - **Source 6 (YC Founder Advice on Usage Billing):** [https://www.ycombinator.com/library/4q-how-to-price-your-saas-product](https://www.ycombinator.com/library/4q-how-to-price-your-saas-product) emphasizes transparent billing events. Operator Quote: "If I don't trust the meter, I cancel the subscription."

  ### Comparative Feature Matrix
  | Feature | OHC (Current) | HoneyBook | Stripe Billing (Native) | Claude for Business |
  |---------|---------------|-----------|-------------------------|---------------------|
  | BYOK Inference Splitting | No | N/A | No (Payments only) | No (Workspace only) |
  | Micro-unit Compute Tracking | No | No | Yes (Metered) | N/A |
  | Idempotent Event Deduplication | Partial | N/A | Yes | N/A |
  | Owner-facing Cost Estimates | No | No | No | N/A |

  ## Design Doc
  - **Architecture**: Introduce a centralized usage accounting service that captures and persists micro-unit usage events.

  ```mermaid
  graph TD
      A[Agent Workflow] -->|Emits micro-unit usage event| B(Usage Event Receiver);
      B --> C{Deduplication Check};
      C -->|Duplicate| D[Drop Event];
      C -->|New| E[Usage Datastore];
      E --> F{Payer Identity Check};
      F -->|BYOK| G[Record as Customer-funded];
      F -->|OHC Managed| H[Record as OHC-funded];
      H --> I[Aggregate for Billing];
      G --> J[Display as Zero-cost on Invoice];
  ```

  - **Data Flow**: Usage records will separate OHC-funded compute from BYOK inference. Customer BYOK usage will be tracked for visibility but excluded from the OHC inference debit.
  - **User Experience**: Owners will see estimated task costs and maximum authorized spend before execution. After execution, they will receive an itemized bill distinguishing OHC services from direct provider usage.

  ## Implementation Prompt
  Implement a granular usage billing and telemetry system. Capture durable, deduplicated events for resource consumption (CPU, memory, tool usage) and model token usage. Ensure that BYOK inference costs are explicitly separated from OHC's own resource costs. Provide an itemized cost summary for the owner. Do not implement UI changes, focus on the telemetry and backend event persistence with 100% test coverage.

  ## Priority
  P1

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - OHC Target ID: OHC-09/10
  - Stage: Run
  - Evidence level: Inferred gap based on codebase audit and provider documentation.
  - Baseline and Result: Currently no transparent usage attribution. Result is accurate, deduplicated micro-unit usage events per tenant.
  - Authority class: Strict read-only for usage aggregation.
  - Cost impact: Reduces unbilled inference overhead, enables accurate contribution margin calculation.
  - Acceptance checks: Deduplicated events binding tenant, task, provider, and payer; BYOK inference explicitly excluded from OHC debit.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
