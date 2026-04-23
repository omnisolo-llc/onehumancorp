1. **Explore Codebase**: Discover the required layout and existing schemas for `shared_tasks_decomposition`.
2. **Implement Task Decomposition Schema**: I've created the `20260429000000_shared_tasks_decomposition_table.sql` migration to properly define the PostgreSQL schema and registered it within `srcs/server/db/BUILD.bazel`.
3. **Implement Service Logic**: I've created the `TaskDecompositionService` in `srcs/server/orchestration/tasks/service.go` containing methods to create tasks, update their status to `CLAIMED`, `DONE`, `FAILED`, and specifically pulling tasks with concurrency safely utilizing `FOR UPDATE SKIP LOCKED`.
4. **Implement Service Tests**: Wrote comprehensive test cases in `srcs/server/orchestration/tasks/service_test.go` utilizing the test database provider, achieving 100% coverage on the new service package.
5. **Add Package Build File**: Created `srcs/server/orchestration/tasks/BUILD.bazel` to register the new Go module.
6. **Complete pre commit steps**: Complete pre commit steps to make sure proper testing, verifications, reviews and reflections are done.
7. **Submit**: Once all tests pass, the repository will be correctly aligned with the specification. Output a YAML block containing `issue_id: 5926`.
