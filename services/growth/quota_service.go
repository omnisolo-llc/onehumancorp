package growth

import (
	"context"
	"fmt"
	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/redis/go-redis/v9"
	"strconv"
)

type QuotaService struct {
	tracker *analytics.Tracker
	rdb     *redis.Client
	limit   int
}

func NewQuotaService(tracker *analytics.Tracker, rdb *redis.Client, limit int) *QuotaService {
	return &QuotaService{
		tracker: tracker,
		rdb:     rdb,
		limit:   limit,
	}
}

func (s *QuotaService) CheckQuota(ctx context.Context, tenantID string) (bool, error) {
	if tenantID == "" {
		return false, fmt.Errorf("invalid tenant ID")
	}

	if s.rdb == nil {
		// Standalone mode: graceful degradation
		return true, nil
	}

	key := fmt.Sprintf("quota:%s", tenantID)
	val, err := s.rdb.Get(ctx, key).Result()
	if err == redis.Nil {
		return true, nil
	} else if err != nil {
		return false, err
	}

	usage, err := strconv.Atoi(val)
	if err != nil {
		return false, err
	}

	if usage >= s.limit {
		s.tracker.TrackEvent(ctx, "quota_exceeded", map[string]interface{}{
			"tenant_id": tenantID,
		})
		return false, nil
	}
	return true, nil
}

func (s *QuotaService) IncrementUsage(ctx context.Context, tenantID string) error {
	if tenantID == "" {
		return fmt.Errorf("invalid tenant ID")
	}

	if s.rdb == nil {
		// Standalone mode: graceful degradation
		return nil
	}

	key := fmt.Sprintf("quota:%s", tenantID)
	_, err := s.rdb.Incr(ctx, key).Result()
	if err != nil {
		return err
	}

	s.tracker.TrackEvent(ctx, "quota_usage_incremented", map[string]interface{}{
		"tenant_id": tenantID,
	})
	return nil
}
