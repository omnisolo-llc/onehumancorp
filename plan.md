1.  **Phase 1 (UltraPlan/Decomposition):** Update database designs
    *   Create a new migration `032_shared_tasks_parent_plan_id.sql` to add the `parent_plan_id` column to `shared_tasks`.
    *   Update `srcs/server/db/BUILD.bazel` to include this new migration in `embedsrcs`.
2.  **Phase 2 (Orchestration):** Update Teammate Mesh APIs (Tasks)
    *   Modify `CreateTaskWithPlan` in `srcs/server/orchestration/tasks.go` to properly insert the `parent_plan_id` when inserting into the database, handling SQLite and Postgres syntax.
    *   Update the `SharedTask` model.
    *   If tests fail, fix them!
    *   Update `srcs/server/orchestration/tasks_test.go` to cover creating a task with a plan ID.
3.  **Phase 3 (autoDream):** Teammate mesh integration
    *   Ensure any sub-task routing honors the `parent_plan_id` effectively. (Checked: tasks.go has basic support but requires DB column and query updates to fully flow through).
4.  **Finalize:** Pre-commit checks
    *   Run all tests using `bazelisk test //srcs/server/...`
    *   Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
    *   Submit
