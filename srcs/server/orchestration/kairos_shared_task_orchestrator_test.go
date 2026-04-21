package orchestration

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/google/uuid"
)

type mockDBProvider struct {
	db.Provider
	isSQLite bool
	tx       *mockTx
	err      error
}

func (m *mockDBProvider) IsSQLite() bool {
	return m.isSQLite
}

func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	if m.err != nil {
		return nil, m.err
	}
	m.tx = &mockTx{}
	return m.tx, nil
}

func (m *mockDBProvider) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	return &mockTaskRow{err: m.err}
}

type mockTx struct {
	db.Tx
}

func (m *mockTx) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
	return &mockTaskRow{}
}

func (m *mockTx) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
	return 0, nil
}

func (m *mockTx) Commit(ctx context.Context) error {
	return nil
}

func (m *mockTx) Rollback(ctx context.Context) error {
	return nil
}

type mockTaskRow struct {
	err error
}

func (m *mockTaskRow) Scan(dest ...interface{}) error {
	if m.err != nil {
		if m.err == sql.ErrNoRows {
			return sql.ErrNoRows
		}
		return m.err
	}

	for i, d := range dest {
		switch v := d.(type) {
		case *string:
			if i == 0 {
				*v = "task-123"
			} else if i == 1 {
				*v = "org-1"
			} else if i == 2 {
				*v = "Test Task"
			} else if i == 4 {
				*v = "PENDING"
			} else if i == 5 {
				*v = "P0"
			} else if i == 7 || i == 8 {
				*v = "2026-04-27 10:00:00"
			} else if i == 9 {
				*v = "2026-04-27 10:00:00"
			}
		case *sql.NullString:
			v.String = ""
			v.Valid = false
		case *time.Time:
			*v = time.Now()
		}
	}
	return nil
}

func TestClaimSharedTask_SQLite(t *testing.T) {
	mockDB := &mockDBProvider{isSQLite: true}
	ctx := context.Background()

	task, err := ClaimSharedTask(ctx, mockDB, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if task == nil {
		t.Fatalf("expected task, got nil")
	}

	if task.Status != "IN_PROGRESS" {
		t.Errorf("expected status IN_PROGRESS, got %s", task.Status)
	}

	if task.AgentID == nil || *task.AgentID != "agent-1" {
		t.Errorf("expected agent ID agent-1, got %v", task.AgentID)
	}
}

func TestClaimSharedTask_Postgres(t *testing.T) {
	mockDB := &mockDBProvider{isSQLite: false}
	ctx := context.Background()

	task, err := ClaimSharedTask(ctx, mockDB, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if task == nil {
		t.Fatalf("expected task, got nil")
	}

	if task.Status != "PENDING" { // The mock scan doesn't change it, just populates struct from query response
		t.Errorf("expected status PENDING, got %s", task.Status)
	}
}

func TestClaimSharedTask_NoRows(t *testing.T) {
	mockDB := &mockDBProvider{isSQLite: false, err: sql.ErrNoRows}
	ctx := context.Background()

	task, err := ClaimSharedTask(ctx, mockDB, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	if task != nil {
		t.Fatalf("expected nil task, got %+v", task)
	}
}

func TestClaimSharedTask_SQLite_NoRows(t *testing.T) {
	mockDB := &mockDBProvider{isSQLite: true, err: sql.ErrNoRows}
	ctx := context.Background()

	task, err := ClaimSharedTask(ctx, mockDB, "org-1", "agent-1")
	if err != nil {
		t.Fatalf("expected no error for no rows, got %v", err)
	}

	if task != nil {
		t.Fatalf("expected nil task, got %+v", task)
	}
}
