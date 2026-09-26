issue_title: '🚀 Nova: Evaluate conversion friction for proposals'
issue_description: |
  # Superpowers Workflow Provenance
  - Loaded Skills: `using-superpowers`, `brainstorming`, `writing-plans`, `executing-plans`
  - Repository URL: `https://github.com/obra/superpowers`
  - Revision Hash: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks Performed: Loaded skill `using-superpowers` and read docs/research/native_migration_and_remediation.md and docs/research/business_capability_and_usage_economics_audit.md.
  - Outcomes: Determined that the current target is an implementation PR or a valid no-work/blocked output due to lack of owner metrics or exact conversion friction points.

  ## Funnel / Flow Blockers
  ```mermaid
  graph TD
    A[Visitor] -->|Friction unknown| B[Draft Proposal]
    B -->|Conversion unknown| C[Approved Proposal]
    C -->|Completion unknown| D[Paid Invoice]
  ```

  **Analysis & Outcome:**
  A "no-work" finding is reported because there is no concrete, numerical baseline or owner interview data establishing where the exact friction lies in the proposal/conversion process. The prior hypothesis of a fixed segment/price is suspended, so building a hardcoded viral loop or paywall without tracking real funnel dropout rates would be premature and violates the "Evidence comes before another concrete product plan" directive.

  **Blocked Prerequisites:**
  - Need explicit owner interviews or telemetry baselines identifying where leads abandon proposals.
  - Need a verified, reproducible tracking metric for the current acquisition channel before injecting a new growth feature like a paywall or referral widget.
issue_priority: 'P2'
issue_category: 'bug'
issue_type: 'bug'
issue_label: 'growth'
assignees: []
