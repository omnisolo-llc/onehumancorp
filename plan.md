Wait! I need to ensure that when I pick up the mission I mark it `status: IN_PROGRESS` and `agent: Implementer` (which is already there, but I will make sure the frontmatter only has one `status` and `agent` line as per the memory rule).

Let's review the required steps:
1. Update `.agent-task/missions/2026-04-12T20-40-33Z_kairos_master_mission.md` to `status: IN_PROGRESS`.
2. Create `srcs/server/db/migrations/035_kairos_shared_tasks.sql` based exactly on the provided snippet.
3. Create `srcs/server/db/migrations/035_kairos_autodream.sql` based exactly on the provided snippet.
4. Run `bazelisk test //...` to ensure no tests are broken and coverage is maintained.
5. Create `.agent-task/status/$(date +%s).yml` observability heartbeat metric file.
6. Clean up temporary files.
7. Complete pre-commit instructions.
8. Submit PR and update `.agent-task/missions/2026-04-12T20-40-33Z_kairos_master_mission.md` to `status: DONE`.
Wait, the order of 8 is: Update to DONE, then run pre-commit instructions, then Submit PR. Let me make sure `status: DONE` is set BEFORE submitting.

Wait! The agent identity rule says: "When acting as the Implementer agent, manage mission states by updating the target mission file's frontmatter in `.agent-task/missions/`: set `status: IN_PROGRESS` and `agent: {name}` when starting, `status: DONE` when finished..." My name is `Implementer` based on Swarm Category.

Let's write out the plan.
