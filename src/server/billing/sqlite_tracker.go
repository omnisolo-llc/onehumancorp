package billing

import (
	"context"
	"fmt"

	"github.com/onehumancorp/mono/src/server/db"
)

// SqliteUsageRepository implements UsageRepository backed by SQLite.
type SqliteUsageRepository struct {
	pool    db.Provider
	catalog map[string]Price
}

// NewSqliteUsageRepository creates a SQLite-backed usage repository.
func NewSqliteUsageRepository(pool db.Provider, catalog map[string]Price) *SqliteUsageRepository {
	copied := make(map[string]Price, len(catalog))
	for model, price := range catalog {
		copied[model] = price
	}
	return &SqliteUsageRepository{pool: pool, catalog: copied}
}

func (r *SqliteUsageRepository) Track(ctx context.Context, usage Usage) (Usage, error) {
	price, ok := r.catalog[usage.Model]
	if !ok {
		return Usage{}, fmt.Errorf("unknown model pricing: %s", usage.Model)
	}

	usage.CostUSD = (float64(usage.PromptTokens)/1_000_000.0)*price.InputPerMillionUSD +
		(float64(usage.CompletionTokens)/1_000_000.0)*price.OutputPerMillionUSD +
		(float64(usage.CachedTokens)/1_000_000.0)*price.CachedPerMillionUSD
	usage.OccurredAt = usage.OccurredAt.UTC()

	_, err := r.pool.Exec(ctx, `
		INSERT INTO usage_events (agent_id, agent_role, organization_id, model, prompt_tokens, completion_tokens, cached_tokens, is_action, cost_usd, occurred_at)
		VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`,
		usage.AgentID, usage.AgentRole, usage.OrganizationID, usage.Model,
		usage.PromptTokens, usage.CompletionTokens, usage.CachedTokens, usage.IsAction, usage.CostUSD, usage.OccurredAt,
	)
	if err != nil {
		return Usage{}, fmt.Errorf("sqlite: track usage: %w", err)
	}
	return usage, nil
}

func (r *SqliteUsageRepository) Summary(ctx context.Context, organizationID string) (Summary, error) {
	rows, err := r.pool.Query(ctx, `
		SELECT agent_id,
		       COALESCE(SUM(cost_usd), 0),
		       COALESCE(SUM(prompt_tokens + completion_tokens + cached_tokens), 0),
		       COALESCE(SUM(CASE WHEN is_action THEN 1 ELSE 0 END), 0)
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
	var totalActions int64

	for rows.Next() {
		var a AgentSummary
		if err := rows.Scan(&a.AgentID, &a.CostUSD, &a.TokenUsed, &a.TotalActions); err != nil {
			return Summary{}, fmt.Errorf("sqlite: scan agent summary: %w", err)
		}
		totalCost += a.CostUSD
		totalTokens += a.TokenUsed
		totalActions += a.TotalActions
		agents = append(agents, a)
	}

	return Summary{
		OrganizationID:      organizationID,
		TotalCostUSD:        totalCost,
		TotalTokens:         totalTokens,
		TotalActions:        totalActions,
		ProjectedMonthlyUSD: totalCost * 30,
		Agents:              agents,
	}, nil
}

func (r *SqliteUsageRepository) ActiveOrganizations(ctx context.Context) ([]string, error) {
	rows, err := r.pool.Query(ctx, `SELECT DISTINCT organization_id FROM usage_events`)
	if err != nil {
		return nil, fmt.Errorf("sqlite: active organizations: %w", err)
	}
	defer rows.Close()

	var orgs []string
	for rows.Next() {
		var org string
		if err := rows.Scan(&org); err != nil {
			return nil, fmt.Errorf("sqlite: scan active organization: %w", err)
		}
		orgs = append(orgs, org)
	}
	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("scan active organization rows: %w", err)
	}
	return orgs, nil
}
