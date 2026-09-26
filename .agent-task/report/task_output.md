issue_title: '🚀 Nova: OHC-09 Measurable retention/acquisition experiment'
issue_description: |
  **Title:** OHC-09: Measurable retention/acquisition experiment

  **Problem Statement:**
  We need a consent-aware campaign/rebooking loop that attributes qualified leads, paid work, spend, and margin while automatically respecting limits.

  **Research Report:**
  According to the current audit and `RESEARCH.md`, the `OHC-09` target is blocked. It depends on `OHC-04–08` (inquiry/proposal, booking/deposit, delivery/review, final payment, durable exceptions). The active business-capability map and the current remediation ledger show that full real-stack execution of the core loop (like provider event reconciliation in F08, real send paths in F09) remains outstanding. We cannot build a measurable retention/acquisition experiment until the core client-to-cash funnel is verified. Therefore, this is a blocked/no-work outcome.

  **Blocked Prerequisites:**
  - OHC-04: Customer inquiry → qualified quote
  - OHC-05: Accepted quote → deposit → conflict-free booking
  - OHC-06: Delivery → invoice → collected/reconciled balance
  - OHC-07: Autonomous exceptions and owner outcome feed
  - OHC-08: Overdue collection, refunds and customer recovery

  **Funnel Diagram:**
  ```mermaid
  flowchart TD
      Lead[Qualified Lead] --> Booking[Booking/Deposit]
      Booking --> Delivery[Delivery & Invoice]
      Delivery --> Collection[Collected Balance]
      Collection --> Rebook[Consent-aware Rebooking Campaign]
      Rebook --> Lead
  ```

  **Superpowers Workflow Provenance:**
  - Loaded skills: `using-superpowers`, `writing-plans`, `executing-plans`, `verification-before-completion`, `subagent-driven-development`
  - Repository URL: https://github.com/obra/superpowers.git
  - Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks performed: Read skills documentation locally in `.scratch/superpowers/skills`
  - Outcomes: Verified requirements for process workflows, validation, and execution. Determined this is a research task based on scope and role instructions, producing a strictly formatted YAML report in `.agent-task/report/task_output.md` with blocked prerequisites.

issue_priority: 'P1'
issue_category: 'growth'
issue_type: 'feature'
issue_label: 'ohc:lane:growth'
assignees: []
