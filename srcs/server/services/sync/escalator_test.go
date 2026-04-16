package sync

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestEscalator(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open in-memory db: %v", err)
	}
	defer db.Close()

	escalator := NewEscalator(db)
	ctx := context.Background()

	err = escalator.InitSchema(ctx)
	if err != nil {
		t.Fatalf("InitSchema failed: %v", err)
	}

	meter := noop.NewMeterProvider().Meter("test")
	err = escalator.InitWithMeter(meter)
	if err != nil {
		t.Fatalf("InitWithMeter failed: %v", err)
	}

	_, err = db.ExecContext(ctx, "INSERT INTO local_mcp_rag_tasks (id, payload, escalation_status) VALUES (?, ?, ?)", "task-1", "{}", "pending")
	if err != nil {
		t.Fatalf("failed to insert test task: %v", err)
	}

	err = escalator.EscalateTask(ctx, "task-1")
	if err != nil {
		t.Fatalf("EscalateTask failed: %v", err)
	}

	var status string
	err = db.QueryRowContext(ctx, "SELECT escalation_status FROM local_mcp_rag_tasks WHERE id = ?", "task-1").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query task status: %v", err)
	}

	if status != "escalated" {
		t.Fatalf("expected status 'escalated', got '%s'", status)
	}
}
