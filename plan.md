1. **Analyze existing missions**: Check `.agent-task/missions/` for existing `shared_tasks` missions.
2. **Draft new mission file**: Follow the protocol to construct a new mission file named `.agent-task/missions/$(date -u +%Y-%m-%dT%H-%M-%SZ)_kairos_shared_task_list_schema.md`.
3. **Write Mission content**: Include Title, Problem Statement, Research Report, Design Doc, Implementation Prompt, Priority, and Estimated Scope as per KAIROS architecture doc. Ensure it addresses the backend database design (PostgreSQL & SQLite) and sequence diagrams for the Shared Task List.
4. **Run pre-commit instructions**: To verify the plan is properly documented and checks are done.
5. **Submit**: Create PR.
