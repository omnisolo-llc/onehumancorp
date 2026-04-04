1. **Understand the Goal**: The objective is to ensure multi-tenant isolation in the orchestration layer by verifying that `organization_id` is properly required and enforced in all task lifecycle functions within `TaskManager` (e.g., `ClaimTask`, `PollTasks`, `ReviewTask`, `CompleteTask`).
2. **Examine `TaskManager` Implementation**: Find where `TaskManager` is implemented, typically in `srcs/server/orchestration/`. Read through the task lifecycle functions to identify missing `organization_id` enforcement.
3. **Modify `TaskManager`**: Update the interface and implementation of `TaskManager` to explicitly require `organization_id` as a parameter and enforce it in the underlying data access logic.
4. **Update Callers**: Update all callers of these functions across the codebase to pass the `organization_id`.
5. **Pre-commit Checks**: Run tests and follow `pre_commit_instructions`.
6. **Submit PR**.
