issue_title: Evaluation of BYOK, API charging and provider-permitted subscription access
issue_description: |
  # Evaluation of BYOK, API charging and provider-permitted subscription access

  This is a research report detailing the evaluation of metered compute/API charging, customer-funded inference (BYOK), and provider-permitted native-client subscription access for the One Human Corp product, as required by the 2026-09-18-usage-audit scope.

  ## 1. Study populations and usage sources
  The evaluation draws on the constraints and definitions provided in `docs/research/business_capability_and_usage_economics_audit.md`. No new interviews, owner identity verification, payment commitments, provider charges, or production measurements were obtained in the process of generating this report. The report is a synthesis of the existing documentation, source findings, and product requirements. Source date for the audit is 2026-09-18.

  ## 2. Metric definitions and scope
  The analysis differentiates between the following billing and inference modes:
  - **Managed API:** OHC's contracted API account where OHC tracks compute/resources and explicitly prices API consumption.
  - **Customer API key / cloud account (BYOK):** Paid directly by the customer to their provider. OHC must track hosting, storage, and tools, but must not rebill customer-paid inference as OHC consumption.
  - **Provider-native client with an eligible subscription:** Usage funded by a customer's subscription or credits, within provider-permitted integration and auth patterns.
  - **Customer machine / local model:** Funded by customer hardware.

  Cost accounting rules:
  - Usage must be durable, deduplicated, and bound to tenant/project/task/attempt, provider request ID, payer/auth mode, provider/model, and rate-card version.
  - BYOK usage must not be double-charged.

  ## 3. Findings
  The findings reinforce the need to implement reliable accounting and clearly defined payer/auth modes. The current telemetry and usage reporting, while observable, do not meet the criteria for an invoice-grade meter.

  ## 4. Uncertainties and open questions
  - The exact willingness to pay, usage tolerance, privacy preferences, and desired autonomy of the target customer segment remain unknown.
  - A representative workload distribution and reconciled provider invoice have not yet been measured in production.
  - End-to-end provider-permitted native-client subscription access is not fully implemented or validated against provider terms.

  ## 5. Next Actions
  Based on this evaluation, the following steps are recommended:
  1. Complete the implementation of the invoice-grade telemetry system.
  2. Implement strict enforcement and testing for BYOK vs subscription modes.
  3. Validate a small set of permissioned owner workflows to gather actual economic and usage data.

  ## 6. Superpowers workflow provenance
  - Loaded skills: `using-superpowers`, `verification-before-completion`
  - Upstream revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
issue_priority: P1
issue_category: research
issue_type: agent-report
issue_label: [agent-report]
assignees: [jules]
