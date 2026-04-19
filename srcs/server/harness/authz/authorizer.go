package authz

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"

	"github.com/onehumancorp/mono/srcs/server/db"
)

var ErrCapabilityDenied = errors.New("capability denied")

type CapabilityAuthorizer struct {
	provider db.Provider
}

func NewCapabilityAuthorizer(provider db.Provider) *CapabilityAuthorizer {
	return &CapabilityAuthorizer{
		provider: provider,
	}
}

func (a *CapabilityAuthorizer) Authorize(ctx context.Context, sessionID string, capability string) error {
	var capabilitiesJSON []byte
	var qry string
	if a.provider.IsSQLite() {
		qry = "SELECT capabilities FROM agent_session_data WHERE session_id = ?"
	} else {
		qry = "SELECT capabilities FROM agent_session_data WHERE session_id = $1"
	}
	err := a.provider.QueryRow(ctx, qry, sessionID).Scan(&capabilitiesJSON)
	if err != nil {
		return fmt.Errorf("failed to get session capabilities: %w", err)
	}

	var caps []string
	if len(capabilitiesJSON) > 0 {
		if err := json.Unmarshal(capabilitiesJSON, &caps); err != nil {
			return fmt.Errorf("failed to parse session capabilities: %w", err)
		}
	}

	for _, cap := range caps {
		if cap == capability || cap == "*" {
			return nil
		}
	}

	// TODO: Log violation using Violation Telemetry Engine
	return fmt.Errorf("%w: requested %s", ErrCapabilityDenied, capability)
}
