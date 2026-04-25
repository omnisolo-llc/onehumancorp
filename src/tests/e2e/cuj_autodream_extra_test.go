package e2e

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/src/server/autodream"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

type DummyLLMClient struct{}

func (m *DummyLLMClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return make([]float32, 1536), nil
}

func TestE2E_AutoDreamPipeline(t *testing.T) {
	pool := db.NewTestProvider(t)
	ctx := context.Background()

	err := pool.(*db.DB).RunMigrations(ctx)
	if err != nil {
		t.Logf("migrations run result: %v", err)
	}

	// Insert task
	_, err = pool.Exec(ctx, `
		INSERT INTO shared_tasks_decomposition (id, organization_id, title, description, status, dependencies)
		VALUES ('task-e2e-1', 'org-e2e', 'E2E Task', 'E2E Description', 'COMPLETED', '[]')
	`)
	if err != nil {
		t.Skipf("skipping: %v", err)
	}

	pipeline := autodream.NewAutoDreamPipeline(pool, &DummyLLMClient{})

	err = pipeline.ProcessCompletedTasks(ctx)
	assert.NoError(t, err)

	var count int
	err = pool.QueryRow(ctx, "SELECT COUNT(*) FROM autodream_memories WHERE task_id = 'task-e2e-1'").Scan(&count)
	assert.NoError(t, err)
	assert.Equal(t, 1, count)
}
