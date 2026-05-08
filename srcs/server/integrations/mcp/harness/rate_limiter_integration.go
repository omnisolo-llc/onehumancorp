package harness

import (
	"context"
	"onehumancorp/srcs/server/lib/integrations/rate_limiter"
)

// Global rate limiter manager used by the harness
var GlobalRateLimiter *rate_limiter.RateLimiterManager

func init() {
	// Initialize with empty string for soft-fail or local mode
	GlobalRateLimiter = rate_limiter.NewRateLimiterManager("")
}

// RequestAgentTokens is a helper to request tokens for an agent
func RequestAgentTokens(ctx context.Context, tenantID string, amount int) (bool, error) {
	bucket := "tenant:" + tenantID + ":agent_calls"
	return GlobalRateLimiter.RequestTokens(ctx, bucket, amount)
}
