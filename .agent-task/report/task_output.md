issue_title: "Resolve F12 Simulation, Unknown Provider Outcome, and Approval Paths Looking Like Completion"
issue_description: |
  **Research findings & blocked implementation**

  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md, skills/brainstorming/SKILL.md, skills/systematic-debugging/SKILL.md)

  The assigned F12 defect states: "Simulation, unknown provider outcome and approval paths can look like completion. Required remediation: Truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths."

  The problem states that paths awaiting external approvals, as well as provider responses that are simulated or unknown, present themselves in the system as "completed." A step should not be complete until a confirmed, non-simulated provider receipt or explicit owner/client approval is recorded in the ledger.

  However, this task is marked as blocked and returning `no_work` because there is currently no implementation within the codebase to apply this exact transition. The F12 requirement describes changes to the state transitions of external provider responses and explicit tracking via `provider_receipt_id`.

  A full search for `provider_receipt_id` and the context around `provider_receipt` reveals that there are no such implementations.

  The F12 issue as detailed in the user's issue prompt cannot be completed because the codebase is missing the specific provider execution framework (e.g. Stripe checkout URL) that should be refactored to emit state changes rather than false completions.

  Without existing provider-specific code that exhibits these symptoms, there is no place to implement a bug fix. The codebase needs the provider framework to be completely built out first.
outcome: blocked
