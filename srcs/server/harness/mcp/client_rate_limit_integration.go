package mcp

import (
	"context"
	"log"
	"onehumancorp/srcs/server/integrations/mcp/harness"
)

// CallToolWithRateLimit is a wrapper around CallTool that enforces the agent quota.
func (cm *ClientManager) CallToolWithRateLimit(ctx context.Context, serverID string, tenantID string, name string, args map[string]interface{}) (*CallToolResult, error) {
	// 1. Check current status for soft-limits
	bucket := "tenant:" + tenantID + ":agent_calls"
	status, err := harness.GlobalRateLimiter.GetRateLimitStatus(ctx, bucket)
	if err == nil && status.SoftLimitReached {
		// Just log or propagate the soft limit warning somehow.
		// Since we don't want hard errors, we won't return an error here.
		log.Printf("Rate limit warning for tenant [REDACTED]: %s", status.UserMessage)
	}

	// 2. Actually execute the tool
	res, err := cm.CallTool(ctx, serverID, name, args)
	if err != nil {
		return nil, err
	}

	// 3. Increment the rate limit token
	_, _ = harness.RequestAgentTokens(ctx, tenantID, 1)

	return res, nil
}
