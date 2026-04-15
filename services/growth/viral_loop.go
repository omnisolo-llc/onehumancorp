package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type ViralLoop struct {
	analytics *analytics.ViralAnalytics
	db        *db.DB
}

func NewViralLoop(database *db.DB, a *analytics.ViralAnalytics) *ViralLoop {
	return &ViralLoop{
		analytics: a,
		db:        database,
	}
}

func (vl *ViralLoop) RecordInvite(ctx context.Context, source string) error {
	query := `
		INSERT INTO viral_invites (source, count)
		VALUES ($1, 1)
		ON CONFLICT (source) DO UPDATE SET count = viral_invites.count + 1
	`
	_, err := vl.db.Exec(ctx, query, source)
	if err != nil {
		return fmt.Errorf("failed to record invite: %w", err)
	}
	return nil
}

func (vl *ViralLoop) CalculateKFactor(ctx context.Context, source string) (float64, error) {
	query := `SELECT count FROM viral_invites WHERE source = $1 FOR UPDATE SKIP LOCKED`
	var invites int
	row := vl.db.QueryRow(ctx, query, source)
	err := row.Scan(&invites)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			invites = 0
		} else {
			return 0, fmt.Errorf("failed to get invites count: %w", err)
		}
	}

	if invites == 0 {
		return 0, nil
	}

	conversions, err := vl.analytics.GetConversions(ctx, source)
	if err != nil {
		return 0, fmt.Errorf("failed to get conversions: %w", err)
	}

	return float64(conversions) / float64(invites), nil
}
