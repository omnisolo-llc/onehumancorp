package workers

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"
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

	competitors := map[string]bool{
		"Claude Code":  true,
		"OpenClaw":     false,
		"Replit Agent": false,
	}

	tx, err := w.pool.Begin(ctx)
	if err != nil {
		slog.Error("CompetitorAuditWorker: failed to begin tx", "error", err)
		return
	}
	defer tx.Rollback(ctx)

	type metricData struct {
		ID             string `json:"id"`
		OrganizationID string `json:"organization_id"`
		CompetitorName string `json:"competitor_name"`
		MetricType     string `json:"metric_type"`
		MetricValue    string `json:"metric_value"`
	}
	var findings []metricData

	for comp, offline := range competitors {
		id := uuid.New().String()
		val := "false"
		if offline {
			val = "true"
		}
		m := metricData{
			ID:             id,
			OrganizationID: "system",
			CompetitorName: comp,
			MetricType:     "offline_support",
			MetricValue:    val,
		}
		findings = append(findings, m)

		_, err := tx.Exec(ctx, `
			INSERT INTO competitor_metrics (id, organization_id, competitor_name, metric_type, metric_value)
			VALUES ($1, $2, $3, $4, $5)
		`, m.ID, m.OrganizationID, m.CompetitorName, m.MetricType, m.MetricValue)
		if err != nil {
			slog.Error("CompetitorAuditWorker: failed to insert metric", "error", err)
			return
		}
	}

	if err := tx.Commit(ctx); err != nil {
		slog.Error("CompetitorAuditWorker: failed to commit tx", "error", err)
		return
	}

	// Write findings to .agent-task/memory/
	if err := os.MkdirAll(".agent-task/memory", 0755); err != nil {
		slog.Error("CompetitorAuditWorker: failed to create memory directory", "error", err)
		return
	}

	findingsData, err := json.MarshalIndent(findings, "", "  ")
	if err != nil {
		slog.Error("CompetitorAuditWorker: failed to marshal findings", "error", err)
		return
	}

	if err := os.WriteFile(".agent-task/memory/competitor_audit.json", findingsData, 0644); err != nil {
		slog.Error("CompetitorAuditWorker: failed to write findings file", "error", err)
		return
	}

	slog.Info("CompetitorAuditWorker: finished audit")
}
