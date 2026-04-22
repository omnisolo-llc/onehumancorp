package mesh

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/db/models"
)

type CapabilityRouter struct {
	db   db.Provider
	mesh TeammateMesh
}

func NewCapabilityRouter(dbProvider db.Provider, mesh TeammateMesh) *CapabilityRouter {
	return &CapabilityRouter{
		db:   dbProvider,
		mesh: mesh,
	}
}

func (r *CapabilityRouter) RouteJob(ctx context.Context, requiredSkill string, payload []byte) error {
	// 1. Get active agents from mesh
	activeAgents, err := r.mesh.GetActiveAgents(ctx)
	if err != nil {
		return fmt.Errorf("failed to get active agents: %w", err)
	}

	// 2. Query DB to find agents with the required skill
	// Agent capability is in agent_session_data
	var bestAgentID string

	for _, activeAgent := range activeAgents {
		// Only consider agents that are IDLE/Available
		if activeAgent.Status != "IDLE" {
			continue
		}

		session, err := r.getAgentSession(ctx, activeAgent.AgentID)
		if err != nil {
			slog.Warn("Failed to get agent session", "agent_id", activeAgent.AgentID, "error", err)
			continue
		}

		// Check if the agent has the required skill
		if hasCapability(session.Capabilities, requiredSkill) {
			bestAgentID = activeAgent.AgentID
			break
		}
	}

	if bestAgentID == "" {
		return fmt.Errorf("no available agent with capability: %s", requiredSkill)
	}

	// 3. Dispatch the job to the best agent via mesh
	topic := "agent:" + bestAgentID
	err = r.mesh.Publish(ctx, topic, payload)
	if err != nil {
		return fmt.Errorf("failed to dispatch job to agent %s: %w", bestAgentID, err)
	}

	slog.Info("Successfully routed job", "agent_id", bestAgentID, "capability", requiredSkill)
	return nil
}

func (r *CapabilityRouter) getAgentSession(ctx context.Context, agentID string) (*models.Session, error) {
	query := `SELECT session_id, agent_id, context_data, capabilities, created_at, last_accessed FROM agent_session_data WHERE agent_id = $1 ORDER BY last_accessed DESC LIMIT 1`

	var session models.Session
	var capabilitiesBytes []byte

	err := r.db.QueryRow(ctx, query, agentID).Scan(
		&session.SessionID,
		&session.AgentID,
		&session.ContextData,
		&capabilitiesBytes,
		&session.CreatedAt,
		&session.LastAccessed,
	)

	if err != nil {
		return nil, err
	}

	if len(capabilitiesBytes) > 0 {
		err = json.Unmarshal(capabilitiesBytes, &session.Capabilities)
		if err != nil {
			return nil, fmt.Errorf("failed to unmarshal capabilities: %w", err)
		}
	}

	return &session, nil
}

func hasCapability(capabilities []string, skill string) bool {
	for _, cap := range capabilities {
		if cap == skill {
			return true
		}
	}
	return false
}
