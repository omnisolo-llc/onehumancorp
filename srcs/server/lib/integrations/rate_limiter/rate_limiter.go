package rate_limiter

import (
	"context"
	"os"
	"time"
	"sync"
	"github.com/go-redis/redis/v8"
	"fmt"
)

type RateLimitInfo struct {
	IsAllowed          bool
	SoftLimitReached   bool
	UserMessage        string
}

type RateLimiterProvider interface {
	RequestTokens(ctx context.Context, bucket string, amount int) (bool, error)
	GetRateLimitStatus(ctx context.Context, bucket string) (RateLimitInfo, error)
}

type RateLimiterManager struct {
	provider RateLimiterProvider
}

type cloudRateLimiter struct {
	redisClient *redis.Client
}

type standaloneRateLimiter struct {
	localBuckets map[string]*localBucket
	mu           sync.Mutex
}

type localBucket struct {
	actionsUsed int
	monthKey    string
	mu         sync.Mutex
}

func NewRateLimiterManager(redisURL string) *RateLimiterManager {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		var client *redis.Client
		if redisURL != "" {
			opt, _ := redis.ParseURL(redisURL)
			client = redis.NewClient(opt)
		}
		return &RateLimiterManager{
			provider: &cloudRateLimiter{redisClient: client},
		}
	}

	return &RateLimiterManager{
		provider: &standaloneRateLimiter{
			localBuckets: make(map[string]*localBucket),
		},
	}
}

func (m *RateLimiterManager) CallTool(ctx context.Context, toolName string, args map[string]interface{}) (interface{}, error) {
	if toolName == "RequestTokens" {
		bucket, _ := args["bucket"].(string)
		amountFloat, _ := args["amount"].(float64)
		amount := int(amountFloat)
		return m.RequestTokens(ctx, bucket, amount)
	} else if toolName == "GetRateLimitStatus" {
		bucket, _ := args["bucket"].(string)
		return m.GetRateLimitStatus(ctx, bucket)
	}
	return nil, fmt.Errorf("unknown tool: %s", toolName)
}

func (m *RateLimiterManager) RequestTokens(ctx context.Context, bucket string, amount int) (bool, error) {
	return m.provider.RequestTokens(ctx, bucket, amount)
}

func (m *RateLimiterManager) GetRateLimitStatus(ctx context.Context, bucket string) (RateLimitInfo, error) {
	return m.provider.GetRateLimitStatus(ctx, bucket)
}

func (c *cloudRateLimiter) RequestTokens(ctx context.Context, bucket string, amount int) (bool, error) {
	if c.redisClient == nil {
		return true, nil // Soft fail if Redis isn't configured
	}

	monthKey := time.Now().Format("2006-01")
	key := fmt.Sprintf("rate_limit:%s:%s", bucket, monthKey)

	val, err := c.redisClient.IncrBy(ctx, key, int64(amount)).Result()
	if err != nil {
		return false, err
	}

	if val == int64(amount) {
		c.redisClient.Expire(ctx, key, 60*24*time.Hour)
	}

	return true, nil
}

func (c *cloudRateLimiter) GetRateLimitStatus(ctx context.Context, bucket string) (RateLimitInfo, error) {
	if c.redisClient == nil {
		return RateLimitInfo{IsAllowed: true, SoftLimitReached: false}, nil
	}

	monthKey := time.Now().Format("2006-01")
	key := fmt.Sprintf("rate_limit:%s:%s", bucket, monthKey)

	val, err := c.redisClient.Get(ctx, key).Int()

	if err == redis.Nil {
		return RateLimitInfo{IsAllowed: true, SoftLimitReached: false}, nil
	} else if err != nil {
		return RateLimitInfo{IsAllowed: true, SoftLimitReached: false}, err
	}

	limit := 100 // default fallback
	softLimit := val >= limit

	return RateLimitInfo{
		IsAllowed:        true,
		SoftLimitReached: softLimit,
		UserMessage:      "You've reached your tier limit. Consider upgrading to keep your business running smoothly!",
	}, nil
}

func (s *standaloneRateLimiter) RequestTokens(ctx context.Context, bucket string, amount int) (bool, error) {
	s.mu.Lock()
	lb, exists := s.localBuckets[bucket]
	monthKey := time.Now().Format("2006-01")

	if !exists || lb.monthKey != monthKey {
		lb = &localBucket{
			actionsUsed: 0,
			monthKey: monthKey,
		}
		s.localBuckets[bucket] = lb
	}
	s.mu.Unlock()

	lb.mu.Lock()
	defer lb.mu.Unlock()

	lb.actionsUsed += amount

	return true, nil
}

func (s *standaloneRateLimiter) GetRateLimitStatus(ctx context.Context, bucket string) (RateLimitInfo, error) {
	s.mu.Lock()
	lb, exists := s.localBuckets[bucket]
	s.mu.Unlock()

	if !exists {
		return RateLimitInfo{IsAllowed: true, SoftLimitReached: false}, nil
	}

	lb.mu.Lock()
	defer lb.mu.Unlock()

	monthKey := time.Now().Format("2006-01")
	if lb.monthKey != monthKey {
		lb.actionsUsed = 0
		lb.monthKey = monthKey
	}

	limit := 100 // default fallback
	softLimit := lb.actionsUsed >= limit

	return RateLimitInfo{
		IsAllowed:        true,
		SoftLimitReached: softLimit,
		UserMessage:      "You've reached your tier limit. Consider upgrading to keep your business running smoothly!",
	}, nil
}
