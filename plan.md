1. **Understand Task and Current State**
   - The original user task was to implement `ClaimTask` in `srcs/server/orchestration/tasks_db.go` using database-level locking (`SELECT FOR UPDATE SKIP LOCKED`).
   - The PR reviewer requested this fix because my previous implementation was hallucinated and completely incorrect.
   - We must edit `srcs/server/orchestration/tasks_db.go`.

2. **Implement `ClaimTask`**
   - The query in `claimTaskPostgres` uses `FOR UPDATE SKIP LOCKED`. Wait, `FOR UPDATE SKIP LOCKED` applies to `SELECT`, which the code does, but then we do an `UPDATE` in a separate query. Is there a better way, like combining them in one query? Let's look at standard PG queue tables.
   - Instead of a `SELECT ... FOR UPDATE SKIP LOCKED` followed by `UPDATE`, we can do a single `UPDATE ... WHERE id = (SELECT id FROM ... FOR UPDATE SKIP LOCKED) RETURNING ...`. Let's implement this.

3. **Verify Tests**
   - Run `bazelisk test //srcs/server/orchestration/...`.

4. **Mark Mission In-Progress and Done**
   - Since the previous mission state was wiped when reverting, I should create a new `DONE` mission file mimicking completion of the original task.
