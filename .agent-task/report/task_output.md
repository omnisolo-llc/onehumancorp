issue_title: "OHC-09: Measurable retention/acquisition experiment"
issue_description: |
  Title: OHC-09: Measurable retention/acquisition experiment
  Problem Statement: The current business capability map lists the target OHC-09 as a "measurable retention/acquisition experiment" in the growth lane. However, this is blocked by the lack of verified core business loop execution and baseline economics. We must defer new viral generators, referral loops, and gamified acquisition until the first digital-service loop meets paid retention, reliability, and economic viability gates.
  Research Report: Growth loops (like referral widgets or viral loops) require that the core retention and economic baseline are verified first. Current target OHC-09 is listed as Verified (No-Work) in RESEARCH.md. Growth tasks are constrained to not deploy unproven mechanics when earlier requirements (such as customer acquisition to quote, and payment) remain untrusted.

  Verification Details:
  - loaded Superpowers skills revision: `8ca22dba9a94f28898bbce59f2537ff4d87c747d`
  - `make lint` failed via timeout due to missing dependencies/environmental limits (glib-2.0, PG module). Fallback `npm run lint:node` executed successfully.
  - `make test` failed via timeout due to environmental limits.

  Design Doc: N/A - Blocked.
  Implementation Prompt: N/A - Blocked due to missing prerequisites.
  Priority: P2
  Estimated Scope: Blocked / No-Work
issue_priority: P2
issue_category: growth
issue_type: research
issue_label: ohc:lane:growth
assignees: []
