issue_title: Usage Economics and BYOK Architecture Evaluation
issue_description: "Title: Usage Economics and BYOK Architecture Evaluation\n\nProblem\
  \ Statement:\nAs a small business owner (like Nora or Maya), I want to use AI to\
  \ handle routine operations, but I need clear predictability around costs. If I\
  \ am charged based on API usage, I need an invoice-grade meter that accurately attributes\
  \ my cost without duplicate charges if I bring my own API key (BYOK) or native subscription.\
  \ Right now, there is no measured representative serving cost, owner outcome baselines,\
  \ or a durable idempotent usage ledger to support reliable billing, meaning we cannot\
  \ safely charge owners yet.\n\nResearch Report:\n- **Findings**: The `business_capability_and_usage_economics_audit.md`\
  \ shows telemetry/cost reports are not invoice-grade (F05) and there are no measured\
  \ representative serving costs or owner outcomes (F14, marked as Verified blocked\
  \ due to missing prerequisites and economic data). Existing DB telemetry coexists\
  \ with process counters but does not provide a durable, idempotent ledger with integer\
  \ subunits and exact payer attribution.\n- **BYOK and Provider Access**: The system\
  \ must distinguish between OHC-funded inference and customer-funded API keys/subscriptions.\
  \ We cannot duplicate debits or pool subscriptions.\n- **Strategy Admission**: Stable\
  \ OHC target ID: OHC-14. Selected customer/stage: General AI Operations. Evidence\
  \ level: Source code inspection reveals lack of invoice-grade metering and measured\
  \ baselines. Baseline/result metric: Need 100% accurate, idempotent attribution\
  \ of AI compute to tenant IDs before enabling billing. Dependencies/reuse: Existing\
  \ `auditor.rs` and `pricing/budget.rs`. Non-goals: Implementing a generic ERP or\
  \ arbitrary subscription billing outside of AI compute. Authority class: Server-enforced\
  \ limits and billing policies. Cost plan: Unblocks usage billing. Happy/failure-path\
  \ acceptance checks: Invoice agrees exactly with provider receipt, BYOK users are\
  \ not billed by OHC.\n\nDesign Doc:\n- **Architecture diagram**:\n```mermaid\nsequenceDiagram\n\
  \    participant Owner\n    participant App as Mobile UI\n    participant ProviderFacade\
  \ as Provider Facade\n    participant Auditor as Cost Auditor\n    participant Provider\
  \ as External AI / API\n    \n    Owner->>App: Initiate AI Task\n    App->>ProviderFacade:\
  \ Request Inference (with Auth Mode: BYOK or Managed)\n    alt BYOK / Native Subscription\n\
  \        ProviderFacade->>Provider: Forward Request with Owner Credentials\n   \
  \     Provider-->>ProviderFacade: Response + Usage\n        ProviderFacade->>Auditor:\
  \ Record Usage (Zero Cost to OHC, BYOK Flag=true)\n    else Managed API\n      \
  \  ProviderFacade->>Provider: Request with OHC Key\n        Provider-->>ProviderFacade:\
  \ Response + Usage\n        ProviderFacade->>Auditor: Record Usage (Idempotent,\
  \ Cost Assigned to Tenant)\n    end\n    Auditor-->>App: Return Updated Budget &\
  \ Outcome\n```\n- **UI wireframes**:\n  - The \"Billing & Limits\" screen displays\
  \ current usage versus budget using macOS-style Translucent Glass cards.\n  - A\
  \ clear toggle or credential input for \"Bring Your Own Key\" (BYOK) or native AI\
  \ subscription login.\n  - Cost breakdowns show integer subunit accuracy and clearly\
  \ distinguish between OHC usage and BYOK usage.\n- **Mobile UX flow**:\n  - On a\
  \ 375px viewport, the user taps the \"Settings\" tab, then \"Billing\".\n  - They\
  \ see a simple progress bar indicating their current AI budget usage.\n  - If they\
  \ tap \"Connect my API Key\", a safe, validated modal appears to enter the key securely.\n\
  - **AI agent integration points**:\n  - The `ProviderFacade` must reliably intercept\
  \ usage statistics from every model interaction and emit an immutable, idempotent\
  \ event to the `CostAuditor`.\n  - Agents must query budget reservations before\
  \ executing large tasks.\n- **Key design decisions**:\n  - Multi-tenant isolation\
  \ must guarantee usage is never attributed to the wrong tenant.\n  - Decouple ingestion,\
  \ accounting, and export (resolving F01 risks) to prevent exponential queue growth\
  \ or duplicate charges.\n\nImplementation Prompt:\nImplement the durable usage metering\
  \ and BYOK classification architecture.\n- Update the `ProviderFacade` to explicitly\
  \ distinguish between BYOK/native subscription requests and OHC-managed requests.\n\
  - Implement an idempotent, invoice-grade meter in `CostAuditor` that records usage\
  \ with exact payer, auth mode, and rate attribution.\n- Expose a mobile-friendly\
  \ API endpoint for owners to view their exact usage and connect their own API credentials\
  \ securely.\n- **Acceptance Criteria**: BYOK requests result in zero OHC cost attribution.\
  \ Managed requests deduct from tenant reservations precisely. Usage events are deduplicated.\n\
  \nPriority: P2\nEstimated Scope: Large"
issue_priority: P2
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
