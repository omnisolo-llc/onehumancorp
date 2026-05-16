package sync

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
	"go.opentelemetry.io/otel/metric/noop"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open test database: %v", err)
	}

	_, err = db.Exec(`
		CREATE TABLE local_mcp_rag_tasks (
			id TEXT PRIMARY KEY,
			task_data TEXT NOT NULL,
			escalation_status TEXT NOT NULL DEFAULT 'local',
			created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("Failed to create table: %v", err)
	}

	return db
}

func TestEscalator_ProcessEscalations(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	meter := noop.NewMeterProvider().Meter("test")
	escalator, err := InitWithMeter(db, meter)
	if err != nil {
		t.Fatalf("Failed to init escalator: %v", err)
	}

	// Insert test data
	_, err = db.Exec(`
		INSERT INTO local_mcp_rag_tasks (id, task_data, escalation_status)
		VALUES
			('task-1', '{"query":"test","session_id":"123"}', 'pending_escalation'),
			('task-2', '{"query":"test2","session_id":"456"}', 'local')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	ctx := context.Background()
	err = escalator.ProcessEscalations(ctx)
	if err != nil {
		t.Fatalf("ProcessEscalations failed: %v", err)
	}

	// Verify status update
	var status string
	err = db.QueryRow("SELECT escalation_status FROM local_mcp_rag_tasks WHERE id = 'task-1'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "escalated" {
		t.Errorf("Expected status 'escalated', got '%s'", status)
	}

	// Verify non-escalated task remained local
	err = db.QueryRow("SELECT escalation_status FROM local_mcp_rag_tasks WHERE id = 'task-2'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "local" {
		t.Errorf("Expected status 'local', got '%s'", status)
	}
}

func TestEscalator_escalateTask_InvalidJSON(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	meter := noop.NewMeterProvider().Meter("test")
	escalator, err := InitWithMeter(db, meter)
	if err != nil {
		t.Fatalf("Failed to init escalator: %v", err)
	}

	// Insert invalid JSON test data
	_, err = db.Exec(`
		INSERT INTO local_mcp_rag_tasks (id, task_data, escalation_status)
		VALUES ('task-err', 'invalid-json', 'pending_escalation')
	`)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	ctx := context.Background()
	err = escalator.ProcessEscalations(ctx)
	if err != nil {
		t.Fatalf("ProcessEscalations should swallow error on invalid json, but got: %v", err)
	}

	// Verify status did not change
	var status string
	err = db.QueryRow("SELECT escalation_status FROM local_mcp_rag_tasks WHERE id = 'task-err'").Scan(&status)
	if err != nil {
		t.Fatalf("Failed to query status: %v", err)
	}

	if status != "pending_escalation" {
		t.Errorf("Expected status 'pending_escalation', got '%s'", status)
	}
}

func TestEscalator_Start(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()

	meter := noop.NewMeterProvider().Meter("test")
	escalator, err := InitWithMeter(db, meter)
	if err != nil {
		t.Fatalf("Failed to init escalator: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())

	// Start in background
	go escalator.Start(ctx, 10 * time.Millisecond)

	// Wait briefly to allow tick
	time.Sleep(30 * time.Millisecond)

	// Cancel to stop daemon
	cancel()
	time.Sleep(10 * time.Millisecond) // Give time to exit
}
