issue_title: '🚀 Nova: No-work/blocked on Growth Funnel due to missing data'
issue_description: |
  # Superpowers Workflow Provenance
  - **Loaded skills:** skills/using-superpowers/SKILL.md, skills/brainstorming/SKILL.md, skills/systematic-debugging/SKILL.md
  - **Repository URL:** https://github.com/obra/superpowers.git
  - **Revision hash:** 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - **Checks performed:** Codebase structure mapping (specifically looking at API and billing flows), verification of funnel capabilities, review of existing OHC-01-12 priorities.
  - **Outcomes:** Blocked. As Principal Growth Engineer & Nova (L7), the task requires optimizing an evidenced customer acquisition or retention gap. The current environment lacks customer telemetry, active funnel data, or user willingness-to-pay evidence. Attempting to build an arbitrary growth loop without this data violates the explicit requirement to use evidence-backed gaps rather than fabricating an arbitrary viral widget or conversion target.

  # Problem Statement
  We lack a verifiable baseline to build a concrete growth optimization. The audit confirms that while foundational features (onboarding, invoicing, checkout drafts) exist, real-world customer usage, baseline conversion, and provider metrics are absent. Specifically, the mandate prohibits inventing a viral loop, paywall, or new funnel step based on an assumed segment.

  # Blocked Prerequisites
  - Representative customer-serving cost and real billing reconciliation evidence.
  - Baseline funnel telemetry metrics (e.g., proposal conversion rate, inquiry qualification success, retention tracking) for an active user segment.
  - Completed owner workflow profiles proving the initial acquisition-to-payment cycle works end-to-end with real money.

  # Missing External Data
  We do not have actual customer interactions, connected Stripe live data, or true cost measurements (beyond engineering compilation times) necessary to prioritize the highest-impact growth funnel drop-off points.

  # Funnel Diagram
  ```mermaid
  graph TD
    A[Visitor views offering] -->|Lack conversion data| B[Submit inquiry / intake]
    B -->|Lack proposal data| C[Proposal sent]
    C -->|Lack billing metrics| D[Accept and Pay deposit]
    D --> E[Fulfillment & Delivery]
    E --> F[Final Payment & Retained Value]
  ```

issue_priority: 'P2'
issue_category: 'growth'
issue_type: 'report'
issue_label: 'agent-report'
assignees: []
