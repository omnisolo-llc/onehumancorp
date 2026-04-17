1. **Define the `VectorStore` interface**
   - File: `srcs/server/autodream/store.go`
   - Action: `run_in_bash_session`
   - Code: Define `package autodream`. Define `type VectorStore interface { Store(ctx context.Context, id string, vector []float32, metadata map[string]interface{}) error; Search(ctx context.Context, vector []float32, limit int) ([]SearchResult, error) }`. Define `type SearchResult struct { ID string; Distance float64; Metadata map[string]interface{} }`. Verify with `ls -l srcs/server/autodream/store.go`.

2. **Implement `PGVectorStore`**
   - File: `srcs/server/autodream/pg_store.go`
   - Action: `run_in_bash_session`
   - Code: Implement `PGVectorStore` conforming to `VectorStore`. Pass `github.com/onehumancorp/mono/srcs/server/db.Provider` as dependency to execute queries (aligning with `s.hub.SIPDB().Provider()`). Write `Store` to insert into `autodream_memories`. Write `Search` using `<->` operator in Postgres (`SELECT id, embedding <=> $1::vector as distance FROM autodream_memories ORDER BY distance LIMIT $2`). Format the vector slice into `[0.1, ...]` string using `fmt.Sprintf` for standard `db.Provider.Exec` execution since we don't have a direct pgx handle. Verify with `cat srcs/server/autodream/pg_store.go`.

3. **Implement `SQLiteVectorStore`**
   - File: `srcs/server/autodream/sqlite_store.go`
   - Action: `run_in_bash_session`
   - Code: Implement `SQLiteVectorStore` conforming to `VectorStore`. Pass `github.com/onehumancorp/mono/srcs/server/db.Provider`. In `Store`, serialize the float32 array into JSON string to store in `autodream_memories`. In `Search`, fetch all vectors in the DB, deserialize JSON, and compute cosine distance in Go memory since SQLite fallback lacks pgvector extensions. Sort and limit results in Go memory. Verify with `cat srcs/server/autodream/sqlite_store.go`.

4. **Implement `AutoDreamWorker`**
   - File: `srcs/server/autodream/worker.go`
   - Action: `run_in_bash_session`
   - Code: Define `type AutoDreamWorker struct { store VectorStore; provider db.Provider; stop chan struct{} }`. Add `NewWorker` constructor. Add `Start(ctx)` loop that runs every 5 seconds. In the loop, fetch un-processed tasks from `agent_session_data` or `autodream_memories` depending on logic, construct a mock embedding (`[]float32` of 1536 size filled with 0.1 for now since external LLMs are mocked out in this pipeline layer), and call `worker.store.Store(...)`. Verify with `cat srcs/server/autodream/worker.go`.

5. **Provide Unit Tests with 100% Coverage**
   - File: `srcs/server/autodream/store_test.go` and `srcs/server/autodream/worker_test.go`
   - Action: `run_in_bash_session`
   - Code: Use `github.com/onehumancorp/mono/srcs/server/db.NewTestProvider()` to mock the db connection. In `store_test.go`, test `PGVectorStore` (skip if driver doesn't support pgvector natively in test or use `SQLiteVectorStore` primarily if test db is sqlite) - actually `NewTestProvider()` creates an SQLite memory DB. So test `SQLiteVectorStore` comprehensively: `TestSQLiteVectorStore_Store` and `TestSQLiteVectorStore_Search`. In `worker_test.go`, test `AutoDreamWorker_Start` by inserting a row, running a tick, and verifying `store.Store` was called (or verifying the DB state). Verify with `cat srcs/server/autodream/store_test.go`. Ensure tests pass via `./bazelisk test //srcs/server/autodream/...`. Since there's no BUILD.bazel in `srcs/server/autodream/` yet, I will create `srcs/server/autodream/BUILD.bazel`.

6. **Create BUILD.bazel for the autodream package**
   - File: `srcs/server/autodream/BUILD.bazel`
   - Action: `run_in_bash_session`
   - Code: Create Bazel target for `go_library` and `go_test` using rules_go. Verify with `ls -l srcs/server/autodream/BUILD.bazel`.

7. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done**
   - Action: `pre_commit_instructions` and follow steps.

8. **Submit the change**
   - Action: `submit`
