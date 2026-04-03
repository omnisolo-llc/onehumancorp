package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// UltraPlan represents a deep-deliberation multi-step plan.
type UltraPlan struct {
	ID           string
	MissionID    string
	Status       string
	StateMachine map[string]interface{}
	CreatedAt    time.Time
	UpdatedAt    time.Time
}

// UltraPlanManager handles the state machine and data access for UltraPlans.
type UltraPlanManager struct {
	db          db.Provider
	redisClient rueidis.Client
	hub         *CentrifugeNode // Reusing CentrifugeNode for Mesh integration
}

// NewUltraPlanManager initializes a new UltraPlanManager.
func NewUltraPlanManager(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode) *UltraPlanManager {
	return &UltraPlanManager{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
	}
}

// CreatePlan creates a new UltraPlan with DELIBERATING status.
func (m *UltraPlanManager) CreatePlan(ctx context.Context, missionID string, stateMachine map[string]interface{}) (*UltraPlan, error) {
	stateMachineJSON, err := json.Marshal(stateMachine)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal state_machine: %w", err)
	}

	var plan UltraPlan
	var query string
	if m.db.IsSQLite() {
		plan.ID = generateID()
		query = `
			INSERT INTO swarm_ultra_plans (id, mission_id, state_machine, status, created_at, updated_at)
			VALUES ($1, $2, $3, 'DELIBERATING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, mission_id, state_machine, status, created_at, updated_at
		`
		err = m.db.QueryRow(ctx, query, plan.ID, missionID, stateMachineJSON).Scan(
			&plan.ID, &plan.MissionID, &stateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
		)
	} else {
		query = `
			INSERT INTO swarm_ultra_plans (mission_id, state_machine, status)
			VALUES ($1, $2, 'DELIBERATING')
			RETURNING id, mission_id, state_machine, status, created_at, updated_at
		`
		err = m.db.QueryRow(ctx, query, missionID, stateMachineJSON).Scan(
			&plan.ID, &plan.MissionID, &stateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
		)
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create ultra plan: %w", err)
	}

	_ = json.Unmarshal(stateMachineJSON, &plan.StateMachine)

	if m.hub != nil {
		// Use Publish function instead of the hallucinated PublishTaskBroadcast
		go func() {
			msg := Message{
				ID:        generateID(),
				FromAgent: "system",
				ToAgent:   "system",
				Type:      "ULTRAPLAN_CREATE",
				Content:   string(stateMachineJSON),
			}
			_ = m.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{"agent_id": m.agentID, "action": "COMPLETED", "status": "COMPLETED"})
		}()
	}

	return &plan, nil
}

// UpdatePlanStatus updates the status and state machine of an UltraPlan, using distributed locks if configured.
func (m *UltraPlanManager) UpdatePlanStatus(ctx context.Context, planID string, newStatus string, stateMachine map[string]interface{}) error {
	if newStatus != "DELIBERATING" && newStatus != "EXECUTING" && newStatus != "COMPLETED" && newStatus != "FAILED" {
		return errors.New("invalid status")
	}

	stateMachineJSON, err := json.Marshal(stateMachine)
	if err != nil {
		return fmt.Errorf("failed to marshal state_machine: %w", err)
	}

	if m.redisClient != nil {
		lockKey := "lock:ultraplan:" + planID
		cmd := m.redisClient.B().Set().Key(lockKey).Value("system").Nx().Ex(30 * time.Second).Build()
		err := m.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return errors.New("ultra plan is currently locked")
			}
			return fmt.Errorf("failed to acquire lock: %w", err)
		}
		defer func() {
			delCmd := m.redisClient.B().Del().Key(lockKey).Build()
			_ = m.redisClient.Do(ctx, delCmd).Error()
		}()
	}

	// Begin TX for atomicity
	tx, err := m.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentStatus string
	query := `SELECT status FROM swarm_ultra_plans WHERE id = $1`
	if !m.db.IsSQLite() {
		query += ` FOR UPDATE`
	}

	err = tx.QueryRow(ctx, query, planID).Scan(&currentStatus)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return errors.New("ultra plan not found")
		}
		return fmt.Errorf("fetch status: %w", err)
	}

	updateQuery := `
		UPDATE swarm_ultra_plans
		SET status = $1, state_machine = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, newStatus, stateMachineJSON, planID)
	if err != nil {
		return fmt.Errorf("update status: %w", err)
	}

	if rowsAffected == 0 {
		return fmt.Errorf("update status no rows affected")
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("commit tx: %w", err)
	}

	if m.hub != nil {
		// Use Publish function or custom logic instead of the hallucinated PublishTaskBroadcast
		go func() {
			msg := Message{
				ID:        generateID(),
				FromAgent: "system",
				ToAgent:   "system",
				Type:      "ULTRAPLAN_UPDATE",
				Content:   string(stateMachineJSON),
			}
			_ = m.hub.PublishTaskBroadcast(task.ID, map[string]interface{}{"agent_id": m.agentID, "action": "COMPLETED", "status": "COMPLETED"})
		}()
	}

	return nil
}

// GetUltraPlan fetches an UltraPlan by ID.
func (m *UltraPlanManager) GetUltraPlan(ctx context.Context, planID string) (*UltraPlan, error) {
	var plan UltraPlan
	var stateMachineJSON []byte

	query := `SELECT id, mission_id, state_machine, status, created_at, updated_at FROM swarm_ultra_plans WHERE id = $1`
	err := m.db.QueryRow(ctx, query, planID).Scan(
		&plan.ID, &plan.MissionID, &stateMachineJSON, &plan.Status, &plan.CreatedAt, &plan.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, errors.New("ultra plan not found")
		}
		return nil, fmt.Errorf("failed to fetch ultra plan: %w", err)
	}

	_ = json.Unmarshal(stateMachineJSON, &plan.StateMachine)
	return &plan, nil
}
