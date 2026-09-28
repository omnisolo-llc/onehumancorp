issue_title: "💰 Miser: [blocked no-work finding: F13]"
issue_description: |
  **Title**: Investigating F13 - BYOK versus subscriptions

  **Problem Statement**:
  The F13 finding requires evaluating API key, consumer plan, and native-client subscription modes and ensuring provider-specific modes and fail-closed unsupported combinations. It prohibits session-token relay, pooling, silent paid fallback, or rebilling direct inference.

  **Research Report**:
  This task is primarily research-oriented to evaluate pricing capability, BYOK versus subscriptions. A blocked no-work finding is appropriate since no code changes are required for research-only tasks and there's no native implementation that needs changing to complete this requirement. The documentation confirms that API key, consumer plan, and native-client subscription are distinct, and that provider-specific modes should be used. However, it also explicitly states that "Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented."

  **Executed test commands**:
  - `git status` (Passed)

  **Verified trace limitations**:
  This is a blocked no-work finding. As a primarily research-oriented task investigating F13, no code changes are required or possible given the current implementation state.

  **Final Evidence**:
  Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)
  (Passed)
issue_priority: "P0"
issue_category: "research"
issue_type: "blocked"
issue_label: "ohc:lane:finance"
assignees: []
