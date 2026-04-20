1. **Add `shared_tasks` table migrations in Go:**
   - Create `srcs/server/db/migrations/20260424010000_shared_tasks_kairos_pg.sql` and `srcs/server/db/migrations/20260424010000_shared_tasks_kairos_sqlite.sql` as requested.
   - Wait, looking closely at my previous `ls srcs/server/db/migrations/ | grep shared_tasks`, I see there are already numerous migrations touching `shared_tasks`. Does it matter if I add another one with exact CREATE TABLE IF NOT EXISTS? Let me check `srcs/server/db/BUILD.bazel`. I will just add the sql files and include them in `BUILD.bazel`.
   - Update `BUILD.bazel` to include `"migrations/20260424010000_shared_tasks_kairos_pg.sql"` and `"migrations/20260424010000_shared_tasks_kairos_sqlite.sql"` in `embedsrcs`.

2. **Implement `LocalTeammateMesh`:**
   - Modify `srcs/server/orchestration/kairos/mesh.go` to include a `LocalTeammateMesh` struct that implements `TeammateMesh` and explicitly exposes the `mesh:tasks` and `mesh:coordination` channels.
   - Modify `srcs/server/orchestration/kairos/mesh_test.go` to test `LocalTeammateMesh` logic.
   - (I've already applied this change to both files, and tests pass.)

3. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
