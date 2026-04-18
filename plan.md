1. **Understand the Goal**: The user (Automator) has created a test issue (#6076) and says "Please ignore."
2. **Review Role Context**: The prompt says "Do not ask the user for clarification, permission, or confirmation to proceed at any stage (including pre-commit or before opening a PR). You must act independently, trust your judgment, complete all tasks, and submit the changes autonomously." However, this is an Automator test issue meant to be ignored.
3. Wait, let me check the instruction: "This is a test issue for the automator verification loop. [reliability] Please ignore."
4. I should just complete the issue by sending a done message or by submitting an empty commit to "ignore" it? Or I should just submit with a commit that mentions "Automator test issue".
5. Wait, the prompt says "Create a PR for every change. Each PR MUST contain one **FULL feature** or reliability module." and "If you already submitted a change previously, you should continue using the same branch name. Before completing your work with the submit tool, you **must** call `pre_commit_instructions` and follow its instructions to complete pre commit steps. Then call `submit` using a short, descriptive branch name."
6. Let's look at the memory:
   - "When acting as the Principal Reliability Engineer & Sentry (L7) agent in the MAINTAINER swarm, pull request titles must strictly follow the format: `🧹 Maintainer: [Compliance] description`." Wait, that's for Compliance Officer. For Sentry? Memory says: "When acting as the Principal Reliability Engineer & Sentry (L7) agent in the MAINTAINER swarm, strict domain ownership applies to tests/, monitoring/, and lib/resilience/. Modifying files outside this scope is prohibited to prevent merge conflicts."
   - Wait, wait. "When acting as the Principal Release Manager (L7) agent in the MAINTAINER swarm, pull request titles must strictly follow the format: `🧹 Maintainer: [Release] description`."
   - The memory does not specify a PR title format for Sentry, but says: "You are a Maintainer agent... PR titles...". Actually, the prompt says: "When acting as the Principal Reliability Engineer & Sentry (L7) agent in the MAINTAINER swarm, strict domain ownership applies to tests/, monitoring/, and lib/resilience/."
7. The Automator created this test issue. Do I need to create a commit and PR, or just finish the task? Let me check the user prompt again. "Now focus on finishing the following task: GitHub Issue #6076: [reliability] Test issue for Automator. This is a test issue for the automator verification loop. [reliability] Please ignore."
8. If the instruction is "Please ignore", doing nothing and just submitting, or creating a dummy file? If I just call `submit`, it needs some commit. Maybe I can create a dummy chaos test or an empty file? Or just submit with no changes? But `submit` requires a branch, title, etc. Let's see if git allows empty commits.
9. Or I can just write a short script or comment in `lib/resilience/chaos/chaos.go` or `.agent-task/` as a status?
10. Wait! "Agents operating within the RESEARCHER swarm must strictly limit their output...". I'm in MAINTAINER swarm.
11. "Following the Swarm Intelligence Protocol (OHC-SIP), agents must record global intelligence/findings to `.agent-task/memory/{timestamp}.yml` and observability health/status heartbeats to `.agent-task/status/{timestamp}.yml`." I should definitely write to these status and memory files.
12. And since it's an "ignore" issue, I should just create the status heartbeat and then submit.
13. Let's create an execution plan.

Plan:
1. Create a timestamp variable.
2. Record heartbeat/status in `.agent-task/status/{timestamp}.yml`.
3. Force add `.agent-task` to git.
4. Complete pre commit steps to ensure proper testing, verification, review, and reflection are done.
5. Submit the change.
