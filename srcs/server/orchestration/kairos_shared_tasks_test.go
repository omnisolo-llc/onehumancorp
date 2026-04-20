package orchestration

import (
	"context"
	"database/sql"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestClaimTask_SQLite(t *testing.T) {
	dbProvider := db.NewTestProvider(t)
	ctx := context.Background()

	_, err := dbProvider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            organization_id TEXT NOT NULL,
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'PENDING',
            priority TEXT NOT NULL DEFAULT 'P0',
            agent_id TEXT,
            assigned_agent TEXT,
            dependencies TEXT DEFAULT '[]',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = dbProvider.Exec(ctx, `
        INSERT INTO shared_tasks (id, organization_id, title, status)
        VALUES ('task-1', 'org-1', 'Task 1', 'PENDING')
    `)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	repo := NewSharedTaskRepo(dbProvider)

	task, err := repo.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if task == nil {
		t.Fatalf("expected task, got nil")
	}
	if task.ID != "task-1" {
		t.Errorf("expected task-1, got %v", task.ID)
	}
	if task.Status != "IN_PROGRESS" {
		t.Errorf("expected IN_PROGRESS, got %v", task.Status)
	}
	if *task.AssignedAgent != "agent-1" {
		t.Errorf("expected agent-1, got %v", *task.AssignedAgent)
	}

	// Claim again, should be no tasks
	task2, err := repo.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("failed to claim task: %v", err)
	}
	if task2 != nil {
		t.Errorf("expected nil task, got %v", task2)
	}
}

type mockTx struct {
	db.Tx
	lastQuery string
}

func (m *mockTx) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	return 0, nil
}

func (m *mockTx) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	m.lastQuery = query
	return &mockPGRow2{}
}
func (m *mockTx) Commit(ctx context.Context) error   { return nil }
func (m *mockTx) Rollback(ctx context.Context) error { return nil }

type mockPGProvider2 struct {
	db.Provider
	tx *mockTx
}

func (m *mockPGProvider2) IsSQLite() bool { return false }
func (m *mockPGProvider2) Begin(ctx context.Context) (db.Tx, error) {
	m.tx = &mockTx{}
	return m.tx, nil
}

type mockPGRow2 struct{}

func (m *mockPGRow2) Scan(dest ...interface{}) error { return sql.ErrNoRows }

func TestClaimTask_Kairos_PostgresLocking(t *testing.T) {
	ctx := context.Background()
	provider := &mockPGProvider2{}
	repo := NewSharedTaskRepo(provider)

	_, err := repo.ClaimTask(ctx, "agent-1")
	if err != nil {
		t.Fatalf("expected nil error on sql.ErrNoRows, got %v", err)
	}

	if !strings.Contains(provider.tx.lastQuery, "FOR UPDATE SKIP LOCKED") {
		t.Errorf("expected query to contain FOR UPDATE SKIP LOCKED, got: %s", provider.tx.lastQuery)
	}
}
