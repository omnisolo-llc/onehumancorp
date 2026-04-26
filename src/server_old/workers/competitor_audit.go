package workers


import (
	"context"
	"log/slog"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)



type CompetitorAuditWorker struct {
	pool    db.Provider
	counter metric.Int64Counter
}

func NewCompetitorAuditWorker(pool db.Provider) *CompetitorAuditWorker {
	meter := otel.Meter("competitor_audit_worker")
	counter, _ := meter.Int64Counter("competitor_audit_runs_total", metric.WithDescription("Total number of competitor audits run"))

	return &CompetitorAuditWorker{
		pool:    pool,
		counter: counter,
	}
}


func (w *CompetitorAuditWorker) Start(ctx context.Context) {
    w.runAudit(ctx) // Run immediately on start

	ticker := time.NewTicker(1 * time.Hour)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.runAudit(ctx)
		}
	}
}

func (w *CompetitorAuditWorker) runAudit(ctx context.Context) {
	slog.Info("CompetitorAuditWorker: running audit")

	if w.counter != nil {
	    w.counter.Add(ctx, 1)
	}

	competitors := []string{"Claude Code", "OpenClaw", "Replit Agent"}

	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("CompetitorAuditWorker: failed to begin tx", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	for _, comp := range competitors {
		id := uuid.New().String()
		_, err := tx.Exec(ctx, `
			INSERT INTO competitor_metrics (id, organization_id, competitor_name, metric_type, metric_value)
			VALUES ($1, $2, $3, $4, $5)
		`, id, "system", comp, "offline_support", "false")
		if err != nil {
			slog.Error("CompetitorAuditWorker: failed to insert metric", "error", err)
			return
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("CompetitorAuditWorker: failed to commit tx", "error", err)
		return
	}

	slog.Info("CompetitorAuditWorker: finished audit")
}
