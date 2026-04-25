package harness

import (
	"context"
	"strings"

	"github.com/onehumancorp/mono/src/server/harness/authz"
)

// SessionContextKey is the context key for the active session ID.
type SessionContextKey struct{}

// AuthorizingHarness wraps an AgentHarness to provide capability-based authorization.
type AuthorizingHarness struct {
	target     AgentHarness
	authorizer authz.CapabilityAuthorizer
}

// NewAuthorizingHarness creates a new AuthorizingHarness.
func NewAuthorizingHarness(target AgentHarness, authorizer authz.CapabilityAuthorizer) *AuthorizingHarness {
	return &AuthorizingHarness{
		target:     target,
		authorizer: authorizer,
	}
}

// Execute validates the capability before delegating to the target harness.
func (a *AuthorizingHarness) Execute(ctx context.Context, command string) (Result, error) {
	if a.authorizer != nil {
		sessionID, ok := ctx.Value(SessionContextKey{}).(string)
		if ok && sessionID != "" {
			// Extract capability from command (simplified mapping for demonstration)
			capability := extractCapabilityFromCommand(command)
			if capability != "" {
				if err := a.authorizer.Authorize(ctx, sessionID, capability); err != nil {
					return Result{}, err
				}
			}
		}
	}
	return a.target.Execute(ctx, command)
}

// extractCapabilityFromCommand provides a basic mapping of commands to capabilities.
func extractCapabilityFromCommand(command string) string {
	cmd := strings.TrimSpace(command)
	if strings.HasPrefix(cmd, "bash") || strings.HasPrefix(cmd, "sh") {
		return "bash"
	}
	if strings.HasPrefix(cmd, "curl") || strings.HasPrefix(cmd, "wget") {
		return "network"
	}
	if strings.HasPrefix(cmd, "playwright") || strings.HasPrefix(cmd, "browser") {
		return "browser"
	}
	if strings.HasPrefix(cmd, "cat") || strings.HasPrefix(cmd, "ls") || strings.HasPrefix(cmd, "grep") {
		return "read"
	}
	return "execute" // default fallback
}
