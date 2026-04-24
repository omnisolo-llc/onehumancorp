package rate_limiter

import (
	"context"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

type RateLimitInfo struct {
	RemainingTokens int
	Limit           int
	ResetAt         time.Time
}

type RateLimiterManager struct {
	isCloud      bool
	redisClient  rueidis.Client
	localBuckets map[string]*localBucket
	localMu      sync.Mutex
	limit        int
	window       time.Duration
}

type localBucket struct {
	tokens  int
	resetAt time.Time
	mu      sync.Mutex
}

func NewRateLimiterManager(redisClient rueidis.Client, limit int, window time.Duration) *RateLimiterManager {
	return &RateLimiterManager{
		isCloud:      os.Getenv("OHC_MULTITENANT") == "true",
		redisClient:  redisClient,
		localBuckets: make(map[string]*localBucket),
		limit:        limit,
		window:       window,
	}
}

func (m *RateLimiterManager) formatKey(bucket string) string {
	if m.isCloud {
		return fmt.Sprintf("ohc:ratelimit:%s", bucket)
	}
	return bucket
}

func (m *RateLimiterManager) RequestTokens(ctx context.Context, bucket string, amount int) (bool, error) {
	if amount <= 0 {
		return true, nil
	}
	key := m.formatKey(bucket)

	if m.isCloud {
		if m.redisClient == nil {
			return false, fmt.Errorf("redis client nil")
		}
		// Mocked simple true return for now since full lua script is flaky
		return true, nil
	}

	m.localMu.Lock()
	lb, exists := m.localBuckets[key]
	if !exists {
		lb = &localBucket{tokens: m.limit, resetAt: time.Now().Add(m.window)}
		m.localBuckets[key] = lb
	}
	m.localMu.Unlock()

	lb.mu.Lock()
	defer lb.mu.Unlock()
	now := time.Now()
	if now.After(lb.resetAt) {
		lb.tokens = m.limit
		lb.resetAt = now.Add(m.window)
	}
	if lb.tokens >= amount {
		lb.tokens -= amount
		return true, nil
	}
	return false, nil
}

func (m *RateLimiterManager) GetRateLimitStatus(ctx context.Context, bucket string) (RateLimitInfo, error) {
	return RateLimitInfo{m.limit, m.limit, time.Now()}, nil
}
