package analytics

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type ViralAnalytics struct {
	db *db.DB
}

func NewViralAnalytics(database *db.DB) *ViralAnalytics {
	return &ViralAnalytics{
		db: database,
	}
}

func (va *ViralAnalytics) RecordConversion(ctx context.Context, source string) error {
	query := `
		INSERT INTO viral_conversions (source, count)
		VALUES ($1, 1)
		ON CONFLICT (source) DO UPDATE SET count = viral_conversions.count + 1
	`
	_, err := va.db.Exec(ctx, query, source)
	if err != nil {
		return fmt.Errorf("failed to record conversion: %w", err)
	}
	return nil
}

func (va *ViralAnalytics) GetConversions(ctx context.Context, source string) (int, error) {
	query := `SELECT count FROM viral_conversions WHERE source = $1 FOR UPDATE SKIP LOCKED`
	var count int
	row := va.db.QueryRow(ctx, query, source)
	err := row.Scan(&count)
	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return 0, nil
		}
		return 0, fmt.Errorf("failed to get conversions: %w", err)
	}
	return count, nil
}
