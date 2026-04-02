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

type UltraPlanStatus string

const (
	PlanStatusDeliberating UltraPlanStatus = "DELIBERATING"
	PlanStatusExecuting    UltraPlanStatus = "EXECUTING"
	PlanStatusCompleted    UltraPlanStatus = "COMPLETED"
	PlanStatusFailed       UltraPlanStatus = "FAILED"
)

type UltraPlan struct {
	ID           string
	MissionID    string
	Status       UltraPlanStatus
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

func (upm *UltraPlanManager) releaseRedisLock(ctx context.Context, lockKey string, lockValue string) {
	if upm.redisClient != nil {
		script := rueidis.NewLuaScript("if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end")
		upm.redisClient.Do(ctx, script.Exec(upm.redisClient, []string{lockKey}, []string{lockValue}))
	}
}

func (upm *UltraPlanManager) CreatePlan(ctx context.Context, missionID string) (*UltraPlan, error) {
	id := generateID()
	plan := &UltraPlan{
		ID:           id,
		MissionID:    missionID,
		Status:       PlanStatusDeliberating,
		StateMachine: make(map[string]interface{}),
		CreatedAt:    time.Now(),
		UpdatedAt:    time.Now(),
	}

	stateMachineBytes, err := json.Marshal(plan.StateMachine)
	if err != nil {
		return nil, fmt.Errorf("failed to marshal state machine: %w", err)
	}

	query := `
		INSERT INTO swarm_ultra_plans (id, mission_id, status, state_machine, created_at, updated_at)
		VALUES ($1, $2, $3, $4, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	_, err = upm.db.Exec(ctx, query, plan.ID, plan.MissionID, plan.Status, string(stateMachineBytes))
	if err != nil {
		return nil, fmt.Errorf("failed to create ultra plan: %w", err)
	}

	if upm.hub != nil {
		upm.hub.PublishUltraPlanBroadcast(plan.ID, map[string]interface{}{
			"action":     "CREATE",
			"plan_id":    plan.ID,
			"mission_id": plan.MissionID,
			"status":     string(plan.Status),
			"state":      plan.StateMachine,
		})
	}

	return plan, nil
}

func (upm *UltraPlanManager) TransitionPlan(ctx context.Context, planID string, newStatus UltraPlanStatus) error {
	lockKey := "lock:ultraplan:" + planID
	lockValue := generateID()
	if upm.redisClient != nil {
		cmd := upm.redisClient.B().Set().Key(lockKey).Value(lockValue).Nx().Ex(10 * time.Second).Build()
		if err := upm.redisClient.Do(ctx, cmd).Error(); err != nil {
			if rueidis.IsRedisNil(err) {
				return fmt.Errorf("could not acquire lock for plan transition")
			}
			return fmt.Errorf("failed to acquire distributed lock: %w", err)
		}
		defer upm.releaseRedisLock(ctx, lockKey, lockValue)
	}

	tx, err := upm.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentStatus, missionID string
	var stateMachineStr string
	query := "SELECT status, mission_id, state_machine FROM swarm_ultra_plans WHERE id = $1"
	if !upm.db.IsSQLite() {
		query += " FOR UPDATE"
	}
	err = tx.QueryRow(ctx, query, planID).Scan(&currentStatus, &missionID, &stateMachineStr)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("plan not found")
		}
		return fmt.Errorf("failed to fetch plan status: %w", err)
	}

	updateQuery := "UPDATE swarm_ultra_plans SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
	_, err = tx.Exec(ctx, updateQuery, string(newStatus), planID)
	if err != nil {
		return fmt.Errorf("failed to update plan status: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if upm.hub != nil {
		var stateMachine map[string]interface{}
		_ = json.Unmarshal([]byte(stateMachineStr), &stateMachine)
		upm.hub.PublishUltraPlanBroadcast(planID, map[string]interface{}{
			"action":     "TRANSITION",
			"plan_id":    planID,
			"mission_id": missionID,
			"status":     string(newStatus),
			"state":      stateMachine,
		})
	}

	return nil
}

func (upm *UltraPlanManager) UpdateStateMachine(ctx context.Context, planID string, stateMachine map[string]interface{}) error {
	lockKey := "lock:ultraplan:" + planID
	lockValue := generateID()
	if upm.redisClient != nil {
		cmd := upm.redisClient.B().Set().Key(lockKey).Value(lockValue).Nx().Ex(10 * time.Second).Build()
		if err := upm.redisClient.Do(ctx, cmd).Error(); err != nil {
			if rueidis.IsRedisNil(err) {
				return fmt.Errorf("could not acquire lock for plan update")
			}
			return fmt.Errorf("failed to acquire distributed lock: %w", err)
		}
		defer upm.releaseRedisLock(ctx, lockKey, lockValue)
	}

	tx, err := upm.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var currentStatus, missionID string
	query := "SELECT status, mission_id FROM swarm_ultra_plans WHERE id = $1"
	if !upm.db.IsSQLite() {
		query += " FOR UPDATE"
	}
	err = tx.QueryRow(ctx, query, planID).Scan(&currentStatus, &missionID)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return fmt.Errorf("plan not found")
		}
		return fmt.Errorf("failed to fetch plan status: %w", err)
	}

	stateMachineBytes, err := json.Marshal(stateMachine)
	if err != nil {
		return fmt.Errorf("failed to marshal state machine: %w", err)
	}

	updateQuery := "UPDATE swarm_ultra_plans SET state_machine = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
	_, err = tx.Exec(ctx, updateQuery, string(stateMachineBytes), planID)
	if err != nil {
		return fmt.Errorf("failed to update state machine: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	if upm.hub != nil {
		upm.hub.PublishUltraPlanBroadcast(planID, map[string]interface{}{
			"action":     "UPDATE_STATE",
			"plan_id":    planID,
			"mission_id": missionID,
			"status":     currentStatus,
			"state":      stateMachine,
		})
	}

	return nil
}

func (upm *UltraPlanManager) GetPlan(ctx context.Context, planID string) (*UltraPlan, error) {
	query := "SELECT id, mission_id, status, state_machine, created_at, updated_at FROM swarm_ultra_plans WHERE id = $1"
	var plan UltraPlan
	var stateMachineStr sql.NullString
	var status string
	err := upm.db.QueryRow(ctx, query, planID).Scan(
		&plan.ID, &plan.MissionID, &status, &stateMachineStr, &plan.CreatedAt, &plan.UpdatedAt,
	)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to fetch plan: %w", err)
	}

	plan.Status = UltraPlanStatus(status)
	if stateMachineStr.Valid {
		err = json.Unmarshal([]byte(stateMachineStr.String), &plan.StateMachine)
		if err != nil {
			return nil, fmt.Errorf("failed to unmarshal state machine: %w", err)
		}
	} else {
		plan.StateMachine = make(map[string]interface{})
	}

	return &plan, nil
}
