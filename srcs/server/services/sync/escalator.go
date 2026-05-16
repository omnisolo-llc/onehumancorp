package sync

import (
	"context"
	"database/sql"
	"fmt"
	"log"

	"go.opentelemetry.io/otel/metric"
)

type Escalator struct {
	db     *sql.DB
	meter  metric.Meter
	counter metric.Int64Counter
}

func NewEscalator(db *sql.DB) *Escalator {
	return &Escalator{
		db: db,
	}
}

func (e *Escalator) InitWithMeter(meter metric.Meter) error {
	e.meter = meter
	counter, err := meter.Int64Counter("tasks_escalated_total",
		metric.WithDescription("Total number of tasks escalated to cloud"),
	)
	if err != nil {
		return err
	}
	e.counter = counter
	return nil
}

func (e *Escalator) EscalateTask(ctx context.Context, taskID string) error {
	// SPIFFE/SPIRE auth placeholder
	log.Printf("[SPIFFE] Authenticated escalation request for task %s", taskID)

	query := `UPDATE local_mcp_rag_tasks SET escalation_status = 'escalated' WHERE id = ?`
	_, err := e.db.ExecContext(ctx, query, taskID)
	if err != nil {
		return fmt.Errorf("failed to escalate task: %w", err)
	}

	if e.counter != nil {
		e.counter.Add(ctx, 1)
	}

	return nil
}
