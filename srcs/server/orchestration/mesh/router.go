package mesh

import (
	"context"
	"fmt"
	"math/rand"

	"github.com/onehumancorp/mono/srcs/server/db/repositories"
)

// CapabilityRouter handles routing of tasks to agents based on their capabilities and availability.
type CapabilityRouter struct {
	mesh TeammateMesh
	repo repositories.MeshRepository
}

// NewCapabilityRouter creates a new CapabilityRouter.
func NewCapabilityRouter(mesh TeammateMesh, repo repositories.MeshRepository) *CapabilityRouter {
	return &CapabilityRouter{
		mesh: mesh,
		repo: repo,
	}
}

// RouteToCapability finds an active agent with the requested capability and dispatches the payload.
// It returns the ID of the selected agent.
func (r *CapabilityRouter) RouteToCapability(ctx context.Context, capability string, payload []byte) (string, error) {
	// 1. Retrieve active agents from the Teammate Mesh presence layer.
	activePresences, err := r.mesh.GetActiveAgents(ctx)
	if err != nil {
		return "", fmt.Errorf("failed to retrieve active agents from mesh: %w", err)
	}

	if len(activePresences) == 0 {
		return "", fmt.Errorf("no active agents available in the mesh")
	}

	// 2. Retrieve agents that possess the required skill/capability from the database.
	capableAgentIDs, err := r.repo.GetAgentsWithCapability(ctx, capability)
	if err != nil {
		return "", fmt.Errorf("failed to query capable agents from database: %w", err)
	}

	if len(capableAgentIDs) == 0 {
		return "", fmt.Errorf("no agents found with capability: %s", capability)
	}

	// 3. Find the intersection: agents that are both active and capable.
	activeMap := make(map[string]bool)
	for _, p := range activePresences {
		activeMap[p.AgentID] = true
	}

	var suitableAgents []string
	for _, id := range capableAgentIDs {
		if activeMap[id] {
			suitableAgents = append(suitableAgents, id)
		}
	}

	if len(suitableAgents) == 0 {
		return "", fmt.Errorf("no active agents currently possess the required capability: %s", capability)
	}

	// 4. Select a suitable agent.
	// For basic load balancing, we select one at random among those suitable.
	selectedAgentID := suitableAgents[rand.Intn(len(suitableAgents))]

	// 5. Dispatch the job via the Redis Teammate Mesh.
	// Convention: Agents listen on "agent:job:<agent_id>" for direct task assignments.
	topic := fmt.Sprintf("agent:job:%s", selectedAgentID)
	if err := r.mesh.Publish(ctx, topic, payload); err != nil {
		return "", fmt.Errorf("failed to dispatch job to agent %s via mesh: %w", selectedAgentID, err)
	}

	return selectedAgentID, nil
}
