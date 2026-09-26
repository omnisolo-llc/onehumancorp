issue_title: "F14: No measured representative serving costs or owner outcomes"
issue_description: |
  # F14: No measured representative serving costs or owner outcomes

  ## Problem Statement
  The usage audit and remediation ledger identified issue F14: "No measured representative serving costs or owner outcomes". The codebase lacks instrumentation to measure deployment costs, representative workload distributions, or reconciled provider invoices. Additionally, there are no recorded customer interviews to establish a willingness-to-pay baseline or actual owner outcomes from the onboarding flows.

  ## Research Report
  The current onboarding flow in `src/ui/next/src/app/onboarding/page.tsx` implements a simulated business setup sequence. While the UI visually demonstrates "building a business" and "generating a product catalog," this is not backed by actual customer data or verified economic viability.

  The instruction manual explicitly states:
  > Collect a small, permissioned set of recent owner workflows across candidate segments before choosing a segment. Compare each against both its existing manual/SaaS process and the current AI business tools. Exact willingness to pay, usage tolerance, privacy preference and desired autonomy remain unknown. Public stories help select questions, not answer those commercial questions conclusively.

  ## Blocked Prerequisites
  A "no-work" finding is reported because creating further UI features is blocked by missing critical business and usage data:
  1. No real-world customer usage metrics to analyze cost structure (CPU, API tokens, wait time).
  2. No permissioned customer interviews exist to validate the current simulated onboarding utility versus existing manual tools.
  3. No willingness-to-pay validation exists to determine pricing and budget structures.

  ## Required Diagram
  ```mermaid
  graph TD
    A[Simulated Onboarding Flow] --> B{Lack of Real World Metrics}
    B --> C[Missing CPU/Token Data]
    B --> D[Missing Willingness-to-Pay Baseline]
    B --> E[Missing Owner Context]
    C --> F[Blocked: Cannot Design Cost Structure]
    D --> F
    E --> F
  ```

  ## Superpowers Workflow Provenance
  - Loaded skills: `superpowers:using-superpowers`
  - Repository URL: `https://github.com/obra/superpowers.git`
  - Revision hash: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - Checks performed: Codebase search for existing onboarding state (`src/ui/next/src/app/onboarding/page.tsx`); review of `docs/research/business_capability_and_usage_economics_audit.md` for commercial constraints and `docs/research/native_migration_and_remediation.md` for issue F14.
  - Outcomes: Confirmed that further visual wizard updates are blocked by the absence of required owner metrics and willingness-to-pay data. This requires external business research prior to code implementation.
issue_priority: P1
issue_category: RESEARCH
issue_type: TASK
issue_label: ohc:lane:activation
assignees: []
