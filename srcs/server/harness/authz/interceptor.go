package authz

import (
	"context"
	"fmt"
)

type CapabilityInterceptor struct {
	authorizer *CapabilityAuthorizer
}

func NewCapabilityInterceptor(authorizer *CapabilityAuthorizer) *CapabilityInterceptor {
	return &CapabilityInterceptor{
		authorizer: authorizer,
	}
}

// Intercept ensures the requested tool capability is authorized before execution.
func (i *CapabilityInterceptor) Intercept(ctx context.Context, sessionID string, capability string, execute func() error) error {
	if err := i.authorizer.Authorize(ctx, sessionID, capability); err != nil {
		return fmt.Errorf("authorization failed: %w", err)
	}
	return execute()
}
