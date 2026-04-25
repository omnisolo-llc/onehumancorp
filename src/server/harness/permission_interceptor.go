package harness

import (
	"context"
	"fmt"
	"io"
)

// PermissionInterceptor wraps a PolicyExecutor and intercepts execution calls to enforce
// authorization via a BridgeTransport.
type PermissionInterceptor struct {
	target PolicyExecutor
	bridge BridgeTransport
}

// NewPermissionInterceptor creates a new PermissionInterceptor.
func NewPermissionInterceptor(target PolicyExecutor, bridge BridgeTransport) *PermissionInterceptor {
	return &PermissionInterceptor{
		target: target,
		bridge: bridge,
	}
}

func (p *PermissionInterceptor) checkPermission(ctx context.Context, command string) error {
	req := PermissionRequest{
		Command: command,
	}
	resp, err := p.bridge.RequestPermission(ctx, req)
	if err != nil {
		return fmt.Errorf("failed to request permission: %w", err)
	}
	if !resp.Authorized {
		return fmt.Errorf("execution denied by bridge: %s", resp.Reason)
	}
	return nil
}

// Execute wraps the underlying PolicyExecutor's Execute method with a permission check.
func (p *PermissionInterceptor) Execute(ctx context.Context, command string) (Result, error) {
	if err := p.checkPermission(ctx, command); err != nil {
		return Result{}, err
	}
	return p.target.Execute(ctx, command)
}

// ExecuteWithPolicy wraps the underlying PolicyExecutor's ExecuteWithPolicy method with a permission check.
func (p *PermissionInterceptor) ExecuteWithPolicy(ctx context.Context, command string, policy *Policy) (Result, error) {
	if err := p.checkPermission(ctx, command); err != nil {
		return Result{}, err
	}
	return p.target.ExecuteWithPolicy(ctx, command, policy)
}

// ExecuteStream wraps the underlying PolicyExecutor's ExecuteStream method with a permission check.
func (p *PermissionInterceptor) ExecuteStream(ctx context.Context, command string, policy *Policy, stdin io.Reader, stdout, stderr io.Writer) (Result, error) {
	if err := p.checkPermission(ctx, command); err != nil {
		return Result{}, err
	}
	return p.target.ExecuteStream(ctx, command, policy, stdin, stdout, stderr)
}
