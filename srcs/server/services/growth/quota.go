package growth

import (
    "context"
    "database/sql"
    "errors"
    "fmt"
    "time"

    "github.com/onehumancorp/mono/srcs/server/db"
    "go.opentelemetry.io/otel"
    "go.opentelemetry.io/otel/metric"
)

var quotaUsageCounter metric.Int64Counter

func init() {
    meter := otel.Meter("github.com/onehumancorp/mono/ohc")
    var err error
    quotaUsageCounter, err = meter.Int64Counter("growth_quota_usage_total")
    if err != nil {
        panic(err)
    }
}


type QuotaTracker struct {
    provider db.Provider
}

func NewQuotaTracker(provider db.Provider) *QuotaTracker {
    return &QuotaTracker{provider: provider}
}

func (qt *QuotaTracker) GetQuota(ctx context.Context, orgID, resourceType string) (int, int, error) {
    query := `SELECT used, max FROM growth_quotas WHERE organization_id = $1 AND resource_type = $2`
    var used, max int
    err := qt.provider.QueryRow(ctx, query, orgID, resourceType).Scan(&used, &max)
    if err != nil {
        if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
            return 0, 100, nil // Default quota
        }
        return 0, 0, err
    }
    return used, max, nil
}

func (qt *QuotaTracker) IncrementQuota(ctx context.Context, orgID, resourceType string, amount int) error {
    query := `
        INSERT INTO growth_quotas (id, organization_id, resource_type, used, max, updated_at)
        VALUES ($1, $2, $3, $4, 100, CURRENT_TIMESTAMP)
        ON CONFLICT (organization_id, resource_type) DO UPDATE SET used = growth_quotas.used + EXCLUDED.used, updated_at = CURRENT_TIMESTAMP
    `
    id := fmt.Sprintf("quota-%d", time.Now().UnixNano())
    _, err := qt.provider.Exec(ctx, query, id, orgID, resourceType, amount)
    if err == nil && quotaUsageCounter != nil {
        quotaUsageCounter.Add(ctx, int64(amount))
    }
    return err
}
