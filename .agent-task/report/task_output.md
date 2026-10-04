outcome: blocked
issue_title: "GitHub Issue #32693: Implement AI Work Triage & Automated Lead Recovery Feed"
issue_description: |
  Upon review of the provided task "Implement AI Work Triage & Automated Lead Recovery Feed", and cross-referencing it with the One Human Corp operating contract (revision 2026-09-18-usage-audit) and the provided `RESEARCH.md`, I am determining this task is currently blocked due to missing explicit evidence-backed approval.

  The instruction requires us to "Implement AI Work Triage & Automated Lead Recovery Feed". However, `RESEARCH.md` states:
  > The original audit found the desktop shell consuming exported next_out assets. ... This is not yet a proven moat.
  > New epics need an explicit evidence-backed decision; assigned concrete defect work may continue.
  > Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately... Before choosing rates, measure actual model/tool usage, compute and reserved capacity...
  > "Existing OHC-01-12 identifiers remain cross-references; F01-F15 are audit/remediation references. Neither list is a substitute for a current GitHub issue with reproduced evidence, a bounded scope and acceptance criteria. A research session may return no new work without creating a duplicate issue."

  The current codebase contains foundational models for `triage_items`, and endpoints to load triage items (`load_ui_triage_from_db`), however, building out the requested complete "AI Work Triage & Automated Lead Recovery Feed" with LLM intent parsing, automated draft responses, and an optimized 375px native mobile UI is an expansion of the "business capability map" (specifically addressing "Answer and qualify inquiries" and "Turn interest into revenue") that constitutes a new epic.

  According to `RESEARCH.md` / `AGENTS.md`: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require explicit evidence and authorization via the expansion gate in RESEARCH.md; otherwise, return a no_work or blocked finding."

  Furthermore, the issue description states: "Through deep research into r/smallbusiness and app store reviews, we found that missed leads cost service businesses up to 30% of potential revenue."
  However, `RESEARCH.md` strictly instructs: "Public owner stories do not establish willingness to pay or a statistically dominant customer segment." and "These are vendor evidence, not proof of OHC demand. ... Compare each against both its existing manual/SaaS process and the current AI business tools. Exact willingness to pay, usage tolerance, privacy preference and desired autonomy remain unknown. Public stories help select questions, not answer those commercial questions conclusively."

  The requested work is a new epic lacking the required owner economic/metric data and explicit strategy approval. I am blocked from implementing this feature until the prerequisite research (actual usage economics, owner interviews, willingness to pay, and a reconciled cost model) is conducted and formally approved in the `RESEARCH.md` expansion gate.

  Skill Provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Loaded Skills: skills/using-superpowers/SKILL.md
