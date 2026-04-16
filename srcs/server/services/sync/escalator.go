package sync

import (
	"context"
	"database/sql"
	"fmt"
	"sync"

	"go.opentelemetry.io/otel/metric"
)

type Escalator struct {
	db                  *sql.DB
	poolInit            sync.Once
	tasksEscalatedTotal metric.Int64Counter
}

func NewEscalator(db *sql.DB) *Escalator {
	return &Escalator{
		db: db,
	}
}

func (e *Escalator) InitWithMeter(meter metric.Meter) error {
	var err error
	e.poolInit.Do(func() {
		e.tasksEscalatedTotal, err = meter.Int64Counter("tasks_escalated_total", metric.WithDescription("Total number of locally escalated tasks"))
	})
	return err
}

func (e *Escalator) EscalateTask(ctx context.Context, taskID string) error {
	if e.tasksEscalatedTotal == nil {
		return fmt.Errorf("Escalator not initialized with meter")
	}

	_, err := e.db.ExecContext(ctx, "UPDATE local_mcp_rag_tasks SET escalation_status = 'escalated' WHERE id = ?", taskID)
	if err != nil {
		return fmt.Errorf("failed to escalate task: %w", err)
	}

	e.tasksEscalatedTotal.Add(ctx, 1)
	return nil
}

func (e *Escalator) InitSchema(ctx context.Context) error {
	query := `
		CREATE TABLE IF NOT EXISTS local_mcp_rag_tasks (
			id TEXT PRIMARY KEY,
			payload TEXT NOT NULL,
			escalation_status TEXT NOT NULL
		);
	`
	_, err := e.db.ExecContext(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to create local_mcp_rag_tasks table: %w", err)
	}
	return nil
}
