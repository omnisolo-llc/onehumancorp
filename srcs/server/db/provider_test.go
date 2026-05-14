package db

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func TestAcquireTask(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE tasks (
		id TEXT PRIMARY KEY,
		parent_task_id TEXT,
		agent_id TEXT,
		status TEXT NOT NULL,
		payload TEXT,
		created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	_, err = db.Exec(`INSERT INTO tasks (id, status, payload) VALUES ('task-1', 'PENDING', '{"action":"test"}')`)
	if err != nil {
		t.Fatalf("Failed to insert task: %v", err)
	}

	provider := &DB{
		db:       db,
		isSQLite: true,
	}

	task, err := provider.AcquireTask(context.Background(), "agent-1")
	if err != nil {
		t.Fatalf("Failed to acquire task: %v", err)
	}

	if task.ID != "task-1" {
		t.Errorf("Expected task ID 'task-1', got %s", task.ID)
	}
}

func TestAcquireTaskPostgres(t *testing.T) {
    db, _ := sql.Open("sqlite3", ":memory:")
    defer db.Close()
	provider := &DB{
		db:       db,
		isSQLite: false,
	}
	_, err := provider.AcquireTask(context.Background(), "agent-1")
    if err == nil {
        t.Errorf("Expected syntax error for postgres query in sqlite db")
    }
}

func TestAcquireTaskError(t *testing.T) {
	db, _ := sql.Open("sqlite3", ":memory:")
	defer db.Close()
	provider := &DB{
		db:       db,
		isSQLite: true,
	}
	_, err := provider.AcquireTask(context.Background(), "agent-1")
	if err == nil {
		t.Error("Expected error due to missing table")
	}
}

func TestIsSQLite(t *testing.T) {
	provider := &DB{isSQLite: true}
	if !provider.IsSQLite() {
		t.Error("Expected IsSQLite to be true")
	}

	provider2 := &DB{isSQLite: false}
	if provider2.IsSQLite() {
		t.Error("Expected IsSQLite to be false")
	}
}
