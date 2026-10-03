outcome: blocked
issue_title: "Implement Agentic Hybrid Scheduling & Deposit Collection Flow"
issue_description: |
  **Loaded skills:**
  - `skills/using-superpowers/SKILL.md`
  - `skills/brainstorming/SKILL.md`
  - `skills/systematic-debugging/SKILL.md`
  - `skills/writing-plans/SKILL.md`
  - `skills/executing-plans/SKILL.md`

  **Skill Provenance:** 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  **Findings & Blockers:**
  The issue requests building an "Agentic Hybrid Scheduling & Deposit Collection Flow" based on legacy assumptions (like Maya the Baker persona and a missing AI agent capability). The repository's `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md` explicitly state:

  1. "The previous $99 subscription, 300-step allowance, $299 setup, fixed pilot conversion/margin thresholds and exclusive web/design/marketing segment are suspended hypotheses, not accepted requirements."
  2. "Evaluation of managed API vs API-key/cloud-account billing is currently blocked pending real usage data and owner interviews."
  3. "Evaluation of provider-permitted native-client subscription vs local inference is blocked due to missing specific provider access prerequisites."
  4. The implementation of new capabilities like "Agentic Hybrid Scheduling & Deposit Collection Flow" is blocked because "Before choosing rates, measure actual model/tool usage, compute and reserved capacity...". The baseline metrics are not yet established.
  5. The task instructs to implement a new `OrderProposalCard` component with specific design constraints, but `RESEARCH.md` states: "Keep proposal, launch, retail and field-service paths as candidates until code verification and owner evidence justify selection." There is no owner evidence or verified code to justify implementing this specific feature yet.

  Therefore, the requested work (creating the `OrderProposalCard` and related workflows) is blocked by the explicit requirement to gather real usage data, owner interviews, and establish baseline economics before proceeding with new feature implementation.
