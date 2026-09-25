issue_title: '🛡️ Sentry: [No-work: missing target for reliability fix]'
issue_description: |
  # Task Evaluation

  ## Goal
  Act as Principal Reliability Engineer & Sentry (L7) and "fix one reproduced gap; preserve meaningful end-to-end coverage."

  ## Blockers
  1. No specific open test failure or reproduced defect is given or identified in the project state. The migration ledger indicates many findings are actually "Closed" or have already been implemented ("Production changes already present").
  2. For the open items (F04, F05, F08, F10, F11, F12, F13, F14, F15), they are large architectural milestones or dependent on outside contexts (like provider sandbox replay, owner interviews, usage instrumentation framework changes) rather than a simple failing test repair.
  3. The local test environment inside the sandbox times out or fails on build steps (e.g. NextJS `tailwindcss` PostCSS issues that affect the baseline environment, unconnected to any code changes I would make here), making confident test-driven-development and reliability verification impossible here.

  As per constraints: "When forced to submit an implementation Pull Request for a 'no-work/blocked' finding (e.g., lacking required external data or owner interviews), do not fabricate code or create dummy files (e.g., `.jules-dummy-change`) just to create a diff, as this pollutes the repository. Document the blocked prerequisites in `.agent-task/report/task_output.md` formatted as YAML, ensure tests pass without unrelated out-of-scope changes, and submit."
issue_priority: 'High'
issue_category: 'Reliability'
issue_type: 'Blocked'
issue_label: 'Sentry'
assignees: []
