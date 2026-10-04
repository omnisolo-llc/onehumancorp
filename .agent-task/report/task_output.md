outcome: blocked
issue_title: Implement Multi-Channel Unified Inbox with AI Contextual Drafting
issue_description: |
  **Blocked Finding: Missing Prerequisites**

  The task requires implementing a "Multi-Channel Unified Inbox with AI Contextual Drafting" (Issue #28114).

  However, the `RESEARCH.md` document states:
  > New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md.

  And further in `RESEARCH.md`:
  > Defer new viral generators, referral badges, paywalls, agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome.

  And `AGENTS.md` states:
  > Current work is evidence-first: inspect implemented/mounted behavior, public owner stories and current provider products before committing to new features or a segment.
  > New epics need an explicit evidence-backed decision; assigned concrete defect work may continue.

  Since we lack the required "expansion gate" evidence, authority, and live API credentials/sandbox configurations for Meta Graph API (Instagram/WhatsApp) and Twilio (SMS), this implementation is blocked. The prompt explicitely forbids improvising new features or assuming standing authority outside of existing authorized bounds without explicit approval.

  I am reporting a blocked/no-work outcome due to these missing prerequisites.
