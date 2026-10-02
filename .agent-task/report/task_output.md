outcome: no_work
issue_title: "Implement Native Rust Omnichannel Inbox & Chat Engine"
issue_description: |
  **Issue Resolution Engineer & Taskmaster (L5) Report**

  **Issue:** #36305: Implement Native Rust Omnichannel Inbox & Chat Engine

  **Finding:** No-work outcome.

  **Rationale:** The issue requests the implementation of a native Rust replacement for the external third-party service Chatwoot. However, the `RESEARCH.md` strategy (revision 2026-09-18-usage-audit) explicitly states: "Integrate existing tools instead of rebuilding them... New epics need an explicit evidence-backed decision; assigned concrete defect work may continue... The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES, including wherever role prompts below repeat them... Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately... Measure OHC-funded inference... Track owner review/correction effort and verified business results alongside cost". `RESEARCH.md` also notes: "Existing OHC-01–12 IDs remain useful references, not permission to implement their former scopes." `RESEARCH.md` further states: "A justified no-change result is valid."

  Building native replacements for external services (like Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected with a `no_work` finding.

  **Superpowers Workflow Provenance:**
  Loaded skills from https://github.com/obra/superpowers/
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Skills verified: `using-superpowers`
