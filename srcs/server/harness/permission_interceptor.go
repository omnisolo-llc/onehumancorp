package harness

import (
	"context"
	"fmt"

	"github.com/google/uuid"
)

// PermissionInterceptor wraps an AgentHarness to require authorization before execution.
type PermissionInterceptor struct {
	transport BridgeTransport
}

// NewPermissionInterceptor creates a new PermissionInterceptor.
func NewPermissionInterceptor(transport BridgeTransport) *PermissionInterceptor {
	return &PermissionInterceptor{
		transport: transport,
	}
}

// CheckPermission blocks until permission is granted by the cloud for the given command.
func (p *PermissionInterceptor) CheckPermission(ctx context.Context, command string) error {
	reqID := uuid.New().String()
	req := PermissionRequest{
		RequestID: reqID,
		Command:   command,
	}

	if err := p.transport.SendRequest(req); err != nil {
		return fmt.Errorf("failed to send permission request: %w", err)
	}

	resp, err := p.transport.ReceiveResponse(ctx, reqID)
	if err != nil {
		return fmt.Errorf("failed to receive permission response: %w", err)
	}

	if !resp.Allowed {
		errMsg := "permission denied by bridge"
		if resp.ErrorMsg != "" {
			errMsg = fmt.Sprintf("%s: %s", errMsg, resp.ErrorMsg)
		}
		return fmt.Errorf(errMsg)
	}

	return nil
}
