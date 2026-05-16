package sync

import (
	"context"
	"database/sql"
	"testing"

	_ "github.com/mattn/go-sqlite3"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestEscalator_EscalateTask(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec(`CREATE TABLE local_mcp_rag_tasks (
		id TEXT PRIMARY KEY,
		payload TEXT NOT NULL,
		escalation_status TEXT NOT NULL DEFAULT 'local',
		created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
		updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	_, err = db.Exec(`INSERT INTO local_mcp_rag_tasks (id, payload) VALUES ('task-1', '{"data": "test"}')`)
	if err != nil {
		t.Fatalf("failed to insert task: %v", err)
	}

	esc := NewEscalator(db)
	err = esc.InitWithMeter(noop.NewMeterProvider().Meter("test"))
	if err != nil {
		t.Fatalf("failed to init meter: %v", err)
	}

	err = esc.EscalateTask(context.Background(), "task-1")
	if err != nil {
		t.Fatalf("failed to escalate task: %v", err)
	}

	var status string
	err = db.QueryRow(`SELECT escalation_status FROM local_mcp_rag_tasks WHERE id = 'task-1'`).Scan(&status)
	if err != nil {
		t.Fatalf("failed to query status: %v", err)
	}

	if status != "escalated" {
		t.Errorf("expected status 'escalated', got '%s'", status)
	}
}
