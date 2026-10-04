outcome: blocked
issue_title: Autonomous Agent-Driven Invoicing & Receivables Management
issue_description: |
  Blocked finding for Issue #34755 (Autonomous Agent-Driven Invoicing & Receivables Management).

  The issue requests building proactive drafting of invoices based on event hooks, and automated receivable follow-ups (drafting polite reminder emails). However, based on the `RESEARCH.md` (revision 2026-09-18-usage-audit) and `docs/research/native_migration_and_remediation.md`, simulating autonomous actions or generating completed invoices without verified owner-approved line items is explicitly identified as false completion (F12) and fictitious receivables reminders (F09). Both F09 and F08 (fabricated checkout links) have been remediated in the codebase, preventing the creation of fictitious drafts, URLs, and states.

  Additionally, the `RESEARCH.md` states:
  > "Do not implement them as billing or segment constraints... Do not deploy, publish releases, contact prospects, spend money or mutate live provider/customer data without explicit applicable authorization."
  > "The existing Codex adapter and API/OAuth credential types are reuse assets, not that demonstration. ... Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research."
  > "Can a recurring follow-up stop on payment, cancellation or revoked authority? ... Whether ongoing automation saves net owner time rather than requiring supervision. ... These are decision prerequisites, not an approved feature backlog"

  The audit `docs/research/business_capability_and_usage_economics_audit.md` explicitly lists:
  > "Implementation Blockers (Recorded 2026-09-19)
  > - Evaluation of managed API vs API-key/cloud-account billing is currently blocked pending real usage data and owner interviews.
  > - Evaluation of provider-permitted native-client subscription vs local inference is blocked due to missing specific provider access prerequisites."

  The requested workflow relies on simulated milestone completion events to trigger autonomous invoicing, which violates the strict rule against simulated provider sandbox behavior without explicit, separate authorization. Furthermore, `docs/research/native_migration_and_remediation.md` shows F09 ("fictitious receivable reminders") has been closed by removing unused mock modules that logged drafts, and the mounted worker persists *actual* source-grounded drafts. The issue explicitly requests generating these drafts *autonomously* via an event trigger, which currently lacks the necessary real usage data, owner interviews, and strict business-rules engine needed to safely determine scope/amounts (which is why `draft_invoice_from_context` explicitly returns a `FailedPrecondition`).

  The codebase currently implements a strict `CreateInvoiceRequest` requiring owner-approved explicit line items and blocks generic context-based drafting. Attempting to build the requested proactive agent feature would require fabricating authorization logic and bypassing these newly remediated safety constraints.

  Loaded skills:
  - using-superpowers/SKILL.md
  - brainstorming/SKILL.md
  - systematic-debugging/SKILL.md
  - writing-plans/SKILL.md
  - executing-plans/SKILL.md

  Skill Revision Hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
