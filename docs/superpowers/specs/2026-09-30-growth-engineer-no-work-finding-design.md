# Growth Engineer [no-work finding] Design

## Context
The task is to act as Principal Growth Engineer & Nova (L7) and "Improve an evidenced customer acquisition or retention gap: qualified inquiries, accepted offers, delivered value, collected revenue or repeat paid work."

## Requirements constraints
* **RESEARCH.md**: "The legacy categories below are reference ideas only; referral widgets, share cards, paywalls and viral loops are deferred until retained paid value is proven."
* **RESEARCH.md**: "A no-work finding is valid; do not manufacture a viral feature."
* **Memory**: "In the One Human Corp repository, when submitting a 'no-work finding' research report (e.g., as Principal Performance Engineer), the agent must not submit a generic 'pretend feature issue'. It must perform genuine investigation of the codebase and report concrete findings (e.g., specific files checked, existing caching strategies, or lack of baseline metrics) within `.agent-task/report/task_output.md`."
* **Memory**: "When acting as Principal Growth Engineer & Nova (L7), format the PR title exactly as `🚀 Nova: [new growth feature]` (or appropriate bracketed text like `[no-work finding]`). The PR description must include funnel diagrams or flow screenshots (use 'N/A for no-work finding' if applicable)."

## Approach
The agent will create a `.agent-task/report/task_output.md` file reporting a `[no-work finding]`. The report will include evidence of the existing viral/growth features that were discovered (e.g., `src/ui/next/src/app/viral-goal-tracker/page.tsx`, `src/e2e/viral_goal_tracker.spec.ts`) and state that new viral features are deferred per the `RESEARCH.md` guidelines until retained paid value is proven. No codebase changes will be made other than creating the report.

## Implementation Steps
1. Create `.agent-task/report/task_output.md`.
2. Ensure the content uses valid YAML format as required by the environment (`issue_title`, `issue_description`, etc.).
3. Run `make test && make lint` synchronously to verify workspace is intact.
4. Stage and commit the file.
5. Submit the PR using the submit tool with the title `🚀 Nova: [no-work finding]`.
