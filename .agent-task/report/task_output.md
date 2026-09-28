issue_title: "🗺️ Guide: [blocked no-work finding: F13]"
issue_description: |
  Target F13: API key, consumer plan and native-client subscription are distinct.

  **Executed test commands:**
  - `make test-e2e`

  **Verified trace limitations:**
  - `make test-e2e` timed out after 401 seconds. Compiling the required Cargo binaries takes longer than the available session limit.

  This environment limitation prevents running full acceptance gates and justifies a blocked no-work finding.

  Loaded skills: `skills/using-superpowers/SKILL.md` (rev 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: "P0"
issue_category: "reliability"
issue_type: "defect"
issue_label: "ohc:lane:reliability"
assignees: []
