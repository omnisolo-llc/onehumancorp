package mesh

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"log"

	"github.com/go-redis/redis/v8"
	_ "github.com/lib/pq"
)

// AgentProfile represents an agent in Postgres
type AgentProfile struct {
	ID        string
	Name      string
	Skills    []string // E.g. ["support", "sales"]
	Status    string   // E.g. "available", "busy"
}

// Router handles capability-based routing
type Router struct {
	db    *sql.DB
	redis *redis.Client
}

// NewRouter creates a new mesh Router
func NewRouter(db *sql.DB, redisClient *redis.Client) *Router {
	return &Router{
		db:    db,
		redis: redisClient,
	}
}

// RouteJob finds the best available agent with the required skill and dispatches the job via Redis
func (r *Router) RouteJob(ctx context.Context, jobID string, requiredSkill string, payload map[string]interface{}) error {
	// 1. Find available agents with the required skill in Postgres

	// Query string utilizing ANY operator for array elements matching in postgres.
	query := `
		SELECT id
		FROM agent_profiles
		WHERE status = 'available' AND $1 = ANY(skills)
		LIMIT 1;
	`

	var agentID string
	err := r.db.QueryRowContext(ctx, query, requiredSkill).Scan(&agentID)

	if err != nil {
		if err == sql.ErrNoRows {
			return fmt.Errorf("no available agents found with skill: %s", requiredSkill)
		}
		return fmt.Errorf("failed to query available agents: %w", err)
	}

	// 2. Prepare the payload
	// KAIROS OHC-SIP compliance payload
	meshPayload := map[string]interface{}{
		"agent_id": agentID,
		"action":   "TaskAssigned",
		"status":   "pending",
		"payload": map[string]interface{}{
			"job_id":        jobID,
			"required_skill": requiredSkill,
			"data":          payload,
		},
	}

	jsonPayload, err := json.Marshal(meshPayload)
	if err != nil {
		return fmt.Errorf("failed to marshal mesh payload: %w", err)
	}

	// 3. Dispatch the message via Redis Teammate Mesh
	channel := fmt.Sprintf("mesh:agent:%s", agentID)

	cmd := r.redis.Publish(ctx, channel, jsonPayload)
	if cmd.Err() != nil {
		return fmt.Errorf("failed to publish to redis mesh: %w", cmd.Err())
	}

	log.Printf("Successfully routed job %s to agent %s via channel %s", jobID, agentID, channel)
	return nil
}
