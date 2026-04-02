package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

type UltraPlan struct {
	ID           string
	MissionID    string
	Status       string // DELIBERATING, EXECUTING, COMPLETED, FAILED
	StateMachine map[string]interface{}
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

type UltraPlanManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode
}

func NewUltraPlanManager(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode) *UltraPlanManager {
	return &UltraPlanManager{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
	}
}

// CreatePlan initializes a new UltraPlan
func (upm *UltraPlanManager) CreatePlan(ctx context.Context, missionID string) (*UltraPlan, error) {
	id := generateID()

	query := `
		INSERT INTO swarm_ultra_plans (id, mission_id, status, state_machine, created_at, updated_at)
		VALUES ($1, $2, 'DELIBERATING', '{}', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	_, err := upm.db.Exec(ctx, query, id, missionID)
	if err != nil {
		return nil, fmt.Errorf("failed to create ultra plan: %w", err)
	}

	plan := &UltraPlan{
		ID:           id,
		MissionID:    missionID,
		Status:       "DELIBERATING",
		StateMachine: make(map[string]interface{}),
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}

	upm.broadcast(plan.ID, "CREATE", plan)
	return plan, nil
}

// UpdatePlanStatus updates the plan status and state machine json
func (upm *UltraPlanManager) UpdatePlanStatus(ctx context.Context, planID, status string, stateMachine map[string]interface{}) error {
	var smBytes []byte
	var err error
	if stateMachine != nil {
		smBytes, err = json.Marshal(stateMachine)
		if err != nil {
			return fmt.Errorf("failed to marshal state machine: %w", err)
		}
	} else {
		smBytes = []byte("{}")
	}

	query := `
		UPDATE swarm_ultra_plans
		SET status = $1, state_machine = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3
	`
	_, err = upm.db.Exec(ctx, query, status, string(smBytes), planID)
	if err != nil {
		return fmt.Errorf("failed to update ultra plan: %w", err)
	}

	// Fetch to broadcast correctly
	var missionID string
	err = upm.db.QueryRow(ctx, "SELECT mission_id FROM swarm_ultra_plans WHERE id = $1", planID).Scan(&missionID)
	if err == nil {
		upm.broadcast(planID, "UPDATE", &UltraPlan{
			ID:           planID,
			MissionID:    missionID,
			Status:       status,
			StateMachine: stateMachine,
		})
	}

	return nil
}

func (upm *UltraPlanManager) broadcast(planID, action string, plan *UltraPlan) {
	if upm.hub == nil {
		return
	}
	channel := "mesh:ultraplan:" + planID
	payload := map[string]interface{}{
		"action":     action,
		"plan_id":    plan.ID,
		"mission_id": plan.MissionID,
		"status":     plan.Status,
		"state":      plan.StateMachine,
	}
	// Emulate publish interface for custom channel
	// CentrifugeNode.Publish takes channel and JSON byte payload.
	dataBytes, _ := json.Marshal(payload)
	_ = upm.hub.Publish(context.Background(), channel, dataBytes)
}
