outcome: no_work
issue_title: "Implement High-Converting Viral Share-to-Unlock Loop for Promoters"
issue_description: |
  I investigated the request to implement a High-Converting Viral Share-to-Unlock Loop for Promoters (GitHub Issue #36891).
  Based on `RESEARCH.md`, under the "What to defer" section, it explicitly states: "Defer new viral generators, referral badges, paywalls, agent marketplaces, additional harness adapters, visual workflow builders, simultaneous HR/payroll/MRP coverage, generic UI restyling and unsupported global compliance claims unless an accepted issue shows they block the selected business outcome."
  Since this feature is a new viral generator/referral badge and there is no evidence it blocks a selected business outcome, this feature is deferred. Thus, no functional code changes are required.

  Loaded Superpowers skills and revision hash:
  - Repository: https://github.com/obra/superpowers.git
  - Revision hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Skills loaded: skills/using-superpowers/SKILL.md (fetched to /tmp/superpowers)

  Verification checks and outcomes:
  - Checked `RESEARCH.md` constraints: Confirmed conflict with "What to defer".
  - Executed `make test` and `make lint` to ensure repository integrity.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
