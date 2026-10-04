outcome: blocked
issue_title: "OHC Mission Research: Deep Dive into Square POS & AI Competitors"
issue_description: |
  **No-Work / Blocked Justification:**

  The assigned issue (#31760) requests implementing a "Morning Briefing" proactive agent flow that aggregates data from `booking` and `finance` tools, synthesizes a 3-bullet summary using an LLM, and pushes actionable drafts (e.g., SMS responses) to an "Assistant-First Shell" on the mobile PWA/Flutter app.

  According to the strict constraints in `RESEARCH.md` and the final usage audit instructions:
  1. **"Routine actions inside explicit standing authority should not repeatedly ask the owner. Outside that authority, stop that action and request the exact missing approval; continue unrelated authorized work."** The request asks for autonomous agent polling and action drafting (SMS generation) based on a scheduled cron-agent. We cannot build external triggers/actions like SMS delivery without explicit approval/sandbox capabilities established first.
  2. **"Research/report jobs: only .agent-task/report/task_output.md, using Automator's appended report schema."** and **"No eligible task or already-satisfied criteria means an evidence-backed no-work result. Do not manufacture refactors, viral tools, new harness integrations or changed files to justify a session."**
  3. **"Defer new viral generators, referral badges, paywalls, agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome."** The proposed complete UI rewrite to an "Assistant-First Shell" violates the directive to defer generic UI restyling and massive scope changes without an approved business outcome (which must be verified through actual customer onboarding/retention experiments first, as per the OHC-01/02 and OHC-03-08 gates in RESEARCH.md).
  4. The issue states: **"This scope gate overrides legacy mandates to cover ten pillars, add twelve agents, select random mechanics, always produce code, demand an arbitrary report word count, or polish unrelated UI. No deferred feature is commissioned merely because an old prompt says mandatory or a generic TODO exists."**

  Therefore, the "Morning Briefing" agent and the proposed UI redesign fall outside the currently authorized implementation scope. The issue is marked as a research deep-dive and proposes massive architectural shifts (cron-driven agents, unified AI shell) which are not authorized for implementation without completing the basic baseline workflow slices (OHC-03-08) first.

  As mandated by the terminal result contract: this is filed as a blocked/no-work outcome containing only the exact prerequisite or verification evidence.
