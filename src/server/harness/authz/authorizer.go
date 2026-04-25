package authz

import (
	"context"
	"errors"
	"fmt"
	"github.com/onehumancorp/mono/src/server/db"

	"encoding/json"
)

var ErrCapabilityDenied = errors.New("capability denied")

// CapabilityAuthorizer defines the interface for authorizing capabilities.
type CapabilityAuthorizer interface {
	Authorize(ctx context.Context, sessionID string, capability string) error
}

type authorizer struct {
	provider db.Provider
}

// NewAuthorizer creates a new CapabilityAuthorizer.
func NewAuthorizer(provider db.Provider) CapabilityAuthorizer {
	return &authorizer{provider: provider}
}

// Authorize checks if the given session has the required capability.
func (a *authorizer) Authorize(ctx context.Context, sessionID string, capability string) error {
	var capsJSON []byte
	err := a.provider.QueryRow(ctx, "SELECT capabilities FROM agent_session_data WHERE session_id = $1", sessionID).Scan(&capsJSON)
	if err != nil {
		return fmt.Errorf("failed to fetch session capabilities: %w", err)
	}

	if len(capsJSON) == 0 {
		return ErrCapabilityDenied
	}

	var capabilities []string
	if err := json.Unmarshal(capsJSON, &capabilities); err != nil {
		return fmt.Errorf("failed to unmarshal capabilities: %w", err)
	}

	for _, c := range capabilities {
		if c == capability {
			return nil
		}
	}

	// TODO: log violation using Violation Telemetry Engine
	return ErrCapabilityDenied
}
