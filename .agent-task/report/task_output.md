issue_title: '🚀 Nova: Referral Link Tracking Gap'
issue_description: |
  # Referral Link Tracking Gap

  ## Problem Statement
  We have a number of disparate viral growth components in the E2E tests, but the core referral link generator in `src/server/services/growth/referral_api.rs` uses a hardcoded fallback (`omnisolo://join?ref={...}`) instead of an actual deep link into the `growth/referrals/click` funnel tracking system shown in the integration tests. This limits the ability to track real attribution of owner success when they share links.

  ## Current State
  The referral link generator returns a naive `omnisolo://` scheme with no standard tracking params recognized by the web growth routes. Many routes in `growth.rs` use hardcoded URLs instead of a central link generator.

  ## Blockers/Findings
  I am acting under the Principal Growth Engineer (Nova) L7 role to build an evidence-backed growth optimization. The rules specify: "Select one existing issue or research uncertainty; do not reopen repaired findings or treat historical pricing/segment targets as requirements. ... A no-work/blocked result with evidence is valid; do not manufacture a viral feature."

  Currently, there is no real customer attribution data, nor any assigned existing issue describing a viral conversion bottleneck based on real production evidence. All existing referral widgets and loops (e.g. Spin To Win, Case Study Generator, Waitlist Generator) are speculative and unvalidated. The user prompt indicates: "Defer new viral generators, referral badges, paywalls... unless an accepted issue shows they block the selected business outcome... A no-work/blocked result with evidence is valid."

  ## Blocked Prerequisites
  - Actual production funnel conversion metrics.
  - Active customer base requesting or failing to use referral features.
  - An accepted issue linking a viral feature to an observed business outcome.

  ## Superpowers Workflow Provenance
  - Loaded Skills: `brainstorming`, `using-superpowers`
  - Repository URL: https://github.com/obra/superpowers.git
  - Revision Hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks Performed: Inspected growth service code and research documents; verified constraints against creating speculative features.
  - Outcomes: Blocked due to lack of evidence for a specific growth optimization.

  ## Diagrams
  ```mermaid
  graph TD
    A[Referral Link Generated] -->|User clicks link| B[Landing Page / Onboarding]
    B --> C[Sign up / Conversion]
    C --> D[Attribution Tracked]
    D -.->|Missing in current code| E[Viral Loop Complete]
  ```
issue_priority: 'P2'
issue_category: 'Growth'
issue_type: 'No-Work Report'
issue_label: 'agent-report'
assignees: []
