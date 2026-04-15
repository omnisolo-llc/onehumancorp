package autodream

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	_ "modernc.org/sqlite"
)

type mockEmbeddingService struct{}

func (m *mockEmbeddingService) EmbedText(ctx context.Context, text string) ([]float32, error) {
	return make([]float32, 1536), nil
}

func newTestProvider(t *testing.T) db.Provider {
    sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	database := &db.DB{Provider: db.NewSqliteProvider(sqlDB)}
    return database
}

func TestConsolidator(t *testing.T) {
	database := newTestProvider(t)
	ctx := context.Background()

	consolidator := NewConsolidator(database, &mockEmbeddingService{})

    assignedAgent := "agent-x"
	task := &orchestration.SharedTaskDecomposition{
		ID:             "task-123",
		OrganizationID: "org-1",
		Title:          "Test Task",
		Status:         "DONE",
        AssignedAgentID: &assignedAgent,
		CreatedAt:      time.Now(),
		UpdatedAt:      time.Now(),
	}

    // We don't have shared_tasks_master table setup in this pure db test provider,
    // it requires db migrations. Let's just create table manually for testing.
    _, err := database.Exec(ctx, `CREATE TABLE IF NOT EXISTS shared_tasks_master (
        id VARCHAR PRIMARY KEY,
        organization_id VARCHAR NOT NULL,
        title VARCHAR NOT NULL,
        description TEXT,
        status VARCHAR NOT NULL DEFAULT 'PENDING',
        agent_id VARCHAR,
        priority VARCHAR NOT NULL DEFAULT 'P2',
        payload JSONB,
        parent_plan_id TEXT,
        dependencies JSONB NOT NULL DEFAULT '[]',
        locked_until TIMESTAMP,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )`)
    if err != nil {
        t.Fatal(err)
    }

    err = orchestration.CreateTask(ctx, database, task)
    if err != nil {
        t.Fatal(err)
    }

    _, err = database.Exec(ctx, `CREATE TABLE IF NOT EXISTS autodream_memories_master (
        id VARCHAR PRIMARY KEY,
        task_id VARCHAR REFERENCES shared_tasks_master(id),
        agent_id VARCHAR NOT NULL,
        memory_type VARCHAR NOT NULL,
        content TEXT NOT NULL,
        embedding JSON,
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    )`)
    if err != nil {
        t.Fatal(err)
    }

	err = consolidator.ProcessCompletedTask(ctx, task)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	var count int
	err = database.QueryRow(ctx, "SELECT count(*) FROM autodream_memories_master").Scan(&count)
	if err != nil {
		t.Fatal(err)
	}
	if count != 1 {
		t.Fatalf("expected 1 memory, got %d", count)
	}
}
