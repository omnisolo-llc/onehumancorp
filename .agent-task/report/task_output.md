issue_title: '🚀 Nova: OHC-09 Measurable retention/acquisition experiment'
issue_description: |
  **Title:** Evaluate Growth Pipeline for Evidence-Backed Acquisition

  **Problem Statement:** The mandate requires implementing a growth loop improvement (e.g., referral widget, paywall). However, the audit explicitly dictates: "choose one evidence-backed bottleneck in the current client-to-cash funnel... A no-work finding is valid; do not manufacture a viral feature." Currently, there is insufficient recorded customer evidence to justify a specific acquisition intervention without resorting to suspended hypotheses like the $99 pricing tier.

  **Superpowers Workflow Provenance:**
  - Loaded Skills: `using-superpowers`, `brainstorming`, `writing-plans`
  - Repository: `https://github.com/obra/superpowers/`
  - Revision: HEAD at time of cloning.
  - Checks: Validated lack of required prerequisites in `docs/research/business_capability_and_usage_economics_audit.md`.
  - Outcomes: Blocked.

  **Research Report:**
  The `docs/research/business_capability_and_usage_economics_audit.md` indicates that we need to identify an evidence-backed bottleneck. The current state lacks verified funnel metrics for the client-to-cash process. Proceeding with a referral widget or paywall would violate the instruction: "Do not fabricate baselines or participants."

  **Design Doc:**
  - Architecture: N/A
  - UI wireframes: N/A
  - Mobile UX flow: N/A
  - AI agent integration points: N/A

  ```mermaid
  graph TD
      A[Inquiry] --> B[Quote]
      B --> C[Deposit]
      C --> D[Delivery]
      D --> E[Invoice]
      E --> F[Collection]
      F --> G[Repeat Business]
      style G fill:#f9f,stroke:#333,stroke-width:2px
  ```
  *The referral loop would target the transition from F to G, but current baseline data for F is unverified.*

  **Implementation Prompt:** Blocked due to missing prerequisites. No further coding required.
  **Priority:** P2
  **Estimated Scope:** No-work outcome.

issue_priority: P2
issue_category: growth
issue_type: research
issue_label: ohc:lane:growth
assignees: []
