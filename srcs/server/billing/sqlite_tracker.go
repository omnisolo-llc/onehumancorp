package billing

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

// SqliteUsageRepository implements UsageRepository backed by SQLite.
type SqliteUsageRepository struct {
	db      *sql.DB
	catalog map[string]Price
}

// NewSqliteUsageRepository creates a SQLite-backed usage repository.
func NewSqliteUsageRepository(db *sql.DB, catalog map[string]Price) *SqliteUsageRepository {
	copied := make(map[string]Price, len(catalog))
	for model, price := range catalog {
		copied[model] = price
	}
	return &SqliteUsageRepository{db: db, catalog: copied}
}

func (r *SqliteUsageRepository) Track(ctx context.Context, usage Usage) (Usage, error) {
	price, ok := r.catalog[usage.Model]
	if !ok {
		return Usage{}, fmt.Errorf("unknown model pricing: %s", usage.Model)
	}

	usage.CostUSD = (float64(usage.PromptTokens)/1_000_000.0)*price.InputPerMillionUSD +
		(float64(usage.CompletionTokens)/1_000_000.0)*price.OutputPerMillionUSD
	usage.OccurredAt = usage.OccurredAt.UTC()

	occurredAt := usage.OccurredAt.Format("2006-01-02 15:04:05")
	if occurredAt == "0001-01-01 00:00:00" {
		occurredAt = time.Now().UTC().Format("2006-01-02 15:04:05")
	}

	_, err := r.db.ExecContext(ctx, `
		INSERT INTO usage_events (agent_id, agent_role, organization_id, model, prompt_tokens, completion_tokens, cost_usd, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
		usage.AgentID, usage.AgentRole, usage.OrganizationID, usage.Model,
		usage.PromptTokens, usage.CompletionTokens, usage.CostUSD, occurredAt,
	)
	if err != nil {
		return Usage{}, fmt.Errorf("sqlite: track usage: %w", err)
	}
	return usage, nil
}

func (r *SqliteUsageRepository) Summary(ctx context.Context, organizationID string) (Summary, error) {
	rows, err := r.db.QueryContext(ctx, `
		SELECT agent_id,
		       COALESCE(SUM(cost_usd), 0),
		       COALESCE(SUM(prompt_tokens + completion_tokens), 0)
		FROM usage_events
		WHERE organization_id = ?
		GROUP BY agent_id
		ORDER BY agent_id`, organizationID)
	if err != nil {
		return Summary{}, fmt.Errorf("sqlite: billing summary: %w", err)
	}
	defer rows.Close()

	var agents []AgentSummary
	var totalCost float64
	var totalTokens int64

	for rows.Next() {
		var a AgentSummary
		if err := rows.Scan(&a.AgentID, &a.CostUSD, &a.TokenUsed); err != nil {
			return Summary{}, fmt.Errorf("sqlite: scan agent summary: %w", err)
		}
		totalCost += a.CostUSD
		totalTokens += a.TokenUsed
		agents = append(agents, a)
	}

	return Summary{
		OrganizationID:      organizationID,
		TotalCostUSD:        totalCost,
		TotalTokens:         totalTokens,
		ProjectedMonthlyUSD: totalCost * 30,
		Agents:              agents,
	}, nil
}
