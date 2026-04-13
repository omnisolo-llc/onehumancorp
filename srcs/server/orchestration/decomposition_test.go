package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockMutexProvider struct{}

func (m *mockMutexProvider) NewMutex(key string) Mutex {
	return &mockMutex{}
}

type mockMutex struct{}

func (m *mockMutex) Lock(ctx context.Context, ttl time.Duration) error { return nil }
func (m *mockMutex) Unlock(ctx context.Context) error                  { return nil }

type mockMinimaxClient struct {
	response string
	err      error
}

func (m *mockMinimaxClient) Reason(ctx context.Context, prompt string) (string, error) {
	return m.response, m.err
}
func (m *mockMinimaxClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	return nil, nil
}

func TestDecomposer_DecomposeTask(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")

	prov := db.NewTestProvider(t)
	_, _ = prov.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			parent_plan_id TEXT,
			dependencies JSONB NOT NULL DEFAULT '[]',
			title TEXT NOT NULL,
			description TEXT,
			status TEXT NOT NULL DEFAULT 'PENDING',
			agent_id TEXT,
			priority TEXT NOT NULL DEFAULT 'P2',
			payload TEXT NOT NULL DEFAULT '{}',
			locked_until DATETIME,
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		);

		CREATE TABLE IF NOT EXISTS task_dependencies (
			task_id TEXT NOT NULL,
			depends_on_task_id TEXT NOT NULL,
			PRIMARY KEY (task_id, depends_on_task_id)
		);
	`)

	tm := setupTasksTestDBWithoutCleanup(t, prov)

	llm := &mockMinimaxClient{
		response: `[
			{"title": "Task 1", "description": "Desc 1", "agent_role": "frontend", "priority": "P1", "depends_on": []},
			{"title": "Task 2", "description": "Desc 2", "agent_role": "backend", "priority": "P1", "depends_on": ["Task 1"]}
		]`,
	}
	mp := &mockMutexProvider{}

	decomposer := NewDecomposer(llm, tm, mp)

	ctx := auth.ContextWithClaims(context.Background(), &auth.Claims{OrganizationID: "org-1"})

	err := decomposer.DecomposeTask(ctx, "org-1", "plan-1", "Make a new feature")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify tasks were created
	rows, err := prov.Query(ctx, "SELECT title FROM shared_tasks")
	if err != nil {
		t.Fatalf("failed to query tasks: %v", err)
	}
	defer rows.Close()

	var titles []string
	for rows.Next() {
		var title string
		_ = rows.Scan(&title)
		titles = append(titles, title)
	}
	if len(titles) != 2 {
		t.Errorf("expected 2 tasks, got %d", len(titles))
	}

	// Verify dependencies
	var count int
	_ = prov.QueryRow(ctx, "SELECT COUNT(*) FROM task_dependencies").Scan(&count)
	if count != 1 {
		t.Errorf("expected 1 dependency, got %d", count)
	}
}

func setupTasksTestDBWithoutCleanup(t *testing.T, prov db.Provider) *TaskManager {
	t.Helper()
	tm := NewTaskManager(prov, nil)
	return tm
}
