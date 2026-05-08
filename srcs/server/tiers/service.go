package tiers

import (
	"context"
	"database/sql"
	"fmt"
	"time"
)

// TierService provides methods to check limits
type TierService struct {
	db *sql.DB
}

// NewTierService creates a new TierService
func NewTierService(database *sql.DB) *TierService {
	return &TierService{db: database}
}

// CheckLimit checks if a specific limit is exceeded for a tenant
func (s *TierService) CheckLimit(ctx context.Context, tenantID string, metric string, increment int) (bool, error) {
	// First, get the tenant's tier
	var tierStr string
	err := s.db.QueryRowContext(ctx, "SELECT tier FROM tenants WHERE id = $1", tenantID).Scan(&tierStr)
	if err != nil {
		if err == sql.ErrNoRows {
			// default to Free tier if tenant doesn't exist yet for some reason
			tierStr = string(TierFree)
		} else {
			return false, fmt.Errorf("failed to get tenant tier: %w", err)
		}
	}

	if tierStr == "" {
		tierStr = string(TierFree)
	}

	tierLimits, ok := LimitsByTier[TierType(tierStr)]
	if !ok {
		// Fallback to free tier
		tierLimits = LimitsByTier[TierFree]
	}

	// Now check usage
	var usage UsageMetrics
	var lastReset time.Time
	err = s.db.QueryRowContext(ctx, "SELECT product_count, ai_actions_month, storage_bytes, last_reset_date FROM tier_usage WHERE tenant_id = $1", tenantID).
		Scan(&usage.ProductCount, &usage.AIActionsMonth, &usage.StorageBytes, &lastReset)

	if err != nil {
		if err == sql.ErrNoRows {
			usage = UsageMetrics{}
		} else {
			return false, fmt.Errorf("failed to get tenant usage: %w", err)
		}
	}

	// Reset monthly counters if needed (simplified for demonstration)
	if time.Since(lastReset) > 30*24*time.Hour {
		usage.AIActionsMonth = 0
		// In a real app we'd update the db here
	}

	switch metric {
	case "products":
		if tierLimits.MaxProducts != -1 && usage.ProductCount+increment > tierLimits.MaxProducts {
			return false, nil // Limit exceeded
		}
	case "ai_actions":
		if tierLimits.MaxAIActions != -1 && usage.AIActionsMonth+increment > tierLimits.MaxAIActions {
			return false, nil // Soft limit exceeded (handled by caller)
		}
	case "storage":
		if tierLimits.MaxStorageBytes != -1 && usage.StorageBytes+int64(increment) > tierLimits.MaxStorageBytes {
			return false, nil // Limit exceeded
		}
	case "ai_departments":
		// Example: limit logic for departments can be queried differently, but keeping standard interface
		if tierLimits.MaxAIDepartments != -1 && increment > tierLimits.MaxAIDepartments {
			return false, nil
		}
	default:
		return false, fmt.Errorf("unknown metric: %s", metric)
	}

	return true, nil // Allowed
}

// UpdateUsage updates the usage for a tenant
func (s *TierService) UpdateUsage(ctx context.Context, tenantID string, metric string, increment int) error {
	var query string
	switch metric {
	case "products":
		query = "INSERT INTO tier_usage (tenant_id, product_count) VALUES ($1, $2) ON CONFLICT (tenant_id) DO UPDATE SET product_count = tier_usage.product_count + EXCLUDED.product_count"
	case "ai_actions":
		query = "INSERT INTO tier_usage (tenant_id, ai_actions_month) VALUES ($1, $2) ON CONFLICT (tenant_id) DO UPDATE SET ai_actions_month = tier_usage.ai_actions_month + EXCLUDED.ai_actions_month"
	case "storage":
		query = "INSERT INTO tier_usage (tenant_id, storage_bytes) VALUES ($1, $2) ON CONFLICT (tenant_id) DO UPDATE SET storage_bytes = tier_usage.storage_bytes + EXCLUDED.storage_bytes"
	default:
		return fmt.Errorf("unknown metric: %s", metric)
	}

	_, err := s.db.ExecContext(ctx, query, tenantID, increment)
	return err
}
