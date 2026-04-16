1. Create the worker Go code using a bash command:
```bash
cat << 'GO_EOF' > srcs/server/workers/autodream_worker.go
package workers

import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"strings"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type AutoDreamWorker struct {
	pool db.Provider
}

func NewAutoDreamWorker(pool db.Provider) *AutoDreamWorker {
	return &AutoDreamWorker{
		pool: pool,
	}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	slog.Info("Starting AutoDreamWorker")
	ticker := time.NewTicker(30 * time.Second)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.ProcessCompletedTasks(ctx)
		}
	}
}

func (w *AutoDreamWorker) ProcessCompletedTasks(ctx context.Context) {
	query := `SELECT id, organization_id, COALESCE(description, '') AS content
	          FROM shared_tasks_decomposition
	          WHERE status IN ('DONE', 'COMPLETED')
	          AND id NOT IN (SELECT task_id FROM autodream_memories WHERE task_id IS NOT NULL)`

	rows, err := w.pool.Query(ctx, query)
	if err != nil {
		slog.Error("AutoDreamWorker: failed to query completed tasks", "error", err)
		return
	}
	defer rows.Close()

	var tasks []struct {
		ID             string
		OrganizationID string
		Content        string
	}

	for rows.Next() {
		var t struct {
			ID             string
			OrganizationID string
			Content        string
		}
		if err := rows.Scan(&t.ID, &t.OrganizationID, &t.Content); err != nil {
			slog.Error("AutoDreamWorker: failed to scan task", "error", err)
			continue
		}
		tasks = append(tasks, t)
	}

	minimaxKey := os.Getenv("MINIMAX_API_KEY")
	var client orchestration.MinimaxClient
	if minimaxKey != "" {
		client = orchestration.NewCachedMinimaxClient(orchestration.NewMinimaxClient(minimaxKey), w.pool, nil)
	}

	for _, t := range tasks {
		var embedding []float32
		if client != nil {
			embCtx, cancel := context.WithTimeout(ctx, 30*time.Second)
			resp, embErr := client.GenerateEmbedding(embCtx, t.Content)
			cancel()
			if embErr == nil && len(resp) == 1536 {
				embedding = resp
			}
		}

		if len(embedding) == 0 {
			embedding = make([]float32, 1536)
		}

		strs := make([]string, len(embedding))
		for i, v := range embedding {
			strs[i] = fmt.Sprintf("%f", v)
		}
		embStr := "[" + strings.Join(strs, ",") + "]"

		memID := uuid.New().String()

		if w.pool.IsSQLite() {
			insertQuery := `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, created_at)
			               VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)`
			_, err := w.pool.Exec(ctx, insertQuery, memID, t.OrganizationID, t.ID, t.Content, embStr)
			if err != nil {
				slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", err)
			} else {
				slog.Info("AutoDreamWorker: ingested completed task", "task_id", t.ID)
			}
		} else {
			insertQueryPg := `INSERT INTO autodream_memories (id, organization_id, task_id, content, embedding, created_at)
			               VALUES ($1, $2, $3, $4, $5::vector, CURRENT_TIMESTAMP)`
			_, errPg := w.pool.Exec(ctx, insertQueryPg, memID, t.OrganizationID, t.ID, t.Content, embStr)
			if errPg != nil {
				slog.Error("AutoDreamWorker: failed to insert memory", "task_id", t.ID, "error", errPg)
			} else {
				slog.Info("AutoDreamWorker: ingested completed task (pg fallback)", "task_id", t.ID)
			}
		}
	}
}
GO_EOF
```

2. Verify worker Go code using a bash command:
```bash
cat srcs/server/workers/autodream_worker.go
```

3. Create the test Go code using a bash command:
```bash
cat << 'GO_EOF' > srcs/server/workers/autodream_worker_test.go
package workers

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

func TestAutoDreamWorker_ProcessCompletedTasks(t *testing.T) {
	provider := setupTestDB(t)

	// Ensure shared_tasks_decomposition table exists
	_, err := provider.Exec(context.Background(), `
	CREATE TABLE IF NOT EXISTS shared_tasks_decomposition (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		title TEXT NOT NULL,
		description TEXT,
		status TEXT NOT NULL DEFAULT 'PENDING'
	)`)
	assert.NoError(t, err)

	// Ensure autodream_memories table exists with correct schema for test
	_, err = provider.Exec(context.Background(), `
	CREATE TABLE IF NOT EXISTS autodream_memories (
		id TEXT PRIMARY KEY,
		organization_id TEXT NOT NULL,
		task_id TEXT,
		content TEXT NOT NULL,
		embedding TEXT,
		metadata TEXT,
		created_at TEXT DEFAULT CURRENT_TIMESTAMP
	)`)
	assert.NoError(t, err)

	// Insert a test task
	_, err = provider.Exec(context.Background(), `
	INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status)
	VALUES ('task-1', 'org-1', 'Test Task', 'Test Description', 'DONE')
	`)
	assert.NoError(t, err)

	worker := NewAutoDreamWorker(provider)
	worker.ProcessCompletedTasks(context.Background())

	var count int
	err = provider.QueryRow(context.Background(), "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count, "Expected 1 memory entry for the completed task")

	var content string
	err = provider.QueryRow(context.Background(), "SELECT content FROM autodream_memories WHERE task_id = 'task-1'").Scan(&content)
	assert.NoError(t, err)
	assert.Equal(t, "Test Description", content)
}
GO_EOF
```

4. Verify test Go code using a bash command:
```bash
cat srcs/server/workers/autodream_worker_test.go
```

5. Update Bazel build files using bash commands:
```bash
sed -i '/"mission_ingestion.go",/a \        "autodream_worker.go",' srcs/server/workers/BUILD.bazel
sed -i '/"mission_ingestion_test.go",/a \        "autodream_worker_test.go",' srcs/server/workers/BUILD.bazel
```

6. Verify Bazel updates using a bash command:
```bash
cat srcs/server/workers/BUILD.bazel
```

7. Run tests using a bash command:
```bash
bazelisk test //srcs/server/workers:workers_test
```

8. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.

9. Record mission completion and push:
```bash
TS=$(date +%s%N) && cat << EOF > .agent-task/status/${TS}.yml
status: DONE
task: Implement KAIROS autoDream Vector Memory Consolidation Pipeline
